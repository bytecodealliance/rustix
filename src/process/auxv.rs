//! On mustang, the `init` function is unsafe because it operates on raw
//! pointers.
#![cfg_attr(target_vendor = "mustang", allow(unsafe_code))]

#[cfg(any(
    linux_raw,
    all(
        libc,
        any(
            all(target_os = "android", target_pointer_width = "64"),
            target_os = "linux"
        )
    )
))]
use crate::ffi::ZStr;
use crate::imp;

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
#[cfg(any(
    linux_raw,
    all(
        libc,
        any(
            all(target_os = "android", target_pointer_width = "64"),
            target_os = "linux"
        )
    )
))]
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
#[cfg(any(
    linux_raw,
    all(
        libc,
        any(
            all(target_os = "android", target_pointer_width = "64"),
            target_os = "linux"
        )
    )
))]
#[inline]
pub fn linux_execfn() -> &'static ZStr {
    imp::process::linux_execfn()
}

/// Initialize process-wide state.
#[cfg(target_vendor = "mustang")]
#[inline]
#[doc(hidden)]
pub unsafe fn init(envp: *mut *mut u8) {
    imp::process::init(envp)
}
