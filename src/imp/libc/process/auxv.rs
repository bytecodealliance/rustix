#[cfg(any(target_os = "android", target_os = "linux"))]
use libc::c_char;
#[cfg(any(target_os = "android", target_os = "linux"))]
use std::ffi::CStr;

#[inline]
pub(crate) fn page_size() -> usize {
    unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
#[inline]
pub(crate) fn linux_hwcap() -> (usize, usize) {
    unsafe {
        let hwcap = libc::getauxval(libc::AT_HWCAP) as usize;
        let hwcap2 = libc::getauxval(libc::AT_HWCAP2) as usize;
        (hwcap, hwcap2)
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
#[inline]
pub(crate) fn linux_execfn() -> &'static CStr {
    unsafe { CStr::from_ptr(libc::getauxval(libc::AT_EXECFN) as *const c_char) }
}
