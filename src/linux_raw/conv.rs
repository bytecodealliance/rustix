//! System call arguments and return values are all `usize`. This module
//! provides functions for converting into and out of `usize` values.

use crate::io;
use io_lifetimes::{BorrowedFd, OwnedFd};
use linux_raw_sys::general::{__kernel_clockid_t, __kernel_loff_t, socklen_t, umode_t};
use std::{
    ffi::CStr,
    mem::{transmute, MaybeUninit},
    os::raw::{c_int, c_uint, c_void},
    ptr::null,
};
use unsafe_io::os::posish::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

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
    t as *const T as usize
}

#[inline]
pub(super) fn by_mut<T: Sized>(t: &mut T) -> usize {
    t as *mut T as usize
}

#[inline]
pub(super) unsafe fn opt_ref<T: Sized>(t: Option<&T>) -> usize {
    transmute(t)
}

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

#[inline]
pub(super) fn loff_t(i: __kernel_loff_t) -> usize {
    i as usize
}

#[inline]
pub(super) fn loff_t_from_u64(i: u64) -> usize {
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

#[inline]
pub(super) fn ret_owned_fd(raw: usize) -> io::Result<OwnedFd> {
    check_error(raw)?;
    Ok(unsafe { OwnedFd::from_raw_fd(raw as RawFd) })
}

#[inline]
pub(super) fn ret_void_star(raw: usize) -> io::Result<*mut c_void> {
    check_error(raw)?;
    Ok(raw as *mut c_void)
}
