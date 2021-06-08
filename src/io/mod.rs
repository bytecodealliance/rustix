//! I/O operations.

mod errno;
mod fd;
mod ioctl;
mod poll;
#[cfg(not(target_os = "wasi"))]
mod socketpair;

pub use crate::io::errno::Errno;
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
pub use ioctl::ioctl_tcgets;
pub use poll::{PollFd, PollFdVec};
#[cfg(not(target_os = "wasi"))]
pub use socketpair::socketpair_stream;

/// Re-export `termios`.
#[cfg(all(libc, not(target_os = "wasi")))]
pub type Termios = libc::termios;

/// Re-export `termios`.
#[cfg(linux_raw)]
pub type Termios = linux_raw_sys::general::termios;

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
