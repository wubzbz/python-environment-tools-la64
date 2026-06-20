// LoongArch64 musl workarounds for directory operations.
//
// Root cause: musl's `struct stat` layout on loongarch64 does not match the
// kernel's layout, causing `st_mode` to always be 0. Neither `Path::is_dir()`
// nor `libc::stat`/`fstatat`/`fstat` can correctly determine file type.
// `fs::create_dir_all` also fails: it returns EEXIST on already-existing
// directories because its internal directory-existence check relies on `stat`.
//
// These workarounds are gated behind the cfg flag:
//   loongarch64_musl_workaround
//
// Set via .cargo/config.toml:
//   [target.loongarch64-unknown-linux-musl]
//   rustflags = ["--cfg", "loongarch64_musl_workaround"]
//
// To revert (when musl is fixed): remove the rustflags line and rebuild.

use std::io;
use std::path::Path;

/// Returns true if `path` is a directory.
///
/// On loongarch64 musl, uses `openat(O_DIRECTORY)` because `stat` is broken.
/// On all other platforms, delegates to `Path::is_dir()`.
#[cfg(loongarch64_musl_workaround)]
pub fn is_dir(path: &Path) -> bool {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    let c_path = match CString::new(path.as_os_str().as_bytes()) {
        Ok(c) => c,
        Err(_) => return false,
    };
    unsafe {
        let fd = libc::openat(libc::AT_FDCWD, c_path.as_ptr(), libc::O_RDONLY | libc::O_DIRECTORY);
        if fd >= 0 {
            libc::close(fd);
            true
        } else {
            false
        }
    }
}

#[cfg(not(loongarch64_musl_workaround))]
pub fn is_dir(path: &Path) -> bool {
    path.is_dir()
}

/// Creates a directory and all parent directories, ignoring spurious errors.
///
/// On loongarch64 musl, uses recursive `libc::mkdir` because
/// `fs::create_dir_all` returns EEXIST on already-existing directories.
/// On all other platforms, delegates to `fs::create_dir_all`.
#[cfg(loongarch64_musl_workaround)]
pub fn create_dir_all(path: &Path) -> io::Result<()> {
    use std::os::unix::ffi::OsStrExt;

    if path.as_os_str().is_empty() {
        return Ok(());
    }

    // Recurse to create parent first
    if let Some(parent) = path.parent() {
        if parent.as_os_str().is_empty() {
            return Ok(()); // reached root
        }
        create_dir_all(parent)?;
    }

    let c_path = std::ffi::CString::new(path.as_os_str().as_bytes())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "path contains null byte"))?;

    unsafe {
        let ret = libc::mkdir(c_path.as_ptr(), 0o755);
        if ret == 0 {
            return Ok(());
        }
    }

    let errno = io::Error::last_os_error();
    // EEXIST on an already-existing directory is expected; verify before ignoring.
    if errno.raw_os_error() == Some(libc::EEXIST) && is_dir(path) {
        return Ok(());
    }
    Err(errno)
}

#[cfg(not(loongarch64_musl_workaround))]
pub fn create_dir_all(path: &Path) -> io::Result<()> {
    std::fs::create_dir_all(path)
}
