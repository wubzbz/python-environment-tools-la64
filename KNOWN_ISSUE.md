# Known Issues

## 1. `is_dir()` failures in musl target tests

- **Status**: Known — waiting on upstream musl/libc fix

### Symptom

Running `cargo test --target loongarch64-unknown-linux-musl` causes 4 tests in
`pet::find::tests` to fail:

- `test_regular_directory_is_detected`
- `test_symlinked_directory_is_detected`
- `test_symlinked_venv_in_envs_directory`
- `test_symlink_path_is_preserved_not_resolved`

All failures share the same root cause: `Path::is_dir()` returns `false` for
directories created under `TempDir` or for symlink targets.

The gnu target (`loongarch64-unknown-linux-gnu`) passes all tests.

This issue is **specific to loongarch64** — musl on x86_64 and aarch64 handles
`stat`/`fstatat` correctly. The loongarch64 musl port is relatively young and
these edge cases have not been fully addressed upstream.

### Solution

- Use the gnu target for testing:
  `cargo test --release --target loongarch64-unknown-linux-gnu`
- Build the musl target without testing:
  `cargo build --release --target loongarch64-unknown-linux-musl`

A platform-specific workaround (e.g. using `std::fs::metadata()` + manual
`S_ISDIR` check instead of `Path::is_dir()`) could be applied in the future
if musl upstream does not fix this.

### Root Cause

musl libc's `stat`/`fstatat` implementation on loongarch64 diverges from glibc
behaviour. Under certain conditions (e.g. paths inside `TempDir`, or symlink
targets) the kernel returns results that musl does not correctly interpret as
directories.

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

## 3. `create_dir_all` returns `EEXIST` on musl

- **Status**: Known — same root cause as #1, non-fatal

### Symptom

When PET runs under the musl target on loongarch64, the following error may
appear in the extension output panel:

```
[error] [pet] Error creating cache directory ".../pythonLocator"
  Os { code: 17, kind: AlreadyExists, message: "File exists" }
```

Despite this log entry, PET continues to operate correctly — environment
resolution and discovery succeed as normal.

The relevant code is in `crates/pet-python-utils/src/fs_cache.rs`:

```rust
match std::fs::create_dir_all(cache_directory) {
    Ok(_) => { /* write cache */ }
    Err(err) => error!("Error creating cache directory {:?} {:?}", ...),
}
```

### Solution

None required. The error is logged but not fatal. If desired, the log level
could be lowered from `error!` to `warn!` to avoid alarming users.

### Root Cause

Musl libc's `mkdir` syscall wrapper on loongarch64 returns `EEXIST` when the
target directory already exists, whereas glibc returns success. This triggers
`std::fs::create_dir_all`'s error path even though no actual failure occurred.
Same underlying loongarch64 musl port immaturity as issue #1.
