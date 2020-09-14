//! I/O operations.

mod fd;
mod poll;

pub use fd::{fionread, is_read_write, isatty};
pub use poll::{PollFd, PollFdVec};
