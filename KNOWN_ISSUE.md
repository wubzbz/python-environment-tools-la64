# Known Issues

## 1. `is_dir()` / `create_dir_all` failures on musl loongarch64

- **Status**: Workaround applied — see `crates/pet-fs/src/workaround.rs`

### Symptom

`Path::is_dir()` returns `false` for directories created under `TempDir` or
for symlink targets. `std::fs::create_dir_all()` returns `EEXIST` on
already-existing directories.  This causes `pet::find::tests` to fail on
musl:

- `test_regular_directory_is_detected`
- `test_symlinked_directory_is_detected`
- `test_symlinked_venv_in_envs_directory`
- `test_symlink_path_is_preserved_not_resolved`

The gnu target passes all tests.  This issue is **specific to loongarch64** —
musl on x86_64 and aarch64 handles `stat`/`fstatat` correctly.

### Root Cause

`libc::stat` layout differs between the musl and gnu ABIs on loongarch64,
confirmed by `stat_dump` example in `test-musl/`:

| | GNU | MUSL |
|---|---|---|
| `sizeof(libc::stat)` | **128** bytes | **208** bytes |
| `st_dev` offset | 0 | 0 |
| `st_ino` offset | 8 | 24 |
| `st_mode` offset | 16 | 32 |
| `st_size` offset | 48 | 64 |
| `st_blksize` offset | 56 | 128 |

The kernel uses a layout closer to the GNU definition.  musl's `stat()`
/ `fstatat()` wrapper on loongarch64 appears to misinterpret kernel
`struct stat` fields — the outcome is that `st_mode` is unreliable (often
reported as 0), so `Path::is_dir()` (which calls `fstatat`) cannot
correctly determine file type.  The same underlying mismatch causes
`mkdir` to report `EEXIST` on already-existing directories.

Because `openat(O_DIRECTORY)` talks directly to the kernel without going
through the broken `stat` wrapper, it works correctly and serves as the
basis for the workaround.

### Workaround

`crates/pet-fs/src/workaround.rs` provides two replacement functions gated
behind `#[cfg(loongarch64_musl_workaround)]`:

- **`is_dir(path)`** — uses `openat(AT_FDCWD, path, O_RDONLY | O_DIRECTORY)`
  followed by `close(fd)`.  On all other targets, delegates to
  `Path::is_dir()`.

- **`create_dir_all(path)`** — recursively creates directories via
  `libc::mkdir`.  `EEXIST` is tolerated after an `is_dir()` sanity check.
  On all other targets, delegates to `std::fs::create_dir_all()`.

Activation: `.cargo/config.toml` → `[target.loongarch64-unknown-linux-musl]`
→ `rustflags = ["--cfg", "loongarch64_musl_workaround"]`.

Ten call sites across `pet` (`find.rs`, `jsonrpc.rs`, `lib.rs`,
`resolve.rs`) and `pet-python-utils` (`fs_cache.rs`) now use
`pet_fs::workaround::is_dir()` / `create_dir_all()` instead of the
standard library equivalents.

### Reverting

When upstream musl on loongarch64 is fixed, remove the two rustflags
lines from `.cargo/config.toml` and delete the `#[cfg(…)]` branches in
`workaround.rs`.  The default (non-musl) code paths are already the
standard library calls.

---

## 2. Test fixture `history` file overwritten with local paths

- **Status**: Mitigated

### Symptom

After running `pet-conda` tests, the file
`crates/pet-conda/tests/unix/conda_hist/env_python_3/conda-meta/history`
has its `# cmd:` lines replaced with absolute paths from the local machine,
causing spurious `git status` modifications.

### Solution

Run the following once after cloning the repository:

```bash
cd python-environment-tools-la64
git update-index --skip-worktree \
  crates/pet-conda/tests/unix/conda_hist/env_python_3/conda-meta/history
```

This file is regenerated from `history_template` at test time and should not be
tracked for local changes.

---

## 3. `create_dir_all` returns `EEXIST` on musl (non-fatal log noise)

- **Status**: Resolved — covered by workaround in issue #1.

The `pet-fs::workaround::create_dir_all()` replacement silently tolerates
`EEXIST` after verifying the path is indeed a directory.  The error-level
log line in `fs_cache.rs` will no longer appear on musl.

---

## 4. `crt-static` breaks proc-macro compilation on loongarch64 host

- **Status**: Resolved

### Symptom

On a loongarch64 host, `cargo check --workspace` (and therefore
`rust-analyzer` flycheck) fails with:

```
error: cannot produce proc-macro for `clap_derive v4.5.5` as the target
`loongarch64-unknown-linux-gnu` does not support these crate types
```

### Root Cause

`.cargo/config.toml` contained a Linux-wide rustflag:

```toml
[target.'cfg(target_os = "linux")']
rustflags = ["-Ctarget-feature=+crt-static"]
```

`-Ctarget-feature=+crt-static` forces static CRT linking.
Proc-macro crates require dynamic linking (they are loaded as `.so` files
by the compiler).  When rustc sees `crt-static` it drops the `proc-macro`
crate type, and any crate depending on `clap_derive` fails to compile.

Because loongarch64 is a Linux target, the flag was applied even though
`crt-static` is only needed for Azure Pipelines / OneBranch x86_64 builds.

### Fix

Changed the target predicate to exclude loongarch64:

```toml
[target.'cfg(all(target_os = "linux", not(target_arch = "loongarch64")))']
rustflags = ["-Ctarget-feature=+crt-static"]
```

---

## 5. `unexpected_cfgs` warning for `loongarch64_musl_workaround`

- **Status**: Resolved

### Symptom

Rust ≥ 1.80 emits a lint warning for custom `cfg` names that are not
declared:

```
warning: unexpected `cfg` condition name: `loongarch64_musl_workaround`
```

### Fix

Declared the custom cfg in the relevant `Cargo.toml` files:

- `crates/pet-fs/Cargo.toml` — `[lints.rust] unexpected_cfgs = …`
- `Cargo.toml` (workspace) — `[workspace.lints.rust] unexpected_cfgs = …`
- `../test-musl/Cargo.toml` — same for the independent test harness

This tells the compiler that `loongarch64_musl_workaround` is an expected
custom cfg and silences the warning.
