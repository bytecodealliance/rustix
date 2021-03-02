//! Filesystem operations.

#[cfg(target_os = "android")]
mod android;
#[cfg(not(target_os = "redox"))]
mod at;
mod constants;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod copy_file_range;
#[cfg(not(target_os = "redox"))]
mod dir;
#[cfg(not(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "redox"
)))]
mod fadvise;
pub(crate) mod fcntl;
#[cfg(any(target_os = "macos", target_os = "ios"))]
mod fcopyfile;
pub(crate) mod fd;
mod file_type;
#[cfg(any(target_os = "macos", target_os = "ios"))]
mod getpath;
#[cfg(not(any(
    target_os = "ios",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "macos",
    target_os = "redox",
    target_os = "wasi"
)))]
mod makedev;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod openat2;
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "redox")))]
// Most Modern OS's have `preadv`/`pwritev`.
mod pv;
#[cfg(any(target_os = "macos", target_os = "ios"))]
mod rdadvise;
#[cfg(all(target_os = "linux", target_env = "gnu"))]
mod statx;

#[cfg(not(any(target_os = "wasi", target_os = "redox")))]
pub use at::chmodat;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use at::fclonefileat;
#[cfg(not(target_os = "redox"))]
pub use at::{
    accessat, cwd, linkat, mkdirat, openat, readlinkat, renameat, statat, symlinkat, unlinkat,
    utimensat,
};
#[cfg(not(target_os = "redox"))]
pub use constants::AtFlags;
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use constants::ResolveFlags;
pub use constants::{Access, FdFlags, Mode, OFlags};
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use constants::{CloneFlags, CopyfileFlags};
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use copy_file_range::copy_file_range;
#[cfg(not(target_os = "redox"))]
pub use dir::{Dir, Entry, SeekLoc};
#[cfg(not(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "redox"
)))]
pub use fadvise::{fadvise, Advice};
#[cfg(not(any(
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
pub use fcntl::get_seals;
pub use fcntl::{getfd, getfl, setfd, setfl};
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use fcopyfile::{
    copyfile_state_alloc, copyfile_state_free, copyfile_state_get_copied, copyfile_state_t,
    fcopyfile,
};
#[cfg(not(target_os = "wasi"))]
pub use fd::fchmod;
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "wasi")))]
// not implemented in libc for netbsd yet
pub use fd::fstatfs;
#[cfg(not(any(target_os = "netbsd", target_os = "redox")))]
pub use fd::posix_fallocate;
pub use fd::{futimens, is_file_read_write, seek, tell};
pub use file_type::FileType;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use getpath::getpath;
#[cfg(not(any(
    target_os = "ios",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "macos",
    target_os = "redox",
    target_os = "wasi"
)))]
pub use makedev::{major, makedev, minor};
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use openat2::openat2;
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "redox")))]
pub use pv::{preadv, pwritev};
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use rdadvise::rdadvise;
#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub use statx::statx;

/// Re-export `libc::stat` (or `libc::stat64` where applicable).
#[cfg(not(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re"
)))]
pub type LibcStat = libc::stat;

/// Re-export `libc::statfs` (or `libc::statfs64` where applicable).
#[cfg(not(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re",
    target_os = "netbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
#[allow(clippy::module_name_repetitions)]
pub type LibcStatFs = libc::statfs;

/// Re-export `libc::stat` (or `libc::stat64` where applicable).
#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re"
))]
pub type LibcStat = libc::stat64;

/// Re-export `libc::statfs` (or `libc::statfs64` where applicable).
#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re"
))]
pub type LibcStatFs = libc::statfs64;

/// Re-export `libc::statx`. Only available on Linux with GLIBC for now.
#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub type LibcStatx = libc::statx;

/// Re-export types common to Posix-ish platforms.
#[cfg(unix)]
pub use std::os::unix::fs::{DirEntryExt, FileExt, FileTypeExt, MetadataExt, OpenOptionsExt};

/// Re-export types common to Posix-ish platforms.
#[cfg(target_os = "wasi")]
pub use std::os::wasi::fs::{DirEntryExt, FileExt, FileTypeExt, MetadataExt, OpenOptionsExt};
