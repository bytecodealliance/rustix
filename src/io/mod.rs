//! I/O operations.

mod errno;
mod fd;
mod poll;
#[cfg(not(target_os = "wasi"))]
mod socketpair;

pub use crate::io::errno::Errno;
#[cfg(not(target_os = "wasi"))]
pub use fd::dup;
#[cfg(not(target_os = "redox"))]
pub use fd::fionread;
#[cfg(not(target_os = "redox"))]
pub use fd::is_read_write;
pub use fd::isatty;
#[cfg(not(any(target_os = "wasi", target_os = "fuchsia")))]
pub use fd::ttyname;
pub use poll::{PollFd, PollFdVec};
#[cfg(not(target_os = "wasi"))]
pub use socketpair::socketpair_stream;
