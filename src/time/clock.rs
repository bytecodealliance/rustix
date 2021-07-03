use crate::{imp, io, time::Timespec};

/// `clockid_t`
#[cfg(any(linux_raw, all(libc, not(target_os = "wasi"))))]
pub use imp::time::ClockId;

/// `clock_getres(id)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/clock_getres.html
/// [Linux]: https://man7.org/linux/man-pages/man2/clock_getres.2.html
#[cfg(any(linux_raw, all(libc, not(target_os = "wasi"))))]
#[inline]
#[must_use]
pub fn clock_getres(id: ClockId) -> Timespec {
    imp::syscalls::clock_getres(id)
}

/// `clock_gettime(id)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/clock_gettime.html
/// [Linux]: https://man7.org/linux/man-pages/man2/clock_gettime.2.html
#[cfg(any(linux_raw, all(libc, not(target_os = "wasi"))))]
#[inline]
#[must_use]
pub fn clock_gettime(id: ClockId) -> Timespec {
    imp::syscalls::clock_gettime(id)
}

/// `clock_nanosleep(id, 0, request, remain)`
///
/// This is `clock_nanosleep` specialized for the case of a relative sleep
/// interval. See [`clock_nanosleep_absolute`] for absolute intervals.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/clock_nanosleep.html
/// [Linux]: https://man7.org/linux/man-pages/man2/clock_nanosleep.2.html
#[cfg(any(linux_raw, all(libc, not(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "ios",
    target_os = "redox",
    target_os = "freebsd", // FreeBSD 12 has clock_nanosleep, but libc targets FreeBSD 11.
    target_os = "openbsd",
    target_os = "emscripten",
    target_os = "wasi",
)))))]
#[inline]
#[must_use]
pub fn clock_nanosleep_relative(id: ClockId, request: &Timespec) -> NanosleepRelativeResult {
    imp::syscalls::clock_nanosleep_relative(id, request)
}

/// `clock_nanosleep(id, TIMER_ABSTIME, request, NULL)`
///
/// This is `clock_nanosleep` specialized for the case of an absolute sleep
/// interval. See [`clock_nanosleep_relative`] for relative intervals.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/clock_nanosleep.html
/// [Linux]: https://man7.org/linux/man-pages/man2/clock_nanosleep.2.html
#[cfg(any(linux_raw, all(libc, not(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "ios",
    target_os = "redox",
    target_os = "freebsd", // FreeBSD 12 has clock_nanosleep, but libc targets FreeBSD 11.
    target_os = "openbsd",
    target_os = "emscripten",
    target_os = "wasi",
)))))]
#[inline]
pub fn clock_nanosleep_absolute(id: ClockId, request: &Timespec) -> io::Result<()> {
    imp::syscalls::clock_nanosleep_absolute(id, request)
}

/// `nanosleep(request, remain)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/nanosleep.html
/// [Linux]: https://man7.org/linux/man-pages/man2/nanosleep.2.html
#[inline]
#[must_use]
pub fn nanosleep(request: &Timespec) -> NanosleepRelativeResult {
    imp::syscalls::nanosleep(request)
}

/// A return type for `nanosleep` and `clock_nanosleep_relative`.
#[derive(Debug, Clone)]
pub enum NanosleepRelativeResult {
    /// The sleep completed normally.
    Ok,
    /// The sleep was interrupted, the remaining time is returned.
    Interrupted(Timespec),
    /// An invalid time value was provided.
    Err(io::Error),
}
