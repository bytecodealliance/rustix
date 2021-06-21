//! Filesystem operations.

#[cfg(not(target_os = "redox"))]
mod cwd;
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
mod memfd_create;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod openat2;
#[cfg(target_os = "linux")]
mod sendfile;
#[cfg(all(target_os = "linux", target_env = "gnu"))]
mod statx;

#[cfg(not(target_os = "redox"))]
pub use cwd::cwd;
#[cfg(not(any(target_os = "wasi", target_os = "redox")))]
pub use at::chmodat;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use at::fclonefileat;
#[cfg(not(target_os = "redox"))]
pub use at::{
    accessat, linkat, mkdirat, openat, readlinkat, renameat, statat, symlinkat, unlinkat,
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
pub use dir::{Dir, Entry};
#[cfg(not(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox"
)))]
pub use fadvise::{fadvise, Advice};
#[cfg(not(target_os = "wasi"))]
pub use fcntl::fcntl_dupfd_cloexec;
#[cfg(not(any(
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
pub use fcntl::fcntl_get_seals;
pub use fcntl::{fcntl_getfd, fcntl_getfl, fcntl_setfd, fcntl_setfl};
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use fcopyfile::{
    copyfile_state_alloc, copyfile_state_free, copyfile_state_get_copied, copyfile_state_t,
    fcopyfile,
};
#[cfg(not(target_os = "wasi"))]
pub use fd::fchmod;
#[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "redox")))]
pub use fd::fdatasync;
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "wasi")))]
// not implemented in libc for netbsd yet
pub use fd::fstatfs;
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "openbsd")))]
pub use fd::posix_fallocate;
pub use fd::{fstat, fsync, ftruncate, futimens, is_file_read_write, seek, tell};
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
pub use memfd_create::{memfd_create, MemfdFlags};
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use openat2::openat2;
#[cfg(target_os = "linux")]
pub use sendfile::sendfile;
#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub use statx::statx;

/// Re-export `stat` (or `stat64` where applicable).
#[cfg(all(
    libc,
    not(any(
        target_os = "android",
        target_os = "linux",
        target_os = "emscripten",
        target_os = "l4re"
    ))
))]
pub type Stat = libc::stat;

/// Re-export `stat` (or `stat64` where applicable).
#[cfg(all(
    libc,
    any(
        target_os = "android",
        target_os = "linux",
        target_os = "emscripten",
        target_os = "l4re"
    )
))]
pub type Stat = libc::stat64;

/// Re-export `stat` (or `stat64` where applicable).
#[cfg(all(linux_raw, target_pointer_width = "32"))]
pub type Stat = linux_raw_sys::general::stat64;

/// Re-export `stat` (or `stat64` where applicable).
#[cfg(all(linux_raw, target_pointer_width = "64"))]
pub type Stat = linux_raw_sys::general::stat;

/// Re-export `statfs` (or `statfs64` where applicable).
#[cfg(all(
    libc,
    not(any(
        target_os = "android",
        target_os = "linux",
        target_os = "emscripten",
        target_os = "l4re",
        target_os = "netbsd",
        target_os = "redox",
        target_os = "wasi",
    ))
))]
#[allow(clippy::module_name_repetitions)]
pub type StatFs = libc::statfs;

/// Re-export `stat` (or `stat64` where applicable).
#[cfg(all(
    libc,
    any(
        target_os = "android",
        target_os = "linux",
        target_os = "emscripten",
        target_os = "l4re"
    )
))]
pub type StatFs = libc::statfs64;

/// Re-export `statfs` (or `statfs64` where applicable).
#[cfg(all(linux_raw, target_pointer_width = "32"))]
#[allow(clippy::module_name_repetitions)]
pub type StatFs = linux_raw_sys::general::statfs64;

/// Re-export `statfs` (or `statfs64` where applicable).
#[cfg(all(linux_raw, target_pointer_width = "64"))]
#[allow(clippy::module_name_repetitions)]
pub type StatFs = linux_raw_sys::general::statfs64;

/// Re-export `statx`. Only available on Linux with GLIBC for now.
#[cfg(all(libc, all(target_os = "linux", target_env = "gnu")))]
pub type Statx = libc::statx;

/// Re-export `statx`.
#[cfg(linux_raw)]
pub type Statx = linux_raw_sys::v5_4::general::statx;

/// Re-export `UTIME_NOW` and `UTIME_OMIT`.
#[cfg(all(libc, not(target_os = "redox")))]
pub use libc::{UTIME_NOW, UTIME_OMIT};

/// Re-export `UTIME_NOW` and `UTIME_OMIT`.
#[cfg(linux_raw)]
pub use linux_raw_sys::general::{UTIME_NOW, UTIME_OMIT};

/// Re-export `__fsword_t`.
#[cfg(all(libc, all(target_os = "linux", not(target_env = "musl"))))]
pub type FsWord = libc::__fsword_t;

/// Re-export `__fsword_t`.
#[cfg(all(
    libc,
    any(target_os = "android", all(target_os = "linux", target_env = "musl")),
    target_pointer_width = "32"
))]
pub type FsWord = u32;

/// Re-export `__fsword_t`.
#[cfg(all(
    libc,
    any(target_os = "android", all(target_os = "linux", target_env = "musl")),
    target_pointer_width = "64"
))]
pub type FsWord = u64;

/// Re-export `__fsword_t`.
#[cfg(linux_raw)]
pub type FsWord = linux_raw_sys::general::__fsword_t;

/// The filesystem magic number for procfs.
/// <https://man7.org/linux/man-pages/man2/fstatfs.2.html#DESCRIPTION>
#[cfg(all(
    libc,
    any(target_os = "android", target_os = "linux"),
    not(target_env = "musl")
))]
pub const PROC_SUPER_MAGIC: FsWord = libc::PROC_SUPER_MAGIC as FsWord;

/// The filesystem magic number for procfs.
/// <https://man7.org/linux/man-pages/man2/fstatfs.2.html#DESCRIPTION>
#[cfg(all(
    libc,
    any(target_os = "android", target_os = "linux"),
    target_env = "musl"
))]
pub const PROC_SUPER_MAGIC: FsWord = 0x0000_9fa0;

/// The filesystem magic number for procfs.
/// <https://man7.org/linux/man-pages/man2/fstatfs.2.html#DESCRIPTION>
#[cfg(linux_raw)]
pub const PROC_SUPER_MAGIC: FsWord = linux_raw_sys::general::PROC_SUPER_MAGIC as FsWord;

/// Re-export `mode_t`.
#[cfg(libc)]
pub type RawMode = libc::mode_t;

/// Re-export `mode_t`.
#[cfg(linux_raw)]
pub type RawMode = linux_raw_sys::general::__kernel_mode_t;

/// Re-export types common to Posix-ish platforms.
#[cfg(unix)]
pub use std::os::unix::fs::{DirEntryExt, FileExt, FileTypeExt, MetadataExt, OpenOptionsExt};

/// Re-export types common to Posix-ish platforms.
#[cfg(target_os = "wasi")]
pub use std::os::wasi::fs::{DirEntryExt, FileExt, FileTypeExt, MetadataExt, OpenOptionsExt};
