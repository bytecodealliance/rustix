//! Time-related operations.

#[cfg(not(target_os = "wasi"))] // not implemented in libc for WASI yet
mod clock;

#[cfg(not(target_os = "wasi"))] // not implemented in libc for WASI yet
pub use clock::{clock_getres, clock_gettime, timespec, ClockId, UTIME_NOW, UTIME_OMIT};
