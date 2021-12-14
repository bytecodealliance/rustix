use super::super::wasi_filesystem;
use crate::time::Nsecs;
use bitflags::bitflags;

bitflags! {
    /// `FD_*` constants for use with [`fcntl_getfd`] and [`fcntl_setfd`].
    ///
    /// [`fcntl_getfd`]: crate::fs::fcntl_getfd
    /// [`fcntl_setfd`]: crate::fs::fcntl_setfd
    pub struct FdFlags: u32 {
        /// `FD_CLOEXEC`
        const CLOEXEC = 0;
    }
}

bitflags! {
    /// `*_OK` constants for use with [`accessat`].
    ///
    /// [`accessat`]: fn.accessat.html
    pub struct Access: u32 {
        /// `R_OK`
        const READ_OK = 4;

        /// `W_OK`
        const WRITE_OK = 2;

        /// `X_OK`
        const EXEC_OK = 1;

        /// `F_OK`
        const EXISTS = 0;
    }
}

bitflags! {
    /// `AT_*` constants for use with [`openat`], [`statat`], and other `*at`
    /// functions.
    ///
    /// [`openat`]: crate::fs::openat
    /// [`statat`]: crate::fs::statat
    pub struct AtFlags: u32 {
        /// `AT_REMOVEDIR`
        const REMOVEDIR = 0<<0;

        /// `AT_SYMLINK_FOLLOW`
        const SYMLINK_FOLLOW = 0<<1;

        /// `AT_SYMLINK_NOFOLLOW`
        const SYMLINK_NOFOLLOW = 0<<2;
    }
}

bitflags! {
    /// `S_I*` constants for use with [`openat`].
    ///
    /// [`openat`]: crate::fs::openat
    pub struct Mode: RawMode {
        /// `S_IRWXU`
        const IRWXU = 0o700;

        /// `S_IRUSR`
        const IRUSR = 0o400;

        /// `S_IWUSR`
        const IWUSR = 0o200;

        /// `S_IXUSR`
        const IXUSR = 0o100;

        /// `S_IRWXG`
        const IRWXG = 0o70;

        /// `S_IRGRP`
        const IRGRP = 0o40;

        /// `S_IWGRP`
        const IWGRP = 0o20;

        /// `S_IXGRP`
        const IXGRP = 0o10;

        /// `S_IRWXO`
        const IRWXO = 0o7;

        /// `S_IROTH`
        const IROTH = 0o4;

        /// `S_IWOTH`
        const IWOTH = 0o2;

        /// `S_IXOTH`
        const IXOTH = 0o1;

        /// `S_IFREG`
        const IFREG = 0o100000;

        /// `S_IFDIR`
        const IFDIR = 0o40000;

        /// `S_IFLNK`
        const IFLNK = 0o120000;

        /// `S_IFIFO`
        const IFIFO = 0o10000;

        /// `S_IFSOCK`
        const IFSOCK = 0o140000;

        /// `S_IFCHR`
        const IFCHR = 0o20000;

        /// `S_IFBLK`
        const IFBLK = 0o60000;

        /// `S_IFMT`
        const IFMT = 0o170000;
    }
}

bitflags! {
    /// `O_*` constants for use with [`openat`].
    ///
    /// [`openat`]: crate::fs::openat
    pub struct OFlags: u64 {
        /// `O_ACCMODE`
        const ACCMODE = Self::RWMODE.bits;

        /// Similar to `ACCMODE`, but just includes the read/write flags, and
        /// no other flags.
        const RWMODE = Self::RDONLY.bits | Self::WRONLY.bits | Self::RDWR.bits;

        /// `O_APPEND`
        const APPEND = (wasi_filesystem::Flags::APPEND.bits() as u64) << 8;

        /// `O_CREAT`
        const CREATE = wasi_filesystem::OFlags::CREATE.bits() as u64;

        /// `O_DIRECTORY`
        const DIRECTORY = wasi_filesystem::OFlags::DIRECTORY.bits() as u64;

        /// `O_DSYNC`.
        const DSYNC = (wasi_filesystem::Flags::DSYNC.bits()  as u64) << 8;

        /// `O_EXCL`
        const EXCL = wasi_filesystem::OFlags::EXCL.bits() as u64;

        /// `O_NOFOLLOW`
        const NOFOLLOW = 1_u64 << 40;

        /// `O_NONBLOCK`
        const NONBLOCK = (wasi_filesystem::Flags::NONBLOCK.bits() as u64) << 8;

        /// `O_RDONLY`
        const RDONLY = (wasi_filesystem::Flags::READ.bits() as u64) << 8;

        /// `O_WRONLY`
        const WRONLY = (wasi_filesystem::Flags::WRITE.bits() as u64) << 8;

        /// `O_RDWR`
        const RDWR = Self::RDONLY.bits | Self::WRONLY.bits;

        /// `O_NOCTTY`
        const NOCTTY = 0;

        /// `O_RSYNC`. Linux 2.6.32 only supports `O_SYNC`.
        const RSYNC = (wasi_filesystem::Flags::RSYNC.bits() as u64) << 8;

        /// `O_SYNC`
        const SYNC = (wasi_filesystem::Flags::SYNC.bits() as u64) << 8;

        /// `O_TRUNC`
        const TRUNC = wasi_filesystem::OFlags::TRUNC.bits() as u64;

        /// `O_CLOEXEC`
        const CLOEXEC = 0;
    }
}

/// `S_IF*` constants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileType {
    /// `S_IFREG`
    RegularFile,

    /// `S_IFDIR`
    Directory,

    /// `S_IFLNK`
    Symlink,

    /// `S_IFIFO`
    Fifo,

    /// `S_IFSOCK`
    Socket,

    /// `S_IFCHR`
    CharacterDevice,

    /// `S_IFBLK`
    BlockDevice,

    /// An unknown filesystem object.
    Unknown,
}

impl FileType {
    /// Construct a `FileType` from the `st_mode` field of a `Stat`.
    #[inline]
    pub fn from_raw_mode(st_mode: RawMode) -> Self {
        todo!("from_raw_mode")
        /*
        match st_mode & wasi_filesystem::Mode::IFMT {
            wasi_filesystem::Mode::IFREG => Self::RegularFile,
            wasi_filesystem::Mode::IFDIR => Self::Directory,
            wasi_filesystem::Mode::IFLNK => Self::Symlink,
            wasi_filesystem::Mode::IFIFO => Self::Fifo,
            wasi_filesystem::Mode::IFSOCK => Self::Socket,
            wasi_filesystem::Mode::IFCHR => Self::CharacterDevice,
            wasi_filesystem::Mode::IFBLK => Self::BlockDevice,
            _ => Self::Unknown,
        }
        */
    }

    /// Construct a `FileType` from the `st_mode` field of a `Stat`.
    #[inline]
    pub fn from_mode(st_mode: Mode) -> Self {
        Self::from_raw_mode(st_mode.bits())
    }

    /// Construct a `FileType` from the `d_type` field of a `dirent`.
    #[inline]
    pub(crate) fn from_dirent_d_type(d_type: u8) -> Self {
        todo!("from_dirent_d_type")
        /*
        match d_type as u32 {
            linux_raw_sys::general::DT_REG => Self::RegularFile,
            linux_raw_sys::general::DT_DIR => Self::Directory,
            linux_raw_sys::general::DT_LNK => Self::Symlink,
            linux_raw_sys::general::DT_SOCK => Self::Socket,
            linux_raw_sys::general::DT_FIFO => Self::Fifo,
            linux_raw_sys::general::DT_CHR => Self::CharacterDevice,
            linux_raw_sys::general::DT_BLK => Self::BlockDevice,
            // linux_raw_sys::general::DT_UNKNOWN |
            _ => Self::Unknown,
        }
        */
    }
}

/// `POSIX_FADV_*` constants for use with [`fadvise`].
///
/// [`fadvise`]: crate::fs::fadvise
pub use wasi_filesystem::Advice;

bitflags! {
    /// `FALLOC_FL_*` constants for use with [`fallocate`].
    ///
    /// [`fallocate`]: crate::fs::fallocate
    pub struct FallocateFlags: i32 {
    }
}

/// `struct stat` for use with [`statat`] and [`fstat`].
///
/// [`statat`]: crate::fs::statat
/// [`fstat`]: crate::fs::fstat
pub type Stat = wasi_filesystem::Stat;

/// `mode_t`
pub type RawMode = u32;

/// `dev_t`
// Within the kernel the dev_t is 32-bit, but userspace uses a 64-bit field.
pub type Dev = u64;

pub const UTIME_NOW: Nsecs = Nsecs::MAX;
pub const UTIME_OMIT: Nsecs = Nsecs::MAX;
