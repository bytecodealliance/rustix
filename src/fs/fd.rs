//! Functions which operate on file descriptors.

use crate::{imp, io};
#[cfg(not(target_os = "wasi"))]
use imp::fs::Mode;
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "wasi")))]
// not implemented in libc for netbsd yet
use imp::fs::StatFs;
use imp::{fs::Stat, time::Timespec};
use io_lifetimes::{AsFd, BorrowedFd};
use std::io::SeekFrom;

/// `lseek(fd, offset, whence)`
#[inline]
pub fn seek<Fd: AsFd>(fd: &Fd, pos: SeekFrom) -> io::Result<u64> {
    let fd = fd.as_fd();
    imp::syscalls::seek(fd, pos)
}

/// `lseek(fd, 0, SEEK_CUR)`
#[inline]
pub fn tell<Fd: AsFd>(fd: &Fd) -> io::Result<u64> {
    let fd = fd.as_fd();
    imp::syscalls::tell(fd)
}

/// `fchmod(fd)`.
///
/// Note that this implementation does not support `O_PATH` file descriptors,
/// even on platforms where the host libc emulates it.
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn fchmod<Fd: AsFd>(fd: &Fd, mode: Mode) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::fchmod(fd, mode)
}

/// `fstat(fd)`
#[inline]
pub fn fstat<Fd: AsFd>(fd: &Fd) -> io::Result<Stat> {
    let fd = fd.as_fd();
    imp::syscalls::fstat(fd)
}

/// `fstatfs(fd)`
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "wasi")))] // not implemented in libc for netbsd yet
#[inline]
pub fn fstatfs<Fd: AsFd>(fd: &Fd) -> io::Result<StatFs> {
    let fd = fd.as_fd();
    imp::syscalls::fstatfs(fd)
}

/// `futimens(fd, times)`
#[inline]
pub fn futimens<Fd: AsFd>(fd: &Fd, times: &[Timespec; 2]) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::futimens(fd, times)
}

/// `posix_fallocate(fd, offset, len)`
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "openbsd")))] // not implemented in libc for netbsd yet
#[inline]
pub fn posix_fallocate<Fd: AsFd>(fd: &Fd, offset: u64, len: u64) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::posix_fallocate(fd, offset, len)
}

/// `fcntl(fd, F_GETFL) & O_ACCMODE`.
///
/// Returns a pair of booleans indicating whether the file descriptor is
/// readable and/or writeable, respectively. This is only reliable on files;
/// for example, it doesn't reflect whether sockets have been shut down; for
/// general I/O handle support, use [`io::is_read_write`].
#[inline]
pub fn is_file_read_write<Fd: AsFd>(fd: &Fd) -> io::Result<(bool, bool)> {
    let fd = fd.as_fd();
    _is_file_read_write(fd)
}

pub(crate) fn _is_file_read_write(fd: BorrowedFd<'_>) -> io::Result<(bool, bool)> {
    let mode = imp::syscalls::fcntl_getfl(fd)?;

    // Check for `O_PATH`.
    #[cfg(any(
        target_os = "android",
        target_os = "fuchsia",
        target_os = "linux",
        target_os = "emscripten"
    ))]
    if mode.contains(crate::fs::OFlags::PATH) {
        return Ok((false, false));
    }

    // Use `RWMODE` rather than `ACCMODE` as `ACCMODE` may include `O_PATH`.
    // We handled `O_PATH` above.
    match mode & crate::fs::OFlags::RWMODE {
        crate::fs::OFlags::RDONLY => Ok((true, false)),
        crate::fs::OFlags::RDWR => Ok((true, true)),
        crate::fs::OFlags::WRONLY => Ok((false, true)),
        _ => unreachable!(),
    }
}

/// `fsync(fd)`
#[inline]
pub fn fsync<Fd: AsFd>(fd: &Fd) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::fsync(fd)
}

/// `fdatasync(fd)`
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "redox")))]
#[inline]
pub fn fdatasync<Fd: AsFd>(fd: &Fd) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::fdatasync(fd)
}

/// `ftruncate(fd, length)`
#[inline]
pub fn ftruncate<Fd: AsFd>(fd: &Fd, length: u64) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::ftruncate(fd, length)
}
