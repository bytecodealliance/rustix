use crate::imp;
use imp::fd::AsFd;

/// `isatty(fd)`—Tests whether a file descriptor refers to a terminal.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/isatty.html
/// [Linux]: https://man7.org/linux/man-pages/man3/isatty.3.html
#[inline]
pub fn isatty<Fd: AsFd>(fd: Fd) -> bool {
    imp::termios::syscalls::isatty(fd.as_fd())
}
