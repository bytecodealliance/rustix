//! Filesystem operations.

use crate::imp;
use imp::time::Nsecs;

mod abs;
#[cfg(not(target_os = "redox"))]
mod at;
mod constants;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod copy_file_range;
#[cfg(not(target_os = "redox"))]
mod cwd;
#[cfg(not(target_os = "redox"))]
mod dir;
#[cfg(not(any(
    target_os = "dragonfly",
    target_os = "illumos",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox"
)))]
mod fadvise;
pub(crate) mod fcntl;
#[cfg(any(target_os = "ios", target_os = "macos"))]
mod fcntl_darwin;
#[cfg(any(target_os = "ios", target_os = "macos"))]
mod fcopyfile;
pub(crate) mod fd;
mod file_type;
#[cfg(any(target_os = "ios", target_os = "macos"))]
mod getpath;
#[cfg(not(any(
    target_os = "dragonfly",
    target_os = "illumos",
    target_os = "ios",
    target_os = "freebsd",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi"
)))]
mod makedev;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod memfd_create;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod openat2;
#[cfg(target_os = "linux")]
mod sendfile;
#[cfg(all(target_os = "linux", target_env = "gnu"))]
mod statx;

#[cfg(not(any(
    target_os = "illumos",
    target_os = "netbsd",
    target_os = "redox",
    target_os = "wasi"
)))]
pub use abs::statfs;
#[cfg(not(any(target_os = "illumos", target_os = "redox")))]
pub use at::accessat;
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use at::fclonefileat;
#[cfg(not(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "redox",
    target_os = "wasi",
)))]
pub use at::mknodat;
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use at::renameat_with;
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub use at::{chmodat, chownat};
#[cfg(not(target_os = "redox"))]
pub use at::{
    linkat, mkdirat, openat, readlinkat, renameat, statat, symlinkat, unlinkat, utimensat,
};
#[cfg(not(target_os = "redox"))]
pub use constants::AtFlags;
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use constants::CloneFlags;
/// `copyfile_flags_t`
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use constants::CopyfileFlags;
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use constants::RenameFlags;
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use constants::ResolveFlags;
pub use constants::{Access, FdFlags, Mode, OFlags};
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use copy_file_range::copy_file_range;
#[cfg(not(target_os = "redox"))]
pub use cwd::cwd;
#[cfg(not(target_os = "redox"))]
pub use dir::{Dir, DirEntry};
#[cfg(not(any(
    target_os = "dragonfly",
    target_os = "illumos",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox"
)))]
pub use fadvise::{fadvise, Advice};
#[cfg(not(target_os = "wasi"))]
pub use fcntl::fcntl_dupfd_cloexec;
#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "fuchsia",
    target_os = "freebsd",
))]
pub use fcntl::{fcntl_add_seals, fcntl_get_seals, SealFlags};
pub use fcntl::{fcntl_getfd, fcntl_getfl, fcntl_setfd, fcntl_setfl};
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use fcntl_darwin::{fcntl_fullfsync, fcntl_rdadvise};
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use fcopyfile::{
    copyfile_state_alloc, copyfile_state_free, copyfile_state_get, copyfile_state_get_copied,
    copyfile_state_t, fcopyfile,
};
#[cfg(not(any(
    target_os = "dragonfly",
    target_os = "illumos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox"
)))]
pub use fd::fallocate;
#[cfg(not(any(
    target_os = "dragonfly",
    target_os = "ios",
    target_os = "macos",
    target_os = "redox"
)))]
pub use fd::fdatasync;
#[cfg(not(any(
    target_os = "illumos",
    target_os = "netbsd",
    target_os = "redox",
    target_os = "wasi"
)))]
// not implemented in libc for netbsd yet
pub use fd::fstatfs;
#[cfg(not(target_os = "wasi"))]
pub use fd::{fchmod, fchown, flock};
pub use fd::{fstat, fsync, ftruncate, futimens, is_file_read_write, seek, tell};
pub use file_type::FileType;
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use getpath::getpath;
#[cfg(not(any(
    target_os = "dragonfly",
    target_os = "illumos",
    target_os = "ios",
    target_os = "freebsd",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi"
)))]
pub use makedev::{major, makedev, minor};
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use memfd_create::{memfd_create, MemfdFlags};
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use openat2::openat2;
#[cfg(target_os = "linux")]
pub use sendfile::sendfile;
#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub use statx::{statx, StatxFlags};

pub use imp::fs::Stat;

#[cfg(not(any(
    target_os = "illumos",
    target_os = "netbsd",
    target_os = "redox",
    target_os = "wasi"
)))]
pub use imp::fs::StatFs;

#[cfg(any(linux_raw, all(libc, target_os = "linux", target_env = "gnu")))]
pub use imp::fs::Statx;

#[cfg(not(any(
    target_os = "illumos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox"
)))]
pub use imp::fs::FallocateFlags;

/// `UTIME_NOW` for use with [`utimensat`].
///
/// [`utimensat`]: crate::fs::utimensat
#[cfg(any(linux_raw, all(libc, not(target_os = "redox"))))]
pub const UTIME_NOW: Nsecs = imp::fs::UTIME_NOW as Nsecs;

/// `UTIME_OMIT` for use with [`utimensat`].
///
/// [`utimensat`]: crate::fs::utimensat
#[cfg(any(linux_raw, all(libc, not(target_os = "redox"))))]
pub const UTIME_OMIT: Nsecs = imp::fs::UTIME_OMIT as Nsecs;

#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use imp::fs::FsWord;

/// The filesystem magic number for procfs.
///
/// See [the `fstatfs` man page] for more information.
///
/// [the `fstatfs` man page]: https://man7.org/linux/man-pages/man2/fstatfs.2.html#DESCRIPTION
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub const PROC_SUPER_MAGIC: FsWord = imp::fs::PROC_SUPER_MAGIC;

/// The filesystem magic number for NFS.
///
/// See [the `fstatfs` man page] for more information.
///
/// [the `fstatfs` man page]: https://man7.org/linux/man-pages/man2/fstatfs.2.html#DESCRIPTION
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub const NFS_SUPER_MAGIC: FsWord = imp::fs::NFS_SUPER_MAGIC;

#[cfg(not(target_os = "wasi"))]
pub use imp::fs::FlockOperation;
pub use imp::fs::{Dev, RawMode};

/// Timestamps used by [`utimensat`] and [`futimens`].
//
// This is `repr(c)` and specifically laid out to match the representation
// used by `utimensat` and `futimens`, which expect 2-element arrays of
// timestamps.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Timestamps {
    /// The timestamp of the last access to a filesystem object.
    pub last_access: crate::time::Timespec,

    /// The timestamp of the last modification of a filesystem object.
    pub last_modification: crate::time::Timespec,
}

/// Re-export types common to POSIX-ish platforms.
#[cfg(feature = "std")]
#[cfg(unix)]
pub use std::os::unix::fs::{DirEntryExt, FileExt, FileTypeExt, MetadataExt, OpenOptionsExt};
#[cfg(feature = "std")]
#[cfg(target_os = "wasi")]
pub use std::os::wasi::fs::{DirEntryExt, FileExt, FileTypeExt, MetadataExt, OpenOptionsExt};
