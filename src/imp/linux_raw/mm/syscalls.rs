//! linux_raw syscalls supporting `rustix::io`.
//!
//! # Safety
//!
//! See the `rustix::imp` module documentation for details.
#![allow(unsafe_code)]

use super::super::arch::choose::{
    syscall1_readonly, syscall2, syscall3, syscall4, syscall5, syscall6,
};
use super::super::c;
use super::super::conv::{
    borrowed_fd, c_uint, no_fd, pass_usize, ret, ret_owned_fd, ret_void_star, void_star,
};
use super::super::reg::nr;
use super::types::{
    Advice, MapFlags, MlockFlags, MprotectFlags, MremapFlags, MsyncFlags, ProtFlags,
    UserfaultfdFlags,
};
use crate::fd::BorrowedFd;
use crate::io::{self, OwnedFd};
#[cfg(target_pointer_width = "32")]
use core::convert::TryInto;
#[cfg(any(target_arch = "arm", target_arch = "mips", target_arch = "x86"))]
use linux_raw_sys::general::__NR_mmap2;
use linux_raw_sys::general::{
    __NR_madvise, __NR_mlock, __NR_mlock2, __NR_mprotect, __NR_mremap, __NR_msync, __NR_munlock,
    __NR_munmap, __NR_userfaultfd,
};
#[cfg(target_pointer_width = "64")]
use {super::super::conv::loff_t_from_u64, linux_raw_sys::general::__NR_mmap};

#[inline]
pub(crate) fn madvise(addr: *mut c::c_void, len: usize, advice: Advice) -> io::Result<()> {
    unsafe {
        ret(syscall3(
            nr(__NR_madvise),
            void_star(addr),
            pass_usize(len),
            c_uint(advice as c::c_uint),
        ))
    }
}

#[inline]
pub(crate) unsafe fn msync(addr: *mut c::c_void, len: usize, flags: MsyncFlags) -> io::Result<()> {
    ret(syscall3(
        nr(__NR_msync),
        void_star(addr),
        pass_usize(len),
        c_uint(flags.bits()),
    ))
}

/// # Safety
///
/// `mmap` is primarily unsafe due to the `addr` parameter, as anything working
/// with memory pointed to by raw pointers is unsafe.
#[inline]
pub(crate) unsafe fn mmap(
    addr: *mut c::c_void,
    length: usize,
    prot: ProtFlags,
    flags: MapFlags,
    fd: BorrowedFd<'_>,
    offset: u64,
) -> io::Result<*mut c::c_void> {
    #[cfg(target_pointer_width = "32")]
    {
        ret_void_star(syscall6(
            nr(__NR_mmap2),
            void_star(addr),
            pass_usize(length),
            c_uint(prot.bits()),
            c_uint(flags.bits()),
            borrowed_fd(fd),
            (offset / 4096)
                .try_into()
                .map(|scaled_offset| pass_usize(scaled_offset))
                .map_err(|_| io::Error::INVAL)?,
        ))
    }
    #[cfg(target_pointer_width = "64")]
    {
        ret_void_star(syscall6(
            nr(__NR_mmap),
            void_star(addr),
            pass_usize(length),
            c_uint(prot.bits()),
            c_uint(flags.bits()),
            borrowed_fd(fd),
            loff_t_from_u64(offset),
        ))
    }
}

/// # Safety
///
/// `mmap` is primarily unsafe due to the `addr` parameter, as anything working
/// with memory pointed to by raw pointers is unsafe.
#[inline]
pub(crate) unsafe fn mmap_anonymous(
    addr: *mut c::c_void,
    length: usize,
    prot: ProtFlags,
    flags: MapFlags,
) -> io::Result<*mut c::c_void> {
    #[cfg(target_pointer_width = "32")]
    {
        ret_void_star(syscall6(
            nr(__NR_mmap2),
            void_star(addr),
            pass_usize(length),
            c_uint(prot.bits()),
            c_uint(flags.bits() | linux_raw_sys::general::MAP_ANONYMOUS),
            no_fd(),
            pass_usize(0),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    {
        ret_void_star(syscall6(
            nr(__NR_mmap),
            void_star(addr),
            pass_usize(length),
            c_uint(prot.bits()),
            c_uint(flags.bits() | linux_raw_sys::general::MAP_ANONYMOUS),
            no_fd(),
            loff_t_from_u64(0),
        ))
    }
}

#[inline]
pub(crate) unsafe fn mprotect(
    ptr: *mut c::c_void,
    len: usize,
    flags: MprotectFlags,
) -> io::Result<()> {
    ret(syscall3(
        nr(__NR_mprotect),
        void_star(ptr),
        pass_usize(len),
        c_uint(flags.bits()),
    ))
}

/// # Safety
///
/// `munmap` is primarily unsafe due to the `addr` parameter, as anything
/// working with memory pointed to by raw pointers is unsafe.
#[inline]
pub(crate) unsafe fn munmap(addr: *mut c::c_void, length: usize) -> io::Result<()> {
    ret(syscall2(
        nr(__NR_munmap),
        void_star(addr),
        pass_usize(length),
    ))
}

/// # Safety
///
/// `mremap` is primarily unsafe due to the `old_address` parameter, as
/// anything working with memory pointed to by raw pointers is unsafe.
#[inline]
pub(crate) unsafe fn mremap(
    old_address: *mut c::c_void,
    old_size: usize,
    new_size: usize,
    flags: MremapFlags,
) -> io::Result<*mut c::c_void> {
    ret_void_star(syscall4(
        nr(__NR_mremap),
        void_star(old_address),
        pass_usize(old_size),
        pass_usize(new_size),
        c_uint(flags.bits()),
    ))
}

/// # Safety
///
/// `mremap_fixed` is primarily unsafe due to the `old_address` and
/// `new_address` parameters, as anything working with memory pointed to by raw
/// pointers is unsafe.
#[inline]
pub(crate) unsafe fn mremap_fixed(
    old_address: *mut c::c_void,
    old_size: usize,
    new_size: usize,
    flags: MremapFlags,
    new_address: *mut c::c_void,
) -> io::Result<*mut c::c_void> {
    ret_void_star(syscall5(
        nr(__NR_mremap),
        void_star(old_address),
        pass_usize(old_size),
        pass_usize(new_size),
        c_uint(flags.bits()),
        void_star(new_address),
    ))
}

/// # Safety
///
/// `mlock` operates on raw pointers and may round out to the nearest page
/// boundaries.
#[inline]
pub(crate) unsafe fn mlock(addr: *mut c::c_void, length: usize) -> io::Result<()> {
    ret(syscall2(
        nr(__NR_mlock),
        void_star(addr),
        pass_usize(length),
    ))
}

/// # Safety
///
/// `mlock_with` operates on raw pointers and may round out to the nearest page
/// boundaries.
#[inline]
pub(crate) unsafe fn mlock_with(
    addr: *mut c::c_void,
    length: usize,
    flags: MlockFlags,
) -> io::Result<()> {
    ret(syscall3(
        nr(__NR_mlock2),
        void_star(addr),
        pass_usize(length),
        c_uint(flags.bits()),
    ))
}

/// # Safety
///
/// `munlock` operates on raw pointers and may round out to the nearest page
/// boundaries.
#[inline]
pub(crate) unsafe fn munlock(addr: *mut c::c_void, length: usize) -> io::Result<()> {
    ret(syscall2(
        nr(__NR_munlock),
        void_star(addr),
        pass_usize(length),
    ))
}

#[inline]
pub(crate) unsafe fn userfaultfd(flags: UserfaultfdFlags) -> io::Result<OwnedFd> {
    ret_owned_fd(syscall1_readonly(
        nr(__NR_userfaultfd),
        c_uint(flags.bits()),
    ))
}
