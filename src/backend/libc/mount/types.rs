use core::marker::PhantomData;

use bitflags::bitflags;

use crate::{
    backend::c,
    fd::{AsRawFd, BorrowedFd},
    ffi,
};

#[cfg(linux_kernel)]
bitflags! {
    /// `MS_*` constants for use with [`mount`].
    ///
    /// [`mount`]: crate::mount::mount
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct MountFlags: ffi::c_ulong {
        /// `MS_BIND`
        const BIND = c::MS_BIND;

        /// `MS_DIRSYNC`
        const DIRSYNC = c::MS_DIRSYNC;

        /// `MS_LAZYTIME`
        const LAZYTIME = c::MS_LAZYTIME;

        /// `MS_MANDLOCK`
        #[doc(alias = "MANDLOCK")]
        const PERMIT_MANDATORY_FILE_LOCKING = c::MS_MANDLOCK;

        /// `MS_NOATIME`
        const NOATIME = c::MS_NOATIME;

        /// `MS_NODEV`
        const NODEV = c::MS_NODEV;

        /// `MS_NODIRATIME`
        const NODIRATIME = c::MS_NODIRATIME;

        /// `MS_NOEXEC`
        const NOEXEC = c::MS_NOEXEC;

        /// `MS_NOSUID`
        const NOSUID = c::MS_NOSUID;

        /// `MS_RDONLY`
        const RDONLY = c::MS_RDONLY;

        /// `MS_REC`
        const REC = c::MS_REC;

        /// `MS_RELATIME`
        const RELATIME = c::MS_RELATIME;

        /// `MS_SILENT`
        const SILENT = c::MS_SILENT;

        /// `MS_STRICTATIME`
        const STRICTATIME = c::MS_STRICTATIME;

        /// `MS_SYNCHRONOUS`
        const SYNCHRONOUS = c::MS_SYNCHRONOUS;

        /// `MS_NOSYMFOLLOW`
        #[cfg(linux_raw_dep)]
        const NOSYMFOLLOW = c::MS_NOSYMFOLLOW;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

#[cfg(linux_kernel)]
bitflags! {
    /// `MNT_*` constants for use with [`unmount`].
    ///
    /// [`unmount`]: crate::mount::unmount
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct UnmountFlags: u32 {
        /// `MNT_FORCE`
        const FORCE = bitcast!(c::MNT_FORCE);
        /// `MNT_DETACH`
        const DETACH = bitcast!(c::MNT_DETACH);
        /// `MNT_EXPIRE`
        const EXPIRE = bitcast!(c::MNT_EXPIRE);
        /// `UMOUNT_NOFOLLOW`
        const NOFOLLOW = bitcast!(c::UMOUNT_NOFOLLOW);

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

#[cfg(linux_kernel)]
bitflags! {
    /// `FSOPEN_*` constants for use with [`fsopen`].
    ///
    /// [`fsopen`]: crate::mount::fsopen
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct FsOpenFlags: ffi::c_uint {
        /// `FSOPEN_CLOEXEC`
        const FSOPEN_CLOEXEC = 0x0000_0001;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

#[cfg(linux_kernel)]
bitflags! {
    /// `FSMOUNT_*` constants for use with [`fsmount`].
    ///
    /// [`fsmount`]: crate::mount::fsmount
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct FsMountFlags: ffi::c_uint {
        /// `FSMOUNT_CLOEXEC`
        const FSMOUNT_CLOEXEC = 0x0000_0001;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

/// `FSCONFIG_*` constants for use with the `fsconfig` syscall.
#[cfg(linux_kernel)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub(crate) enum FsConfigCmd {
    /// `FSCONFIG_SET_FLAG`
    SetFlag = 0,

    /// `FSCONFIG_SET_STRING`
    SetString = 1,

    /// `FSCONFIG_SET_BINARY`
    SetBinary = 2,

    /// `FSCONFIG_SET_PATH`
    SetPath = 3,

    /// `FSCONFIG_SET_PATH_EMPTY`
    SetPathEmpty = 4,

    /// `FSCONFIG_SET_FD`
    SetFd = 5,

    /// `FSCONFIG_CMD_CREATE`
    Create = 6,

    /// `FSCONFIG_CMD_RECONFIGURE`
    Reconfigure = 7,

    /// `FSCONFIG_CMD_CREATE_EXCL` (since Linux 6.6)
    CreateExclusive = 8,
}

#[cfg(linux_kernel)]
bitflags! {
    /// `MOUNT_ATTR_*` constants for use with [`fsmount`] and [`mount_setattr`].
    ///
    /// [`fsmount`]: crate::mount::fsmount
    /// [`mount_setattr`]: crate::mount::mount_setattr
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct MountAttrFlags: ffi::c_uint {
        /// `MOUNT_ATTR_RDONLY`
        const MOUNT_ATTR_RDONLY =  c::MOUNT_ATTR_RDONLY as c::c_uint;

        /// `MOUNT_ATTR_NOSUID`
        const MOUNT_ATTR_NOSUID = c::MOUNT_ATTR_NOSUID as c::c_uint;

        /// `MOUNT_ATTR_NODEV`
        const MOUNT_ATTR_NODEV = c::MOUNT_ATTR_NODEV as c::c_uint;

        /// `MOUNT_ATTR_NOEXEC`
        const MOUNT_ATTR_NOEXEC = c::MOUNT_ATTR_NOEXEC as c::c_uint;

        /// `MOUNT_ATTR__ATIME`
        const MOUNT_ATTR__ATIME = c::MOUNT_ATTR__ATIME as c::c_uint;

        /// `MOUNT_ATTR_RELATIME`
        const MOUNT_ATTR_RELATIME = c::MOUNT_ATTR_RELATIME as c::c_uint;

        /// `MOUNT_ATTR_NOATIME`
        const MOUNT_ATTR_NOATIME = c::MOUNT_ATTR_NOATIME as c::c_uint;

        /// `MOUNT_ATTR_STRICTATIME`
        const MOUNT_ATTR_STRICTATIME =c::MOUNT_ATTR_STRICTATIME as c::c_uint;

        /// `MOUNT_ATTR_NODIRATIME`
        const MOUNT_ATTR_NODIRATIME = c::MOUNT_ATTR_NODIRATIME as c::c_uint;

        /// `MOUNT_ATTR_NOSYMFOLLOW`
        const MOUNT_ATTR_NOSYMFOLLOW = c::MOUNT_ATTR_NOSYMFOLLOW as c::c_uint;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

#[cfg(linux_kernel)]
bitflags! {
    /// `MOVE_MOUNT_*` constants for use with [`move_mount`].
    ///
    /// [`move_mount`]: crate::mount::move_mount
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct MoveMountFlags: ffi::c_uint {
        /// `MOVE_MOUNT_F_EMPTY_PATH`
        const MOVE_MOUNT_F_SYMLINKS = 0x0000_0001;

        /// `MOVE_MOUNT_F_AUTOMOUNTS`
        const MOVE_MOUNT_F_AUTOMOUNTS = 0x0000_0002;

        /// `MOVE_MOUNT_F_EMPTY_PATH`
        const MOVE_MOUNT_F_EMPTY_PATH = 0x0000_0004;

        /// `MOVE_MOUNT_T_SYMLINKS`
        const MOVE_MOUNT_T_SYMLINKS = 0x0000_0010;

        /// `MOVE_MOUNT_T_AUTOMOUNTS`
        const MOVE_MOUNT_T_AUTOMOUNTS = 0x0000_0020;

        /// `MOVE_MOUNT_T_EMPTY_PATH`
        const MOVE_MOUNT_T_EMPTY_PATH = 0x0000_0040;

        /// `MOVE_MOUNT__MASK`
        const MOVE_MOUNT_SET_GROUP = 0x0000_0100;

        /// `MOVE_MOUNT_BENEATH` (since Linux 6.5)
        const MOVE_MOUNT_BENEATH = 0x0000_0200;

        /// `MOVE_MOUNT__MASK`
        const MOVE_MOUNT__MASK = 0x0000_0377;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

#[cfg(linux_kernel)]
bitflags! {
    /// `OPENTREE_*` constants for use with [`open_tree`].
    ///
    /// [`open_tree`]: crate::mount::open_tree
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct OpenTreeFlags: ffi::c_uint {
        /// `OPENTREE_CLONE`
        const OPEN_TREE_CLONE = 1;

        /// `OPENTREE_CLOEXEC`
        const OPEN_TREE_CLOEXEC = c::O_CLOEXEC as c::c_uint;

        /// `AT_EMPTY_PATH`
        const AT_EMPTY_PATH = c::AT_EMPTY_PATH as c::c_uint;

        /// `AT_NO_AUTOMOUNT`
        const AT_NO_AUTOMOUNT = c::AT_NO_AUTOMOUNT as c::c_uint;

        /// `AT_RECURSIVE`
        const AT_RECURSIVE = c::AT_RECURSIVE as c::c_uint;

        /// `AT_SYMLINK_NOFOLLOW`
        const AT_SYMLINK_NOFOLLOW = c::AT_SYMLINK_NOFOLLOW as c::c_uint;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

#[cfg(linux_kernel)]
bitflags! {
    /// `FSPICK_*` constants for use with [`fspick`].
    ///
    /// [`fspick`]: crate::mount::fspick
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct FsPickFlags: ffi::c_uint {
        /// `FSPICK_CLOEXEC`
        const FSPICK_CLOEXEC = 0x0000_0001;

        /// `FSPICK_SYMLINK_NOFOLLOW`
        const FSPICK_SYMLINK_NOFOLLOW = 0x0000_0002;

        /// `FSPICK_NO_AUTOMOUNT`
        const FSPICK_NO_AUTOMOUNT = 0x0000_0004;

        /// `FSPICK_EMPTY_PATH`
        const FSPICK_EMPTY_PATH = 0x0000_0008;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

#[cfg(linux_kernel)]
bitflags! {
    /// `AT_*` flags accepted by [`mount_setattr`].
    ///
    /// [`mount_setattr`]: crate::mount::mount_setattr
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct MountSetAttrFlags: ffi::c_uint {
        /// `AT_EMPTY_PATH`
        const AT_EMPTY_PATH = c::AT_EMPTY_PATH as c::c_uint;

        /// `AT_RECURSIVE`
        const AT_RECURSIVE = c::AT_RECURSIVE as c::c_uint;

        /// `AT_SYMLINK_NOFOLLOW`
        const AT_SYMLINK_NOFOLLOW = c::AT_SYMLINK_NOFOLLOW as c::c_uint;

        /// `AT_NO_AUTOMOUNT`
        const AT_NO_AUTOMOUNT = c::AT_NO_AUTOMOUNT as c::c_uint;
    }
}

/// `MOUNT_ATTR_*` flags that also carry a parameter.
#[cfg(linux_kernel)]
pub enum MountAttrParamFlags<'a> {
    /// `MOUNT_ATTR_IDMAP`, which carries the descriptor of a user namespace.
    IdMap(BorrowedFd<'a>),
}

#[cfg(linux_kernel)]
impl<'a> MountAttrParamFlags<'a> {
    fn apply(self, attr: MountAttr<'a>, operation: MountAttrOperation) -> MountAttr<'a> {
        let mut raw = attr.raw;
        let flags = match operation {
            MountAttrOperation::Set => &mut raw.attr_set,
            MountAttrOperation::Clear => &mut raw.attr_clr,
        };

        match self {
            Self::IdMap(userns_fd) => {
                *flags |=
                    MountAttrFlags::from_bits_retain(c::MOUNT_ATTR_IDMAP as u32).bits() as u64;
                raw.userns_fd = userns_fd.as_raw_fd() as u64;
            }
        }

        MountAttr {
            raw,
            userns_fd: PhantomData::<BorrowedFd<'a>>,
        }
    }
}

/// `struct mount_attr`
#[cfg(linux_kernel)]
#[derive(Clone)]
#[doc(alias = "mount_attr")]
pub struct MountAttr<'a> {
    pub(crate) raw: c::mount_attr,
    userns_fd: PhantomData<BorrowedFd<'a>>,
}

#[cfg(linux_kernel)]
impl<'a> MountAttr<'a> {
    /// Create a `MountAttr` with the given simple fields.
    pub fn new(
        attr_set: MountAttrFlags,
        attr_clr: MountAttrFlags,
        propagation: MountPropagationFlags,
    ) -> Self {
        Self {
            raw: c::mount_attr {
                attr_set: attr_set.bits() as u64,
                attr_clr: attr_clr.bits() as u64,
                propagation: propagation.bits() as u64,
                userns_fd: 0,
            },
            userns_fd: PhantomData,
        }
    }

    /// Set a parameterized flag.
    pub fn set_param_flag(self, param: MountAttrParamFlags<'a>) -> Self {
        param.apply(self, MountAttrOperation::Set)
    }

    /// Clear a parameterized flag.
    pub fn clear_param_flag(self, param: MountAttrParamFlags<'a>) -> Self {
        param.apply(self, MountAttrOperation::Clear)
    }
}

#[cfg(linux_kernel)]
enum MountAttrOperation {
    Set,
    Clear,
}

#[cfg(linux_kernel)]
bitflags! {
    /// `MS_*` constants for use with [`mount_change`].
    ///
    /// [`mount_change`]: crate::mount::mount_change
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct MountPropagationFlags: ffi::c_ulong {
        /// `MS_SILENT`
        const SILENT = c::MS_SILENT;
        /// `MS_SHARED`
        const SHARED = c::MS_SHARED;
        /// `MS_PRIVATE`
        const PRIVATE = c::MS_PRIVATE;
        /// Mark a mount as a downstream of its current peer group.
        ///
        /// Mount and unmount events propagate from the upstream peer group
        /// into the downstream.
        ///
        /// In Linux documentation, this flag is named `MS_SLAVE`, and the
        /// concepts of “upstream” and “downstream” are called
        /// “master” and “slave”.
        #[doc(alias = "SLAVE")]
        const DOWNSTREAM = c::MS_SLAVE;
        /// `MS_UNBINDABLE`
        const UNBINDABLE = c::MS_UNBINDABLE;
        /// `MS_REC`
        const REC = c::MS_REC;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

#[cfg(linux_kernel)]
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub(crate) struct InternalMountFlags: c::c_ulong {
        const REMOUNT = c::MS_REMOUNT;
        const MOVE = c::MS_MOVE;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

#[cfg(linux_kernel)]
pub(crate) struct MountFlagsArg(pub(crate) c::c_ulong);
