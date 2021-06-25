use bitflags::bitflags;

bitflags! {
    /// `FD_*` constants for use with [`fcntl_getfd`] and [`fcntl_setfd`].
    ///
    /// [`fcntl_getfd`]: crate::fs::fcntl_getfd
    /// [`fcntl_setfd`]: crate::fs::fcntl_setfd
    pub struct FdFlags: libc::c_int {
        /// `FD_CLOEXEC`
        const CLOEXEC = libc::FD_CLOEXEC;
    }
}

bitflags! {
    /// `*_OK` constants for use with [`accessat`].
    ///
    /// [`accessat`]: fn.accessat.html
    pub struct Access: libc::c_int {
        /// `R_OK`
        const READ_OK = libc::R_OK;

        /// `W_OK`
        const WRITE_OK = libc::W_OK;

        /// `X_OK`
        const EXEC_OK = libc::X_OK;

        /// `F_OK`
        const EXISTS = libc::F_OK;
    }
}

#[cfg(not(target_os = "redox"))]
bitflags! {
    /// `AT_*` constants for use with [`openat`], [`statat`], and other `*at`
    /// functions.
    ///
    /// [`openat`]: crate::fs::openat
    /// [`statat`]: crate::fs::statat
    pub struct AtFlags: libc::c_int {
        /// `AT_REMOVEDIR`
        const REMOVEDIR = libc::AT_REMOVEDIR;

        /// `AT_SYMLINK_FOLLOW`
        const SYMLINK_FOLLOW = libc::AT_SYMLINK_FOLLOW;

        /// `AT_SYMLINK_NOFOLLOW`
        const SYMLINK_NOFOLLOW = libc::AT_SYMLINK_NOFOLLOW;

        /// `AT_EMPTY_PATH`
        #[cfg(any(target_os = "android",
                  target_os = "fuchsia",
                  target_os = "linux"))]
        const EMPTY_PATH = libc::AT_EMPTY_PATH;

        /// `AT_EACCESS`
        #[cfg(not(any(target_os = "emscripten", target_os = "android")))]
        const EACCESS = libc::AT_EACCESS;
    }
}

bitflags! {
    /// `S_I*` constants for use with [`openat`].
    ///
    /// [`openat`]: crate::fs::openat
    pub struct Mode: RawMode {
        /// `S_IRWXU`
        #[cfg(not(target_os = "wasi"))] // WASI doesn't have Unix-style mode flags.
        const IRWXU = libc::S_IRWXU;

        /// `S_IRUSR`
        #[cfg(not(target_os = "wasi"))] // WASI doesn't have Unix-style mode flags.
        const IRUSR = libc::S_IRUSR;

        /// `S_IWUSR`
        #[cfg(not(target_os = "wasi"))] // WASI doesn't have Unix-style mode flags.
        const IWUSR = libc::S_IWUSR;

        /// `S_IXUSR`
        #[cfg(not(target_os = "wasi"))] // WASI doesn't have Unix-style mode flags.
        const IXUSR = libc::S_IXUSR;

        /// `S_IRWXG`
        #[cfg(not(target_os = "wasi"))] // WASI doesn't have Unix-style mode flags.
        const IRWXG = libc::S_IRWXG;

        /// `S_IRGRP`
        #[cfg(not(target_os = "wasi"))] // WASI doesn't have Unix-style mode flags.
        const IRGRP = libc::S_IRGRP;

        /// `S_IWGRP`
        #[cfg(not(target_os = "wasi"))] // WASI doesn't have Unix-style mode flags.
        const IWGRP = libc::S_IWGRP;

        /// `S_IXGRP`
        #[cfg(not(target_os = "wasi"))] // WASI doesn't have Unix-style mode flags.
        const IXGRP = libc::S_IXGRP;

        /// `S_IRWXO`
        #[cfg(not(target_os = "wasi"))] // WASI doesn't have Unix-style mode flags.
        const IRWXO = libc::S_IRWXO;

        /// `S_IROTH`
        #[cfg(not(target_os = "wasi"))] // WASI doesn't have Unix-style mode flags.
        const IROTH = libc::S_IROTH;

        /// `S_IWOTH`
        #[cfg(not(target_os = "wasi"))] // WASI doesn't have Unix-style mode flags.
        const IWOTH = libc::S_IWOTH;

        /// `S_IXOTH`
        #[cfg(not(target_os = "wasi"))] // WASI doesn't have Unix-style mode flags.
        const IXOTH = libc::S_IXOTH;

        /// `S_ISUID`
        #[cfg(not(target_os = "wasi"))] // WASI doesn't have Unix-style mode flags.
        const ISUID = libc::S_ISUID as libc::mode_t;

        /// `S_ISGID`
        #[cfg(not(target_os = "wasi"))] // WASI doesn't have Unix-style mode flags.
        const ISGID = libc::S_ISGID as libc::mode_t;

        /// `S_ISVTX`
        #[cfg(not(target_os = "wasi"))] // WASI doesn't have Unix-style mode flags.
        const ISVTX = libc::S_ISVTX as libc::mode_t;

        /// `S_IFREG`
        const IFREG = libc::S_IFREG;

        /// `S_IFDIR`
        const IFDIR = libc::S_IFDIR;

        /// `S_IFLNK`
        const IFLNK = libc::S_IFLNK;

        /// `S_IFIFO`
        #[cfg(not(target_os = "wasi"))] // TODO: Use WASI's `S_IFIFO`.
        const IFIFO = libc::S_IFIFO;

        /// `S_IFSOCK`
        #[cfg(not(target_os = "wasi"))] // TODO: Use WASI's `S_IFSOCK`.
        const IFSOCK = libc::S_IFSOCK;

        /// `S_IFCHR`
        const IFCHR = libc::S_IFCHR;

        /// `S_IFBLK`
        const IFBLK = libc::S_IFBLK;

        /// `S_IFMT`
        const IFMT = libc::S_IFMT;
    }
}

bitflags! {
    /// `O_*` constants for use with [`openat`].
    ///
    /// [`openat`]: crate::fs::openat
    pub struct OFlags: libc::c_int {
        /// `O_ACCMODE`
        const ACCMODE = libc::O_ACCMODE;

        /// Similar to `ACCMODE`, but just includes the read/write flags, and
        /// no other flags.
        ///
        /// Some implementations include `O_PATH` in `O_ACCMODE`, when
        /// sometimes we really just want the read/write bits. Caution is
        /// indicated, as the presence of `O_PATH` may mean that the read/write
        /// bits don't have their usual meaning.
        const RWMODE = libc::O_RDONLY | libc::O_WRONLY | libc::O_RDWR;

        /// `O_APPEND`
        const APPEND = libc::O_APPEND;

        /// `O_CREAT`
        const CREATE = libc::O_CREAT;

        /// `O_DIRECTORY`
        const DIRECTORY = libc::O_DIRECTORY;

        /// `O_DSYNC`
        #[cfg(not(any(target_os = "freebsd", target_os = "redox")))]
        const DSYNC = libc::O_DSYNC;

        /// `O_EXCL`
        const EXCL = libc::O_EXCL;

        /// `O_FSYNC`
        #[cfg(any(target_os = "dragonfly",
                  target_os = "freebsd",
                  target_os = "ios",
                  all(target_os = "linux", not(target_env = "musl")),
                  target_os = "macos",
                  target_os = "netbsd",
                  target_os = "openbsd"))]
        const FSYNC = libc::O_FSYNC;

        /// `O_NOFOLLOW`
        const NOFOLLOW = libc::O_NOFOLLOW;

        /// `O_NONBLOCK`
        const NONBLOCK = libc::O_NONBLOCK;

        /// `O_RDONLY`
        const RDONLY = libc::O_RDONLY;

        /// `O_WRONLY`
        const WRONLY = libc::O_WRONLY;

        /// `O_RDWR`
        const RDWR = libc::O_RDWR;

        /// `O_NOCTTY`
        #[cfg(not(target_os = "redox"))]
        const NOCTTY = libc::O_NOCTTY;

        /// `O_RSYNC`
        #[cfg(any(target_os = "android",
                  target_os = "emscripten",
                  target_os = "linux",
                  target_os = "netbsd",
                  target_os = "openbsd",
                  target_os = "wasi",
                  ))]
        const RSYNC = libc::O_RSYNC;

        /// `O_SYNC`
        #[cfg(not(target_os = "redox"))]
        const SYNC = libc::O_SYNC;

        /// `O_TRUNC`
        const TRUNC = libc::O_TRUNC;

        /// `O_PATH`
        #[cfg(any(target_os = "android",
                  target_os = "emscripten",
                  target_os = "fuchsia",
                  target_os = "linux",
                  target_os = "redox"))]
        const PATH = libc::O_PATH;

        /// `O_CLOEXEC`
        #[cfg(any(target_os = "android",
                  target_os = "dragonfly",
                  target_os = "emscripten",
                  target_os = "fuchsia",
                  target_os = "freebsd",
                  target_os = "haiku",
                  target_os = "hermit",
                  target_os = "ios",
                  target_os = "linux",
                  target_os = "macos",
                  target_os = "netbsd",
                  target_os = "openbsd",
                  target_os = "redox",
                  target_os = "solaris",
                  target_os = "vxworks"))]
        const CLOEXEC = libc::O_CLOEXEC;

        /// `O_TMPFILE`
        #[cfg(any(target_os = "android",
                  target_os = "emscripten",
                  target_os = "fuchsia",
                  target_os = "linux"))]
        const TMPFILE = libc::O_TMPFILE;
    }
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
bitflags! {
    /// `CLONE_*` constants for use with [`fclonefileat`].
    ///
    /// [`fclonefileat`]: crate::fs::fclonefileat
    pub struct CloneFlags: libc::c_int {
        /// `CLONE_NOFOLLOW`
        const NOFOLLOW = 1;

        /// `CLONE_NOOWNERCOPY`
        const NOOWNERCOPY = 2;
    }
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
mod copyfile {
    pub(super) const ACL: u32 = 1 << 0;
    pub(super) const STAT: u32 = 1 << 1;
    pub(super) const XATTR: u32 = 1 << 2;
    pub(super) const DATA: u32 = 1 << 3;
    pub(super) const SECURITY: u32 = STAT | ACL;
    pub(super) const METADATA: u32 = SECURITY | XATTR;
    pub(super) const ALL: u32 = METADATA | DATA;
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
bitflags! {
    /// `COPYFILE_*` constants.
    pub struct CopyfileFlags: libc::c_uint {
        /// `COPYFILE_ACL`
        const ACL = copyfile::ACL;

        /// `COPYFILE_STAT`
        const STAT = copyfile::STAT;

        /// `COPYFILE_XATTR`
        const XATTR = copyfile::XATTR;

        /// `COPYFILE_DATA`
        const DATA = copyfile::DATA;

        /// `COPYFILE_SECURITY`
        const SECURITY = copyfile::SECURITY;

        /// `COPYFILE_METADATA`
        const METADATA = copyfile::METADATA;

        /// `COPYFILE_ALL`
        const ALL = copyfile::ALL;
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
bitflags! {
    /// `RESOLVE_*` constants for use with [`openat2`].
    ///
    /// [`openat2`]: crate::fs::openat2
    pub struct ResolveFlags: u64 {
        /// `RESOLVE_NO_MAGICLINKS`
        const NO_MAGICLINKS = 0x02;

        /// `RESOLVE_BENEATH`
        const BENEATH = 0x08;
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
        match st_mode & libc::S_IFMT {
            libc::S_IFREG => Self::RegularFile,
            libc::S_IFDIR => Self::Directory,
            libc::S_IFLNK => Self::Symlink,
            #[cfg(not(target_os = "wasi"))] // TODO: Use WASI's `S_IFIFO`.
            libc::S_IFIFO => Self::Fifo,
            #[cfg(not(target_os = "wasi"))] // TODO: Use WASI's `S_IFSOCK`.
            libc::S_IFSOCK => Self::Socket,
            libc::S_IFCHR => Self::CharacterDevice,
            libc::S_IFBLK => Self::BlockDevice,
            _ => Self::Unknown,
        }
    }

    /// Construct a `FileType` from the `st_mode` field of a `Stat`.
    #[inline]
    pub const fn from_mode(st_mode: Mode) -> Self {
        Self::from_raw_mode(st_mode.bits())
    }

    /// Construct a `FileType` from the `d_type` field of a `libc::dirent`.
    #[cfg(not(target_os = "redox"))]
    pub(crate) const fn from_dirent_d_type(d_type: u8) -> Self {
        match d_type {
            libc::DT_REG => Self::RegularFile,
            libc::DT_DIR => Self::Directory,
            libc::DT_LNK => Self::Symlink,
            #[cfg(not(target_os = "wasi"))] // TODO: Use WASI's `DT_SOCK`.
            libc::DT_SOCK => Self::Socket,
            #[cfg(not(target_os = "wasi"))] // TODO: Use WASI's `DT_FIFO`.
            libc::DT_FIFO => Self::Fifo,
            libc::DT_CHR => Self::CharacterDevice,
            libc::DT_BLK => Self::BlockDevice,
            // libc::DT_UNKNOWN |
            _ => Self::Unknown,
        }
    }
}

/// `POSIX_FADV_*` constants for use with [`fadvise`].
///
/// [`fadvise`]: crate::fs::fadvise
#[cfg(not(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox"
)))]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum Advice {
    /// `POSIX_FADV_NORMAL`
    Normal = libc::POSIX_FADV_NORMAL as libc::c_uint,

    /// `POSIX_FADV_SEQUENTIAL`
    Sequential = libc::POSIX_FADV_SEQUENTIAL as libc::c_uint,

    /// `POSIX_FADV_RANDOM`
    Random = libc::POSIX_FADV_RANDOM as libc::c_uint,

    /// `POSIX_FADV_NOREUSE`
    NoReuse = libc::POSIX_FADV_NOREUSE as libc::c_uint,

    /// `POSIX_FADV_WILLNEED`
    WillNeed = libc::POSIX_FADV_WILLNEED as libc::c_uint,

    /// `POSIX_FADV_DONTNEED`
    DontNeed = libc::POSIX_FADV_DONTNEED as libc::c_uint,
}

#[cfg(any(target_os = "android", target_os = "linux"))]
bitflags! {
    /// `MFD_*` constants for use with [`memfd_create`].
    ///
    /// [`memfd_create`]: crate::fs::memfd_create
    pub struct MemfdFlags: std::os::raw::c_uint {
        /// `MFD_CLOEXEC`
        const CLOEXEC = libc::MFD_CLOEXEC;

        /// `MFD_ALLOW_SEALING`
        const ALLOW_SEALING = libc::MFD_ALLOW_SEALING;
    }
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
bitflags! {
    pub struct StatxFlags: u32 {
        /// `STATX_TYPE`
        const TYPE = libc::STATX_TYPE;

        /// `STATX_MODE`
        const MODE = libc::STATX_MODE;

        /// `STATX_NLINK`
        const NLINK = libc::STATX_NLINK;

        /// `STATX_UID`
        const UID = libc::STATX_UID;

        /// `STATX_GID`
        const GID = libc::STATX_GID;

        /// `STATX_ATIME`
        const ATIME = libc::STATX_ATIME;

        /// `STATX_MTIME`
        const MTIME = libc::STATX_MTIME;

        /// `STATX_CTIME`
        const CTIME = libc::STATX_CTIME;

        /// `STATX_INO`
        const INO = libc::STATX_INO;

        /// `STATX_SIZE`
        const SIZE = libc::STATX_SIZE;

        /// `STATX_BLOCKS`
        const BLOCKS = libc::STATX_BLOCKS;

        /// `STATX_BASIC_STATS`
        const BASIC_STATS = libc::STATX_BASIC_STATS;

        /// `STATX_BTIME`
        const BTIME = libc::STATX_BTIME;

        /// `STATX_ALL`
        const ALL = libc::STATX_ALL;
    }
}

/// `struct stat` for use with [`statat`] and [`fstat`].
///
/// [`statat`]: crate::fs::statat
/// [`fstat`]: crate::fs::fstat
#[cfg(not(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re"
)))]
pub type Stat = libc::stat;

/// `struct stat` for use with [`statat`] and [`fstat`].
///
/// [`statat`]: crate::fs::statat
/// [`fstat`]: crate::fs::fstat
#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re"
))]
pub type Stat = libc::stat64;

/// `struct statfs` for use with [`fstatfs`].
///
/// [`fstatfs`]: crate::fs::fstatfs
#[cfg(not(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re",
    target_os = "netbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
#[allow(clippy::module_name_repetitions)]
pub type StatFs = libc::statfs;

/// `struct statfs` for use with [`fstatfs`].
///
/// [`fstatfs`]: crate::fs::fstatfs
#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re"
))]
pub type StatFs = libc::statfs64;

/// `struct statx` for use with [`statx`].
///
/// Only available on Linux with GLIBC for now.
///
/// [`statx`]: crate::fs::statx
#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub type Statx = libc::statx;

/// `mode_t`.
pub type RawMode = libc::mode_t;

/// `dev_t`.
pub type Dev = libc::dev_t;

/// `__fsword_t`.
#[cfg(all(target_os = "linux", not(target_env = "musl")))]
pub type FsWord = libc::__fsword_t;

/// `__fsword_t`.
#[cfg(all(
    any(target_os = "android", all(target_os = "linux", target_env = "musl")),
    target_pointer_width = "32"
))]
pub type FsWord = u32;

/// `__fsword_t`.
#[cfg(all(
    any(target_os = "android", all(target_os = "linux", target_env = "musl")),
    target_pointer_width = "64"
))]
pub type FsWord = u64;

#[cfg(not(target_os = "redox"))]
pub use libc::{UTIME_NOW, UTIME_OMIT};

#[cfg(all(
    any(target_os = "android", target_os = "linux"),
    not(target_env = "musl")
))]
pub const PROC_SUPER_MAGIC: FsWord = libc::PROC_SUPER_MAGIC as FsWord;

#[cfg(all(any(target_os = "android", target_os = "linux"), target_env = "musl"))]
pub const PROC_SUPER_MAGIC: FsWord = 0x0000_9fa0;

#[cfg(any(target_os = "ios", target_os = "macos"))]
#[allow(non_camel_case_types)]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct copyfile_state_t(pub(crate) *mut libc::c_void);
