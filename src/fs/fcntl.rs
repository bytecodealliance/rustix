use crate::{imp, io};
use imp::fs::{FdFlags, OFlags};
use io_lifetimes::{AsFd, OwnedFd};

/// `fcntl(fd, F_GETFD)`
#[inline]
pub fn fcntl_getfd<Fd: AsFd>(fd: &Fd) -> io::Result<FdFlags> {
    let fd = fd.as_fd();
    imp::syscalls::fcntl_getfd(fd)
}

/// `fcntl(fd, F_SETFD, flags)`
#[inline]
pub fn fcntl_setfd<Fd: AsFd>(fd: &Fd, flags: FdFlags) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::fcntl_setfd(fd, flags)
}

/// `fcntl(fd, F_GETFL)`
#[inline]
pub fn fcntl_getfl<Fd: AsFd>(fd: &Fd) -> io::Result<OFlags> {
    let fd = fd.as_fd();
    imp::syscalls::fcntl_getfl(fd)
}

/// `fcntl(fd, F_SETFL, flags)`
#[inline]
pub fn fcntl_setfl<Fd: AsFd>(fd: &Fd, flags: OFlags) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::fcntl_setfl(fd, flags)
}

/// `fcntl(fd, F_GET_SEALS)`
#[cfg(any(
    linux_raw,
    all(
        libc,
        not(any(
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

/// `fcntl(fd, F_DUPFD_CLOEXEC)`
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn fcntl_dupfd_cloexec<Fd: AsFd>(fd: &Fd) -> io::Result<OwnedFd> {
    let fd = fd.as_fd();
    imp::syscalls::fcntl_dupfd_cloexec(fd)
}
