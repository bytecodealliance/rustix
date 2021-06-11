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
    target_os = "openbsd",
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
    target_os = "openbsd",
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
    target_os = "openbsd",
    target_os = "redox"
)))]
pub use fadvise::{fadvise, Advice};
#[cfg(not(any(
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
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
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "openbsd")))]
pub use fd::posix_fallocate;
pub use fd::{futimens, is_file_read_write, seek, tell};
pub use file_type::FileType;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use getpath::getpath;
#[cfg(not(any(
    target_os = "ios",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
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
pub type Stat = libc::stat;

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
pub type StatFs = libc::statfs;

/// Re-export `libc::stat` (or `libc::stat64` where applicable).
#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re"
))]
pub type Stat = libc::stat64;

/// Re-export `libc::statfs` (or `libc::statfs64` where applicable).
#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re"
))]
pub type StatFs = libc::statfs64;

/// Re-export `libc::statx`. Only available on Linux with GLIBC for now.
#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub type Statx = libc::statx;

/// Re-export `UTIME_NOW` and `UTIME_OMIT`.
pub use libc::{UTIME_NOW, UTIME_OMIT};

/// Re-export `__fsword_t`.
#[cfg(all(target_os = "linux", not(target_env = "musl")))]
pub type FsWord = libc::__fsword_t;

/// Re-export `__fsword_t`.
#[cfg(all(
    any(target_os = "android", all(target_os = "linux", target_env = "musl")),
    target_pointer_width = "32"
))]
pub type FsWord = u32;

/// Re-export `__fsword_t`.
#[cfg(all(
    any(target_os = "android", all(target_os = "linux", target_env = "musl")),
    target_pointer_width = "64"
))]
pub type FsWord = u64;

#[cfg(target_os = "linux")]
pub type FsWord = libc::__fsword_t;

/// The filesystem magic number for procfs.
/// <https://man7.org/linux/man-pages/man2/fstatfs.2.html#DESCRIPTION>
#[cfg(all(
    any(target_os = "android", target_os = "linux"),
    not(target_env = "musl")
))]
pub const PROC_SUPER_MAGIC: FsWord = libc::PROC_SUPER_MAGIC as FsWord;

/// The filesystem magic number for procfs.
/// <https://man7.org/linux/man-pages/man2/fstatfs.2.html#DESCRIPTION>
#[cfg(all(any(target_os = "android", target_os = "linux"), target_env = "musl"))]
pub const PROC_SUPER_MAGIC: FsWord = 0x0000_9fa0;

/// Re-export `PROC_SUPER_MAGIC`.
#[cfg(target_os = "linux")]
pub use libc::PROC_SUPER_MAGIC;

/// Re-export `mode_t`.
#[cfg(libc)]
pub type RawMode = libc::mode_t;

/// Re-export `mode_t`.
#[cfg(linux)]
pub type RawMode = linux_headers_sys::stat::__kernel_mode_t;

/// Re-export types common to Posix-ish platforms.
#[cfg(unix)]
pub use std::os::unix::fs::{DirEntryExt, FileExt, FileTypeExt, MetadataExt, OpenOptionsExt};

/// Re-export types common to Posix-ish platforms.
#[cfg(target_os = "wasi")]
pub use std::os::wasi::fs::{DirEntryExt, FileExt, FileTypeExt, MetadataExt, OpenOptionsExt};
