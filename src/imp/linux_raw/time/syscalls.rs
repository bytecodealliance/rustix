//! linux_raw syscalls supporting `rustix::time`.
//!
//! # Safety
//!
//! See the `rustix::imp::syscalls` module documentation for details.

#![allow(unsafe_code)]

use super::super::arch::choose::syscall2;
use super::super::conv::{clockid_t, out};
use super::super::reg::nr;
use crate::time::ClockId;
use core::mem::MaybeUninit;
use linux_raw_sys::general::{__NR_clock_getres, __kernel_timespec};
#[cfg(target_pointer_width = "32")]
use {
    super::super::conv::ret, crate::io, linux_raw_sys::general::timespec as __kernel_old_timespec,
    linux_raw_sys::v5_4::general::__NR_clock_getres_time64,
};

#[inline]
pub(crate) fn clock_getres(which_clock: ClockId) -> __kernel_timespec {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let mut result = MaybeUninit::<__kernel_timespec>::uninit();
        let _ = ret(syscall2(
            nr(__NR_clock_getres_time64),
            clockid_t(which_clock),
            out(&mut result),
        ))
        .or_else(|err| {
            // See the comments in `rustix_clock_gettime_via_syscall` about
            // emulation.
            if err == io::Error::NOSYS {
                let mut old_result = MaybeUninit::<__kernel_old_timespec>::uninit();
                let res = ret(syscall2(
                    nr(__NR_clock_getres),
                    clockid_t(which_clock),
                    out(&mut old_result),
                ));
                let old_result = old_result.assume_init();
                *result.as_mut_ptr() = __kernel_timespec {
                    tv_sec: old_result.tv_sec.into(),
                    tv_nsec: old_result.tv_nsec.into(),
                };
                res
            } else {
                Err(err)
            }
        });
        result.assume_init()
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        let mut result = MaybeUninit::<__kernel_timespec>::uninit();
        let _ = syscall2(
            nr(__NR_clock_getres),
            clockid_t(which_clock),
            out(&mut result),
        );
        result.assume_init()
    }
}
