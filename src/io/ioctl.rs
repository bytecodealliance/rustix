#[cfg(not(target_os = "wasi"))]
use crate::io::{Termios, Winsize};
use crate::{imp, io};
use io_lifetimes::AsFd;

/// `ioctl(fd, TCGETS)`—Get terminal attributes.
///
/// Also known as `tcgetattr`.
///
/// # References
///  - [Linux `ioctl_tty`]
///  - [Linux `termios`]
///
/// [Linux `ioctl_tty`]: https://man7.org/linux/man-pages/man4/tty_ioctl.4.html
/// [Linux `termios`]: https://man7.org/linux/man-pages/man3/termios.3.html
#[cfg(not(target_os = "wasi"))]
#[doc(alias = "tcgetattr")]
#[inline]
pub fn ioctl_tcgets<Fd: AsFd>(fd: &Fd) -> io::Result<Termios> {
    let fd = fd.as_fd();
    imp::syscalls::ioctl_tcgets(fd)
}

/// `ioctl(fd, FIOCLEX)`—Set the close-on-exec flag.
///
/// Also known as `fcntl(fd, F_SETFD, FD_CLOEXEC)`.
#[cfg(any(target_os = "ios", target_os = "macos"))]
#[inline]
pub fn ioctl_fioclex<Fd: AsFd>(fd: &Fd) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::ioctl_fioclex(fd)
}

/// `ioctl(fd, TIOCGWINSZ)`—Get the current terminal window size.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man4/tty_ioctl.4.html
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn ioctl_tiocgwinsz<Fd: AsFd>(fd: &Fd) -> io::Result<Winsize> {
    let fd = fd.as_fd();
    imp::syscalls::ioctl_tiocgwinsz(fd)
}

/// `ioctl(fd, FIONBIO, &value)`—Enables or disables non-blocking mode.
#[inline]
pub fn ioctl_fionbio<Fd: AsFd>(fd: &Fd, value: bool) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::ioctl_fionbio(fd, value)
}

/// `ioctl(fd, TIOCEXCL)`—Enables exclusive mode on a terminal.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man4/tty_ioctl.4.html
#[cfg(any(
    linux_raw,
    all(libc, not(any(target_os = "redox", target_os = "wasi")))
))]
#[inline]
pub fn ioctl_tiocexcl<Fd: AsFd>(fd: &Fd) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::ioctl_tiocexcl(fd)
}

/// `ioctl(fd, TIOCNXCL)`—Disables exclusive mode on a terminal.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man4/tty_ioctl.4.html
#[cfg(any(
    linux_raw,
    all(libc, not(any(target_os = "redox", target_os = "wasi")))
))]
#[inline]
pub fn ioctl_tiocnxcl<Fd: AsFd>(fd: &Fd) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::ioctl_tiocnxcl(fd)
}
