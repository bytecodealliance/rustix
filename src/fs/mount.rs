//! Linux `mount`.

use crate::backend::fs::types::{
    FsMountFlags, FsOpenFlags, FsPickFlags, InternalMountFlags, MountAttrFlags, MountFlags,
    MountFlagsArg, MountPropagationFlags, MoveMountFlags, OpenTreeFlags, UnmountFlags,
};
use crate::fd::{BorrowedFd, OwnedFd};
use crate::{backend, io, path};

/// `mount(source, target, filesystemtype, mountflags, data)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/mount.2.html
#[inline]
pub fn mount<Source: path::Arg, Target: path::Arg, Fs: path::Arg, Data: path::Arg>(
    source: Source,
    target: Target,
    file_system_type: Fs,
    flags: MountFlags,
    data: Data,
) -> io::Result<()> {
    source.into_with_c_str(|source| {
        target.into_with_c_str(|target| {
            file_system_type.into_with_c_str(|file_system_type| {
                data.into_with_c_str(|data| {
                    backend::fs::syscalls::mount(
                        Some(source),
                        target,
                        Some(file_system_type),
                        MountFlagsArg(flags.bits()),
                        Some(data),
                    )
                })
            })
        })
    })
}

/// `mount(NULL, target, NULL, MS_REMOUNT | mountflags, data)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/mount.2.html
#[inline]
#[doc(alias = "mount")]
pub fn remount<Target: path::Arg, Data: path::Arg>(
    target: Target,
    flags: MountFlags,
    data: Data,
) -> io::Result<()> {
    target.into_with_c_str(|target| {
        data.into_with_c_str(|data| {
            backend::fs::syscalls::mount(
                None,
                target,
                None,
                MountFlagsArg(InternalMountFlags::REMOUNT.bits() | flags.bits()),
                Some(data),
            )
        })
    })
}

/// `mount(source, target, NULL, MS_BIND, NULL)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/mount.2.html
#[inline]
#[doc(alias = "mount")]
pub fn bind_mount<Source: path::Arg, Target: path::Arg>(
    source: Source,
    target: Target,
) -> io::Result<()> {
    source.into_with_c_str(|source| {
        target.into_with_c_str(|target| {
            backend::fs::syscalls::mount(
                Some(source),
                target,
                None,
                MountFlagsArg(MountFlags::BIND.bits()),
                None,
            )
        })
    })
}

/// `mount(source, target, NULL, MS_BIND | MS_REC, NULL)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/mount.2.html
#[inline]
#[doc(alias = "mount")]
pub fn recursive_bind_mount<Source: path::Arg, Target: path::Arg>(
    source: Source,
    target: Target,
) -> io::Result<()> {
    source.into_with_c_str(|source| {
        target.into_with_c_str(|target| {
            backend::fs::syscalls::mount(
                Some(source),
                target,
                None,
                MountFlagsArg(MountFlags::BIND.bits() | MountPropagationFlags::REC.bits()),
                None,
            )
        })
    })
}

/// `mount(NULL, target, NULL, mountflags, NULL)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/mount.2.html
#[inline]
#[doc(alias = "mount")]
pub fn change_mount<Target: path::Arg>(
    target: Target,
    flags: MountPropagationFlags,
) -> io::Result<()> {
    target.into_with_c_str(|target| {
        backend::fs::syscalls::mount(None, target, None, MountFlagsArg(flags.bits()), None)
    })
}

/// `mount(source, target, NULL, MS_MOVE, NULL)`
///
/// This is not the same as the `move_mount` syscall. If you want to use that,
/// use [`move_mount_syscall`] instead.
/// Its name will be changed in the next semver bump to avoid confusion.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/mount.2.html
#[inline]
#[doc(alias = "mount")]
pub fn move_mount<Source: path::Arg, Target: path::Arg>(
    source: Source,
    target: Target,
) -> io::Result<()> {
    source.into_with_c_str(|source| {
        target.into_with_c_str(|target| {
            backend::fs::syscalls::mount(
                Some(source),
                target,
                None,
                MountFlagsArg(InternalMountFlags::MOVE.bits()),
                None,
            )
        })
    })
}

/// `umount2(target, flags)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/umount.2.html
#[inline]
#[doc(alias = "umount", alias = "umount2")]
pub fn unmount<Target: path::Arg>(target: Target, flags: UnmountFlags) -> io::Result<()> {
    target.into_with_c_str(|target| backend::fs::syscalls::unmount(target, flags))
}

/// `fsopen(fs_name, flags)`
#[inline]
pub fn fsopen<Fs: path::Arg>(fs_name: Fs, flags: FsOpenFlags) -> io::Result<OwnedFd> {
    fs_name.into_with_c_str(|fs_name| backend::fs::syscalls::fsopen(fs_name, flags))
}

/// `fsmount(fs_fd, flags, attr_flags)`
#[inline]
pub fn fsmount(
    fs_fd: BorrowedFd<'_>,
    flags: FsMountFlags,
    attr_flags: MountAttrFlags,
) -> io::Result<()> {
    backend::fs::syscalls::fsmount(fs_fd, flags, attr_flags)
}

/// `move_mount(from_dfd, from_pathname, to_dfd, to_pathname, flags)`
/// This is the `move_mount` syscall, and it will be renamed to `move_mount`
/// in the next semver bump.
#[inline]
#[doc(alias = "move_mount")]
pub fn move_mount_syscall<From: path::Arg, To: path::Arg>(
    from_dfd: BorrowedFd<'_>,
    from_pathname: From,
    to_dfd: BorrowedFd<'_>,
    to_pathname: To,
    flags: MoveMountFlags,
) -> io::Result<()> {
    from_pathname.into_with_c_str(|from_pathname| {
        to_pathname.into_with_c_str(|to_pathname| {
            backend::fs::syscalls::move_mount(from_dfd, from_pathname, to_dfd, to_pathname, flags)
        })
    })
}

/// `open_tree(dfd, filename, flags)`
#[inline]
pub fn open_tree<Path: path::Arg>(
    dfd: BorrowedFd<'_>,
    filename: Path,
    flags: OpenTreeFlags,
) -> io::Result<OwnedFd> {
    filename.into_with_c_str(|filename| backend::fs::syscalls::open_tree(dfd, filename, flags))
}

/// `fspick(dfd, path, flags)`
#[inline]
pub fn fspick<Path: path::Arg>(
    dfd: BorrowedFd<'_>,
    path: Path,
    flags: FsPickFlags,
) -> io::Result<OwnedFd> {
    path.into_with_c_str(|path| backend::fs::syscalls::fspick(dfd, path, flags))
}

/// `fsconfig(fs_fd, FSCONFIG_SET_FLAG, key, NULL, 0)`
#[inline]
#[doc(alias = "fsconfig")]
pub fn fsconfig_set_flag<Key: path::Arg>(fs_fd: BorrowedFd<'_>, key: Key) -> io::Result<()> {
    key.into_with_c_str(|key| backend::fs::syscalls::fsconfig_set_flag(fs_fd, key))
}

/// `fsconfig(fs_fd, FSCONFIG_SET_STRING, key, value, 0)`
#[inline]
#[doc(alias = "fsconfig")]
pub fn fsconfig_set_string<Key: path::Arg, Value: path::Arg>(
    fs_fd: BorrowedFd<'_>,
    key: Key,
    value: Value,
) -> io::Result<()> {
    key.into_with_c_str(|key| {
        value.into_with_c_str(|value| backend::fs::syscalls::fsconfig_set_string(fs_fd, key, value))
    })
}

/// `fsconfig(fs_fd, FSCONFIG_SET_BINARY, key, value, value.len())`
#[inline]
#[doc(alias = "fsconfig")]
pub fn fsconfig_set_binary<Key: path::Arg>(
    fs_fd: BorrowedFd<'_>,
    key: Key,
    value: &[u8],
) -> io::Result<()> {
    key.into_with_c_str(|key| backend::fs::syscalls::fsconfig_set_binary(fs_fd, key, value))
}

/// `fsconfig(fs_fd, FSCONFIG_SET_PATH, key, path, fd)`
#[inline]
#[doc(alias = "fsconfig")]
pub fn fsconfig_set_path<Key: path::Arg, Path: path::Arg>(
    fs_fd: BorrowedFd<'_>,
    key: Key,
    path: Path,
    fd: BorrowedFd<'_>,
) -> io::Result<()> {
    key.into_with_c_str(|key| {
        path.into_with_c_str(|path| backend::fs::syscalls::fsconfig_set_path(fs_fd, key, path, fd))
    })
}

/// `fsconfig(fs_fd, FSCONFIG_SET_PATH_EMPTY, key, "", fd)`
#[inline]
#[doc(alias = "fsconfig")]
pub fn fsconfig_set_path_empty<Key: path::Arg>(
    fs_fd: BorrowedFd<'_>,
    key: Key,
    fd: BorrowedFd<'_>,
) -> io::Result<()> {
    key.into_with_c_str(|key| backend::fs::syscalls::fsconfig_set_path_empty(fs_fd, key, fd))
}

/// `fsconfig(fs_fd, FSCONFIG_SET_FD, key, NULL, fd)`
#[inline]
#[doc(alias = "fsconfig")]
pub fn fsconfig_set_fd<Key: path::Arg>(
    fs_fd: BorrowedFd<'_>,
    key: Key,
    fd: BorrowedFd<'_>,
) -> io::Result<()> {
    key.into_with_c_str(|key| backend::fs::syscalls::fsconfig_set_fd(fs_fd, key, fd))
}

/// `fsconfig(fs_fd, FSCONFIG_CMD_CREATE, key, NULL, 0)`
#[inline]
#[doc(alias = "fsconfig")]
pub fn fsconfig_create(fs_fd: BorrowedFd<'_>) -> io::Result<()> {
    backend::fs::syscalls::fsconfig_create(fs_fd)
}

/// `fsconfig(fs_fd, FSCONFIG_CMD_RECONFIGURE, key, NULL, 0)`
#[inline]
#[doc(alias = "fsconfig")]
pub fn fsconfig_reconfigure(fs_fd: BorrowedFd<'_>) -> io::Result<()> {
    backend::fs::syscalls::fsconfig_reconfigure(fs_fd)
}
