//! inotify support for working with inotifies

pub use crate::backend::fs::inotify::{CreateFlags, WatchFlags};
use crate::backend::fs::syscalls;
use crate::fd::{BorrowedFd, OwnedFd};
use crate::io;

/// `inotify_init1(flags)`—Creates a new inotify object.
///
/// Use the [`CreateFlags::CLOEXEC`] flag to prevent the resulting file
/// descriptor from being implicitly passed across `exec` boundaries.
#[doc(alias = "inotify_init1")]
#[inline]
pub fn inotify_init(flags: CreateFlags) -> io::Result<OwnedFd> {
    syscalls::inotify_init1(flags)
}

/// `inotify_add_watch(self, path, flags)`—Adds a watch to inotify.
///
/// This registers or updates a watch for the filesystem path `path` and
/// returns a watch descriptor corresponding to this watch.
///
/// Note: Due to the existence of hardlinks, providing two different paths to
/// this method may result in it returning the same watch descriptor. An
/// application should keep track of this externally to avoid logic errors.
#[inline]
pub fn inotify_add_watch<P: crate::path::Arg>(
    inot: BorrowedFd<'_>,
    path: P,
    flags: WatchFlags,
) -> io::Result<i32> {
    path.into_with_c_str(|path| syscalls::inotify_add_watch(inot, path, flags))
}

/// `inotify_rm_watch(self, wd)`—Removes a watch from this inotify.
///
/// The watch descriptor provided should have previously been returned by
/// [`inotify_add_watch`] and not previously have been removed.
#[doc(alias = "inotify_rm_watch")]
#[inline]
pub fn inotify_remove_watch(inot: BorrowedFd<'_>, wd: i32) -> io::Result<()> {
    syscalls::inotify_rm_watch(inot, wd)
}
