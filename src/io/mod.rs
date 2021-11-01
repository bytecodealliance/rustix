//! I/O operations.

use crate::imp;
#[cfg(not(windows))]
#[cfg(not(target_os = "wasi"))]
use imp::io::Tcflag;

#[cfg(windows)]
pub(crate) use imp::net::io_lifetimes::{AsFd, AsSocketAsFd, BorrowedFd};
#[allow(unused_imports)]
#[cfg(windows)]
pub(crate) use imp::net::io_lifetimes::{AsRawFd, FromRawFd, IntoRawFd, LibcFd, OwnedFd, RawFd};
#[cfg(windows)]
#[cfg(not(io_lifetimes_use_std))]
pub(crate) use imp::net::io_lifetimes::{FromFd, IntoFd};
#[cfg(not(windows))]
pub(crate) use io_lifetimes::{AsFd, BorrowedFd};
#[cfg(not(windows))]
#[cfg(not(io_lifetimes_use_std))]
pub(crate) use io_lifetimes::{FromFd, IntoFd};
#[allow(unused_imports)]
#[cfg(unix)]
pub(crate) use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd, RawFd as LibcFd};
#[allow(unused_imports)]
#[cfg(target_os = "wasi")]
pub(crate) use {
    libc::c_int as LibcFd,
    std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd},
};

#[cfg(not(windows))]
mod close;
mod error;
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
mod eventfd;
#[cfg(not(windows))]
mod fd;
mod ioctl;
#[cfg(not(any(windows, target_os = "redox", target_os = "wasi")))]
mod madvise;
#[cfg(not(any(windows, target_os = "wasi")))]
mod mmap;
#[cfg(not(windows))]
mod owned_fd;
#[cfg(not(any(windows, target_os = "wasi")))]
mod pipe;
#[cfg(not(windows))]
mod poll;
#[cfg(all(feature = "procfs", any(target_os = "android", target_os = "linux")))]
mod procfs;
#[cfg(not(windows))]
mod read_write;
#[cfg(not(windows))]
mod stdio;
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
mod userfaultfd;

#[cfg(not(windows))]
pub use close::close;
pub use error::{Error, Result};
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use eventfd::{eventfd, EventfdFlags};
#[cfg(not(any(windows, target_os = "redox")))]
pub use fd::ioctl_fionread;
#[cfg(not(any(windows, target_os = "redox")))]
pub use fd::is_read_write;
#[cfg(not(windows))]
pub use fd::isatty;
#[cfg(any(
    all(linux_raw, feature = "procfs"),
    all(libc, not(any(windows, target_os = "fuchsia", target_os = "wasi")))
))]
pub use fd::ttyname;
#[cfg(not(any(windows, target_os = "wasi")))]
pub use fd::{dup, dup2, dup2_with, DupFlags};
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use imp::io::epoll;
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use ioctl::ioctl_fioclex;
pub use ioctl::ioctl_fionbio;
#[cfg(not(any(windows, target_os = "wasi")))]
pub use ioctl::{ioctl_tcgets, ioctl_tiocgwinsz};
#[cfg(any(
    linux_raw,
    all(libc, not(any(windows, target_os = "redox", target_os = "wasi")))
))]
pub use ioctl::{ioctl_tiocexcl, ioctl_tiocnxcl};
#[cfg(not(any(windows, target_os = "redox", target_os = "wasi")))]
pub use madvise::{madvise, Advice};
#[cfg(not(any(windows, target_os = "wasi")))]
pub use mmap::{
    mlock, mmap, mmap_anonymous, mprotect, munlock, munmap, MapFlags, MprotectFlags, ProtFlags,
};
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use mmap::{mlock_with, MlockFlags};
#[cfg(any(linux_raw, all(libc, target_os = "linux")))]
pub use mmap::{mremap, mremap_fixed, MremapFlags};
#[cfg(not(windows))]
pub use owned_fd::OwnedFd;
#[cfg(not(any(windows, target_os = "wasi")))]
pub use pipe::pipe;
#[cfg(not(any(windows, target_os = "ios", target_os = "macos", target_os = "wasi")))]
pub use pipe::{pipe_with, PipeFlags};
#[cfg(not(windows))]
pub use poll::{poll, PollFd, PollFlags};
#[cfg(all(feature = "procfs", any(target_os = "android", target_os = "linux")))]
pub use procfs::proc_self_fd;
#[cfg(not(windows))]
pub use read_write::{pread, pwrite, read, readv, write, writev};
#[cfg(not(any(windows, target_os = "redox")))]
pub use read_write::{preadv, pwritev};
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use read_write::{preadv2, pwritev2, ReadWriteFlags};
#[cfg(not(windows))]
pub use stdio::{stderr, stdin, stdout, take_stderr, take_stdin, take_stdout};
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use userfaultfd::{userfaultfd, UserfaultfdFlags};

#[cfg(any(linux_raw, not(any(windows, target_os = "wasi"))))]
pub use imp::io::Termios;

#[cfg(any(linux_raw, all(libc, not(any(windows, target_os = "wasi")))))]
pub use imp::io::Winsize;

/// `ICANON`
#[cfg(any(linux_raw, all(libc, not(any(windows, target_os = "wasi")))))]
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
    all(libc, not(any(windows, target_os = "redox", target_os = "wasi")))
))]
pub const PIPE_BUF: usize = imp::io::PIPE_BUF;
