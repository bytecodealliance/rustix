use crate::fs::{FdFlags, OFlags};
use io_lifetimes::{AsFd, BorrowedFd};
use std::io;
#[cfg(libc)]
use {
    crate::{negone_err, zero_ok},
    unsafe_io::os::posish::AsRawFd,
};

/// `fcntl(fd, F_GETFD)`
#[inline]
pub fn fcntl_getfd<'f, Fd: AsFd<'f>>(fd: Fd) -> io::Result<FdFlags> {
    let fd = fd.as_fd();
    _fcntl_getfd(fd)
}

#[cfg(libc)]
fn _fcntl_getfd(fd: BorrowedFd<'_>) -> io::Result<FdFlags> {
    unsafe {
        negone_err(libc::fcntl(fd.as_raw_fd() as libc::c_int, libc::F_GETFD))
            .map(FdFlags::from_bits_truncate)
    }
}

#[cfg(linux_raw)]
#[inline]
fn _fcntl_getfd(fd: BorrowedFd<'_>) -> io::Result<FdFlags> {
    crate::linux_raw::fcntl_getfd(fd).map(FdFlags::from_bits_truncate)
}

/// `fcntl(fd, F_SETFD, flags)`
#[inline]
pub fn fcntl_setfd<'f, Fd: AsFd<'f>>(fd: Fd, flags: FdFlags) -> io::Result<()> {
    let fd = fd.as_fd();
    _fcntl_setfd(fd, flags)
}

#[cfg(libc)]
fn _fcntl_setfd(fd: BorrowedFd<'_>, flags: FdFlags) -> io::Result<()> {
    unsafe {
        zero_ok(libc::fcntl(
            fd.as_raw_fd() as libc::c_int,
            libc::F_SETFD,
            flags.bits(),
        ))
    }
}

#[cfg(linux_raw)]
#[inline]
fn _fcntl_setfd(fd: BorrowedFd<'_>, flags: FdFlags) -> io::Result<()> {
    crate::linux_raw::fcntl_setfd(fd, flags.bits())
}

/// `fcntl(fd, F_GETFL)`
#[inline]
pub fn fcntl_getfl<'f, Fd: AsFd<'f>>(fd: Fd) -> io::Result<OFlags> {
    let fd = fd.as_fd();
    _fcntl_getfl(fd)
}

#[cfg(libc)]
pub(crate) fn _fcntl_getfl(fd: BorrowedFd<'_>) -> io::Result<OFlags> {
    unsafe {
        negone_err(libc::fcntl(fd.as_raw_fd() as libc::c_int, libc::F_GETFL))
            .map(OFlags::from_bits_truncate)
    }
}

#[cfg(linux_raw)]
#[inline]
pub(crate) fn _fcntl_getfl(fd: BorrowedFd<'_>) -> io::Result<OFlags> {
    crate::linux_raw::fcntl_getfl(fd).map(OFlags::from_bits_truncate)
}

/// `fcntl(fd, F_SETFL, flags)`
#[inline]
pub fn fcntl_setfl<'f, Fd: AsFd<'f>>(fd: Fd, flags: OFlags) -> io::Result<()> {
    let fd = fd.as_fd();
    _fcntl_setfl(fd, flags)
}

#[cfg(libc)]
fn _fcntl_setfl(fd: BorrowedFd<'_>, flags: OFlags) -> io::Result<()> {
    unsafe {
        zero_ok(libc::fcntl(
            fd.as_raw_fd() as libc::c_int,
            libc::F_SETFL,
            flags.bits(),
        ))
    }
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
pub fn fcntl_get_seals<'f, Fd: AsFd<'f>>(fd: Fd) -> io::Result<u32> {
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
    unsafe {
        negone_err(libc::fcntl(
            fd.as_raw_fd() as libc::c_int,
            libc::F_GET_SEALS,
        ))
        .map(|s32: i32| s32 as u32)
    }
}

#[cfg(linux_raw)]
#[inline]
fn _fcntl_get_seals(fd: BorrowedFd<'_>) -> io::Result<u32> {
    crate::linux_raw::fcntl_get_seals(fd).map(|seals| seals as u32)
}
