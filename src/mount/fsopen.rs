//! `fsopen` and related functions in Linux's `mount` API.

#![allow(unsafe_code)]

use crate::backend::mount::types::{
    FsMountFlags, FsOpenFlags, FsPickFlags, MountAttrFlags, MoveMountFlags, OpenTreeFlags,
};
use crate::buffer::Buffer;
use crate::fd::{AsFd, OwnedFd};
use crate::{backend, io, path};

/// The value used with [`listmount`] to start at the root mount of the
/// current mount namespace.
pub const LISTMOUNT_ROOT: u64 = linux_raw_sys::general::LSMT_ROOT as u64;

bitflags::bitflags! {
    /// Flags for use with [`listmount`].
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
    pub struct ListMountFlags: u32 {
        /// Return mounts in reverse order.
        const REVERSE = linux_raw_sys::general::LISTMOUNT_REVERSE;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

/// `listmount(req, mount_ids, mount_ids.len(), flags)`
///
/// Return mount IDs below `root_mount_id` in the caller's current mount
/// namespace. Use [`LISTMOUNT_ROOT`] to start at the namespace root. Set
/// `last_mount_id` to zero for the first call, or to the last ID returned by a
/// previous call to continue iterating.
///
/// This function performs one kernel call and does not allocate or retry.
/// Multiple calls are not an atomic snapshot, so callers are responsible for
/// handling concurrent mount-tree changes while paginating.
///
/// # References
///  - [`listmount(2)`]
///
/// [`listmount(2)`]: https://man7.org/linux/man-pages/man2/listmount.2.html
#[inline]
pub fn listmount<Buf: Buffer<u64>>(
    root_mount_id: u64,
    last_mount_id: u64,
    mut mount_ids: Buf,
    flags: ListMountFlags,
) -> io::Result<Buf::Output> {
    let parts = mount_ids.parts_mut();
    let capacity = parts.1;

    // SAFETY: `parts` points to `capacity` writable `u64` elements, and the
    // backend reports how many of them the kernel initialized.
    let initialized = unsafe {
        backend::mount::syscalls::listmount(root_mount_id, last_mount_id, parts, flags.bits())?
    };
    if initialized > capacity {
        return Err(io::Errno::RANGE);
    }

    // SAFETY: The backend initialized exactly `initialized` elements and the
    // capacity check above proves that the prefix is within `mount_ids`.
    unsafe { Ok(mount_ids.assume_init(initialized)) }
}

/// `fsopen(fs_name, flags)`
///
/// # References
///  - [Unfinished draft]
///
/// [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsopen.md
#[inline]
pub fn fsopen<Fs: path::Arg>(fs_name: Fs, flags: FsOpenFlags) -> io::Result<OwnedFd> {
    fs_name.into_with_c_str(|fs_name| backend::mount::syscalls::fsopen(fs_name, flags))
}

/// `fsmount(fs_fd, flags, attr_flags)`
///
/// # References
///  - [Unfinished draft]
///
/// [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsmount.md
#[inline]
pub fn fsmount<Fd: AsFd>(
    fs_fd: Fd,
    flags: FsMountFlags,
    attr_flags: MountAttrFlags,
) -> io::Result<OwnedFd> {
    backend::mount::syscalls::fsmount(fs_fd.as_fd(), flags, attr_flags)
}

/// `move_mount(from_dfd, from_pathname, to_dfd, to_pathname, flags)`
///
/// This is not the same as `mount` with the `MS_MOVE` flag. If you want to
/// use that, use [`mount_move`] instead.
///
/// # References
///  - [Unfinished draft]
///
/// [`mount_move`]: crate::mount::mount_move
/// [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/move_mount.md
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

/// `open_tree(dfd, filename, flags)`
///
/// # References
///  - [Unfinished draft]
///
/// [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/open_tree.md
#[inline]
pub fn open_tree<Path: path::Arg, Fd: AsFd>(
    dfd: Fd,
    filename: Path,
    flags: OpenTreeFlags,
) -> io::Result<OwnedFd> {
    let dfd = dfd.as_fd();
    filename.into_with_c_str(|filename| backend::mount::syscalls::open_tree(dfd, filename, flags))
}

/// `fspick(dfd, path, flags)`
///
/// # References
///  - [Unfinished draft]
///
/// [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fspick.md
#[inline]
pub fn fspick<Path: path::Arg, Fd: AsFd>(
    dfd: Fd,
    path: Path,
    flags: FsPickFlags,
) -> io::Result<OwnedFd> {
    let dfd = dfd.as_fd();
    path.into_with_c_str(|path| backend::mount::syscalls::fspick(dfd, path, flags))
}

/// `fsconfig(fs_fd, FSCONFIG_SET_FLAG, key, NULL, 0)`
///
/// # References
///  - [Unfinished draft]
///
/// [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsconfig.md
#[inline]
#[doc(alias = "fsconfig")]
pub fn fsconfig_set_flag<Key: path::Arg, Fd: AsFd>(fs_fd: Fd, key: Key) -> io::Result<()> {
    let fs_fd = fs_fd.as_fd();
    key.into_with_c_str(|key| backend::mount::syscalls::fsconfig_set_flag(fs_fd, key))
}

/// `fsconfig(fs_fd, FSCONFIG_SET_STRING, key, value, 0)`
///
/// # References
///  - [Unfinished draft]
///
/// [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsconfig.md
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

/// `fsconfig(fs_fd, FSCONFIG_SET_BINARY, key, value, value.len())`
///
/// # References
///  - [Unfinished draft]
///
/// [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsconfig.md
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

/// `fsconfig(fs_fd, FSCONFIG_SET_PATH, key, path, fd)`
///
/// # References
///  - [Unfinished draft]
///
/// [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsconfig.md
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

/// `fsconfig(fs_fd, FSCONFIG_SET_PATH_EMPTY, key, "", fd)`
///
/// # References
///  - [Unfinished draft]
///
/// [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsconfig.md
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

/// `fsconfig(fs_fd, FSCONFIG_SET_FD, key, NULL, fd)`
///
/// # References
///  - [Unfinished draft]
///
/// [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsconfig.md
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

/// `fsconfig(fs_fd, FSCONFIG_CMD_CREATE, key, NULL, 0)`
///
/// # References
///  - [Unfinished draft]
///
/// [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsconfig.md
#[inline]
#[doc(alias = "fsconfig")]
pub fn fsconfig_create<Fd: AsFd>(fs_fd: Fd) -> io::Result<()> {
    backend::mount::syscalls::fsconfig_create(fs_fd.as_fd())
}

/// `fsconfig(fs_fd, FSCONFIG_CMD_RECONFIGURE, key, NULL, 0)`
///
/// # References
///  - [Unfinished draft]
///
/// [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsconfig.md
#[inline]
#[doc(alias = "fsconfig")]
pub fn fsconfig_reconfigure<Fd: AsFd>(fs_fd: Fd) -> io::Result<()> {
    backend::mount::syscalls::fsconfig_reconfigure(fs_fd.as_fd())
}

/// `fsconfig(fs_fd, FSCONFIG_CMD_CREATE_EXCL, key, NULL, 0)`
///
/// This function was added in Linux 6.6.
///
/// # References
///  - [Unfinished draft]
///
/// [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsconfig.md
#[inline]
#[doc(alias = "fsconfig")]
pub fn fsconfig_create_exclusive<Fd: AsFd>(fs_fd: Fd) -> io::Result<()> {
    backend::mount::syscalls::fsconfig_create_excl(fs_fd.as_fd())
}
