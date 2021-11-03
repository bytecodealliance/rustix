use crate::ffi::ZString;
use crate::{imp, io, path};
#[cfg(not(target_os = "fuchsia"))]
use imp::fd::AsFd;

/// `chdir(path)`—Change the working directory.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/chdir.html
/// [Linux]: https://man7.org/linux/man-pages/man2/chdir.2.html
#[inline]
pub fn chdir<P: path::Arg>(path: P) -> io::Result<()> {
    path.into_with_z_str(|path| imp::syscalls::chdir(path))
}

/// `fchdir(fd)`—Change the working directory.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/fchdir.html
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
pub fn getcwd<B: Into<Vec<u8>>>(reuse: B) -> io::Result<ZString> {
    _getcwd(reuse.into())
}

fn _getcwd(mut buffer: Vec<u8>) -> io::Result<ZString> {
    // This code would benefit from having a better way to read into
    // uninitialized memory, but that requires `unsafe`.
    buffer.clear();
    buffer.resize(256, 0_u8);

    loop {
        match imp::syscalls::getcwd(&mut buffer) {
            Err(imp::io::Error::RANGE) => buffer.resize(buffer.len() * 2, 0_u8),
            Ok(_) => {
                let len = buffer.iter().position(|x| *x == b'\0').unwrap();
                buffer.resize(len, 0_u8);
                return Ok(ZString::new(buffer).unwrap());
            }
            Err(errno) => return Err(errno),
        }
    }
}
