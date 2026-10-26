use crate::{backend, io};
use backend::fd::AsFd;
use backend::fs::types::SyncFileRangeFlags;

/// `sync_file_range(fd, offset, nbytes, flags)`—Start or wait for the
/// write-out of a range of a file's pages.
///
/// A `nbytes` of 0 means to the end of the file. `flags` chooses whether to
/// wait for write-out already in flight in the range before starting
/// (`WAIT_BEFORE`), to start write-out of the range's dirty pages (`WRITE`),
/// and to wait for the range's write-out to finish (`WAIT_AFTER`); with no
/// flags the call does nothing.
///
/// This acts on the page cache alone. It writes no metadata and does not
/// flush the disk's write cache, so it does not make data durable the way
/// [`fsync`] and [`fdatasync`] do. It is for pacing write-back of a file that
/// is being written without blocking the writer, and for waiting on a range.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/sync_file_range.2.html
/// [`fsync`]: crate::fs::fsync
/// [`fdatasync`]: crate::fs::fdatasync
#[inline]
pub fn sync_file_range<Fd: AsFd>(
    fd: Fd,
    offset: u64,
    nbytes: u64,
    flags: SyncFileRangeFlags,
) -> io::Result<()> {
    backend::fs::syscalls::sync_file_range(fd.as_fd(), offset, nbytes, flags)
}
