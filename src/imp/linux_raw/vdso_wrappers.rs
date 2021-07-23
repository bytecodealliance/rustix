//! Implement syscalls using the vDSO.
//!
//! <https://man7.org/linux/man-pages/man7/vdso.7.html>
//!
//! # Safety
//!
//! Similar to syscalls.rs, this file performs raw system calls, and sometimes
//! passes them uninitialized memory buffers. This file also calls vDSO
//! functions.
#![allow(unsafe_code)]

use super::{
    arch::asm::syscall2,
    conv::ret,
    time::{ClockId, DynamicClockId, Timespec},
    vdso,
};
use crate::io;
use cstr::cstr;
use linux_raw_sys::general::{__NR_clock_gettime, __kernel_clockid_t, __kernel_timespec};
#[cfg(target_pointer_width = "32")]
use linux_raw_sys::{
    general::timespec as __kernel_old_timespec, v5_4::general::__NR_clock_gettime64,
};
use std::{
    mem::{transmute, MaybeUninit},
    os::raw::c_int,
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
};

#[inline]
pub(crate) fn clock_gettime(which_clock: ClockId) -> __kernel_timespec {
    unsafe {
        let mut result = MaybeUninit::<__kernel_timespec>::uninit();
        let callee = match transmute(CLOCK_GETTIME.load(Relaxed)) {
            Some(callee) => callee,
            None => init_clock_gettime(),
        };
        let r0 = callee(which_clock as _, result.as_mut_ptr());
        assert_eq!(r0, 0);
        result.assume_init()
    }
}

#[inline]
pub(crate) fn clock_gettime_dynamic(which_clock: DynamicClockId) -> io::Result<Timespec> {
    let id = match which_clock {
        DynamicClockId::Known(id) => id as __kernel_clockid_t,

        DynamicClockId::Dynamic(fd) => {
            // See `FD_TO_CLOCKID` in Linux's `clock_gettime` documentation.
            use crate::io::AsRawFd;
            const CLOCKFD: i32 = 3;
            ((!fd.as_raw_fd() << 3) | CLOCKFD) as __kernel_clockid_t
        }

        DynamicClockId::RealtimeAlarm => {
            linux_raw_sys::v5_4::general::CLOCK_REALTIME_ALARM as __kernel_clockid_t
        }
        DynamicClockId::Tai => linux_raw_sys::v5_4::general::CLOCK_TAI as __kernel_clockid_t,
        DynamicClockId::Boottime => {
            linux_raw_sys::v5_4::general::CLOCK_BOOTTIME as __kernel_clockid_t
        }
        DynamicClockId::BoottimeAlarm => {
            linux_raw_sys::v5_4::general::CLOCK_BOOTTIME_ALARM as __kernel_clockid_t
        }
    };

    unsafe {
        let mut timespec = MaybeUninit::<Timespec>::uninit();
        let callee = match transmute(CLOCK_GETTIME.load(Relaxed)) {
            Some(callee) => callee,
            None => init_clock_gettime(),
        };
        ret(callee(id, timespec.as_mut_ptr()) as isize as usize)?;
        Ok(timespec.assume_init())
    }
}

#[cfg(target_arch = "x86")]
pub(crate) mod x86_via_vdso {
    use super::{super::arch::asm, transmute, Relaxed};

    #[inline]
    #[must_use]
    pub(crate) unsafe fn syscall0(nr: u32) -> usize {
        let callee = match transmute(super::SYSCALL.load(Relaxed)) {
            Some(callee) => callee,
            None => super::init_syscall(),
        };
        asm::indirect_syscall0(callee, nr)
    }

    #[inline]
    #[must_use]
    pub(crate) unsafe fn syscall1(nr: u32, a0: usize) -> usize {
        let callee = match transmute(super::SYSCALL.load(Relaxed)) {
            Some(callee) => callee,
            None => super::init_syscall(),
        };
        asm::indirect_syscall1(callee, nr, a0)
    }

    #[inline]
    pub(crate) unsafe fn syscall1_noreturn(nr: u32, a0: usize) -> ! {
        let callee = match transmute(super::SYSCALL.load(Relaxed)) {
            Some(callee) => callee,
            None => super::init_syscall(),
        };
        asm::indirect_syscall1_noreturn(callee, nr, a0)
    }

    #[inline]
    #[must_use]
    pub(crate) unsafe fn syscall2(nr: u32, a0: usize, a1: usize) -> usize {
        let callee = match transmute(super::SYSCALL.load(Relaxed)) {
            Some(callee) => callee,
            None => super::init_syscall(),
        };
        asm::indirect_syscall2(callee, nr, a0, a1)
    }

    #[inline]
    #[must_use]
    pub(crate) unsafe fn syscall3(nr: u32, a0: usize, a1: usize, a2: usize) -> usize {
        let callee = match transmute(super::SYSCALL.load(Relaxed)) {
            Some(callee) => callee,
            None => super::init_syscall(),
        };
        asm::indirect_syscall3(callee, nr, a0, a1, a2)
    }

    #[inline]
    #[must_use]
    pub(crate) unsafe fn syscall4(nr: u32, a0: usize, a1: usize, a2: usize, a3: usize) -> usize {
        let callee = match transmute(super::SYSCALL.load(Relaxed)) {
            Some(callee) => callee,
            None => super::init_syscall(),
        };
        asm::indirect_syscall4(callee, nr, a0, a1, a2, a3)
    }

    #[inline]
    #[must_use]
    pub(crate) unsafe fn syscall5(
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
    ) -> usize {
        let callee = match transmute(super::SYSCALL.load(Relaxed)) {
            Some(callee) => callee,
            None => super::init_syscall(),
        };
        asm::indirect_syscall5(callee, nr, a0, a1, a2, a3, a4)
    }

    #[inline]
    #[must_use]
    pub(crate) unsafe fn syscall6(
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
    ) -> usize {
        let callee = match transmute(super::SYSCALL.load(Relaxed)) {
            Some(callee) => callee,
            None => super::init_syscall(),
        };
        asm::indirect_syscall6(callee, nr, a0, a1, a2, a3, a4, a5)
    }

    // With the indirect call, it isn't meaningful to do a separate
    // `_readonly` optimization.
    pub(crate) use syscall0 as syscall0_readonly;
    pub(crate) use syscall1 as syscall1_readonly;
    pub(crate) use syscall2 as syscall2_readonly;
    pub(crate) use syscall3 as syscall3_readonly;
    pub(crate) use syscall4 as syscall4_readonly;
    pub(crate) use syscall5 as syscall5_readonly;
    pub(crate) use syscall6 as syscall6_readonly;
}

type ClockGettimeType = unsafe extern "C" fn(c_int, *mut Timespec) -> c_int;
#[cfg(target_arch = "x86")]
pub(super) type SyscallType =
    unsafe extern "C" fn(u32, usize, usize, usize, usize, usize, usize) -> usize;

fn init_clock_gettime() -> ClockGettimeType {
    init();
    // Safety: Load the function address from static storage that we
    // just initialized.
    unsafe { transmute(CLOCK_GETTIME.load(Relaxed)) }
}

#[cfg(target_arch = "x86")]
fn init_syscall() -> SyscallType {
    init();
    // Safety: Load the function address from static storage that we
    // just initialized.
    unsafe { transmute(SYSCALL.load(Relaxed)) }
}

static mut CLOCK_GETTIME: AtomicUsize = AtomicUsize::new(0);
#[cfg(target_arch = "x86")]
static mut SYSCALL: AtomicUsize = AtomicUsize::new(0);

#[cfg(target_pointer_width = "32")]
unsafe extern "C" fn posish_clock_gettime_via_syscall(clockid: c_int, res: *mut Timespec) -> c_int {
    let mut r0 = syscall2(__NR_clock_gettime64, clockid as usize, res as usize);
    if r0 == -io::Error::NOSYS.raw_os_error() as usize {
        // Ordinarily posish doesn't like to emulate system calls, but in
        // the case of time APIs, it's specific to Linux, specific to
        // 32-bit architectures *and* specific to old kernel versions, and
        // it's not that hard to fix up here, so that no other code needs
        // to worry about this.
        let mut old_result = MaybeUninit::<__kernel_old_timespec>::uninit();
        r0 = syscall2(
            __NR_clock_gettime,
            clockid as usize,
            old_result.as_mut_ptr() as usize,
        );
        if r0 == 0 {
            let old_result = old_result.assume_init();
            *res = Timespec {
                tv_sec: old_result.tv_sec.into(),
                tv_nsec: old_result.tv_nsec.into(),
            };
        }
    }
    r0 as c_int
}

#[cfg(target_pointer_width = "64")]
unsafe extern "C" fn posish_clock_gettime_via_syscall(clockid: c_int, res: *mut Timespec) -> c_int {
    syscall2(__NR_clock_gettime, clockid as usize, res as usize) as c_int
}

#[cfg(all(linux_raw_inline_asm, target_arch = "x86"))]
#[naked]
unsafe extern "C" fn posish_int_0x80(
    _nr: u32,
    _a0: usize,
    _a1: usize,
    _a2: usize,
    _a3: usize,
    _a4: usize,
    _a5: usize,
) -> usize {
    asm!("int $$0x80", "ret", options(noreturn))
}

#[cfg(all(not(linux_raw_inline_asm), target_arch = "x86"))]
extern "C" {
    fn posish_int_0x80(
        _nr: u32,
        _a0: usize,
        _a1: usize,
        _a2: usize,
        _a3: usize,
        _a4: usize,
        _a5: usize,
    ) -> usize;
}

fn init() {
    // Safety: Store default function addresses in static storage so that if we
    // end up making any system calls while we read the vDSO, they'll work.
    // If the memory happens to already be initialized, this is redundant, but
    // not harmful.
    unsafe {
        CLOCK_GETTIME
            .compare_exchange(
                0,
                posish_clock_gettime_via_syscall as ClockGettimeType as usize,
                Relaxed,
                Relaxed,
            )
            .ok();
        #[cfg(target_arch = "x86")]
        SYSCALL
            .compare_exchange(
                0,
                transmute(posish_int_0x80 as SyscallType),
                Relaxed,
                Relaxed,
            )
            .ok();
    }

    if let Some(vdso) = vdso::Vdso::new() {
        #[cfg(target_arch = "x86_64")]
        let ptr = vdso.sym(cstr!("LINUX_2.6"), cstr!("__vdso_clock_gettime"));
        #[cfg(target_arch = "aarch64")]
        let ptr = vdso.sym(cstr!("LINUX_2.6.39"), cstr!("__kernel_clock_gettime"));
        #[cfg(target_arch = "x86")]
        let ptr = vdso.sym(cstr!("LINUX_2.6"), cstr!("__vdso_clock_gettime64"));
        #[cfg(target_arch = "riscv64")]
        let ptr = vdso.sym(cstr!("LINUX_4.15"), cstr!("__kernel_clock_gettime"));

        assert!(!ptr.is_null());

        // Safety: Store the computed function addresses in static storage
        // so that we don't need to compute it again (but if we do, it doesn't
        // hurt anything).
        unsafe {
            CLOCK_GETTIME.store(ptr as usize, Relaxed);

            #[cfg(target_arch = "x86")]
            {
                let ptr = vdso.sym(cstr!("LINUX_2.5"), cstr!("__kernel_vsyscall"));
                assert!(!ptr.is_null());
                SYSCALL.store(ptr as usize, Relaxed);
            }
        }
    }
}
