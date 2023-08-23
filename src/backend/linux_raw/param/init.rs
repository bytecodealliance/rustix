//! Linux auxv `init` function, for "use-explicitly-provided-auxv" mode.
//!
//! # Safety
//!
//! This uses raw pointers to locate and read the kernel-provided auxv array.
#![allow(unsafe_code)]

use crate::backend::c;
use crate::backend::elf::*;
#[cfg(feature = "param")]
use crate::ffi::CStr;
use core::ffi::c_void;
use core::mem::size_of;
use core::ptr::{null_mut, read, NonNull};
#[cfg(feature = "runtime")]
use core::slice;
use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use linux_raw_sys::general::{
    AT_CLKTCK, AT_EXECFN, AT_HWCAP, AT_HWCAP2, AT_NULL, AT_PAGESZ, AT_PHDR, AT_PHENT, AT_PHNUM,
    AT_SYSINFO_EHDR,
};

#[cfg(feature = "param")]
#[inline]
pub(crate) fn page_size() -> usize {
    unsafe { PAGE_SIZE.load(Ordering::Relaxed) }
}

#[cfg(feature = "param")]
#[inline]
pub(crate) fn clock_ticks_per_second() -> u64 {
    unsafe { CLOCK_TICKS_PER_SECOND.load(Ordering::Relaxed) as u64 }
}

#[cfg(feature = "param")]
#[inline]
pub(crate) fn linux_hwcap() -> (usize, usize) {
    unsafe {
        (
            HWCAP.load(Ordering::Relaxed),
            HWCAP2.load(Ordering::Relaxed),
        )
    }
}

#[cfg(feature = "param")]
#[inline]
pub(crate) fn linux_execfn() -> &'static CStr {
    let execfn = unsafe { EXECFN.load(Ordering::Relaxed) };

    // The user should have called `rustix::param::init` before calling this
    // function.
    assert!(!execfn.is_null());

    // SAFETY: We assume the `AT_EXECFN` value provided by the kernel points
    // to a valid C string.
    unsafe { CStr::from_ptr(execfn.cast()) }
}

#[cfg(feature = "runtime")]
#[inline]
pub(crate) fn exe_phdrs() -> (*const c_void, usize) {
    unsafe {
        (
            PHDR.load(Ordering::Relaxed).cast(),
            PHNUM.load(Ordering::Relaxed),
        )
    }
}

#[cfg(feature = "runtime")]
#[inline]
pub(in super::super) fn exe_phdrs_slice() -> &'static [Elf_Phdr] {
    let (phdr, phnum) = exe_phdrs();

    // SAFETY: We assume the `AT_PHDR` and `AT_PHNUM` values provided by the
    // kernel form a valid slice.
    unsafe { slice::from_raw_parts(phdr.cast(), phnum) }
}

/// `AT_SYSINFO_EHDR` isn't present on all platforms in all configurations, so
/// if we don't see it, this function returns a null pointer.
#[inline]
pub(in super::super) fn sysinfo_ehdr() -> *const Elf_Ehdr {
    unsafe { SYSINFO_EHDR.load(Ordering::Relaxed) }
}

static mut PAGE_SIZE: AtomicUsize = AtomicUsize::new(0);
static mut CLOCK_TICKS_PER_SECOND: AtomicUsize = AtomicUsize::new(0);
static mut HWCAP: AtomicUsize = AtomicUsize::new(0);
static mut HWCAP2: AtomicUsize = AtomicUsize::new(0);
static mut SYSINFO_EHDR: AtomicPtr<Elf_Ehdr> = AtomicPtr::new(null_mut());
// Use `dangling` so that we can always pass it to `slice::from_raw_parts`.
static mut PHDR: AtomicPtr<Elf_Phdr> = AtomicPtr::new(NonNull::dangling().as_ptr());
static mut PHNUM: AtomicUsize = AtomicUsize::new(0);
static mut EXECFN: AtomicPtr<c::c_char> = AtomicPtr::new(null_mut());

/// When "use-explicitly-provided-auxv" is enabled, we export a function to be
/// called during initialization, and passed a pointer to the original
/// environment variable block set up by the OS.
pub(crate) unsafe fn init(envp: *mut *mut u8) {
    init_from_envp(envp);
}

/// # Safety
///
/// This must be passed a pointer to the environment variable buffer
/// provided by the kernel, which is followed in memory by the auxv array.
unsafe fn init_from_envp(mut envp: *mut *mut u8) {
    while !(*envp).is_null() {
        envp = envp.add(1);
    }
    init_from_auxp(envp.add(1).cast())
}

/// Process auxv entries from the auxv array pointed to by `auxp`.
///
/// # Safety
///
/// This must be passed a pointer to an auxv array.
///
/// The buffer contains `Elf_aux_t` elements, though it need not be aligned;
/// function uses `read_unaligned` to read from it.
unsafe fn init_from_auxp(mut auxp: *const Elf_auxv_t) {
    loop {
        let Elf_auxv_t { a_type, a_val } = read(auxp);

        match a_type as _ {
            AT_PAGESZ => PAGE_SIZE.store(a_val as usize, Ordering::Relaxed),
            AT_CLKTCK => CLOCK_TICKS_PER_SECOND.store(a_val as usize, Ordering::Relaxed),
            AT_HWCAP => HWCAP.store(a_val as usize, Ordering::Relaxed),
            AT_HWCAP2 => HWCAP2.store(a_val as usize, Ordering::Relaxed),
            AT_PHDR => PHDR.store(a_val.cast::<Elf_Phdr>(), Ordering::Relaxed),
            AT_PHNUM => PHNUM.store(a_val as usize, Ordering::Relaxed),
            AT_PHENT => assert_eq!(a_val as usize, size_of::<Elf_Phdr>()),
            AT_EXECFN => EXECFN.store(a_val.cast::<c::c_char>(), Ordering::Relaxed),
            AT_SYSINFO_EHDR => SYSINFO_EHDR.store(a_val.cast::<Elf_Ehdr>(), Ordering::Relaxed),
            AT_NULL => break,
            _ => (),
        }
        auxp = auxp.add(1);
    }
}

// ELF ABI

#[repr(C)]
#[derive(Copy, Clone)]
struct Elf_auxv_t {
    a_type: usize,

    // Some of the values in the auxv array are pointers, so we make `a_val` a
    // pointer, in order to preserve their provenance. For the values which are
    // integers, we cast this to `usize`.
    a_val: *mut c_void,
}
