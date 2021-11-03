//! Linux `statx`.

use crate::{imp, io, path};
use imp::fd::AsFd;
use imp::fs::{AtFlags, Statx};

/// `STATX_*` constants.
pub use imp::fs::StatxFlags;

/// `statx(dirfd, path, flags, mask, statxbuf)`
///
/// Note that this isn't available on Linux before 4.11; returns `ENOSYS` in
/// that case.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/statx.2.html
#[inline]
pub fn statx<P: path::Arg, Fd: AsFd>(
    dirfd: &Fd,
    path: P,
    flags: AtFlags,
    mask: StatxFlags,
) -> io::Result<Statx> {
    let dirfd = dirfd.as_fd();
    path.into_with_z_str(|path| imp::syscalls::statx(dirfd, path, flags, mask))
}
