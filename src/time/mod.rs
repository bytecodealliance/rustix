//! Time-related operations.

#[cfg(not(any(target_os = "wasi", target_os = "redox")))] // not implemented in libc for WASI yet
mod clock;

#[cfg(not(any(target_os = "wasi", target_os = "redox")))]
// not implemented in libc for WASI yet
pub use clock::{clock_getres, clock_gettime, timespec, ClockId, UTIME_NOW, UTIME_OMIT};
