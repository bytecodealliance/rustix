//! Miscellaneous mount APIs

use crate::backend::mount::types::MountAttr;
use crate::fd::BorrowedFd;
use crate::fs::AtFlags;
use crate::{backend, io, path};

/// `mount_setattr(dir_fd, path, flags, mount_attr)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/mount_setattr.2.html
#[inline]
pub fn mount_setattr<Path: path::Arg>(
    dir_fd: BorrowedFd<'_>,
    path: Path,
    flags: AtFlags,
    mount_attr: &MountAttr<'_>,
) -> io::Result<()> {
    path.into_with_c_str(|path| {
        backend::mount::syscalls::mount_setattr(dir_fd, path, flags, mount_attr)
    })
}
