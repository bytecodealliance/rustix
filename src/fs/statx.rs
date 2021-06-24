//! Linux `statx`.

use crate::{
    fs::{AtFlags, Statx},
    io, path,
};
use bitflags::bitflags;
use io_lifetimes::{AsFd, BorrowedFd};
use std::ffi::CStr;
#[cfg(libc)]
use {crate::libc::conv::c_str, crate::zero_ok, std::mem::MaybeUninit};

#[cfg(libc)]
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

#[cfg(linux_raw)]
bitflags! {
    /// `STATX_*` constants.
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

/// `statx(dirfd, path, flags, mask, statxbuf)`
///
/// Note that this isn't available on Linux before 4.11; returns `ENOSYS` in
/// that case.
#[inline]
pub fn statx<'f, P: path::Arg, Fd: AsFd>(
    dirfd: &Fd,
    path: P,
    flags: AtFlags,
    mask: StatxFlags,
) -> io::Result<Statx> {
    let dirfd = dirfd.as_fd();
    let path = path.into_c_str()?;
    _statx(dirfd, &path, flags, mask)
}

#[cfg(libc)]
fn _statx(
    dirfd: BorrowedFd<'_>,
    path: &CStr,
    flags: AtFlags,
    mask: StatxFlags,
) -> io::Result<Statx> {
    weakcall! {
        fn statx(
            dirfd: BorrowedFd<'_>,
            path: *const libc::c_char,
            flags: libc::c_int,
            mask: libc::c_uint,
            buf: *mut Statx
        ) -> libc::c_int
    }

    let mut statx_buf = MaybeUninit::<Statx>::uninit();
    unsafe {
        zero_ok(statx(
            dirfd,
            c_str(path),
            flags.bits(),
            mask.bits(),
            statx_buf.as_mut_ptr(),
        ))?;
        Ok(statx_buf.assume_init())
    }
}

#[cfg(linux_raw)]
#[inline]
fn _statx(
    dirfd: BorrowedFd<'_>,
    path: &CStr,
    flags: AtFlags,
    mask: StatxFlags,
) -> io::Result<Statx> {
    crate::linux_raw::statx(dirfd, path, flags.bits(), mask.bits())
}
