use crate::{
    fs::{FdFlags, OFlags},
    negone_err, zero_ok,
};
use std::io;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, RawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, RawFd};

/// `fcntl(fd, F_GETFD)`
#[inline]
pub fn getfd<Fd: AsRawFd>(fd: &Fd) -> io::Result<FdFlags> {
    let fd = fd.as_raw_fd();
    unsafe { _getfd(fd) }
}

unsafe fn _getfd(fd: RawFd) -> io::Result<FdFlags> {
    negone_err(libc::fcntl(fd as libc::c_int, libc::F_GETFD)).map(FdFlags::from_bits_truncate)
}

/// `fcntl(fd, F_SETFD, flags)`
#[inline]
pub fn setfd<Fd: AsRawFd>(fd: &Fd, flags: FdFlags) -> io::Result<()> {
    let fd = fd.as_raw_fd();
    unsafe { _setfd(fd, flags) }
}

unsafe fn _setfd(fd: RawFd, flags: FdFlags) -> io::Result<()> {
    zero_ok(libc::fcntl(fd as libc::c_int, libc::F_SETFD, flags.bits()))
}

/// `fcntl(fd, F_GETFL)`
#[inline]
pub fn getfl<Fd: AsRawFd>(fd: &Fd) -> io::Result<OFlags> {
    let fd = fd.as_raw_fd();
    unsafe { _getfl(fd) }
}

unsafe fn _getfl(fd: RawFd) -> io::Result<OFlags> {
    negone_err(libc::fcntl(fd as libc::c_int, libc::F_GETFL)).map(OFlags::from_bits_truncate)
}

/// `fcntl(fd, F_SETFL, flags)`
#[inline]
pub fn setfl<Fd: AsRawFd>(fd: &Fd, flags: OFlags) -> io::Result<()> {
    let fd = fd.as_raw_fd();
    unsafe { _setfl(fd, flags) }
}

unsafe fn _setfl(fd: RawFd, flags: OFlags) -> io::Result<()> {
    zero_ok(libc::fcntl(fd as libc::c_int, libc::F_SETFL, flags.bits()))
}

/// `fcntl(fd, F_GET_SEALS)`
#[cfg(not(any(
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
#[inline]
pub fn get_seals<Fd: AsRawFd>(fd: &Fd) -> io::Result<i32> {
    let fd = fd.as_raw_fd();
    unsafe { _get_seals(fd) }
}

#[cfg(not(any(
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
unsafe fn _get_seals(fd: RawFd) -> io::Result<i32> {
    negone_err(libc::fcntl(fd as libc::c_int, libc::F_GET_SEALS))
}
