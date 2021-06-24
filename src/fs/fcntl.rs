#[cfg(libc)]
use crate::libc::conv::{borrowed_fd, ret, ret_c_int, ret_owned_fd};
use crate::{
    fs::{FdFlags, OFlags},
    io,
};
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
#[cfg(all(
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
))]
use crate::libc::conv::ret_u32;

/// `fcntl(fd, F_GETFD)`
#[inline]
pub fn fcntl_getfd<Fd: AsFd>(fd: &Fd) -> io::Result<FdFlags> {
    let fd = fd.as_fd();
    _fcntl_getfd(fd)
}

#[cfg(libc)]
fn _fcntl_getfd(fd: BorrowedFd<'_>) -> io::Result<FdFlags> {
    unsafe {
        ret_c_int(libc::fcntl(borrowed_fd(fd), libc::F_GETFD)).map(FdFlags::from_bits_truncate)
    }
}

#[cfg(linux_raw)]
#[inline]
fn _fcntl_getfd(fd: BorrowedFd<'_>) -> io::Result<FdFlags> {
    crate::linux_raw::fcntl_getfd(fd).map(FdFlags::from_bits_truncate)
}

/// `fcntl(fd, F_SETFD, flags)`
#[inline]
pub fn fcntl_setfd<Fd: AsFd>(fd: &Fd, flags: FdFlags) -> io::Result<()> {
    let fd = fd.as_fd();
    _fcntl_setfd(fd, flags)
}

#[cfg(libc)]
fn _fcntl_setfd(fd: BorrowedFd<'_>, flags: FdFlags) -> io::Result<()> {
    unsafe { ret(libc::fcntl(borrowed_fd(fd), libc::F_SETFD, flags.bits())) }
}

#[cfg(linux_raw)]
#[inline]
fn _fcntl_setfd(fd: BorrowedFd<'_>, flags: FdFlags) -> io::Result<()> {
    crate::linux_raw::fcntl_setfd(fd, flags.bits())
}

/// `fcntl(fd, F_GETFL)`
#[inline]
pub fn fcntl_getfl<Fd: AsFd>(fd: &Fd) -> io::Result<OFlags> {
    let fd = fd.as_fd();
    _fcntl_getfl(fd)
}

#[cfg(libc)]
pub(crate) fn _fcntl_getfl(fd: BorrowedFd<'_>) -> io::Result<OFlags> {
    unsafe {
        ret_c_int(libc::fcntl(borrowed_fd(fd), libc::F_GETFL)).map(OFlags::from_bits_truncate)
    }
}

#[cfg(linux_raw)]
#[inline]
pub(crate) fn _fcntl_getfl(fd: BorrowedFd<'_>) -> io::Result<OFlags> {
    crate::linux_raw::fcntl_getfl(fd).map(OFlags::from_bits_truncate)
}

/// `fcntl(fd, F_SETFL, flags)`
#[inline]
pub fn fcntl_setfl<Fd: AsFd>(fd: &Fd, flags: OFlags) -> io::Result<()> {
    let fd = fd.as_fd();
    _fcntl_setfl(fd, flags)
}

#[cfg(libc)]
fn _fcntl_setfl(fd: BorrowedFd<'_>, flags: OFlags) -> io::Result<()> {
    unsafe { ret(libc::fcntl(borrowed_fd(fd), libc::F_SETFL, flags.bits())) }
}

#[cfg(linux_raw)]
#[inline]
fn _fcntl_setfl(fd: BorrowedFd<'_>, flags: OFlags) -> io::Result<()> {
    crate::linux_raw::fcntl_setfl(fd, flags.bits())
}

/// `fcntl(fd, F_GET_SEALS)`
#[cfg(not(any(
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
#[inline]
pub fn fcntl_get_seals<Fd: AsFd>(fd: &Fd) -> io::Result<u32> {
    let fd = fd.as_fd();
    _fcntl_get_seals(fd)
}

#[cfg(all(
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
))]
fn _fcntl_get_seals(fd: BorrowedFd<'_>) -> io::Result<u32> {
    unsafe { ret_u32(libc::fcntl(borrowed_fd(fd), libc::F_GET_SEALS)) }
}

#[cfg(linux_raw)]
#[inline]
fn _fcntl_get_seals(fd: BorrowedFd<'_>) -> io::Result<u32> {
    crate::linux_raw::fcntl_get_seals(fd).map(|seals| seals as u32)
}

/// `fcntl(fd, F_DUPFD_CLOEXEC)`
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn fcntl_dupfd_cloexec<Fd: AsFd>(fd: &Fd) -> io::Result<OwnedFd> {
    let fd = fd.as_fd();
    _fcntl_dupfd_cloexec(fd)
}

#[cfg(all(libc, not(target_os = "wasi")))]
fn _fcntl_dupfd_cloexec(fd: BorrowedFd<'_>) -> io::Result<OwnedFd> {
    unsafe { ret_owned_fd(libc::fcntl(borrowed_fd(fd), libc::F_DUPFD_CLOEXEC)) }
}

#[cfg(linux_raw)]
#[inline]
fn _fcntl_dupfd_cloexec(fd: BorrowedFd<'_>) -> io::Result<OwnedFd> {
    crate::linux_raw::fcntl_dupfd_cloexec(fd)
}
