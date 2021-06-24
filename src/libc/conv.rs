#![allow(dead_code)]

use crate::io;
use crate::negone_err;
use io_lifetimes::{BorrowedFd, OwnedFd};
use libc::{c_char, c_int, c_long};
use std::ffi::CStr;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

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

/// Convert a c_int returned from a libc function to an `OwnedFd`, if valid.
///
/// # Safety
///
/// The caller must ensure that this is the return value of a libc function
/// which returns an owned file descriptor.
#[inline]
pub(crate) unsafe fn ret_owned_fd(raw: c_int) -> io::Result<OwnedFd> {
    negone_err(raw)?;
    Ok(OwnedFd::from_raw_fd(raw as RawFd))
}

/// Convert a c_long returned from `syscall` to an `OwnedFd`, if valid.
///
/// # Safety
///
/// The caller must ensure that this is the return value of a `syscall` call
/// which returns an owned file descriptor.
#[inline]
pub(crate) unsafe fn syscall_ret_owned_fd(raw: c_long) -> io::Result<OwnedFd> {
    negone_err(raw)?;
    Ok(OwnedFd::from_raw_fd(raw as RawFd))
}
