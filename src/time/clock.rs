use crate::{imp, io, time::Timespec};

/// `clockid_t`
#[cfg(any(linux_raw, all(libc, not(target_os = "wasi"))))]
pub use imp::time::{ClockId, DynamicClockId};

/// `clock_getres(id)`—Returns the resolution of a clock.
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

/// `clock_gettime(id)`—Returns the current value of a clock.
///
/// This function uses `ClockId` which only contains clocks which are known to
/// always be supported at runtime, allowing this function to be infallible.
/// For a greater set of clocks and dynamic clock support, see
/// [`clock_gettime_dynamic`].
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

/// Like [`clock_gettime`] but with support for dynamic clocks.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/clock_gettime.html
/// [Linux]: https://man7.org/linux/man-pages/man2/clock_gettime.2.html
#[cfg(any(linux_raw, all(libc, not(target_os = "wasi"))))]
#[inline]
pub fn clock_gettime_dynamic(id: DynamicClockId) -> io::Result<Timespec> {
    imp::syscalls::clock_gettime_dynamic(id)
}

/// `clock_nanosleep(id, 0, request, remain)`—Sleeps for a duration on a
/// given clock.
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
pub fn clock_nanosleep_relative(id: ClockId, request: &Timespec) -> NanosleepRelativeResult {
    imp::syscalls::clock_nanosleep_relative(id, request)
}

/// `clock_nanosleep(id, TIMER_ABSTIME, request, NULL)`—Sleeps until an
/// absolute time on a given clock.
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

/// `nanosleep(request, remain)`—Sleeps for a duration.
///
/// This effectively uses the system monotonic clock.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/nanosleep.html
/// [Linux]: https://man7.org/linux/man-pages/man2/nanosleep.2.html
#[inline]
pub fn nanosleep(request: &Timespec) -> NanosleepRelativeResult {
    imp::syscalls::nanosleep(request)
}

/// A return type for `nanosleep` and `clock_nanosleep_relative`.
#[derive(Debug, Clone)]
#[must_use]
pub enum NanosleepRelativeResult {
    /// The sleep completed normally.
    Ok,
    /// The sleep was interrupted, the remaining time is returned.
    Interrupted(Timespec),
    /// An invalid time value was provided.
    Err(io::Error),
}
