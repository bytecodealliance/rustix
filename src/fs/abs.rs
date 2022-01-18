//! POSIX-style filesystem functions which operate on bare paths.

use crate::{imp, io, path};
use imp::fs::StatFs;

/// `statfs`—Queries filesystem metadata.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/statfs.2.html
#[cfg(any(target_os = "android", target_os = "linux"))]
#[inline]
pub fn statfs<P: path::Arg>(path: P) -> io::Result<StatFs> {
    path.into_with_z_str(|path| imp::syscalls::statfs(path))
}
