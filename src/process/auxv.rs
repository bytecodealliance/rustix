use crate::imp;
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
use std::ffi::CStr;

/// `getpagesize()`—Returns the process' page size.
#[inline]
#[doc(alias = "getpagesize")]
pub fn page_size() -> usize {
    imp::process::page_size()
}

/// `(getauxval(AT_HWCAP), getauxval(AT_HWCAP2)`—Returns the Linux "hwcap"
/// data.
///
/// Return the Linux `AT_HWCAP` and `AT_HWCAP2` values passed to the
/// current process.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man3/getauxval.3.html
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
#[inline]
pub fn linux_hwcap() -> (usize, usize) {
    imp::process::linux_hwcap()
}

/// `getauxval(AT_EXECFN)`—Returns the Linux "execfn" string.
///
/// Return the string that Linux has recorded as the filesystem path to the
/// executable.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man3/getauxval.3.html
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
#[inline]
pub fn linux_execfn() -> &'static CStr {
    imp::process::linux_execfn()
}
