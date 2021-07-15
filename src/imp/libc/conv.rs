#![allow(dead_code)]

use super::offset::libc_off_t;
use crate::{
    io,
    io::{AsRawFd, FromRawFd, IntoRawFd, RawFd},
};
use io_lifetimes::{BorrowedFd, OwnedFd};
use libc::{c_char, c_int, c_long, ssize_t};
use std::ffi::CStr;

#[inline]
pub(crate) fn c_str(c: &CStr) -> *const c_char {
    c.as_ptr().cast::<c_char>()
}

#[inline]
pub(crate) fn borrowed_fd(fd: BorrowedFd<'_>) -> c_int {
    fd.as_raw_fd() as c_int
}

#[inline]
pub(crate) fn owned_fd(fd: OwnedFd) -> c_int {
    fd.into_raw_fd() as c_int
}

#[inline]
pub(crate) fn ret(raw: c_int) -> io::Result<()> {
    if raw == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

#[inline]
pub(crate) fn syscall_ret(raw: c_long) -> io::Result<()> {
    if raw == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

#[inline]
pub(crate) fn nonnegative_ret(raw: c_int) -> io::Result<()> {
    if raw >= 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

#[inline]
pub(crate) fn ret_c_int(raw: c_int) -> io::Result<c_int> {
    if raw == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(raw)
    }
}

#[inline]
pub(crate) fn ret_u32(raw: c_int) -> io::Result<u32> {
    if raw == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(raw as u32)
    }
}

#[inline]
pub(crate) fn ret_ssize_t(raw: ssize_t) -> io::Result<ssize_t> {
    if raw == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(raw)
    }
}

#[inline]
pub(crate) fn syscall_ret_ssize_t(raw: c_long) -> io::Result<ssize_t> {
    if raw == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(raw as ssize_t)
    }
}

#[inline]
pub(crate) fn ret_off_t(raw: libc_off_t) -> io::Result<libc_off_t> {
    if raw == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(raw)
    }
}

/// Convert a c_int returned from a libc function to an `OwnedFd`, if valid.
///
/// # Safety
///
/// The caller must ensure that this is the return value of a libc function
/// which returns an owned file descriptor.
#[inline]
pub(crate) unsafe fn ret_owned_fd(raw: c_int) -> io::Result<OwnedFd> {
    if raw == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(OwnedFd::from_raw_fd(raw as RawFd))
    }
}

#[inline]
pub(crate) fn ret_discarded_fd(raw: c_int) -> io::Result<()> {
    if raw == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

/// Convert a c_long returned from `syscall` to an `OwnedFd`, if valid.
///
/// # Safety
///
/// The caller must ensure that this is the return value of a `syscall` call
/// which returns an owned file descriptor.
#[inline]
pub(crate) unsafe fn syscall_ret_owned_fd(raw: c_long) -> io::Result<OwnedFd> {
    if raw == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(OwnedFd::from_raw_fd(raw as RawFd))
    }
}
