//! Libc filesystem constants, translated into `bitflags` constants.

use bitflags::bitflags;
use cfg_if::cfg_if;

bitflags! {
    /// `FD_*` constants.
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

bitflags! {
    /// `AT_*` constants.
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
        // Temporarily disable on Emscripten until https://github.com/rust-lang/libc/pull/1836
        // is available.
        #[cfg(not(any(target_os = "emscripten", target_os = "android")))]
        const EACCESS = libc::AT_EACCESS;
    }
}

#[cfg(not(target_os = "wasi"))]
bitflags! {
    /// `S_I*` constants.
    pub struct Mode: libc::mode_t {
        /// `S_IRWXU`
        const IRWXU = libc::S_IRWXU;

        /// `S_IRUSR`
        const IRUSR = libc::S_IRUSR;

        /// `S_IWUSR`
        const IWUSR = libc::S_IWUSR;

        /// `S_IXUSR`
        const IXUSR = libc::S_IXUSR;

        /// `S_IRWXG`
        const IRWXG = libc::S_IRWXG;

        /// `S_IRGRP`
        const IRGRP = libc::S_IRGRP;

        /// `S_IWGRP`
        const IWGRP = libc::S_IWGRP;

        /// `S_IXGRP`
        const IXGRP = libc::S_IXGRP;

        /// `S_IRWXO`
        const IRWXO = libc::S_IRWXO;

        /// `S_IROTH`
        const IROTH = libc::S_IROTH;

        /// `S_IWOTH`
        const IWOTH = libc::S_IWOTH;

        /// `S_IXOTH`
        const IXOTH = libc::S_IXOTH;

        /// `S_ISUID`
        const ISUID = libc::S_ISUID as libc::mode_t;

        /// `S_ISGID`
        const ISGID = libc::S_ISGID as libc::mode_t;

        /// `S_ISVTX`
        const ISVTX = libc::S_ISVTX as libc::mode_t;
    }
}

#[cfg(target_os = "wasi")]
pub struct Mode {}

#[cfg(target_os = "wasi")]
impl Mode {
    pub fn bits(&self) -> u32 {
        0
    }
}

bitflags! {
    /// `O_*` constants.
    pub struct OFlags: libc::c_int {
        /// `O_ACCMODE`
        const ACCMODE = libc::O_ACCMODE;

        /// Similar to `ACCMODE`, but just includes the read/write flags, and
        /// no other flags.
        ///
        /// Some libc implementations include `O_PATH` in `O_ACCMODE`, which
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
        const DSYNC = {
            cfg_if! {
                if #[cfg(any(target_os = "android",
                             target_os = "emscripten",
                             target_os = "fuchsia",
                             target_os = "ios",
                             target_os = "linux",
                             target_os = "macos",
                             target_os = "netbsd",
                             target_os = "openbsd",
                             target_os = "wasi"))] {
                    libc::O_DSYNC
                } else if #[cfg(target_os = "freebsd")] {
                    // FreeBSD lacks `O_DSYNC`; emulate it with `O_SYNC`, which
                    // is correct, though conservative.
                    // https://reviews.freebsd.org/D19407#inline-118670
                    libc::O_SYNC
                }
            }
        };

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
        const NOCTTY = libc::O_NOCTTY;

        /// `O_RSYNC`
        #[cfg(any(target_os = "emscripten",
                  target_os = "linux",
                  target_os = "netbsd",
                  target_os = "openbsd",
                  target_os = "wasi",
                  ))]
        const RSYNC = libc::O_RSYNC;

        /// `O_SYNC`
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
    /// `CLONE_*` constants.
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
    /// `RESOLVE_*` constants.
    pub struct ResolveFlags: u64 {
        /// `RESOLVE_NO_MAGICLINKS`
        const NO_MAGICLINKS = 0x02;

        /// `RESOLVE_BENEATH`
        const BENEATH = 0x08;
    }
}
