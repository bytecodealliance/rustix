//! Functions which operate on file descriptors.

use crate::imp;
#[cfg(any(
    all(linux_raw, feature = "procfs"),
    all(libc, not(any(target_os = "fuchsia", target_os = "wasi")))
))]
#[cfg_attr(doc_cfg, doc(cfg(feature = "procfs")))]
use crate::io;
use imp::fd::AsFd;
#[cfg(any(
    all(linux_raw, feature = "procfs"),
    all(libc, not(any(target_os = "fuchsia", target_os = "wasi")))
))]
use {imp::fd::BorrowedFd, std::ffi::CString};

/// `isatty(fd)`—Tests whether a file descriptor refers to a terminal.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/isatty.html
/// [Linux]: https://man7.org/linux/man-pages/man3/isatty.3.html
#[inline]
pub fn isatty<Fd: AsFd>(fd: &Fd) -> bool {
    let fd = fd.as_fd();
    imp::syscalls::isatty(fd)
}

/// `ttyname_r(fd)`
///
/// If `reuse` is non-empty, reuse its buffer to store the result if possible.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/ttyname.html
/// [Linux]: https://man7.org/linux/man-pages/man3/ttyname.3.html
#[cfg(any(
    all(linux_raw, feature = "procfs"),
    all(libc, not(any(target_os = "fuchsia", target_os = "wasi")))
))]
#[cfg_attr(doc_cfg, doc(cfg(feature = "procfs")))]
#[inline]
pub fn ttyname<Fd: AsFd, B: Into<Vec<u8>>>(dirfd: &Fd, reuse: B) -> io::Result<CString> {
    let dirfd = dirfd.as_fd();
    _ttyname(dirfd, reuse.into())
}

#[cfg(any(
    all(linux_raw, feature = "procfs"),
    all(libc, not(any(target_os = "fuchsia", target_os = "wasi")))
))]
fn _ttyname(dirfd: BorrowedFd<'_>, mut buffer: Vec<u8>) -> io::Result<CString> {
    // This code would benefit from having a better way to read into
    // uninitialized memory, but that requires `unsafe`.
    buffer.clear();
    buffer.resize(256, 0_u8);

    loop {
        match imp::syscalls::ttyname(dirfd, &mut buffer) {
            Err(imp::io::Error::RANGE) => buffer.resize(buffer.len() * 2, 0_u8),
            Ok(len) => {
                buffer.resize(len, 0);
                return Ok(CString::new(buffer).unwrap());
            }
            Err(errno) => return Err(errno),
        }
    }
}
