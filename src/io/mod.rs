//! I/O operations.

use crate::imp;
#[cfg(not(target_os = "wasi"))]
use imp::io::Tcflag;

#[cfg(unix)]
pub(crate) use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(target_os = "wasi")]
pub(crate) use std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

mod error;
mod fd;
mod ioctl;
#[cfg(not(target_os = "wasi"))]
mod mmap;
mod pipe;
mod poll;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod procfs;
mod read_write;
mod stdio;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod userfaultfd;

pub use error::{Error, Result};
#[cfg(not(target_os = "redox"))]
pub use fd::ioctl_fionread;
#[cfg(not(target_os = "redox"))]
pub use fd::is_read_write;
pub use fd::isatty;
#[cfg(all(libc, not(any(target_os = "wasi", target_os = "fuchsia"))))]
pub use fd::ttyname;
#[cfg(not(target_os = "wasi"))]
pub use fd::{dup, dup2, DupFlags};
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use ioctl::ioctl_fioclex;
#[cfg(not(target_os = "wasi"))]
pub use ioctl::{ioctl_tcgets, ioctl_tiocgwinsz};
#[cfg(not(target_os = "wasi"))]
pub use mmap::{mmap, munmap, MapFlags, ProtFlags};
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use pipe::pipe;
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "wasi")))]
pub use pipe::{pipe2, PipeFlags};
pub use poll::{PollFd, PollFdVec, PollFlags};
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use procfs::{proc, proc_self, proc_self_fd};
pub use read_write::{pread, pwrite, read, readv, write, writev};
#[cfg(not(target_os = "redox"))]
pub use read_write::{preadv, pwritev};
#[cfg(any(
    linux_raw,
    all(
        libc,
        target_pointer_width = "64",
        target_os = "linux",
        target_env = "gnu"
    )
))]
pub use read_write::{preadv2, pwritev2, ReadWriteFlags};
pub use stdio::{stderr, stdin, stdout};
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use userfaultfd::{userfaultfd, UserFaultFdFlags};

#[cfg(any(linux_raw, not(target_os = "wasi")))]
pub use imp::io::Termios;

#[cfg(any(linux_raw, all(libc, not(target_os = "wasi"))))]
pub use imp::io::Winsize;

/// `ICANON`
#[cfg(any(linux_raw, all(libc, not(target_os = "wasi"))))]
pub const ICANON: Tcflag = imp::io::ICANON;

/// `PIPE_BUF`
#[cfg(any(
    linux_raw,
    all(libc, not(any(target_os = "wasi", target_os = "redox")))
))]
pub const PIPE_BUF: usize = imp::io::PIPE_BUF;
