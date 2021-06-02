use crate::{
    fs::{FdFlags, OFlags},
    negone_err, zero_ok,
};
use io_lifetimes::{AsFd, BorrowedFd};
use std::io;
use unsafe_io::os::posish::AsRawFd;

/// `fcntl(fd, F_GETFD)`
#[inline]
pub fn getfd<Fd: AsFd>(fd: &Fd) -> io::Result<FdFlags> {
    let fd = fd.as_fd();
    unsafe { _getfd(fd) }
}

unsafe fn _getfd(fd: BorrowedFd<'_>) -> io::Result<FdFlags> {
    negone_err(libc::fcntl(fd.as_raw_fd() as libc::c_int, libc::F_GETFD))
        .map(FdFlags::from_bits_truncate)
}

/// `fcntl(fd, F_SETFD, flags)`
#[inline]
pub fn setfd<Fd: AsFd>(fd: &Fd, flags: FdFlags) -> io::Result<()> {
    let fd = fd.as_fd();
    unsafe { _setfd(fd, flags) }
}

unsafe fn _setfd(fd: BorrowedFd<'_>, flags: FdFlags) -> io::Result<()> {
    zero_ok(libc::fcntl(
        fd.as_raw_fd() as libc::c_int,
        libc::F_SETFD,
        flags.bits(),
    ))
}

/// `fcntl(fd, F_GETFL)`
#[inline]
pub fn getfl<Fd: AsFd>(fd: &Fd) -> io::Result<OFlags> {
    let fd = fd.as_fd();
    unsafe { _getfl(fd) }
}

pub(crate) unsafe fn _getfl(fd: BorrowedFd<'_>) -> io::Result<OFlags> {
    negone_err(libc::fcntl(fd.as_raw_fd() as libc::c_int, libc::F_GETFL))
        .map(OFlags::from_bits_truncate)
}

/// `fcntl(fd, F_SETFL, flags)`
#[inline]
pub fn setfl<Fd: AsFd>(fd: &Fd, flags: OFlags) -> io::Result<()> {
    let fd = fd.as_fd();
    unsafe { _setfl(fd, flags) }
}

unsafe fn _setfl(fd: BorrowedFd<'_>, flags: OFlags) -> io::Result<()> {
    zero_ok(libc::fcntl(
        fd.as_raw_fd() as libc::c_int,
        libc::F_SETFL,
        flags.bits(),
    ))
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
pub fn get_seals<Fd: AsFd>(fd: &Fd) -> io::Result<i32> {
    let fd = fd.as_fd();
    unsafe { _get_seals(fd) }
}

#[cfg(not(any(
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
unsafe fn _get_seals(fd: BorrowedFd<'_>) -> io::Result<i32> {
    negone_err(libc::fcntl(
        fd.as_raw_fd() as libc::c_int,
        libc::F_GET_SEALS,
    ))
}
