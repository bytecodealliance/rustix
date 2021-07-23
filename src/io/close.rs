//! The unsafe `close` for raw file descriptors.
#![allow(unsafe_code)]

use crate::{imp, io::RawFd};

/// `close(raw_fd)`—Closes a `RawFd` directly.
///
/// Most users won't need to use this, as `OwnedFd` automatically closes its
/// file descriptor on `Drop`.
///
/// # Safety
///
/// This function takes a `RawFd`, which must be valid before the call, and is
/// not valid after the call.
pub unsafe fn close(raw_fd: RawFd) {
    imp::syscalls::close(raw_fd)
}
