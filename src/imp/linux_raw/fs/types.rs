use bitflags::bitflags;

bitflags! {
    /// `FD_*` constants for use with [`fcntl_getfd`] and [`fcntl_setfd`].
    ///
    /// [`fcntl_getfd`]: crate::fs::fcntl_getfd
    /// [`fcntl_setfd`]: crate::fs::fcntl_setfd
    pub struct FdFlags: std::os::raw::c_uint {
        /// `FD_CLOEXEC`
        const CLOEXEC = linux_raw_sys::general::FD_CLOEXEC;
    }
}

bitflags! {
    /// `*_OK` constants for use with [`accessat`].
    ///
    /// [`accessat`]: fn.accessat.html
    pub struct Access: std::os::raw::c_uint {
        /// `R_OK`
        const READ_OK = linux_raw_sys::general::R_OK;

        /// `W_OK`
        const WRITE_OK = linux_raw_sys::general::W_OK;

        /// `X_OK`
        const EXEC_OK = linux_raw_sys::general::X_OK;

        /// `F_OK`
        const EXISTS = linux_raw_sys::general::F_OK;
    }
}

bitflags! {
    /// `AT_*` constants for use with [`openat`], [`statat`], and other `*at`
    /// functions.
    ///
    /// [`openat`]: crate::fs::openat
    /// [`statat`]: crate::fs::statat
    pub struct AtFlags: std::os::raw::c_uint {
        /// `AT_REMOVEDIR`
        const REMOVEDIR = linux_raw_sys::general::AT_REMOVEDIR;

        /// `AT_SYMLINK_FOLLOW`
        const SYMLINK_FOLLOW = linux_raw_sys::general::AT_SYMLINK_FOLLOW;

        /// `AT_SYMLINK_NOFOLLOW`
        const SYMLINK_NOFOLLOW = linux_raw_sys::general::AT_SYMLINK_NOFOLLOW;

        /// `AT_EMPTY_PATH`
        const EMPTY_PATH = linux_raw_sys::v5_4::general::AT_EMPTY_PATH;

        /// `AT_EACCESS`
        const EACCESS = linux_raw_sys::v5_11::general::AT_EACCESS;
    }
}

bitflags! {
    /// `S_I*` constants for use with [`openat`].
    ///
    /// [`openat`]: crate::fs::openat
    pub struct Mode: RawMode {
        /// `S_IRWXU`
        const IRWXU = linux_raw_sys::general::S_IRWXU;

        /// `S_IRUSR`
        const IRUSR = linux_raw_sys::general::S_IRUSR;

        /// `S_IWUSR`
        const IWUSR = linux_raw_sys::general::S_IWUSR;

        /// `S_IXUSR`
        const IXUSR = linux_raw_sys::general::S_IXUSR;

        /// `S_IRWXG`
        const IRWXG = linux_raw_sys::general::S_IRWXG;

        /// `S_IRGRP`
        const IRGRP = linux_raw_sys::general::S_IRGRP;

        /// `S_IWGRP`
        const IWGRP = linux_raw_sys::general::S_IWGRP;

        /// `S_IXGRP`
        const IXGRP = linux_raw_sys::general::S_IXGRP;

        /// `S_IRWXO`
        const IRWXO = linux_raw_sys::general::S_IRWXO;

        /// `S_IROTH`
        const IROTH = linux_raw_sys::general::S_IROTH;

        /// `S_IWOTH`
        const IWOTH = linux_raw_sys::general::S_IWOTH;

        /// `S_IXOTH`
        const IXOTH = linux_raw_sys::general::S_IXOTH;

        /// `S_ISUID`
        const ISUID = linux_raw_sys::general::S_ISUID;

        /// `S_ISGID`
        const ISGID = linux_raw_sys::general::S_ISGID;

        /// `S_ISVTX`
        const ISVTX = linux_raw_sys::general::S_ISVTX;

        /// `S_IFREG`
        const IFREG = linux_raw_sys::general::S_IFREG;

        /// `S_IFDIR`
        const IFDIR = linux_raw_sys::general::S_IFDIR;

        /// `S_IFLNK`
        const IFLNK = linux_raw_sys::general::S_IFLNK;

        /// `S_IFIFO`
        const IFIFO = linux_raw_sys::general::S_IFIFO;

        /// `S_IFSOCK`
        const IFSOCK = linux_raw_sys::general::S_IFSOCK;

        /// `S_IFCHR`
        const IFCHR = linux_raw_sys::general::S_IFCHR;

        /// `S_IFBLK`
        const IFBLK = linux_raw_sys::general::S_IFBLK;

        /// `S_IFMT`
        const IFMT = linux_raw_sys::general::S_IFMT;
    }
}

bitflags! {
    /// `O_*` constants for use with [`openat`].
    ///
    /// [`openat`]: crate::fs::openat
    pub struct OFlags: std::os::raw::c_uint {
        /// `O_ACCMODE`
        const ACCMODE = linux_raw_sys::general::O_ACCMODE;

        /// Similar to `ACCMODE`, but just includes the read/write flags, and
        /// no other flags.
        ///
        /// Some implementations include `O_PATH` in `O_ACCMODE`, when
        /// sometimes we really just want the read/write bits. Caution is
        /// indicated, as the presence of `O_PATH` may mean that the read/write
        /// bits don't have their usual meaning.
        const RWMODE = linux_raw_sys::general::O_RDONLY |
                       linux_raw_sys::general::O_WRONLY |
                       linux_raw_sys::general::O_RDWR;

        /// `O_APPEND`
        const APPEND = linux_raw_sys::general::O_APPEND;

        /// `O_CREAT`
        const CREATE = linux_raw_sys::general::O_CREAT;

        /// `O_DIRECTORY`
        const DIRECTORY = linux_raw_sys::general::O_DIRECTORY;

        /// `O_DSYNC`. Linux 2.6.32 only supports `O_SYNC`.
        const DSYNC = linux_raw_sys::general::O_SYNC;

        /// `O_EXCL`
        const EXCL = linux_raw_sys::general::O_EXCL;

        /// `O_FSYNC`. Linux 2.6.32 only supports `O_SYNC`.
        const FSYNC = linux_raw_sys::general::O_SYNC;

        /// `O_NOFOLLOW`
        const NOFOLLOW = linux_raw_sys::general::O_NOFOLLOW;

        /// `O_NONBLOCK`
        const NONBLOCK = linux_raw_sys::general::O_NONBLOCK;

        /// `O_RDONLY`
        const RDONLY = linux_raw_sys::general::O_RDONLY;

        /// `O_WRONLY`
        const WRONLY = linux_raw_sys::general::O_WRONLY;

        /// `O_RDWR`
        const RDWR = linux_raw_sys::general::O_RDWR;

        /// `O_NOCTTY`
        const NOCTTY = linux_raw_sys::general::O_NOCTTY;

        /// `O_RSYNC`. Linux 2.6.32 only supports `O_SYNC`.
        const RSYNC = linux_raw_sys::general::O_SYNC;

        /// `O_SYNC`
        const SYNC = linux_raw_sys::general::O_SYNC;

        /// `O_TRUNC`
        const TRUNC = linux_raw_sys::general::O_TRUNC;

        /// `O_PATH`
        const PATH = linux_raw_sys::v5_4::general::O_PATH;

        /// `O_CLOEXEC`
        const CLOEXEC = linux_raw_sys::general::O_CLOEXEC;

        /// `O_TMPFILE`
        const TMPFILE = linux_raw_sys::v5_4::general::O_TMPFILE;
    }
}

bitflags! {
    /// `RESOLVE_*` constants for use with [`openat2`].
    ///
    /// [`openat2`]: crate::fs::openat2
    pub struct ResolveFlags: u64 {
        /// `RESOLVE_NO_MAGICLINKS`
        const NO_MAGICLINKS = linux_raw_sys::v5_11::general::RESOLVE_NO_MAGICLINKS as u64;

        /// `RESOLVE_BENEATH`
        const BENEATH = linux_raw_sys::v5_11::general::RESOLVE_BENEATH as u64;
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
    pub const fn from_raw_mode(st_mode: RawMode) -> Self {
        match st_mode & linux_raw_sys::general::S_IFMT {
            linux_raw_sys::general::S_IFREG => Self::RegularFile,
            linux_raw_sys::general::S_IFDIR => Self::Directory,
            linux_raw_sys::general::S_IFLNK => Self::Symlink,
            linux_raw_sys::general::S_IFIFO => Self::Fifo,
            linux_raw_sys::general::S_IFSOCK => Self::Socket,
            linux_raw_sys::general::S_IFCHR => Self::CharacterDevice,
            linux_raw_sys::general::S_IFBLK => Self::BlockDevice,
            _ => Self::Unknown,
        }
    }

    /// Construct a `FileType` from the `st_mode` field of a `Stat`.
    pub const fn from_mode(st_mode: Mode) -> Self {
        Self::from_raw_mode(st_mode.bits())
    }

    /// Construct a `FileType` from the `d_type` field of a `dirent`.
    pub(crate) const fn from_dirent_d_type(d_type: u8) -> Self {
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
    }
}

/// `POSIX_FADV_*` constants for use with [`fadvise`].
///
/// [`fadvise`]: crate::fs::fadvise
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum Advice {
    /// `POSIX_FADV_NORMAL`
    Normal = linux_raw_sys::general::POSIX_FADV_NORMAL,

    /// `POSIX_FADV_SEQUENTIAL`
    Sequential = linux_raw_sys::general::POSIX_FADV_SEQUENTIAL,

    /// `POSIX_FADV_RANDOM`
    Random = linux_raw_sys::general::POSIX_FADV_RANDOM,

    /// `POSIX_FADV_NOREUSE`
    NoReuse = linux_raw_sys::general::POSIX_FADV_NOREUSE,

    /// `POSIX_FADV_WILLNEED`
    WillNeed = linux_raw_sys::general::POSIX_FADV_WILLNEED,

    /// `POSIX_FADV_DONTNEED`
    DontNeed = linux_raw_sys::general::POSIX_FADV_DONTNEED,
}

bitflags! {
    /// `MFD_*` constants for use with [`memfd_create`].
    ///
    /// [`memfd_create`]: crate::fs::memfd_create
    pub struct MemfdFlags: std::os::raw::c_uint {
        /// `MFD_CLOEXEC`
        const CLOEXEC = linux_raw_sys::v5_4::general::MFD_CLOEXEC;

        /// `MFD_ALLOW_SEALING`
        const ALLOW_SEALING = linux_raw_sys::v5_4::general::MFD_ALLOW_SEALING;
    }
}

bitflags! {
    pub struct StatxFlags: u32 {
        /// `STATX_TYPE`
        const TYPE = linux_raw_sys::v5_4::general::STATX_TYPE;

        /// `STATX_MODE`
        const MODE = linux_raw_sys::v5_4::general::STATX_MODE;

        /// `STATX_NLINK`
        const NLINK = linux_raw_sys::v5_4::general::STATX_NLINK;

        /// `STATX_UID`
        const UID = linux_raw_sys::v5_4::general::STATX_UID;

        /// `STATX_GID`
        const GID = linux_raw_sys::v5_4::general::STATX_GID;

        /// `STATX_ATIME`
        const ATIME = linux_raw_sys::v5_4::general::STATX_ATIME;

        /// `STATX_MTIME`
        const MTIME = linux_raw_sys::v5_4::general::STATX_MTIME;

        /// `STATX_CTIME`
        const CTIME = linux_raw_sys::v5_4::general::STATX_CTIME;

        /// `STATX_INO`
        const INO = linux_raw_sys::v5_4::general::STATX_INO;

        /// `STATX_SIZE`
        const SIZE = linux_raw_sys::v5_4::general::STATX_SIZE;

        /// `STATX_BLOCKS`
        const BLOCKS = linux_raw_sys::v5_4::general::STATX_BLOCKS;

        /// `STATX_BASIC_STATS`
        const BASIC_STATS = linux_raw_sys::v5_4::general::STATX_BASIC_STATS;

        /// `STATX_BTIME`
        const BTIME = linux_raw_sys::v5_4::general::STATX_BTIME;

        /// `STATX_ALL`
        const ALL = linux_raw_sys::v5_4::general::STATX_ALL;
    }
}

bitflags! {
    /// `FALLOC_FL_*` constants for use with [`fallocate`].
    ///
    /// [`fallocate`]: crate::fs::fallocate
    pub struct FallocateFlags: u32 {
        /// `FALLOC_FL_KEEP_SIZE`
        const KEEP_SIZE = linux_raw_sys::general::FALLOC_FL_KEEP_SIZE;
        /// `FALLOC_FL_PUNCH_HOLE`
        const PUNCH_HOLE = linux_raw_sys::v5_4::general::FALLOC_FL_PUNCH_HOLE;
        /// `FALLOC_FL_NO_HIDE_STALE`
        const NO_HIDE_STALE = linux_raw_sys::v5_4::general::FALLOC_FL_NO_HIDE_STALE;
        /// `FALLOC_FL_COLLAPSE_RANGE`
        const COLLAPSE_RANGE = linux_raw_sys::v5_4::general::FALLOC_FL_COLLAPSE_RANGE;
        /// `FALLOC_FL_ZERO_RANGE`
        const ZERO_RANGE = linux_raw_sys::v5_4::general::FALLOC_FL_ZERO_RANGE;
        /// `FALLOC_FL_INSERT_RANGE`
        const INSERT_RANGE = linux_raw_sys::v5_4::general::FALLOC_FL_INSERT_RANGE;
        /// `FALLOC_FL_UNSHARE_RANGE`
        const UNSHARE_RANGE = linux_raw_sys::v5_4::general::FALLOC_FL_UNSHARE_RANGE;
    }
}

/// `struct statfs` for use with [`fstatfs`].
///
/// [`fstatfs`]: crate::fs::fstatfs
#[cfg(target_pointer_width = "32")]
pub type Stat = linux_raw_sys::general::stat64;

/// `struct statfs` for use with [`fstatfs`].
///
/// [`fstatfs`]: crate::fs::fstatfs
#[cfg(target_pointer_width = "64")]
pub type Stat = linux_raw_sys::general::stat;

/// `struct statfs` for use with [`fstatfs`].
///
/// [`fstatfs`]: crate::fs::fstatfs
#[cfg(target_pointer_width = "32")]
#[allow(clippy::module_name_repetitions)]
pub type StatFs = linux_raw_sys::general::statfs64;

/// `struct statfs` for use with [`fstatfs`].
///
/// [`fstatfs`]: crate::fs::fstatfs
#[cfg(target_pointer_width = "64")]
#[allow(clippy::module_name_repetitions)]
pub type StatFs = linux_raw_sys::general::statfs64;

/// `struct statx` for use with [`statx`].
///
/// Only available on Linux with GLIBC for now.
///
/// [`statx`]: crate::fs::statx
pub type Statx = linux_raw_sys::v5_4::general::statx;

/// `mode_t`
#[cfg(not(any(
    target_arch = "x86",
    target_arch = "sparc",
    target_arch = "avr",
    target_arch = "arm"
)))]
pub type RawMode = linux_raw_sys::general::__kernel_mode_t;

/// `mode_t
#[cfg(any(
    target_arch = "x86",
    target_arch = "sparc",
    target_arch = "avr",
    target_arch = "arm"
))]
// Don't use `__kernel_mode_t` since it's `u16` which differs from `st_size`.
pub type RawMode = std::os::raw::c_uint;

/// `dev_t`
// Within the kernel the dev_t is 32-bit, but userspace uses a 64-bit field.
pub type Dev = u64;

/// `__fsword_t`
pub type FsWord = linux_raw_sys::general::__fsword_t;

pub use linux_raw_sys::general::{UTIME_NOW, UTIME_OMIT};

pub const PROC_SUPER_MAGIC: FsWord = linux_raw_sys::general::PROC_SUPER_MAGIC as FsWord;
