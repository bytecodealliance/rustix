//! Functions which operate on file descriptors.

use crate::fs::Timestamps;
use crate::io::SeekFrom;
#[cfg(not(target_os = "wasi"))]
use crate::process::{Gid, Uid};
use crate::{imp, io};
use imp::fd::{AsFd, BorrowedFd};
#[cfg(not(any(
    target_os = "dragonfly",
    target_os = "illumos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox"
)))]
use imp::fs::FallocateFlags;
use imp::fs::Stat;
#[cfg(not(any(
    target_os = "illumos",
    target_os = "netbsd",
    target_os = "redox",
    target_os = "wasi"
)))]
// not implemented in libc for netbsd yet
use imp::fs::StatFs;
#[cfg(not(target_os = "wasi"))]
use imp::fs::{FlockOperation, Mode};

/// `lseek(fd, offset, whence)`—Repositions a file descriptor within a file.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/lseek.html
/// [Linux]: https://man7.org/linux/man-pages/man2/lseek.2.html
#[inline]
pub fn seek<Fd: AsFd>(fd: &Fd, pos: SeekFrom) -> io::Result<u64> {
    let fd = fd.as_fd();
    imp::syscalls::seek(fd, pos)
}

/// `lseek(fd, 0, SEEK_CUR)`—Returns the current position within a file.
///
/// Return the current position of the file descriptor. This is a subset of
/// the functionality of `seek`, but this interface makes it easier for users
/// to declare their intent not to mutate any state.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/lseek.html
/// [Linux]: https://man7.org/linux/man-pages/man2/lseek.2.html
#[inline]
pub fn tell<Fd: AsFd>(fd: &Fd) -> io::Result<u64> {
    let fd = fd.as_fd();
    imp::syscalls::tell(fd)
}

/// `fchmod(fd)`—Sets open file or directory permissions.
///
/// Note that this implementation does not support `O_PATH` file descriptors,
/// even on platforms where the host libc emulates it.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/fchmod.html
/// [Linux]: https://man7.org/linux/man-pages/man2/fchmod.2.html
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn fchmod<Fd: AsFd>(fd: &Fd, mode: Mode) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::fchmod(fd, mode)
}

/// `fchown(fd)`—Sets open file or directory ownership.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/fchown.html
/// [Linux]: https://man7.org/linux/man-pages/man2/fchown.2.html
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn fchown<Fd: AsFd>(fd: &Fd, owner: Uid, group: Gid) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::fchown(fd, owner, group)
}

/// `fstat(fd)`—Queries metadata for an open file or directory.
///
/// [`Mode::from_raw_mode`] and [`FileType::from_raw_mode`] may be used to
/// interpret the `st_mode` field.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/fstat.html
/// [Linux]: https://man7.org/linux/man-pages/man2/fstat.2.html
/// [`Mode::from_raw_mode`]: crate::fs::Mode::from_raw_mode
/// [`FileType::from_raw_mode`]: crate::fs::FileType::from_raw_mode
#[inline]
pub fn fstat<Fd: AsFd>(fd: &Fd) -> io::Result<Stat> {
    let fd = fd.as_fd();
    imp::syscalls::fstat(fd)
}

/// `fstatfs(fd)`—Queries filesystem statistics for an open file or directory.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/fstatfs.2.html
#[cfg(not(any(
    target_os = "illumos",
    target_os = "netbsd",
    target_os = "redox",
    target_os = "wasi"
)))] // not implemented in libc for netbsd yet
#[inline]
pub fn fstatfs<Fd: AsFd>(fd: &Fd) -> io::Result<StatFs> {
    let fd = fd.as_fd();
    imp::syscalls::fstatfs(fd)
}

/// `futimens(fd, times)`—Sets timestamps for an open file or directory.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/futimens.html
/// [Linux]: https://man7.org/linux/man-pages/man2/utimensat.2.html
#[inline]
pub fn futimens<Fd: AsFd>(fd: &Fd, times: &Timestamps) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::futimens(fd, times)
}

/// `fallocate(fd, mode, offset, len)`—Adjusts file allocation.
///
/// This is a more general form of `posix_fallocate`, adding a `mode` argument
/// which modifies the behavior. On platforms which only support
/// `posix_fallocate` and not the more general form, no `FallocateFlags` values
/// are defined so it will always be empty.
///
/// # References
///  - [POSIX]
///  - [Linux `fallocate`]
///  - [Linux `posix_fallocate`]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/posix_fallocate.html
/// [Linux `fallocate`]: https://man7.org/linux/man-pages/man2/fallocate.2.html
/// [Linux `posix_fallocate`]: https://man7.org/linux/man-pages/man3/posix_fallocate.3.html
#[cfg(not(any(
    target_os = "dragonfly",
    target_os = "illumos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox"
)))] // not implemented in libc for netbsd yet
#[inline]
#[doc(alias = "posix_fallocate")]
pub fn fallocate<Fd: AsFd>(fd: &Fd, mode: FallocateFlags, offset: u64, len: u64) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::fallocate(fd, mode, offset, len)
}

/// `fcntl(fd, F_GETFL) & O_ACCMODE`
///
/// Returns a pair of booleans indicating whether the file descriptor is
/// readable and/or writable, respectively. This is only reliable on files; for
/// example, it doesn't reflect whether sockets have been shut down; for
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

/// `fsync(fd)`—Ensures that file data and metadata is written to the
/// underlying storage device.
///
/// Note that on iOS and macOS this isn't sufficient to ensure that data has
/// reached persistent storage; use [`fcntl_fullfsync`] to ensure that.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/fsync.html
/// [Linux]: https://man7.org/linux/man-pages/man2/fsync.2.html
/// [`fcntl_fullfsync`]: https://docs.rs/rustix/*/x86_64-apple-darwin/rustix/fs/fn.fcntl_fullfsync.html
#[inline]
pub fn fsync<Fd: AsFd>(fd: &Fd) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::fsync(fd)
}

/// `fdatasync(fd)`—Ensures that file data is written to the underlying
/// storage device.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/fdatasync.html
/// [Linux]: https://man7.org/linux/man-pages/man2/fdatasync.2.html
#[cfg(not(any(
    target_os = "dragonfly",
    target_os = "ios",
    target_os = "macos",
    target_os = "redox"
)))]
#[inline]
pub fn fdatasync<Fd: AsFd>(fd: &Fd) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::fdatasync(fd)
}

/// `ftruncate(fd, length)`—Sets the length of a file.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/ftruncate.html
/// [Linux]: https://man7.org/linux/man-pages/man2/ftruncate.2.html
#[inline]
pub fn ftruncate<Fd: AsFd>(fd: &Fd, length: u64) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::ftruncate(fd, length)
}

/// `flock(fd, operation)`—Acquire or release an advisory lock on an open file.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/flock.2.html
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn flock<Fd: AsFd>(fd: &Fd, operation: FlockOperation) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::flock(fd, operation)
}
