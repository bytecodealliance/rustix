//! Time-related operations.

use crate::imp;

#[cfg(not(target_os = "redox"))]
mod clock;

// TODO: Convert WASI'S clock APIs to use handles rather than ambient
// clock identifiers, update `wasi-libc`, and then add support in `posish`.
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub use clock::{clock_getres, clock_gettime, clock_gettime_dynamic, ClockId, DynamicClockId};
#[cfg(not(target_os = "redox"))]
pub use clock::{nanosleep, NanosleepRelativeResult};

#[cfg(not(any(
    target_os = "emscripten",
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
pub use clock::{clock_nanosleep_absolute, clock_nanosleep_relative};

pub use imp::time::{Nsecs, Secs, Timespec};
