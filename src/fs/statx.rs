//! Linux `statx`.

use crate::{
    fs::{AtFlags, LibcStatx},
    path, zero_ok,
};
use bitflags::bitflags;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, RawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, RawFd};
use std::{ffi::CStr, io, mem::MaybeUninit};

bitflags! {
    /// `STATX_*` constants.
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

/// `statx(dirfd, path, flags, mask, statxbuf)`. Note that this isn't available
/// on older Linux; returns `ENOSYS` in that case.
#[inline]
pub fn statx<P: path::Arg, Fd: AsRawFd>(
    dirfd: &Fd,
    path: P,
    flags: AtFlags,
    mask: StatxFlags,
) -> io::Result<LibcStatx> {
    let dirfd = dirfd.as_raw_fd();
    let path = path.as_cstr()?;
    unsafe { _statx(dirfd, &path, flags, mask) }
}

unsafe fn _statx(
    dirfd: RawFd,
    path: &CStr,
    flags: AtFlags,
    mask: StatxFlags,
) -> io::Result<LibcStatx> {
    weakcall! {
        fn statx(
            dirfd: libc::c_int,
            path: *const libc::c_char,
            flags: libc::c_int,
            mask: libc::c_uint,
            buf: *mut LibcStatx
        ) -> libc::c_int
    }

    let mut statx_buf = MaybeUninit::<LibcStatx>::uninit();
    zero_ok(statx(
        dirfd as libc::c_int,
        path.as_ptr(),
        flags.bits(),
        mask.bits(),
        statx_buf.as_mut_ptr(),
    ))?;
    Ok(statx_buf.assume_init())
}
