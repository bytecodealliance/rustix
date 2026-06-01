//! `fsopen` and related functions in Linux's `mount` API.

use crate::backend::mount::types::{
    FsMountFlags, FsOpenFlags, FsPickFlags, MountAttrFlags, MoveMountFlags, OpenTreeFlags,
};
use crate::fd::{AsFd, OwnedFd};
use crate::{backend, io, path};

/// `fsopen(fsname, flags)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/fsopen.2.html
#[inline]
pub fn fsopen<Fs: path::Arg>(fs_name: Fs, flags: FsOpenFlags) -> io::Result<OwnedFd> {
    fs_name.into_with_c_str(|fs_name| backend::mount::syscalls::fsopen(fs_name, flags))
}

/// `fsmount(fsfd, flags, attr_flags)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/fsmount.2.html
#[inline]
pub fn fsmount<Fd: AsFd>(
    fs_fd: Fd,
    flags: FsMountFlags,
    attr_flags: MountAttrFlags,
) -> io::Result<OwnedFd> {
    backend::mount::syscalls::fsmount(fs_fd.as_fd(), flags, attr_flags)
}

/// `move_mount(from_dirfd, from_path, to_dirfd, to_path, flags)`
///
/// This is not the same as `mount` with the `MS_MOVE` flag. If you want to
/// use that, use [`mount_move`] instead.
///
/// # References
///  - [Linux]
///
/// [`mount_move`]: crate::mount::mount_move
/// [Linux]: https://man7.org/linux/man-pages/man2/move_mount.2.html
#[inline]
pub fn move_mount<From: path::Arg, To: path::Arg, FromFd: AsFd, ToFd: AsFd>(
    from_dfd: FromFd,
    from_pathname: From,
    to_dfd: ToFd,
    to_pathname: To,
    flags: MoveMountFlags,
) -> io::Result<()> {
    let from_dfd = from_dfd.as_fd();
    let to_dfd = to_dfd.as_fd();
    from_pathname.into_with_c_str(|from_pathname| {
        to_pathname.into_with_c_str(|to_pathname| {
            backend::mount::syscalls::move_mount(
                from_dfd,
                from_pathname,
                to_dfd,
                to_pathname,
                flags,
            )
        })
    })
}

/// `open_tree(dirfd, path, flags)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/open_tree.2.html
#[inline]
pub fn open_tree<Path: path::Arg, Fd: AsFd>(
    dfd: Fd,
    filename: Path,
    flags: OpenTreeFlags,
) -> io::Result<OwnedFd> {
    let dfd = dfd.as_fd();
    filename.into_with_c_str(|filename| backend::mount::syscalls::open_tree(dfd, filename, flags))
}

/// `fspick(dirfd, path, flags)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/fspick.2.html
#[inline]
pub fn fspick<Path: path::Arg, Fd: AsFd>(
    dfd: Fd,
    path: Path,
    flags: FsPickFlags,
) -> io::Result<OwnedFd> {
    let dfd = dfd.as_fd();
    path.into_with_c_str(|path| backend::mount::syscalls::fspick(dfd, path, flags))
}

/// `fsconfig(fd, FSCONFIG_SET_FLAG, key, NULL, 0)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/fsconfig.2.html
#[inline]
#[doc(alias = "fsconfig")]
pub fn fsconfig_set_flag<Key: path::Arg, Fd: AsFd>(fs_fd: Fd, key: Key) -> io::Result<()> {
    let fs_fd = fs_fd.as_fd();
    key.into_with_c_str(|key| backend::mount::syscalls::fsconfig_set_flag(fs_fd, key))
}

/// `fsconfig(fd, FSCONFIG_SET_STRING, key, value, 0)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/fsconfig.2.html
#[inline]
#[doc(alias = "fsconfig")]
pub fn fsconfig_set_string<Key: path::Arg, Value: path::Arg, Fd: AsFd>(
    fs_fd: Fd,
    key: Key,
    value: Value,
) -> io::Result<()> {
    let fs_fd = fs_fd.as_fd();
    key.into_with_c_str(|key| {
        value.into_with_c_str(|value| {
            backend::mount::syscalls::fsconfig_set_string(fs_fd, key, value)
        })
    })
}

/// `fsconfig(fd, FSCONFIG_SET_BINARY, key, value, value.len())`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/fsconfig.2.html
#[inline]
#[doc(alias = "fsconfig")]
pub fn fsconfig_set_binary<Key: path::Arg, Fd: AsFd>(
    fs_fd: Fd,
    key: Key,
    value: &[u8],
) -> io::Result<()> {
    let fs_fd = fs_fd.as_fd();
    key.into_with_c_str(|key| backend::mount::syscalls::fsconfig_set_binary(fs_fd, key, value))
}

/// `fsconfig(fd, FSCONFIG_SET_PATH, key, path, fd)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/fsconfig.2.html
#[inline]
#[doc(alias = "fsconfig")]
pub fn fsconfig_set_path<Key: path::Arg, Path: path::Arg, Fd: AsFd, AuxFd: AsFd>(
    fs_fd: Fd,
    key: Key,
    path: Path,
    fd: AuxFd,
) -> io::Result<()> {
    let fs_fd = fs_fd.as_fd();
    let fd = fd.as_fd();
    key.into_with_c_str(|key| {
        path.into_with_c_str(|path| {
            backend::mount::syscalls::fsconfig_set_path(fs_fd, key, path, fd)
        })
    })
}

/// `fsconfig(fd, FSCONFIG_SET_PATH_EMPTY, key, "", fd)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/fsconfig.2.html
#[inline]
#[doc(alias = "fsconfig")]
pub fn fsconfig_set_path_empty<Key: path::Arg, Fd: AsFd, AuxFd: AsFd>(
    fs_fd: Fd,
    key: Key,
    fd: AuxFd,
) -> io::Result<()> {
    let fs_fd = fs_fd.as_fd();
    let fd = fd.as_fd();
    key.into_with_c_str(|key| backend::mount::syscalls::fsconfig_set_path_empty(fs_fd, key, fd))
}

/// `fsconfig(fd, FSCONFIG_SET_FD, key, NULL, fd)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/fsconfig.2.html
#[inline]
#[doc(alias = "fsconfig")]
pub fn fsconfig_set_fd<Key: path::Arg, Fd: AsFd, AuxFd: AsFd>(
    fs_fd: Fd,
    key: Key,
    fd: AuxFd,
) -> io::Result<()> {
    let fs_fd = fs_fd.as_fd();
    let fd = fd.as_fd();
    key.into_with_c_str(|key| backend::mount::syscalls::fsconfig_set_fd(fs_fd, key, fd))
}

/// `fsconfig(fd, FSCONFIG_CMD_CREATE, key, NULL, 0)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/fsconfig.2.html
#[inline]
#[doc(alias = "fsconfig")]
pub fn fsconfig_create<Fd: AsFd>(fs_fd: Fd) -> io::Result<()> {
    backend::mount::syscalls::fsconfig_create(fs_fd.as_fd())
}

/// `fsconfig(fd, FSCONFIG_CMD_RECONFIGURE, key, NULL, 0)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/fsconfig.2.html
#[inline]
#[doc(alias = "fsconfig")]
pub fn fsconfig_reconfigure<Fd: AsFd>(fs_fd: Fd) -> io::Result<()> {
    backend::mount::syscalls::fsconfig_reconfigure(fs_fd.as_fd())
}

/// `fsconfig(fd, FSCONFIG_CMD_CREATE_EXCL, key, NULL, 0)`
///
/// This function was added in Linux 6.6.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/fsconfig.2.html
#[inline]
#[doc(alias = "fsconfig")]
pub fn fsconfig_create_exclusive<Fd: AsFd>(fs_fd: Fd) -> io::Result<()> {
    backend::mount::syscalls::fsconfig_create_excl(fs_fd.as_fd())
}
