//! `read` and `write`, optionally positioned, optionally vectored

use crate::{imp, io};
use io_lifetimes::AsFd;
use std::io::{IoSlice, IoSliceMut};

/// `RWF_*` constants for use with [`preadv2`] and [`pwritev2`].
#[cfg(any(linux_raw, all(libc, target_os = "linux", target_env = "gnu")))]
pub use imp::io::ReadWriteFlags;

/// `read(fd, buf)`—Reads from a stream.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/read.html
/// [Linux]: https://man7.org/linux/man-pages/man2/read.2.html
#[inline]
pub fn read<Fd: AsFd>(fd: &Fd, buf: &mut [u8]) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::read(fd, buf)
}

/// `write(fd, buf)`—Writes to a stream.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/write.html
/// [Linux]: https://man7.org/linux/man-pages/man2/write.2.html
#[inline]
pub fn write<Fd: AsFd>(fd: &Fd, buf: &[u8]) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::write(fd, buf)
}

/// `pread(fd, buf, offset)`—Reads from a file at a given position.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/pread.html
/// [Linux]: https://man7.org/linux/man-pages/man2/pread.2.html
#[inline]
pub fn pread<Fd: AsFd>(fd: &Fd, buf: &mut [u8], offset: u64) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::pread(fd, buf, offset)
}

/// `pwrite(fd, bufs)`—Writes to a file at a given position.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/pwrite.html
/// [Linux]: https://man7.org/linux/man-pages/man2/pwrite.2.html
#[inline]
pub fn pwrite<Fd: AsFd>(fd: &Fd, buf: &[u8], offset: u64) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::pwrite(fd, buf, offset)
}

/// `readv(fd, bufs)`—Reads from a stream into multiple buffers.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/readv.html
/// [Linux]: https://man7.org/linux/man-pages/man2/readv.2.html
#[inline]
pub fn readv<Fd: AsFd>(fd: &Fd, bufs: &[IoSliceMut]) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::readv(fd, bufs)
}

/// `writev(fd, bufs)`—Writes to a stream from multiple buffers.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/writev.html
/// [Linux]: https://man7.org/linux/man-pages/man2/writev.2.html
#[inline]
pub fn writev<Fd: AsFd>(fd: &Fd, bufs: &[IoSlice]) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::writev(fd, bufs)
}

/// `preadv(fd, bufs, offset)`—Reads from a file at a given position into
/// multiple buffers.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/preadv.2.html
#[inline]
#[cfg(not(target_os = "redox"))]
pub fn preadv<Fd: AsFd>(fd: &Fd, bufs: &[IoSliceMut], offset: u64) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::preadv(fd, bufs, offset)
}

/// `pwritev(fd, bufs, offset)`—Writes to a file at a given position from
/// multiple buffers.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/pwritev.2.html
#[cfg(not(target_os = "redox"))]
#[inline]
pub fn pwritev<Fd: AsFd>(fd: &Fd, bufs: &[IoSlice], offset: u64) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::pwritev(fd, bufs, offset)
}

/// `preadv2(fd, bufs, offset, flags)`—Reads data, with several options.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/preadv2.2.html
#[cfg(any(
    linux_raw,
    all(
        libc,
        target_pointer_width = "64",
        target_os = "linux",
        target_env = "gnu"
    )
))]
#[inline]
pub fn preadv2<Fd: AsFd>(
    fd: &Fd,
    bufs: &[IoSliceMut],
    offset: u64,
    flags: ReadWriteFlags,
) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::preadv2(fd, bufs, offset, flags)
}

/// `pwritev2(fd, bufs, offset, flags)`—Writes data, with several options.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/pwritev2.2.html
#[cfg(any(
    linux_raw,
    all(
        libc,
        target_pointer_width = "64",
        target_os = "linux",
        target_env = "gnu"
    )
))]
#[inline]
pub fn pwritev2<Fd: AsFd>(
    fd: &Fd,
    bufs: &[IoSlice],
    offset: u64,
    flags: ReadWriteFlags,
) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::pwritev2(fd, bufs, offset, flags)
}
