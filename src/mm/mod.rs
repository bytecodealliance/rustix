//! Memory map operations.

#[cfg(not(target_os = "redox"))]
mod madvise;
mod mmap;
mod msync;
#[cfg(linux_kernel)]
mod userfaultfd;

#[cfg(not(target_os = "redox"))]
pub use madvise::{Advice, madvise};
pub use mmap::*;
pub use msync::{MsyncFlags, msync};
#[cfg(linux_kernel)]
pub use userfaultfd::{UserfaultfdFlags, userfaultfd};
