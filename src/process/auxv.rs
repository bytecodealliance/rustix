//! APIs which are associated with the auxv array on Linux.
//!
//! # Safety
//!
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
use crate::ffi::CStr;
use crate::imp;

/// `sysconf(_SC_PAGESIZE)`—Returns the process' page size.
///
/// Also known as `getpagesize`.
///
/// # References
///  - [POSIX]
///  - [Linux `sysconf`]
///  - [Linux `getpagesize`]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/sysconf.html
/// [Linux `sysconf`]: https://man7.org/linux/man-pages/man3/sysconf.3.html
/// [Linux `getpagesize`]: https://man7.org/linux/man-pages/man2/getpagesize.2.html
#[inline]
#[doc(alias = "_SC_PAGESIZE")]
#[doc(alias = "_SC_PAGE_SIZE")]
#[doc(alias = "getpagesize")]
pub fn page_size() -> usize {
    imp::process::auxv::page_size()
}

/// `sysconf(_SC_CLK_TCK)`—Returns the process' clock ticks per second.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/sysconf.html
/// [Linux]: https://man7.org/linux/man-pages/man3/sysconf.3.html
#[cfg(not(target_os = "wasi"))]
#[inline]
#[doc(alias = "_SC_CLK_TCK")]
pub fn clock_ticks_per_second() -> u64 {
    imp::process::auxv::clock_ticks_per_second()
}

/// `(getauxval(AT_HWCAP), getauxval(AT_HWCAP2)`—Returns the Linux "hwcap"
/// data.
///
/// Return the Linux `AT_HWCAP` and `AT_HWCAP2` values passed to the
/// current process. Returns 0 for each value if it is not available.
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
    imp::process::auxv::linux_hwcap()
}

/// `getauxval(AT_EXECFN)`—Returns the Linux "execfn" string.
///
/// Return the string that Linux has recorded as the filesystem path to the
/// executable. Returns an empty string if the string is not available.
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
pub fn linux_execfn() -> &'static CStr {
    imp::process::auxv::linux_execfn()
}

/// Initialize process-wide state.
#[cfg(any(
    target_vendor = "mustang",
    not(any(target_env = "gnu", target_env = "musl"))
))]
#[inline]
#[doc(hidden)]
pub unsafe fn init(envp: *mut *mut u8) {
    imp::process::auxv::init(envp)
}
