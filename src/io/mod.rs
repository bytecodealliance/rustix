//! I/O operations.

mod close;
#[cfg(not(windows))]
mod dup;
mod errno;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod eventfd;
#[cfg(not(windows))]
mod fcntl;
#[cfg(not(feature = "std"))]
pub(crate) mod fd;
mod ioctl;
#[cfg(not(any(windows, target_os = "redox")))]
mod is_read_write;
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "openbsd",
    target_os = "netbsd"
))]
pub mod kqueue;
#[cfg(not(any(windows, target_os = "wasi")))]
mod pipe;
mod poll;
#[cfg(any(target_os = "illumos", target_os = "solaris"))]
pub mod port;
#[cfg(all(feature = "procfs", any(target_os = "android", target_os = "linux")))]
mod procfs;
#[cfg(not(windows))]
mod read_write;
mod seek_from;
#[cfg(not(windows))]
mod stdio;

#[cfg(any(target_os = "android", target_os = "linux"))]
pub use crate::backend::io::epoll;
pub use close::close;
#[cfg(not(any(windows, target_os = "aix", target_os = "wasi")))]
pub use dup::{dup, dup2, dup3, DupFlags};
pub use errno::{retry_on_intr, Errno, Result};
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use eventfd::{eventfd, EventfdFlags};
#[cfg(not(windows))]
pub use fcntl::*;
pub use ioctl::*;
#[cfg(not(any(windows, target_os = "redox")))]
#[cfg(all(feature = "fs", feature = "net"))]
pub use is_read_write::is_read_write;
#[cfg(not(any(windows, target_os = "wasi")))]
pub use pipe::*;
pub use poll::{poll, PollFd, PollFlags};
#[cfg(all(feature = "procfs", any(target_os = "android", target_os = "linux")))]
pub use procfs::*;
#[cfg(not(windows))]
pub use read_write::*;
pub use seek_from::SeekFrom;
#[cfg(not(windows))]
pub use stdio::*;
