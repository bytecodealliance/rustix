use crate::imp;
#[cfg(not(target_os = "wasi"))]
use crate::io::{self, Termios, Winsize};
use io_lifetimes::{AsFd, BorrowedFd};

/// `ioctl(fd, TCGETS)`
///
/// Also known as `tcgetattr`.
#[cfg(not(target_os = "wasi"))]
#[doc(alias = "tcgetattr")]
#[inline]
pub fn ioctl_tcgets<Fd: AsFd>(fd: &Fd) -> io::Result<Termios> {
    let fd = fd.as_fd();
    imp::syscalls::ioctl_tcgets(fd)
}

/// `ioctl(fd, FIOCLEX)`
///
/// Also known as `fcntl(fd, F_SETFD, FD_CLOEXEC)`.
#[cfg(any(target_os = "ios", target_os = "macos"))]
#[inline]
pub fn ioctl_fioclex<Fd: AsFd>(fd: &Fd) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::ioctl_fioclex(fd)
}

/// `ioctl(fd, TIOCGWINSZ)`.
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn ioctl_tiocgwinsz(fd: BorrowedFd) -> io::Result<Winsize> {
    let fd = fd.as_fd();
    imp::syscalls::ioctl_tiocgwinsz(fd)
}
