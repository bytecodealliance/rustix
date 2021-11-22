use super::super::c;
#[cfg(any(
    all(target_os = "android", target_pointer_width = "64"),
    target_os = "linux"
))]
use crate::ffi::ZStr;

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
    unsafe {
        let hwcap = c::getauxval(c::AT_HWCAP) as usize;
        let hwcap2 = c::getauxval(c::AT_HWCAP2) as usize;
        (hwcap, hwcap2)
    }
}

#[cfg(any(
    all(target_os = "android", target_pointer_width = "64"),
    target_os = "linux"
))]
#[inline]
pub(crate) fn linux_execfn() -> &'static ZStr {
    unsafe { ZStr::from_ptr(c::getauxval(c::AT_EXECFN) as *const _) }
}
