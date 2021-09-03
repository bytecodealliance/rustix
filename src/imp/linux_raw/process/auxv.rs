//! Linux auxv support.
//!
//! # Safety
//!
//! This uses raw pointers to locate and read the kernel-provided auxv array.
#![allow(unsafe_code)]
#![allow(non_snake_case)]

use linux_raw_sys::general::{AT_HWCAP, AT_NULL, AT_PAGESZ, AT_SYSINFO_EHDR};
use linux_raw_sys::v5_4::general::AT_HWCAP2;
use std::os::raw::c_char;
#[cfg(target_env = "gnu")]
use std::os::raw::c_int;

#[inline]
pub(crate) fn page_size() -> usize {
    auxv().page_size
}

#[inline]
pub(crate) fn linux_hwcap() -> (usize, usize) {
    let auxv = auxv();
    (auxv.hwcap, auxv.hwcap2)
}

#[inline]
pub(in super::super) fn sysinfo_ehdr() -> usize {
    auxv().sysinfo_ehdr
}

#[inline]
fn auxv() -> &'static Auxv {
    // Safety: `AUXV` is initialized from the `.init_array` so it's ready
    // before any user code calls this.
    unsafe { &AUXV }
}

/// A struct for holding fields obtained from the kernel-provided auxv array.
struct Auxv {
    page_size: usize,
    hwcap: usize,
    hwcap2: usize,
    sysinfo_ehdr: usize,
}

/// Data obtained from the kernel-provided auxv array. This is initialized at
/// program startup below.
static mut AUXV: Auxv = Auxv {
    page_size: 0,
    hwcap: 0,
    hwcap2: 0,
    sysinfo_ehdr: 0,
};

/// GLIBC passes argc, argv, and envp to functions in .init_array, as a
/// non-standard extension. Use priority 99 so that we run before any
/// normal user-defined constructor functions.
#[cfg(target_env = "gnu")]
#[used]
#[link_section = ".init_array.00099"]
static INIT_ARRAY: unsafe extern "C" fn(c_int, *mut *mut c_char, *mut *mut c_char) = {
    unsafe extern "C" fn function(_argc: c_int, _argv: *mut *mut c_char, envp: *mut *mut c_char) {
        init_from_envp(envp);
    }
    function
};

/// For musl etc., assume that `__environ` is available and points to the
/// original environment from the kernel, so we can find the auxv array in
/// memory after it. Use priority 99 so that we run before any normal
/// user-defined constructor functions.
#[cfg(not(target_env = "gnu"))]
#[used]
#[link_section = ".init_array.00099"]
static INIT_ARRAY: unsafe extern "C" fn() = {
    unsafe extern "C" fn function() {
        extern "C" {
            static __environ: *mut *mut c_char;
        }

        init_from_envp(__environ)
    }
    function
};

/// # Safety
///
/// This must be passed a pointer to the environment variable buffer
/// provided by the kernel, which is followed in memory by the auxv array.
unsafe fn init_from_envp(mut envp: *mut *mut c_char) {
    while !(*envp).is_null() {
        envp = envp.add(1);
    }
    init_from_auxp(envp.add(1).cast::<_>())
}

/// # Safety
///
/// This must be passed a pointer to the auxv array provided by the kernel.
unsafe fn init_from_auxp(mut auxp: *const Elf_auxv_t) {
    loop {
        let Elf_auxv_t { a_type, a_val } = *auxp;
        match a_type as _ {
            AT_PAGESZ => AUXV.page_size = a_val,
            AT_HWCAP => AUXV.hwcap = a_val,
            AT_HWCAP2 => AUXV.hwcap2 = a_val,
            AT_SYSINFO_EHDR => AUXV.sysinfo_ehdr = a_val,
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
    a_val: usize,
}
