//! linux_raw syscalls supporting `rustix::runtime`.
//!
//! # Safety
//!
//! See the `rustix::imp` module documentation for details.
#![allow(unsafe_code)]

use super::super::arch::choose::{
    syscall1_noreturn, syscall1_readonly, syscall2_readonly, syscall3_readonly, syscall5_readonly,
};
use super::super::c;
use super::super::conv::{
    borrowed_fd, c_int, c_str, c_uint, ret, ret_c_uint, ret_error, ret_usize_infallible, void_star,
    zero,
};
use super::super::reg::nr;
use crate::fd::BorrowedFd;
use crate::ffi::ZStr;
use crate::fs::AtFlags;
use crate::io;
use crate::process::{Pid, RawNonZeroPid};
#[cfg(target_arch = "arm")]
use linux_raw_sys::general::__ARM_NR_set_tls;
use linux_raw_sys::general::{
    __NR_clone, __NR_execve, __NR_execveat, __NR_exit, __NR_prctl, __NR_set_tid_address,
    __kernel_pid_t, PR_SET_NAME, SIGCHLD,
};
#[cfg(target_arch = "x86")]
use {super::super::conv::by_mut, linux_raw_sys::general::__NR_set_thread_area};
#[cfg(target_arch = "x86_64")]
use {
    super::super::conv::ret_infallible,
    linux_raw_sys::general::{__NR_arch_prctl, ARCH_SET_FS},
};

#[inline]
pub(crate) unsafe fn fork() -> io::Result<Option<Pid>> {
    let pid = ret_c_uint(syscall5_readonly(
        nr(__NR_clone),
        c_uint(SIGCHLD),
        zero(),
        zero(),
        zero(),
        zero(),
    ))?;
    Ok(Pid::from_raw(pid))
}

pub(crate) unsafe fn execveat(
    dirfd: BorrowedFd<'_>,
    path: &ZStr,
    args: *const *const u8,
    env_vars: *const *const u8,
    flags: AtFlags,
) -> io::Error {
    ret_error(syscall5_readonly(
        nr(__NR_execveat),
        borrowed_fd(dirfd),
        c_str(path),
        void_star(args as _),
        void_star(env_vars as _),
        c_uint(flags.bits()),
    ))
}

pub(crate) unsafe fn execve(
    path: &ZStr,
    args: *const *const u8,
    env_vars: *const *const u8,
) -> io::Error {
    ret_error(syscall3_readonly(
        nr(__NR_execve),
        c_str(path),
        void_star(args as _),
        void_star(env_vars as _),
    ))
}

pub(crate) mod tls {
    #[cfg(target_arch = "x86")]
    use super::super::tls::UserDesc;
    use super::*;

    #[cfg(target_arch = "x86")]
    #[inline]
    pub(crate) unsafe fn set_thread_area(u_info: &mut UserDesc) -> io::Result<()> {
        ret(syscall1_readonly(nr(__NR_set_thread_area), by_mut(u_info)))
    }

    #[cfg(target_arch = "arm")]
    #[inline]
    pub(crate) unsafe fn arm_set_tls(data: *mut c::c_void) -> io::Result<()> {
        ret(syscall1_readonly(nr(__ARM_NR_set_tls), void_star(data)))
    }

    #[cfg(target_arch = "x86_64")]
    #[inline]
    pub(crate) unsafe fn set_fs(data: *mut c::c_void) {
        ret_infallible(syscall2_readonly(
            nr(__NR_arch_prctl),
            c_uint(ARCH_SET_FS),
            void_star(data),
        ))
    }

    #[inline]
    pub(crate) unsafe fn set_tid_address(data: *mut c::c_void) -> Pid {
        let tid: i32 =
            ret_usize_infallible(syscall1_readonly(nr(__NR_set_tid_address), void_star(data)))
                as __kernel_pid_t;
        debug_assert_ne!(tid, 0);
        Pid::from_raw_nonzero(RawNonZeroPid::new_unchecked(tid as u32))
    }

    #[inline]
    pub(crate) unsafe fn set_thread_name(name: &ZStr) -> io::Result<()> {
        ret(syscall2_readonly(
            nr(__NR_prctl),
            c_uint(PR_SET_NAME),
            c_str(name),
        ))
    }

    #[inline]
    pub(crate) fn exit_thread(code: c::c_int) -> ! {
        unsafe { syscall1_noreturn(nr(__NR_exit), c_int(code)) }
    }
}
