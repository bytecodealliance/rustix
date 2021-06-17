//! I/O operations.

mod error;
mod fd;
mod ioctl;
mod pipe;
mod poll;
#[cfg(any(target_os = "macos", target_os = "ios"))]
mod rdadvise;
mod read_write;
#[cfg(not(target_os = "wasi"))]
mod socketpair;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod userfaultfd;

pub use error::{Error, Result};
#[cfg(not(target_os = "wasi"))]
pub use fd::dup;
#[cfg(not(target_os = "redox"))]
pub use fd::ioctl_fionread;
#[cfg(not(target_os = "redox"))]
pub use fd::is_read_write;
pub use fd::isatty;
#[cfg(all(libc, not(any(target_os = "wasi", target_os = "fuchsia"))))]
pub use fd::ttyname;
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use ioctl::ioctl_fioclex;
#[cfg(not(target_os = "wasi"))]
pub use ioctl::{ioctl_tcgets, ioctl_tiocgwinsz};
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use pipe::pipe;
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "wasi")))]
pub use pipe::{pipe2, PipeFlags};
pub use poll::{PollFd, PollFdVec, PollFlags};
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use rdadvise::rdadvise;
pub use read_write::{pread, pwrite, read, readv, write, writev};
#[cfg(not(target_os = "redox"))]
pub use read_write::{preadv, pwritev};
#[cfg(any(linux_raw, all(libc, target_os = "linux", target_env = "gnu")))]
pub use read_write::{preadv2, pwritev2};
#[cfg(not(target_os = "wasi"))]
pub use socketpair::socketpair_stream;
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use userfaultfd::{userfaultfd, UserFaultFdFlags};

/// Re-export `termios`.
#[cfg(all(libc, not(target_os = "wasi")))]
pub type Termios = libc::termios;

/// Re-export `termios`.
#[cfg(linux_raw)]
pub type Termios = linux_raw_sys::general::termios;

/// Re-export `winsize`.
#[cfg(all(libc, not(target_os = "wasi")))]
pub type Winsize = libc::winsize;

/// Re-export `winsize`.
#[cfg(linux_raw)]
pub type Winsize = linux_raw_sys::general::winsize;

/// Re-export `ICANON`.
#[cfg(all(libc, not(target_os = "wasi")))]
pub const ICANON: libc::tcflag_t = libc::ICANON;

/// Re-export `ICANON`.
#[cfg(linux_raw)]
pub const ICANON: std::os::raw::c_uint = linux_raw_sys::general::ICANON;

/// Re-export `PIPE_BUF`.
#[cfg(all(libc, not(any(target_os = "wasi", target_os = "redox"))))]
pub const PIPE_BUF: usize = libc::PIPE_BUF;

/// Re-export `PIPE_BUF`.
#[cfg(linux_raw)]
pub const PIPE_BUF: usize = linux_raw_sys::general::PIPE_BUF as usize;
