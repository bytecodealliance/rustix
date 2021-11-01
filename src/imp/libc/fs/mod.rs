#[cfg(not(windows))]
pub(crate) mod syscalls;

#[cfg(not(target_os = "redox"))]
mod dir;
#[cfg(not(any(
    target_os = "ios",
    target_os = "freebsd",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi"
)))]
mod makedev;
mod types;

#[cfg(not(target_os = "redox"))]
pub use dir::{Dir, DirEntry};
#[cfg(not(any(
    target_os = "ios",
    target_os = "freebsd",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi"
)))]
pub use makedev::{major, makedev, minor};
#[cfg(not(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox"
)))]
pub use types::Advice;
#[cfg(not(any(target_os = "netbsd", target_os = "openbsd", target_os = "redox")))]
pub use types::FallocateFlags;
#[cfg(not(target_os = "wasi"))]
pub use types::FlockOperation;
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "wasi")))]
pub use types::StatFs;
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use types::{copyfile_state_t, CloneFlags, CopyfileFlags};
pub use types::{Access, Dev, FdFlags, FileType, Mode, OFlags, RawMode, Stat};
#[cfg(not(target_os = "redox"))]
pub use types::{AtFlags, UTIME_NOW, UTIME_OMIT};
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use types::{FsWord, MemfdFlags, RenameFlags, ResolveFlags, PROC_SUPER_MAGIC};
#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub use types::{Statx, StatxFlags};
