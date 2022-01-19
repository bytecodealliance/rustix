use super::super::c;
#[cfg(not(target_os = "wasi"))]
use crate::fd::BorrowedFd;

/// `struct timespec`
pub type Timespec = c::timespec;

/// A type for the `tv_sec` field of [`Timespec`].
#[allow(deprecated)]
pub type Secs = c::time_t;

/// A type for the `tv_nsec` field of [`Timespec`].
#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
pub type Nsecs = i64;

/// A type for the `tv_nsec` field of [`Timespec`].
#[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))]
pub type Nsecs = c::c_long;

/// `CLOCK_*` constants for use with [`clock_gettime`].
///
/// These constants are always supported at runtime so `clock_gettime` never
/// has to fail with `INVAL` due to an unsupported clock. See
/// [`DynamicClockId`] for a greater set of clocks, with the caveat that not
/// all of them are always support
///
/// [`clock_gettime`]: crate::time::clock_gettime
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "wasi")))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(not(target_os = "dragonfly"), repr(i32))]
#[cfg_attr(target_os = "dragonfly", repr(u64))]
#[non_exhaustive]
pub enum ClockId {
    /// `CLOCK_REALTIME`
    Realtime = c::CLOCK_REALTIME,

    /// `CLOCK_MONOTONIC`
    Monotonic = c::CLOCK_MONOTONIC,

    /// `CLOCK_PROCESS_CPUTIME_ID`
    #[cfg(not(any(
        target_os = "illumos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "redox"
    )))]
    ProcessCPUTime = c::CLOCK_PROCESS_CPUTIME_ID,

    /// `CLOCK_THREAD_CPUTIME_ID`
    #[cfg(not(any(
        target_os = "illumos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "redox"
    )))]
    ThreadCPUTime = c::CLOCK_THREAD_CPUTIME_ID,

    /// `CLOCK_REALTIME_COARSE`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    RealtimeCoarse = c::CLOCK_REALTIME_COARSE,

    /// `CLOCK_MONOTONIC_COARSE`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    MonotonicCoarse = c::CLOCK_MONOTONIC_COARSE,

    /// `CLOCK_MONOTONIC_RAW`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    MonotonicRaw = c::CLOCK_MONOTONIC_RAW,
}

/// `CLOCK_*` constants for use with [`clock_gettime`].
///
/// These constants are always supported at runtime so `clock_gettime` never
/// has to fail with `INVAL` due to an unsupported clock. See
/// [`DynamicClockId`] for a greater set of clocks, with the caveat that not
/// all of them are always support
#[cfg(any(target_os = "ios", target_os = "macos"))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
#[non_exhaustive]
pub enum ClockId {
    /// `CLOCK_REALTIME`
    Realtime = c::CLOCK_REALTIME,

    /// `CLOCK_MONOTONIC`
    Monotonic = c::CLOCK_MONOTONIC,

    /// `CLOCK_PROCESS_CPUTIME_ID`
    ProcessCPUTime = c::CLOCK_PROCESS_CPUTIME_ID,

    /// `CLOCK_THREAD_CPUTIME_ID`
    ThreadCPUTime = c::CLOCK_THREAD_CPUTIME_ID,
}

/// `CLOCK_*` constants for use with [`clock_gettime_dynamic`].
///
/// These constants may be unsupported at runtime, depending on the OS version,
/// and `clock_gettime_dynamic` may fail with `INVAL`. See [`ClockId`] for
/// clocks which are always supported at runtime.
///
/// [`clock_gettime_dynamic`]: crate::time::clock_gettime_dynamic
#[cfg(not(target_os = "wasi"))]
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum DynamicClockId<'a> {
    /// `ClockId` values that are always supported at runtime.
    Known(ClockId),

    /// Linux dynamic clocks.
    Dynamic(BorrowedFd<'a>),

    /// `CLOCK_REALTIME_ALARM`, available on Linux >= 3.0
    #[cfg(any(target_os = "android", target_os = "linux"))]
    RealtimeAlarm,

    /// `CLOCK_TAI`, available on Linux >= 3.10
    #[cfg(any(target_os = "android", target_os = "linux"))]
    Tai,

    /// `CLOCK_BOOTTIME`, available on Linux >= 2.6.39
    #[cfg(any(target_os = "android", target_os = "linux"))]
    Boottime,

    /// `CLOCK_BOOTTIME_ALARM`, available on Linux >= 2.6.39
    #[cfg(any(target_os = "android", target_os = "linux"))]
    BoottimeAlarm,
}
