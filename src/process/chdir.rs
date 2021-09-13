use crate::io;
use crate::{imp, path};
#[cfg(not(target_os = "fuchsia"))]
use io_lifetimes::AsFd;

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
