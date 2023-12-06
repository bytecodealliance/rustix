//! Linux auxv support, using libc.
//!
//! # Safety
//!
//! This uses raw pointers to locate and read the kernel-provided auxv array.
#![allow(unsafe_code)]

use crate::backend::c;
#[cfg(feature = "param")]
use crate::ffi::CStr;
#[cfg(not(feature = "runtime"))]
use core::ptr::null;
use linux_raw_sys::elf::*;
#[cfg(target_arch = "x86")]
use {
    core::ffi::c_void, core::ptr::null_mut, core::sync::atomic::AtomicPtr,
    core::sync::atomic::Ordering::Relaxed,
};

extern "C" {
    fn getauxval(type_: c::c_ulong) -> *mut c::c_void;
}

#[cfg(feature = "runtime")]
const AT_PHDR: c::c_ulong = 3;
#[cfg(feature = "runtime")]
const AT_PHENT: c::c_ulong = 4;
#[cfg(feature = "runtime")]
const AT_PHNUM: c::c_ulong = 5;
#[cfg(feature = "runtime")]
const AT_ENTRY: c::c_ulong = 9;
const AT_HWCAP: c::c_ulong = 16;
#[cfg(feature = "runtime")]
const AT_RANDOM: c::c_ulong = 25;
const AT_HWCAP2: c::c_ulong = 26;
const AT_SECURE: c::c_ulong = 23;
const AT_EXECFN: c::c_ulong = 31;
#[cfg(target_arch = "x86")]
const AT_SYSINFO: c::c_ulong = 32;
const AT_SYSINFO_EHDR: c::c_ulong = 33;

// Declare `sysconf` ourselves so that we don't depend on all of libc just for
// this.
extern "C" {
    fn sysconf(name: c::c_int) -> c::c_long;
}

#[cfg(target_os = "android")]
const _SC_PAGESIZE: c::c_int = 39;
#[cfg(target_os = "linux")]
const _SC_PAGESIZE: c::c_int = 30;
#[cfg(target_os = "android")]
const _SC_CLK_TCK: c::c_int = 6;
#[cfg(target_os = "linux")]
const _SC_CLK_TCK: c::c_int = 2;

#[test]
fn test_abi() {
    const_assert_eq!(self::_SC_PAGESIZE, ::libc::_SC_PAGESIZE);
    const_assert_eq!(self::_SC_CLK_TCK, ::libc::_SC_CLK_TCK);
    const_assert_eq!(self::AT_HWCAP, ::libc::AT_HWCAP);
    const_assert_eq!(self::AT_HWCAP2, ::libc::AT_HWCAP2);
    const_assert_eq!(self::AT_EXECFN, ::libc::AT_EXECFN);
    const_assert_eq!(self::AT_SECURE, ::libc::AT_SECURE);
    const_assert_eq!(self::AT_SYSINFO_EHDR, ::libc::AT_SYSINFO_EHDR);
    #[cfg(feature = "runtime")]
    const_assert_eq!(self::AT_PHDR, ::libc::AT_PHDR);
    #[cfg(feature = "runtime")]
    const_assert_eq!(self::AT_PHNUM, ::libc::AT_PHNUM);
    #[cfg(feature = "runtime")]
    const_assert_eq!(self::AT_ENTRY, ::libc::AT_ENTRY);
    #[cfg(feature = "runtime")]
    const_assert_eq!(self::AT_RANDOM, ::libc::AT_RANDOM);
    // TODO: Upstream x86's `AT_SYSINFO` to libc.
    #[cfg(target_arch = "x86")]
    const_assert_eq!(self::AT_SYSINFO, ::linux_raw_sys::general::AT_SYSINFO);
}

#[cfg(feature = "param")]
#[inline]
pub(crate) fn page_size() -> usize {
    unsafe { sysconf(_SC_PAGESIZE) as usize }
}

#[cfg(feature = "param")]
#[inline]
pub(crate) fn clock_ticks_per_second() -> u64 {
    unsafe { sysconf(_SC_CLK_TCK) as u64 }
}

#[cfg(feature = "param")]
#[inline]
pub(crate) fn linux_hwcap() -> (usize, usize) {
    unsafe {
        let hwcap = getauxval(AT_HWCAP) as usize;
        let hwcap2 = getauxval(AT_HWCAP2) as usize;
        (hwcap, hwcap2)
    }
}

#[cfg(feature = "param")]
#[inline]
pub(crate) fn linux_minsigstksz() -> usize {
    // FIXME: reuse const from libc when available?
    const AT_MINSIGSTKSZ: c::c_ulong = 51;

    #[cfg(not(feature = "runtime"))]
    if let Some(libc_getauxval) = getauxval.get() {
        unsafe { libc_getauxval(AT_MINSIGSTKSZ) as usize }
    } else {
        0
    }

    #[cfg(feature = "runtime")]
    unsafe {
        getauxval(AT_MINSIGSTKSZ) as usize
    }
}

#[cfg(feature = "param")]
#[inline]
pub(crate) fn linux_execfn() -> &'static CStr {
    unsafe { CStr::from_ptr(getauxval(AT_EXECFN).cast()) }
}

#[cfg(feature = "runtime")]
#[inline]
pub(crate) fn linux_secure() -> bool {
    unsafe { getauxval(AT_SECURE) as usize != 0 }
}

#[cfg(feature = "runtime")]
#[inline]
pub(crate) fn exe_phdrs() -> (*const c::c_void, usize, usize) {
    unsafe {
        let phdr = getauxval(AT_PHDR) as *const c::c_void;
        let phent = getauxval(AT_PHENT) as usize;
        let phnum = getauxval(AT_PHNUM) as usize;
        (phdr, phent, phnum)
    }
}

/// `AT_SYSINFO_EHDR` isn't present on all platforms in all configurations, so
/// if we don't see it, this function returns a null pointer.
#[inline]
pub(in super::super) fn sysinfo_ehdr() -> *const Elf_Ehdr {
    unsafe { getauxval(AT_SYSINFO_EHDR) as *const Elf_Ehdr }
}

#[cfg(feature = "runtime")]
#[inline]
pub(crate) fn entry() -> usize {
    unsafe { getauxval(AT_ENTRY) as usize }
}

#[cfg(feature = "runtime")]
#[inline]
pub(crate) fn random() -> *const [u8; 16] {
    unsafe { getauxval(AT_RANDOM) as *const [u8; 16] }
}

#[cfg(target_arch = "x86")]
#[inline]
pub(crate) fn vsyscall() -> *const c_void {
    // We call this for every system call, so memoize the value.
    static VSYSCALL: AtomicPtr<c_void> = AtomicPtr::new(null_mut());

    let mut vsyscall = VSYSCALL.load(Relaxed);

    if vsyscall.is_null() {
        #[cold]
        fn compute_vsyscall() -> *mut c_void {
            let vsyscall = unsafe { getauxval(AT_SYSINFO) as *mut c_void };
            VSYSCALL.store(vsyscall, Relaxed);
            vsyscall
        }

        vsyscall = compute_vsyscall();
    }

    vsyscall
}
