use crate::io;
use crate::{imp, path};
#[cfg(not(target_os = "fuchsia"))]
use io_lifetimes::AsFd;
#[cfg(not(target_os = "wasi"))]
use std::ffi::OsString;

/// `chdir(path)`—Change the working directory.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/chdir.2.html
#[inline]
pub fn chdir<P: path::Arg>(path: P) -> io::Result<()> {
    path.into_with_c_str(|path| imp::syscalls::chdir(path))
}

/// `fchdir(fd)`—Change the working directory.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/fchdir.2.html
#[cfg(not(target_os = "fuchsia"))]
#[inline]
pub fn fchdir<Fd: AsFd>(fd: Fd) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::fchdir(fd)
}

/// `getcwd()`
///
/// If `reuse` is non-empty, reuse its buffer to store the result if possible.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getcwd.html
/// [Linux]: https://man7.org/linux/man-pages/man3/getcwd.3.html
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn getcwd(reuse: OsString) -> io::Result<OsString> {
    use std::os::unix::prelude::OsStringExt;

    // This code would benefit from having a better way to read into
    // uninitialized memory, but that requires `unsafe`.
    let mut buffer = reuse.into_vec();
    buffer.clear();
    buffer.resize(256, 0_u8);

    loop {
        match imp::syscalls::getcwd(&mut buffer) {
            Err(imp::io::Error::RANGE) => buffer.resize(buffer.len() * 2, 0_u8),
            Ok(_) => {
                let len = buffer.iter().position(|x| *x == b'\0').unwrap();
                buffer.resize(len, 0_u8);
                return Ok(OsString::from_vec(buffer));
            }
            Err(errno) => return Err(errno),
        }
    }
}
