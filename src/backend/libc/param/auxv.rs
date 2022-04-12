use crate::backend::c;
#[cfg(any(
    all(target_os = "android", target_pointer_width = "64"),
    target_os = "linux",
))]
use crate::ffi::CStr;

// `getauxval` wasn't supported in glibc until 2.16.
#[cfg(any(
    all(target_os = "android", target_pointer_width = "64"),
    target_os = "linux",
))]
weak!(fn getauxval(c::c_ulong) -> *mut c::c_void);

#[inline]
pub(crate) fn page_size() -> usize {
    unsafe { c::sysconf(c::_SC_PAGESIZE) as usize }
}

#[cfg(not(any(target_os = "horizon", target_os = "vita", target_os = "wasi")))]
#[inline]
pub(crate) fn clock_ticks_per_second() -> u64 {
    unsafe { c::sysconf(c::_SC_CLK_TCK) as u64 }
}

#[cfg(any(
    all(target_os = "android", target_pointer_width = "64"),
    target_os = "linux",
))]
#[inline]
pub(crate) fn linux_hwcap() -> (usize, usize) {
    if let Some(libc_getauxval) = getauxval.get() {
        unsafe {
            let hwcap = libc_getauxval(c::AT_HWCAP).addr();
            let hwcap2 = libc_getauxval(c::AT_HWCAP2).addr();
            (hwcap, hwcap2)
        }
    } else {
        (0, 0)
    }
}

#[cfg(any(
    all(target_os = "android", target_pointer_width = "64"),
    target_os = "linux",
))]
#[inline]
pub(crate) fn linux_minsigstksz() -> usize {
    if let Some(libc_getauxval) = getauxval.get() {
        unsafe { libc_getauxval(c::AT_MINSIGSTKSZ).addr() }
    } else {
        0
    }
}

#[cfg(any(
    all(target_os = "android", target_pointer_width = "64"),
    target_os = "linux",
))]
#[inline]
pub(crate) fn linux_execfn() -> &'static CStr {
    if let Some(libc_getauxval) = getauxval.get() {
        unsafe { CStr::from_ptr(libc_getauxval(c::AT_EXECFN).cast()) }
    } else {
        cstr!("")
    }
}
