//! System call arguments and return values are all `usize`. This module
//! provides functions for converting into and out of `usize` values.
//!
//! # Safety
//!
//! Some of these functions are `unsafe` because they `transmute` `Option`
//! types knowing their layouts, or construct owned file descriptors.
#![allow(unsafe_code)]

use crate::{as_mut_ptr, as_ptr, io};
use io_lifetimes::{BorrowedFd, OwnedFd};
#[cfg(target_pointer_width = "64")]
use linux_raw_sys::general::__kernel_loff_t;
use linux_raw_sys::general::{__kernel_clockid_t, socklen_t, umode_t};
use std::{
    ffi::CStr,
    mem::{transmute, MaybeUninit},
    os::raw::{c_int, c_uint, c_void},
    ptr::null,
};
use unsafe_io::os::posish::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

/// Convert `SYS_*` constants for socketcall.
#[cfg(target_arch = "x86")]
pub(super) fn x86_sys(sys: u32) -> usize {
    sys as usize
}

#[cfg(all(target_endian = "little", target_pointer_width = "32"))]
#[inline]
pub(super) fn lo(x: u64) -> usize {
    (x >> 32) as usize
}

#[cfg(all(target_endian = "little", target_pointer_width = "32"))]
#[inline]
pub(super) fn hi(x: u64) -> usize {
    (x & 0xffff_ffff) as usize
}

#[cfg(all(target_endian = "big", target_pointer_width = "32"))]
#[inline]
pub(super) fn lo(x: u64) -> usize {
    (x & 0xffff_ffff) as usize
}

#[cfg(all(target_endian = "big", target_pointer_width = "32"))]
#[inline]
pub(super) fn hi(x: u64) -> usize {
    (x >> 32) as usize
}

#[cfg(target_pointer_width = "32")]
#[inline]
pub(super) fn i64_hi(x: i64) -> usize {
    hi(x as u64)
}

#[cfg(target_pointer_width = "32")]
#[inline]
pub(super) fn i64_lo(x: i64) -> usize {
    lo(x as u64)
}

#[inline]
pub(super) fn void_star(c: *mut c_void) -> usize {
    c as usize
}

#[inline]
pub(super) fn c_str(c: &CStr) -> usize {
    c.as_ptr() as usize
}

#[inline]
pub(super) fn opt_c_str(t: Option<&CStr>) -> usize {
    (match t {
        Some(s) => s.as_ptr(),
        None => null(),
    }) as usize
}

#[inline]
pub(super) fn borrowed_fd(fd: BorrowedFd<'_>) -> usize {
    // File descriptors are never negative on Linux, so use zero-extension
    // rather than sign-extension because it's a smaller instruction.
    fd.as_raw_fd() as c_uint as usize
}

#[inline]
pub(super) fn owned_fd(fd: OwnedFd) -> usize {
    // As above, use zero-extension rather than sign-extension.
    fd.into_raw_fd() as c_uint as usize
}

#[inline]
pub(super) fn slice_addr<T: Sized>(v: &[T]) -> usize {
    v.as_ptr() as usize
}

#[inline]
pub(super) fn slice_as_mut_ptr<T: Sized>(v: &mut [T]) -> usize {
    v.as_mut_ptr() as usize
}

#[inline]
pub(super) fn by_ref<T: Sized>(t: &T) -> usize {
    as_ptr(t) as usize
}

#[inline]
pub(super) fn by_mut<T: Sized>(t: &mut T) -> usize {
    as_mut_ptr(t) as usize
}

/// Convert an optional mutable reference into a `usize` for passing to a
/// syscall.
///
/// # Safety
///
/// `Option<&mut T>` is represented as a nullable pointer to `T`, which is the
/// same size as a `usize`, so we can directly transmute it and pass the result
/// to syscalls expecting nullable pointers.
#[inline]
pub(super) unsafe fn opt_mut<T: Sized>(t: Option<&mut T>) -> usize {
    transmute(t)
}

#[inline]
pub(super) fn c_int(i: c_int) -> usize {
    i as usize
}

#[inline]
pub(super) fn c_uint(i: c_uint) -> usize {
    i as usize
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub(super) fn loff_t(i: __kernel_loff_t) -> usize {
    i as usize
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub(super) fn loff_t_from_u64(i: u64) -> usize {
    // `loff_t` is signed, but syscalls which expect `loff_t` return `EINVAL`
    // if it's outside the signed `i64` range, so we can silently cast.
    i as usize
}

#[inline]
pub(super) fn clockid_t(i: __kernel_clockid_t) -> usize {
    i as usize
}

#[inline]
pub(super) fn socklen_t(i: socklen_t) -> usize {
    i as usize
}

#[inline]
pub(super) fn umode_t(mode: umode_t) -> usize {
    mode as usize
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub(super) fn dev_t(dev: u64) -> usize {
    dev as usize
}

#[cfg(target_pointer_width = "32")]
#[inline]
pub(super) fn dev_t(dev: u64) -> io::Result<usize> {
    use std::convert::TryInto;
    dev.try_into().map_err(|_err| io::Error::INVAL)
}

#[inline]
pub(super) fn out<T: Sized>(t: &mut MaybeUninit<T>) -> usize {
    t.as_mut_ptr() as usize
}

#[inline]
fn check_error(raw: usize) -> io::Result<()> {
    if (-4095..0).contains(&(raw as isize)) {
        Err(io::Error(-(raw as i16) as u16))
    } else {
        Ok(())
    }
}

#[inline]
pub(super) fn ret(raw: usize) -> io::Result<()> {
    check_error(raw)
}

#[inline]
pub(super) fn ret_c_int(raw: usize) -> io::Result<c_int> {
    check_error(raw)?;
    Ok(raw as c_int)
}

#[inline]
pub(super) fn ret_c_uint(raw: usize) -> io::Result<c_uint> {
    check_error(raw)?;
    Ok(raw as c_uint)
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub(super) fn ret_u64(raw: usize) -> io::Result<u64> {
    check_error(raw)?;
    Ok(raw as u64)
}

#[inline]
pub(super) fn ret_usize(raw: usize) -> io::Result<usize> {
    check_error(raw)?;
    Ok(raw)
}

/// Convert a usize returned from a syscall to an `OwnedFd`, if valid.
///
/// # Safety
///
/// The caller must ensure that this is the return value of a syscall which
/// returns an owned file descriptor.
#[inline]
pub(super) unsafe fn ret_owned_fd(raw: usize) -> io::Result<OwnedFd> {
    check_error(raw)?;
    Ok(OwnedFd::from_raw_fd(raw as RawFd))
}

#[inline]
pub(super) fn ret_void_star(raw: usize) -> io::Result<*mut c_void> {
    check_error(raw)?;
    Ok(raw as *mut c_void)
}
