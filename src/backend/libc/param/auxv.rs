use crate::backend::c;
#[cfg(any(
    all(target_os = "android", target_pointer_width = "64"),
    target_os = "linux",
))]
use crate::ffi::CStr;

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
    unsafe {
        let hwcap = c::getauxval(c::AT_HWCAP) as usize;
        let hwcap2 = c::getauxval(c::AT_HWCAP2) as usize;
        (hwcap, hwcap2)
    }
}

#[cfg(any(
    all(target_os = "android", target_pointer_width = "64"),
    target_os = "linux",
))]
#[inline]
pub(crate) fn linux_minsigstksz() -> usize {
    unsafe { c::getauxval(c::AT_MINSIGSTKSZ) as usize }
}

#[cfg(any(
    all(target_os = "android", target_pointer_width = "64"),
    target_os = "linux",
))]
#[inline]
pub(crate) fn linux_execfn() -> &'static CStr {
    unsafe { CStr::from_ptr(c::getauxval(c::AT_EXECFN) as *const _) }
}
