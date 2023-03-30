//! Umask support.

#[cfg(feature = "fs")]
use crate::backend;
#[cfg(feature = "fs")]
use crate::fs::Mode;

/// `umask(mask)`—Set the process file creation mask.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/umask.html
/// [Linux]: https://man7.org/linux/man-pages/man2/umask.2.html
#[inline]
#[cfg_attr(doc_cfg, doc(cfg(feature = "fs")))]
#[cfg(feature = "fs")]
pub fn umask(mask: Mode) -> Mode {
    backend::process::syscalls::umask(mask)
}
