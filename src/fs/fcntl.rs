use crate::imp;
use crate::io::{self, OwnedFd};
use imp::fd::{AsFd, RawFd};
use imp::fs::{FdFlags, OFlags};

/// `fcntl(fd, F_GETFD)`—Returns a file descriptor's flags.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/fcntl.html
/// [Linux]: https://man7.org/linux/man-pages/man2/fcntl.2.html
#[inline]
pub fn fcntl_getfd<Fd: AsFd>(fd: &Fd) -> io::Result<FdFlags> {
    let fd = fd.as_fd();
    imp::syscalls::fcntl_getfd(fd)
}

/// `fcntl(fd, F_SETFD, flags)`—Sets a file descriptor's flags.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/fcntl.html
/// [Linux]: https://man7.org/linux/man-pages/man2/fcntl.2.html
#[inline]
pub fn fcntl_setfd<Fd: AsFd>(fd: &Fd, flags: FdFlags) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::fcntl_setfd(fd, flags)
}

/// `fcntl(fd, F_GETFL)`—Returns a file descriptor's access mode and status.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/fcntl.html
/// [Linux]: https://man7.org/linux/man-pages/man2/fcntl.2.html
#[inline]
pub fn fcntl_getfl<Fd: AsFd>(fd: &Fd) -> io::Result<OFlags> {
    let fd = fd.as_fd();
    imp::syscalls::fcntl_getfl(fd)
}

/// `fcntl(fd, F_SETFL, flags)`—Sets a file descriptor's status.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/fcntl.html
/// [Linux]: https://man7.org/linux/man-pages/man2/fcntl.2.html
#[inline]
pub fn fcntl_setfl<Fd: AsFd>(fd: &Fd, flags: OFlags) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::fcntl_setfl(fd, flags)
}

/// `fcntl(fd, F_GET_SEALS)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/fcntl.2.html
#[cfg(any(
    linux_raw,
    all(
        libc,
        not(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "redox",
            target_os = "wasi",
        ))
    )
))]
#[inline]
pub fn fcntl_get_seals<Fd: AsFd>(fd: &Fd) -> io::Result<u32> {
    let fd = fd.as_fd();
    imp::syscalls::fcntl_get_seals(fd)
}

/// `fcntl(fd, F_DUPFD_CLOEXEC)`—Creates a new `OwnedFd` instance, with value
/// at least `min`, that has `O_CLOEXEC` set and that shares the same
/// underlying [file description] as `fd`.
///
/// POSIX guarantees that `F_DUPFD_CLOEXEC` will use the lowest unused file
/// descriptor which is at least `min`, however it is not safe in general to
/// rely on this, as file descriptors may be unexpectedly allocated on other
/// threads or in libraries.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/fcntl.html
/// [Linux]: https://man7.org/linux/man-pages/man2/fcntl.2.html
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn fcntl_dupfd_cloexec<Fd: AsFd>(fd: &Fd, min: RawFd) -> io::Result<OwnedFd> {
    let fd = fd.as_fd();
    imp::syscalls::fcntl_dupfd_cloexec(fd, min)
}
