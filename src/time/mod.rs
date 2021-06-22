//! Time-related operations.

#[cfg(not(target_os = "redox"))]
mod clock;

// TODO: Convert WASI'S clock APIs to use handles rather than ambient
// clock identifiers, update `wasi-libc`, and then add support in `posish`.
#[cfg(not(any(target_os = "wasi", target_os = "redox")))]
pub use clock::{clock_getres, clock_gettime, ClockId};
#[cfg(not(target_os = "redox"))]
pub use clock::{nanosleep, NanosleepRelativeResult};

#[cfg(not(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "ios",
    target_os = "redox",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "emscripten",
    target_os = "wasi",
)))]
pub use clock::{clock_nanosleep_absolute, clock_nanosleep_relative};

/// `struct timespec`
#[cfg(libc)]
pub type Timespec = libc::timespec;

/// `struct timespec`.
#[cfg(linux_raw)]
pub type Timespec = linux_raw_sys::general::__kernel_timespec;
