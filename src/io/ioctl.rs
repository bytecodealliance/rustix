#[cfg(not(target_os = "wasi"))]
use crate::io::{self, Termios};
use io_lifetimes::{AsFd, BorrowedFd};
#[cfg(libc)]
use {crate::zero_ok, std::mem::MaybeUninit, unsafe_io::os::posish::AsRawFd};

/// ioctl(fd, TCGETS)
///
/// Also known as `tcgetattr`.
#[cfg(not(target_os = "wasi"))]
#[doc(alias = "tcgetattr")]
#[inline]
pub fn ioctl_tcgets<'f, Fd: AsFd<'f>>(fd: Fd) -> io::Result<Termios> {
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
        zero_ok(libc::ioctl(
            fd.as_raw_fd(),
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
        zero_ok(libc::tcgetattr(fd.as_raw_fd(), result.as_mut_ptr())).map(|()| result.assume_init())
    }
}

#[cfg(linux_raw)]
#[inline]
fn _ioctl_tcgets(fd: BorrowedFd<'_>) -> io::Result<Termios> {
    crate::linux_raw::ioctl_tcgets(fd)
}

/// Also known as `fcntl(fildes, F_SETFD, FD_CLOEXEC)`.
#[cfg(any(target_os = "ios", target_os = "macos"))]
#[inline]
pub fn ioctl_fioclex<'f, Fd: AsFd<'f>>(fd: Fd) -> io::Result<()> {
    let fd = fd.as_fd();
    _ioctl_fioclex(fd)
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
fn _ioctl_fioclex(fd: BorrowedFd<'_>) -> io::Result<()> {
    unsafe { zero_ok(libc::ioctl(fd.as_raw_fd(), libc::FIOCLEX)) }
}
