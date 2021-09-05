#[cfg(any(target_os = "android", target_os = "linux"))]
use crate::imp;

/// Exit all the threads in the current process' thread group.
///
/// Note that this does not all any `__cxa_atexit`, `atexit`, or any other
/// destructors. Most programs should use [`std::process::exit`] instead
/// of calling this directly.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/exit_group.2.html
#[cfg(any(target_os = "android", target_os = "linux"))]
#[inline]
pub fn exit_group(status: i32) -> ! {
    imp::syscalls::exit_group(status)
}
