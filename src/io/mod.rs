//! I/O operations.

mod fd;
mod poll;
#[cfg(not(target_os = "wasi"))]
mod socketpair;

#[cfg(not(target_os = "wasi"))]
pub use fd::dup;
#[cfg(not(target_os = "redox"))]
pub use fd::fionread;
#[cfg(not(any(target_os = "wasi", target_os = "fuchsia")))]
pub use fd::ttyname;
pub use fd::{is_read_write, isatty};
pub use poll::{PollFd, PollFdVec};
#[cfg(not(target_os = "wasi"))]
pub use socketpair::socketpair_stream;
