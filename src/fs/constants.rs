//! Filesystem API constants, translated into `bitflags` constants.

use crate::fs::RawMode;
use bitflags::bitflags;

#[cfg(libc)]
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

#[cfg(linux_raw)]
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

#[cfg(libc)]
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

#[cfg(linux_raw)]
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

#[cfg(all(libc, not(target_os = "redox")))]
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

#[cfg(linux_raw)]
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

#[cfg(libc)]
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

#[cfg(linux_raw)]
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

#[cfg(libc)]
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

#[cfg(linux_raw)]
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

#[cfg(all(libc, any(target_os = "android", target_os = "linux")))]
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

#[cfg(linux_raw)]
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
