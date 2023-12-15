use crate::backend::c;
use bitflags::bitflags;

bitflags! {
    /// `FD_*` constants for use with [`fcntl_getfd`] and [`fcntl_setfd`].
    ///
    /// [`fcntl_getfd`]: crate::io::fcntl_getfd
    /// [`fcntl_setfd`]: crate::io::fcntl_setfd
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct FdFlags: c::c_uint {
        /// `FD_CLOEXEC`
        const CLOEXEC = linux_raw_sys::general::FD_CLOEXEC;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

bitflags! {
    /// `RWF_*` constants for use with [`preadv2`] and [`pwritev2`].
    ///
    /// [`preadv2`]: crate::io::preadv2
    /// [`pwritev2`]: crate::io::pwritev
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct ReadWriteFlags: c::c_uint {
        /// `RWF_DSYNC` (since Linux 4.7)
        const DSYNC = linux_raw_sys::general::RWF_DSYNC;
        /// `RWF_HIPRI` (since Linux 4.6)
        const HIPRI = linux_raw_sys::general::RWF_HIPRI;
        /// `RWF_SYNC` (since Linux 4.7)
        const SYNC = linux_raw_sys::general::RWF_SYNC;
        /// `RWF_NOWAIT` (since Linux 4.14)
        const NOWAIT = linux_raw_sys::general::RWF_NOWAIT;
        /// `RWF_APPEND` (since Linux 4.16)
        const APPEND = linux_raw_sys::general::RWF_APPEND;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

bitflags! {
    /// `O_*` constants for use with [`dup2`].
    ///
    /// [`dup2`]: crate::io::dup2
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct DupFlags: c::c_uint {
        /// `O_CLOEXEC`
        const CLOEXEC = linux_raw_sys::general::O_CLOEXEC;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

bitflags! {
    /// `FS_*` constants for use with [`ioctl_getflags`][crate::io::ioctl::ioctl_getflags].
    pub struct IFlags: c::c_uint {
        /// `FS_APPEND_FL`
        const APPEND = linux_raw_sys::general::FS_APPEND_FL;
        /// `FS_COMPR_FL`
        const COMPRESSED = linux_raw_sys::general::FS_COMPR_FL;
        /// `FS_DIRSYNC_FL`
        const DIRSYNC = linux_raw_sys::general::FS_DIRSYNC_FL;
        /// `FS_IMMUTABLE_FL`
        const IMMUTABLE = linux_raw_sys::general::FS_IMMUTABLE_FL;
        /// `FS_JOURNAL_DATA_FL`
        const JOURNALING = linux_raw_sys::general::FS_JOURNAL_DATA_FL;
        /// `FS_NOATIME_FL`
        const NOATIME = linux_raw_sys::general::FS_NOATIME_FL;
        /// `FS_NOCOW_FL`
        const NOCOW = linux_raw_sys::general::FS_NOCOW_FL;
        /// `FS_NODUMP_FL`
        const NODUMP = linux_raw_sys::general::FS_NODUMP_FL;
        /// `FS_NOTAIL_FL`
        const NOTAIL = linux_raw_sys::general::FS_NOTAIL_FL;
        /// `FS_PROJINHERIT_FL`
        const PROJECT_INHERIT = linux_raw_sys::general::FS_PROJINHERIT_FL;
        /// `FS_SECRM_FL`
        const SECURE_REMOVAL = linux_raw_sys::general::FS_SECRM_FL;
        /// `FS_SYNC_FL`
        const SYNC = linux_raw_sys::general::FS_SYNC_FL;
        /// `FS_TOPDIR_FL`
        const TOPDIR = linux_raw_sys::general::FS_TOPDIR_FL;
        /// `FS_UNRM_FL`
        const UNRM = linux_raw_sys::general::FS_UNRM_FL;
    }
}
