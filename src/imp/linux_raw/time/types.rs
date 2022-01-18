use crate::fd::BorrowedFd;

/// `struct timespec`
pub type Timespec = linux_raw_sys::general::__kernel_timespec;

/// A type for the `tv_sec` field of [`Timespec`].
pub type Secs = linux_raw_sys::general::__kernel_time64_t;

/// A type for the `tv_nsec` field of [`Timespec`].
pub type Nsecs = i64;

/// `CLOCK_*` constants for use with [`clock_gettime`].
///
/// These constants are always supported at runtime so `clock_gettime` never
/// has to fail with `INVAL` due to an unsupported clock. See
/// [`DynamicClockId`] for a greater set of clocks, with the caveat that not
/// all of them are always support
///
/// [`clock_gettime`]: crate::time::clock_gettime
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
#[non_exhaustive]
pub enum ClockId {
    /// `CLOCK_REALTIME`
    Realtime = linux_raw_sys::general::CLOCK_REALTIME,

    /// `CLOCK_MONOTONIC`
    Monotonic = linux_raw_sys::general::CLOCK_MONOTONIC,

    /// `CLOCK_PROCESS_CPUTIME_ID`
    ProcessCPUTime = linux_raw_sys::general::CLOCK_PROCESS_CPUTIME_ID,

    /// `CLOCK_THREAD_CPUTIME_ID`
    ThreadCPUTime = linux_raw_sys::general::CLOCK_THREAD_CPUTIME_ID,

    /// `CLOCK_REALTIME_COARSE`
    RealtimeCoarse = linux_raw_sys::general::CLOCK_REALTIME_COARSE,

    /// `CLOCK_MONOTONIC_COARSE`
    MonotonicCoarse = linux_raw_sys::general::CLOCK_MONOTONIC_COARSE,

    /// `CLOCK_MONOTONIC_RAW`
    MonotonicRaw = linux_raw_sys::general::CLOCK_MONOTONIC_RAW,
}

/// `CLOCK_*` constants for use with [`clock_gettime_dynamic`].
///
/// These constants may be unsupported at runtime, depending on the OS version,
/// and `clock_gettime_dynamic` may fail with `INVAL`. See [`ClockId`] for
/// clocks which are always supported at runtime.
///
/// [`clock_gettime_dynamic`]: crate::time::clock_gettime_dynamic
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum DynamicClockId<'a> {
    /// `ClockId` values that are always supported at runtime.
    Known(ClockId),

    /// Linux dynamic clocks.
    Dynamic(BorrowedFd<'a>),

    /// `CLOCK_REALTIME_ALARM`, available on Linux >= 3.0
    RealtimeAlarm,

    /// `CLOCK_TAI`, available on Linux >= 3.10
    Tai,

    /// `CLOCK_BOOTTIME`, available on Linux >= 2.6.39
    Boottime,

    /// `CLOCK_BOOTTIME_ALARM`, available on Linux >= 2.6.39
    BoottimeAlarm,
}
