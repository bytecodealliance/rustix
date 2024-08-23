//! linux_raw syscalls supporting `rustix::futex`.
//!
//! # Safety
//!
//! See the `rustix::backend` module documentation for details.
#![allow(unsafe_code, clippy::undocumented_unsafe_blocks)]

use crate::backend::conv::{c_uint, ret_usize};
use crate::timespec::Timespec;
use crate::{futex, io};
use core::sync::atomic::AtomicU32;
#[cfg(target_pointer_width = "32")]
use linux_raw_sys::general::timespec as __kernel_old_timespec;

#[inline]
pub(crate) unsafe fn futex_val2(
    uaddr: *const AtomicU32,
    op: super::types::Operation,
    flags: futex::Flags,
    val: u32,
    val2: u32,
    uaddr2: *const AtomicU32,
    val3: u32,
) -> io::Result<usize> {
    // The least-significant four bytes of the timeout pointer are used as `val2`.
    // ["the kernel casts the timeout value first to unsigned long, then to uint32_t"](https://man7.org/linux/man-pages/man2/futex.2.html),
    // so we perform that exact conversion in reverse to create the pointer.
    let timeout = val2 as usize as *const Timespec;

    #[cfg(target_pointer_width = "32")]
    {
        ret_usize(syscall!(
            __NR_futex_time64,
            uaddr,
            (op, flags),
            c_uint(val),
            timeout,
            uaddr2,
            c_uint(val3)
        ))
    }
    #[cfg(target_pointer_width = "64")]
    ret_usize(syscall!(
        __NR_futex,
        uaddr,
        (op, flags),
        c_uint(val),
        timeout,
        uaddr2,
        c_uint(val3)
    ))
}

#[inline]
pub(crate) unsafe fn futex_timeout(
    uaddr: *const AtomicU32,
    op: super::types::Operation,
    flags: futex::Flags,
    val: u32,
    timeout: *const Timespec,
    uaddr2: *const AtomicU32,
    val3: u32,
) -> io::Result<usize> {
    #[cfg(target_pointer_width = "32")]
    {
        ret_usize(syscall!(
            __NR_futex_time64,
            uaddr,
            (op, flags),
            c_uint(val),
            timeout,
            uaddr2,
            c_uint(val3)
        ))
        .or_else(|err| {
            // See the comments in `rustix_clock_gettime_via_syscall` about
            // emulation.
            if err == io::Errno::NOSYS {
                futex_old_timespec(uaddr, op, flags, val, timeout, uaddr2, val3)
            } else {
                Err(err)
            }
        })
    }
    #[cfg(target_pointer_width = "64")]
    ret_usize(syscall!(
        __NR_futex,
        uaddr,
        (op, flags),
        c_uint(val),
        timeout,
        uaddr2,
        c_uint(val3)
    ))
}

#[cfg(target_pointer_width = "32")]
unsafe fn futex_old_timespec(
    uaddr: *const AtomicU32,
    op: super::types::Operation,
    flags: futex::Flags,
    val: u32,
    timeout: *const Timespec,
    uaddr2: *const AtomicU32,
    val3: u32,
) -> io::Result<usize> {
    let old_timeout = if timeout.is_null() {
        None
    } else {
        Some(__kernel_old_timespec {
            tv_sec: (*timeout).tv_sec.try_into().map_err(|_| io::Errno::INVAL)?,
            tv_nsec: (*timeout)
                .tv_nsec
                .try_into()
                .map_err(|_| io::Errno::INVAL)?,
        })
    };
    ret_usize(syscall!(
        __NR_futex,
        uaddr,
        (op, flags),
        c_uint(val),
        old_timeout
            .as_ref()
            .map(|timeout| timeout as *const __kernel_old_timespec)
            .unwrap_or(core::ptr::null()),
        uaddr2,
        c_uint(val3)
    ))
}
