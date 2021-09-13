//! I/O operations.

use crate::imp;
#[cfg(not(target_os = "wasi"))]
use imp::io::Tcflag;

#[allow(unused_imports)]
#[cfg(unix)]
pub(crate) use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[allow(unused_imports)]
#[cfg(target_os = "wasi")]
pub(crate) use std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

mod close;
mod error;
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
mod eventfd;
mod fd;
mod ioctl;
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
mod madvise;
#[cfg(not(target_os = "wasi"))]
mod mmap;
mod owned_fd;
#[cfg(not(target_os = "wasi"))]
mod pipe;
mod poll;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod procfs;
mod read_write;
mod stdio;
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
mod userfaultfd;

pub use close::close;
pub use error::{Error, Result};
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use eventfd::{eventfd, EventfdFlags};
#[cfg(not(target_os = "redox"))]
pub use fd::ioctl_fionread;
#[cfg(not(target_os = "redox"))]
pub use fd::is_read_write;
pub use fd::isatty;
#[cfg(not(any(target_os = "fuchsia", target_os = "wasi")))]
pub use fd::ttyname;
#[cfg(not(target_os = "wasi"))]
pub use fd::{dup, dup2, dup2_with, DupFlags};
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use imp::io::epoll;
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use ioctl::ioctl_fioclex;
pub use ioctl::ioctl_fionbio;
#[cfg(not(target_os = "wasi"))]
pub use ioctl::{ioctl_tcgets, ioctl_tiocgwinsz};
#[cfg(any(
    linux_raw,
    all(libc, not(any(target_os = "redox", target_os = "wasi")))
))]
pub use ioctl::{ioctl_tiocexcl, ioctl_tiocnxcl};
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub use madvise::{madvise, Advice};
#[cfg(not(target_os = "wasi"))]
pub use mmap::{
    mlock, mmap, mmap_anonymous, mprotect, munlock, munmap, MapFlags, MprotectFlags, ProtFlags,
};
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use mmap::{mlock_with, MlockFlags};
pub use owned_fd::OwnedFd;
#[cfg(not(target_os = "wasi"))]
pub use pipe::pipe;
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "wasi")))]
pub use pipe::{pipe_with, PipeFlags};
pub use poll::{poll, PollFd, PollFlags};
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use procfs::proc_self_fd;
pub use read_write::{pread, pwrite, read, readv, write, writev};
#[cfg(not(target_os = "redox"))]
pub use read_write::{preadv, pwritev};
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use read_write::{preadv2, pwritev2, ReadWriteFlags};
pub use stdio::{stderr, stdin, stdout, take_stderr, take_stdin, take_stdout};
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use userfaultfd::{userfaultfd, UserfaultfdFlags};

#[cfg(any(linux_raw, not(target_os = "wasi")))]
pub use imp::io::Termios;

#[cfg(any(linux_raw, all(libc, not(target_os = "wasi"))))]
pub use imp::io::Winsize;

/// `ICANON`
#[cfg(any(linux_raw, all(libc, not(target_os = "wasi"))))]
pub const ICANON: Tcflag = imp::io::ICANON;

/// `PIPE_BUF`—The maximum length at which writes to a pipe are atomic.
///
/// # References
///
///  - [Linux]
///  - [POSIX]
///
/// [Linux]: https://man7.org/linux/man-pages/man7/pipe.7.html
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/write.html
#[cfg(any(
    linux_raw,
    all(libc, not(any(target_os = "redox", target_os = "wasi")))
))]
pub const PIPE_BUF: usize = imp::io::PIPE_BUF;
