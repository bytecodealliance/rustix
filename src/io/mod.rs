//! I/O operations.

mod fd;
mod poll;

#[cfg(not(target_os = "redox"))]
pub use fd::fionread;
pub use fd::{is_read_write, isatty};
pub use poll::{PollFd, PollFdVec};
