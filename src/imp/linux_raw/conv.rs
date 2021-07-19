//! System call arguments and return values are all `usize`. This module
//! provides functions for converting into and out of `usize` values.
//!
//! # Safety
//!
//! Some of these functions are `unsafe` because they `transmute` `Option`
//! types knowing their layouts, or construct owned file descriptors.
#![allow(unsafe_code)]

use super::{
    fs::{Mode, OFlags},
    time::ClockId,
};
use crate::{
    as_mut_ptr, as_ptr, io,
    io::{AsRawFd, FromRawFd, RawFd},
};
use io_lifetimes::{BorrowedFd, OwnedFd};
#[cfg(target_pointer_width = "64")]
use linux_raw_sys::general::__kernel_loff_t;
#[cfg(target_pointer_width = "32")]
use linux_raw_sys::general::O_LARGEFILE;
use linux_raw_sys::general::{__kernel_clockid_t, socklen_t};
use std::{
    ffi::CStr,
    mem::{transmute, MaybeUninit},
    os::raw::{c_int, c_uint, c_void},
    ptr::null,
};

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
pub(super) fn raw_fd(fd: c_int) -> usize {
    // As above, use zero-extension rather than sign-extension.
    fd as c_uint as usize
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

/// Convert an optional immutable reference into a `usize` for passing to a
/// syscall.
///
/// # Safety
///
/// `Option<&T>` is represented as a nullable pointer to `T`, which is the
/// same size as a `usize`, so we can directly transmute it and pass the result
/// to syscalls expecting nullable pointers.
#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
#[inline]
pub(super) unsafe fn opt_ref<T: Sized>(t: Option<&T>) -> usize {
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
pub(super) fn clockid_t(i: ClockId) -> usize {
    i as __kernel_clockid_t as usize
}

#[inline]
pub(super) fn socklen_t(i: socklen_t) -> usize {
    i as usize
}

#[inline]
pub(super) fn mode_as(mode: Mode) -> usize {
    mode.bits() as usize
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

#[cfg(target_pointer_width = "32")]
pub(super) fn oflags(oflags: OFlags) -> usize {
    let mut bits = oflags.bits();
    // Add `O_LARGEFILE`, unless `O_PATH` is set, as Linux returns `EINVAL`
    // when both are set.
    if !oflags.contains(OFlags::PATH) {
        bits |= O_LARGEFILE;
    }
    bits as usize
}

#[cfg(target_pointer_width = "64")]
pub(super) fn oflags(oflags: OFlags) -> usize {
    oflags.bits() as usize
}

#[inline]
pub(super) fn out<T: Sized>(t: &mut MaybeUninit<T>) -> usize {
    t.as_mut_ptr() as usize
}

#[inline]
fn check_error(raw: usize) -> io::Result<()> {
    if (-4095..0).contains(&(raw as isize)) {
        // Discourage the optimizer from speculating the compuation of the
        // `Err` above the `if`. The discriminant of the `Result` is often
        // branched on, so if the optimizer speculates here and replaces the
        // `if` with a conditional move, it appears to struggle to undo this
        // once it realizes there's another branch on the same condition.
        //
        // It ends up doing a conditional move to produce a Result value
        // with either the encoded Err or Ok value, and then a branch on the
        // same condition, where both destinations of the branch have to unpack
        // the Result. What we want is to do is just branch, and skip encoding
        // and decoding the Result.
        let raw = suppress_optimization(raw);

        Err(io::Error((raw as u16).wrapping_neg()))
    } else {
        Ok(())
    }
}

#[inline]
pub(super) fn ret(raw: usize) -> io::Result<()> {
    // Instead of using `check_error` here, we just check for zero, since
    // this function is only used for system calls which have no other
    // return value, and this produces smaller code.
    if raw != 0 {
        // As above, discourage the optimizer from speculating the `Err`.
        let raw = suppress_optimization(raw);

        Err(io::Error((raw as u16).wrapping_neg()))
    } else {
        Ok(())
    }
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
    if (raw as isize) < 0 {
        // As above, discourage the optimizer from speculating the `Err`.
        let raw = suppress_optimization(raw);

        Err(io::Error((raw as u16).wrapping_neg()))
    } else {
        Ok(OwnedFd::from_raw_fd(raw as RawFd))
    }
}

#[inline]
pub(super) fn ret_discarded_fd(raw: usize) -> io::Result<()> {
    if (raw as isize) < 0 {
        // As above, discourage the optimizer from speculating the `Err`.
        let raw = suppress_optimization(raw);

        Err(io::Error((raw as u16).wrapping_neg()))
    } else {
        Ok(())
    }
}

#[inline]
pub(super) fn ret_void_star(raw: usize) -> io::Result<*mut c_void> {
    check_error(raw)?;
    Ok(raw as *mut c_void)
}

#[cfg(linux_raw_inline_asm)]
#[inline(always)]
fn suppress_optimization(mut t: usize) -> usize {
    // Safety: This asm block has no semantic effect.
    unsafe {
        asm!("/*{0}*/", inlateout(reg) t, options(pure, nomem, preserves_flags));
    }
    t
}

#[cfg(not(linux_raw_inline_asm))]
#[inline(never)]
fn suppress_optimization(t: usize) -> usize {
    // Without inline asm, we can put this in an inline-never function.
    t
}
