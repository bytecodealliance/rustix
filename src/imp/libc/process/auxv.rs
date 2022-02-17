use super::super::c;
#[cfg(any(
    all(target_os = "android", target_pointer_width = "64"),
    target_os = "linux"
))]
use crate::ffi::ZStr;

// `getauxval` wasn't supported in glibc until 2.16.
#[cfg(any(
    all(target_os = "android", target_pointer_width = "64"),
    target_os = "linux"
))]
weak!(fn getauxval(libc::c_ulong) -> libc::c_ulong);

#[inline]
pub(crate) fn page_size() -> usize {
    unsafe { c::sysconf(c::_SC_PAGESIZE) as usize }
}

#[cfg(any(
    all(target_os = "android", target_pointer_width = "64"),
    target_os = "linux"
))]
#[inline]
pub(crate) fn linux_hwcap() -> (usize, usize) {
    if let Some(libc_getauxval) = getauxval.get() {
        unsafe {
            let hwcap = libc_getauxval(c::AT_HWCAP) as usize;
            let hwcap2 = libc_getauxval(c::AT_HWCAP2) as usize;
            (hwcap, hwcap2)
        }
    } else {
        (0, 0)
    }
}

#[cfg(any(
    all(target_os = "android", target_pointer_width = "64"),
    target_os = "linux"
))]
#[inline]
pub(crate) fn linux_execfn() -> &'static ZStr {
    if let Some(libc_getauxval) = getauxval.get() {
        unsafe { ZStr::from_ptr(libc_getauxval(c::AT_EXECFN) as *const _) }
    } else {
        zstr!("")
    }
}
