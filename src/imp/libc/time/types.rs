#[cfg(not(target_os = "wasi"))]
use io_lifetimes::BorrowedFd;

/// `struct timespec`
pub type Timespec = libc::timespec;

#[allow(deprecated)]
pub type Secs = libc::time_t;
#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
pub type Nsecs = i64;
#[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))]
pub type Nsecs = std::os::raw::c_long;

/// `CLOCK_*` constants for use with [`clock_gettime`].
///
/// These constants are always supported at runtime so `clock_gettime` never
/// has to fail with `INVAL` due to an unsupported clock. See
/// [`DynamicClockId`] for a greater set of clocks, with the caveat that not
/// all of them are always support
///
/// [`clock_gettime`]: crate::time::clock_gettime
#[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "wasi")))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(i32)]
#[non_exhaustive]
pub enum ClockId {
    /// `CLOCK_REALTIME`
    Realtime = libc::CLOCK_REALTIME,

    /// `CLOCK_MONOTONIC`
    Monotonic = libc::CLOCK_MONOTONIC,

    /// `CLOCK_PROCESS_CPUTIME_ID`
    #[cfg(not(any(target_os = "netbsd", target_os = "openbsd", target_os = "redox")))]
    ProcessCPUTime = libc::CLOCK_PROCESS_CPUTIME_ID,

    /// `CLOCK_THREAD_CPUTIME_ID`
    #[cfg(not(any(target_os = "netbsd", target_os = "openbsd", target_os = "redox")))]
    ThreadCPUTime = libc::CLOCK_THREAD_CPUTIME_ID,

    /// `CLOCK_REALTIME_COARSE`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    RealtimeCoarse = libc::CLOCK_REALTIME_COARSE,

    /// `CLOCK_MONOTONIC_COARSE`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    MonotonicCoarse = libc::CLOCK_MONOTONIC_COARSE,

    /// `CLOCK_MONOTONIC_RAW`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    MonotonicRaw = libc::CLOCK_MONOTONIC_RAW,
}

/// `CLOCK_*` constants for use with [`clock_gettime`].
///
/// These constants are always supported at runtime so `clock_gettime` never
/// has to fail with `INVAL` due to an unsupported clock. See
/// [`DynamicClockId`] for a greater set of clocks, with the caveat that not
/// all of them are always support
#[cfg(any(target_os = "macos", target_os = "ios"))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
#[non_exhaustive]
pub enum ClockId {
    /// `CLOCK_REALTIME`
    Realtime = libc::CLOCK_REALTIME,

    /// `CLOCK_MONOTONIC`
    Monotonic = libc::CLOCK_MONOTONIC,

    /// `CLOCK_PROCESS_CPUTIME_ID`
    ProcessCPUTime = libc::CLOCK_PROCESS_CPUTIME_ID,

    /// `CLOCK_THREAD_CPUTIME_ID`
    ThreadCPUTime = libc::CLOCK_THREAD_CPUTIME_ID,
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
