use crate::imp;

pub use crate::imp::process::Resource;

/// `struct rlimit`—Current and maximum values used in [`getrlimit`].
#[derive(Debug)]
pub struct Rlimit {
    /// Current effective, "soft", limit.
    pub current: Option<u64>,
    /// Maximum, "hard", value that `current` may be dynamically increased to.
    pub maximum: Option<u64>,
}

/// `getrlimit(resource)`—Get a process resource limit value.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getrlimit.html
/// [Linux]: https://man7.org/linux/man-pages/man2/getrlimit.2.html
#[inline]
pub fn getrlimit(resource: Resource) -> Rlimit {
    imp::syscalls::getrlimit(resource)
}
