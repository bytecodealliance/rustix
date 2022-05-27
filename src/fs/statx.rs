//! Linux `statx`.

use crate::fs::AtFlags;
use crate::{imp, io, path};
use imp::fd::AsFd;

pub use imp::fs::types::{Statx, StatxFlags, StatxTimestamp};

/// `statx(dirfd, path, flags, mask, statxbuf)`
///
/// This isn't available on Linux before 4.11; it returns `ENOSYS` in that
/// case.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/statx.2.html
#[inline]
pub fn statx<P: path::Arg, Fd: AsFd>(
    dirfd: Fd,
    path: P,
    flags: AtFlags,
    mask: StatxFlags,
) -> io::Result<Statx> {
    path.into_with_c_str(|path| imp::fs::syscalls::statx(dirfd.as_fd(), path, flags, mask))
}
