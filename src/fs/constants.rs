//! Filesystem API constants, translated into `bitflags` constants.

use crate::imp;

pub use imp::fs::types::{Access, FdFlags, Mode, OFlags};

#[cfg(not(target_os = "redox"))]
pub use imp::fs::types::AtFlags;

#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use imp::fs::types::{CloneFlags, CopyfileFlags};

#[cfg(any(target_os = "android", target_os = "linux"))]
pub use imp::fs::types::{RenameFlags, ResolveFlags};

#[cfg(not(target_os = "redox"))]
pub use imp::fs::types::Dev;

pub use imp::time::types::{Nsecs, Secs, Timespec};
