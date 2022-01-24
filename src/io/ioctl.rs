//! The Unix `ioctl` function is effectively lots of different functions
//! hidden behind a single dynamic dispatch interface. In order to provide
//! a type-safe API, rustix makes them all separate functions so that they
//! can have dedicated static type signatures.

#[cfg(not(any(windows, target_os = "wasi")))]
use crate::io::{Termios, Winsize};
use crate::{imp, io};
use imp::fd::AsFd;
#[cfg(windows)]
use imp::fd::AsSocketAsFd;

/// `ioctl(fd, TCGETS)`—Get terminal attributes.
///
/// Also known as `tcgetattr`.
///
/// # References
///  - [POSIX `tcgetattr`]
///  - [Linux `ioctl_tty`]
///  - [Linux `termios`]
///
/// [POSIX `tcgetattr`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/tcgetattr.html
/// [Linux `ioctl_tty`]: https://man7.org/linux/man-pages/man4/tty_ioctl.4.html
/// [Linux `termios`]: https://man7.org/linux/man-pages/man3/termios.3.html
#[cfg(not(any(windows, target_os = "wasi")))]
#[inline]
#[doc(alias = "tcgetattr")]
#[doc(alias = "TCGETS")]
pub fn ioctl_tcgets<Fd: AsFd>(fd: &Fd) -> io::Result<Termios> {
    let fd = fd.as_fd();
    imp::syscalls::ioctl_tcgets(fd)
}

/// `ioctl(fd, FIOCLEX)`—Set the close-on-exec flag.
///
/// Also known as `fcntl(fd, F_SETFD, FD_CLOEXEC)`.
#[cfg(any(target_os = "ios", target_os = "macos"))]
#[inline]
#[doc(alias = "FIOCLEX")]
#[doc(alias = "FD_CLOEXEC")]
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
#[cfg(not(any(windows, target_os = "wasi")))]
#[inline]
#[doc(alias = "TIOCGWINSZ")]
pub fn ioctl_tiocgwinsz<Fd: AsFd>(fd: &Fd) -> io::Result<Winsize> {
    let fd = fd.as_fd();
    imp::syscalls::ioctl_tiocgwinsz(fd)
}

/// `ioctl(fd, FIONBIO, &value)`—Enables or disables non-blocking mode.
#[inline]
#[doc(alias = "FIONBIO")]
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
    all(libc, not(any(windows, target_os = "redox", target_os = "wasi")))
))]
#[inline]
#[doc(alias = "TIOCEXCL")]
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
    all(libc, not(any(windows, target_os = "redox", target_os = "wasi")))
))]
#[inline]
#[doc(alias = "TIOCNXCL")]
pub fn ioctl_tiocnxcl<Fd: AsFd>(fd: &Fd) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::ioctl_tiocnxcl(fd)
}

/// `ioctl(fd, FIONREAD)`—Returns the number of bytes ready to be read.
///
/// The result of this function gets silently coerced into a C `int`
/// by the OS, so it may contain a wrapped value.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/ioctl_tty.2.html
#[cfg(not(any(windows, target_os = "redox")))]
#[inline]
#[doc(alias = "FIONREAD")]
pub fn ioctl_fionread<Fd: AsFd>(fd: &Fd) -> io::Result<u64> {
    let fd = fd.as_fd();
    imp::syscalls::ioctl_fionread(fd)
}

/// `ioctl(fd, BLKSSZGET)`—Returns the logical block size of a block device.
#[cfg(any(target_os = "android", target_os = "linux"))]
#[inline]
#[doc(alias = "BLKSSZGET")]
pub fn ioctl_blksszget<Fd: AsFd>(fd: &Fd) -> io::Result<u32> {
    let fd = fd.as_fd();
    imp::syscalls::ioctl_blksszget(fd)
}

/// `ioctl(fd, BLKPBSZGET)`—Returns the physical block size of a block device.
#[cfg(any(target_os = "android", target_os = "linux"))]
#[inline]
#[doc(alias = "BLKPBSZGET")]
pub fn ioctl_blkpbszget<Fd: AsFd>(fd: &Fd) -> io::Result<u32> {
    let fd = fd.as_fd();
    imp::syscalls::ioctl_blkpbszget(fd)
}
