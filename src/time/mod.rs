//! Time-related operations.

mod clock;
#[cfg(any(linux_kernel, target_os = "fuchsia"))]
#[cfg(feature = "time")]
mod timerfd;

// TODO: Convert WASI'S clock APIs to use handles rather than ambient clock
// identifiers, update `wasi-libc`, and then add support in `rustix`.
pub use clock::*;
#[cfg(any(linux_kernel, target_os = "fuchsia"))]
#[cfg(feature = "time")]
pub use timerfd::*;
