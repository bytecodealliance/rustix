use crate::{imp, io, path};
use imp::fs::{Mode, OFlags, ResolveFlags};
use io_lifetimes::{AsFd, OwnedFd};

/// `openat2(dirfd, path, OpenHow { oflags, mode, resolve }, sizeof(OpenHow))`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/openat2.2.html
#[inline]
pub fn openat2<Fd: AsFd, P: path::Arg>(
    dirfd: &Fd,
    path: P,
    oflags: OFlags,
    mode: Mode,
    resolve: ResolveFlags,
) -> io::Result<OwnedFd> {
    let dirfd = dirfd.as_fd();
    path.into_with_c_str(|path| imp::syscalls::openat2(dirfd, path, oflags, mode, resolve))
}
