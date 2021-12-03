//! Time-related operations.

use crate::imp;

mod clock;

// TODO: Convert WASI'S clock APIs to use handles rather than ambient clock
// identifiers, update `wasi-libc`, and then add support in `rustix`.
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub use clock::clock_getres;
#[cfg(not(target_os = "wasi"))]
pub use clock::{clock_gettime, clock_gettime_dynamic, ClockId, DynamicClockId};

pub use imp::time::{Nsecs, Secs, Timespec};
