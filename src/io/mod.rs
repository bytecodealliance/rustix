//! I/O operations.

#[cfg(not(windows))]
use crate::imp;
#[cfg(not(windows))]
#[cfg(not(target_os = "wasi"))]
use imp::io::Tcflag;

mod close;
#[cfg(not(windows))]
mod dup;
mod error;
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
mod eventfd;
#[cfg(not(feature = "std"))]
pub(crate) mod fd;
mod ioctl;
#[cfg(not(any(windows, target_os = "redox")))]
mod is_read_write;
#[cfg(not(any(windows, target_os = "redox", target_os = "wasi")))]
mod madvise;
#[cfg(not(any(windows, target_os = "wasi")))]
mod mmap;
#[cfg(not(any(windows, target_os = "wasi")))]
mod msync;
mod owned_fd;
#[cfg(not(any(windows, target_os = "wasi")))]
mod pipe;
#[cfg(not(windows))]
mod poll;
#[cfg(all(feature = "procfs", any(target_os = "android", target_os = "linux")))]
mod procfs;
#[cfg(not(windows))]
mod read_write;
#[cfg(not(feature = "std"))]
mod seek_from;
#[cfg(not(windows))]
mod stdio;
#[cfg(not(windows))]
mod tty;
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
mod userfaultfd;

pub use close::close;
#[cfg(not(any(windows, target_os = "wasi")))]
pub use dup::{dup, dup2, dup2_with, DupFlags};
pub use error::{with_retrying, Error, Result};
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use eventfd::{eventfd, EventfdFlags};
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use imp::io::epoll;
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use ioctl::ioctl_fioclex;
pub use ioctl::ioctl_fionbio;
#[cfg(not(any(windows, target_os = "redox")))]
pub use ioctl::ioctl_fionread;
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use ioctl::{ioctl_blkpbszget, ioctl_blksszget};
#[cfg(not(any(windows, target_os = "wasi")))]
pub use ioctl::{ioctl_tcgets, ioctl_tiocgwinsz};
#[cfg(any(
    linux_raw,
    all(libc, not(any(windows, target_os = "redox", target_os = "wasi")))
))]
pub use ioctl::{ioctl_tiocexcl, ioctl_tiocnxcl};
#[cfg(not(any(windows, target_os = "redox")))]
pub use is_read_write::is_read_write;
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
#[cfg(not(any(windows, target_os = "wasi")))]
pub use msync::{msync, MsyncFlags};
pub use owned_fd::OwnedFd;
#[cfg(not(any(windows, target_os = "wasi")))]
pub use pipe::pipe;
#[cfg(not(any(windows, target_os = "ios", target_os = "macos", target_os = "wasi")))]
pub use pipe::{pipe_with, PipeFlags};
#[cfg(not(windows))]
pub use poll::{poll, PollFd, PollFlags};
#[cfg(all(feature = "procfs", any(target_os = "android", target_os = "linux")))]
pub use procfs::{proc_self_fd, proc_self_fdinfo_fd, proc_self_maps, proc_self_pagemap};
#[cfg(not(windows))]
pub use read_write::{pread, pwrite, read, readv, write, writev};
#[cfg(not(any(windows, target_os = "redox")))]
pub use read_write::{preadv, pwritev};
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use read_write::{preadv2, pwritev2, ReadWriteFlags};
#[cfg(not(windows))]
pub use stdio::{stderr, stdin, stdout, take_stderr, take_stdin, take_stdout};
#[cfg(not(windows))]
pub use tty::isatty;
#[cfg(any(
    all(linux_raw, feature = "procfs"),
    all(libc, not(any(windows, target_os = "fuchsia", target_os = "wasi")))
))]
pub use tty::ttyname;
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use userfaultfd::{userfaultfd, UserfaultfdFlags};

#[cfg(any(linux_raw, not(any(windows, target_os = "wasi"))))]
pub use imp::io::Termios;

#[cfg(any(linux_raw, all(libc, not(any(windows, target_os = "wasi")))))]
pub use imp::io::Winsize;

// Declare `IoSlice` and `IoSliceMut`.
#[cfg(not(windows))]
#[cfg(not(feature = "std"))]
pub use imp::io::{IoSlice, IoSliceMut};
#[cfg(not(windows))]
#[cfg(feature = "std")]
pub use std::io::{IoSlice, IoSliceMut};

// Declare `SeekFrom`.
#[cfg(not(feature = "std"))]
pub use seek_from::SeekFrom;
#[cfg(feature = "std")]
pub use std::io::SeekFrom;

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
    all(
        libc,
        not(any(
            windows,
            target_os = "illumos",
            target_os = "redox",
            target_os = "wasi"
        ))
    )
))]
pub const PIPE_BUF: usize = imp::io::PIPE_BUF;
