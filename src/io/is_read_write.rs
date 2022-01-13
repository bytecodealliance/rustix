//! The [`is_read_write`] function.

use crate::{imp, io};
use imp::fd::AsFd;

/// Returns a pair of booleans indicating whether the file descriptor is
/// readable and/or writable, respectively.
///
/// Unlike [`is_file_read_write`], this correctly detects whether sockets
/// have been shutdown, partially or completely.
///
/// [`is_file_read_write`]: crate::fs::is_file_read_write
#[inline]
pub fn is_read_write<Fd: AsFd>(fd: &Fd) -> io::Result<(bool, bool)> {
    let fd = fd.as_fd();
    imp::syscalls::is_read_write(fd)
}
