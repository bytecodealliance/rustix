#[cfg(not(target_os = "wasi"))]
use crate::io::{self, Termios, Winsize};
use io_lifetimes::{AsFd, BorrowedFd};
#[cfg(libc)]
use {
    crate::libc::conv::{borrowed_fd, ret},
    std::mem::MaybeUninit,
};

/// `ioctl(fd, TCGETS)`
///
/// Also known as `tcgetattr`.
#[cfg(not(target_os = "wasi"))]
#[doc(alias = "tcgetattr")]
#[inline]
pub fn ioctl_tcgets<Fd: AsFd>(fd: &Fd) -> io::Result<Termios> {
    let fd = fd.as_fd();
    _ioctl_tcgets(fd)
}

#[cfg(all(
    libc,
    not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "wasi"
    ))
))]
fn _ioctl_tcgets(fd: BorrowedFd<'_>) -> io::Result<Termios> {
    let mut result = MaybeUninit::<Termios>::uninit();
    unsafe {
        ret(libc::ioctl(
            borrowed_fd(fd),
            libc::TCGETS,
            result.as_mut_ptr(),
        ))
        .map(|()| result.assume_init())
    }
}

#[cfg(all(
    libc,
    any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd"
    )
))]
fn _ioctl_tcgets(fd: BorrowedFd<'_>) -> io::Result<Termios> {
    let mut result = MaybeUninit::<Termios>::uninit();
    unsafe {
        ret(libc::tcgetattr(borrowed_fd(fd), result.as_mut_ptr())).map(|()| result.assume_init())
    }
}

#[cfg(linux_raw)]
#[inline]
fn _ioctl_tcgets(fd: BorrowedFd<'_>) -> io::Result<Termios> {
    crate::linux_raw::ioctl_tcgets(fd)
}

/// `ioctl(fd, FIOCLEX)`
///
/// Also known as `fcntl(fd, F_SETFD, FD_CLOEXEC)`.
#[cfg(any(target_os = "ios", target_os = "macos"))]
#[inline]
pub fn ioctl_fioclex<Fd: AsFd>(fd: &Fd) -> io::Result<()> {
    let fd = fd.as_fd();
    _ioctl_fioclex(fd)
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
fn _ioctl_fioclex(fd: BorrowedFd<'_>) -> io::Result<()> {
    unsafe { ret(libc::ioctl(borrowed_fd(fd), libc::FIOCLEX)) }
}

/// `ioctl(fd, TIOCGWINSZ)`.
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn ioctl_tiocgwinsz(fd: BorrowedFd) -> io::Result<Winsize> {
    let fd = fd.as_fd();
    _ioctl_tiocgwinsz(fd)
}

#[cfg(all(libc, not(target_os = "wasi")))]
fn _ioctl_tiocgwinsz(fd: BorrowedFd) -> io::Result<Winsize> {
    unsafe {
        let mut buf = MaybeUninit::<Winsize>::uninit();
        ret(libc::ioctl(
            borrowed_fd(fd),
            libc::TIOCGWINSZ.into(),
            buf.as_mut_ptr(),
        ))?;
        Ok(buf.assume_init())
    }
}

#[cfg(linux_raw)]
#[inline]
fn _ioctl_tiocgwinsz(fd: BorrowedFd) -> io::Result<Winsize> {
    crate::linux_raw::ioctl_tiocgwinsz(fd)
}
