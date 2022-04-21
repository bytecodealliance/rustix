//! Memory map operations.

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
mod madvise;
#[cfg(not(target_os = "wasi"))]
mod mmap;
#[cfg(not(target_os = "wasi"))]
mod msync;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod userfaultfd;

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub use madvise::{madvise, Advice};
#[cfg(not(target_os = "wasi"))]
pub use mmap::{
    mlock, mmap, mmap_anonymous, mprotect, munlock, munmap, MapFlags, MprotectFlags, ProtFlags,
};
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use mmap::{mlock_with, MlockFlags};
#[cfg(any(linux_raw, all(libc, target_os = "linux")))]
pub use mmap::{mremap, mremap_fixed, MremapFlags};
#[cfg(not(target_os = "wasi"))]
pub use msync::{msync, MsyncFlags};
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use userfaultfd::{userfaultfd, UserfaultfdFlags};
