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

### Solution

- Use the gnu target for testing:
  `cargo test --release --target loongarch64-unknown-linux-gnu`
- Build the musl target without testing:
  `cargo build --release --target loongarch64-unknown-linux-musl`

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
