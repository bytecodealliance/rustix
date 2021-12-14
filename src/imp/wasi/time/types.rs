/// `struct timespec`: FIXME: import from wasi-clocks
#[derive(Debug, Clone)]
pub struct Timespec {
    /// seconds
    tv_sec: Secs,
    /// nanoseconds
    tv_nsec: Nsecs,
}

/// A type for the `tv_sec` field of [`Timespec`].
pub type Secs = u64;

/// A type for the `tv_nsec` field of [`Timespec`].
pub type Nsecs = u32;

/// `CLOCK_*` constants for use with [`clock_gettime`].
///
/// These constants are always supported at runtime so `clock_gettime` never
/// has to fail with `INVAL` due to an unsupported clock.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
#[non_exhaustive]
pub enum ClockId {
    /// `CLOCK_REALTIME`
    Realtime = 0, // FIXME

    /// `CLOCK_MONOTONIC`
    Monotonic = 1, // FIXME
}
