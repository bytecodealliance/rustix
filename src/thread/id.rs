use crate::imp;
use crate::process::Pid;

/// `gettid()`—Returns the thread ID.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/gettid.2.html
#[inline]
#[must_use]
pub fn gettid() -> Pid {
    imp::syscalls::gettid()
}
