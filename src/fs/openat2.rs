use crate::{
    fs::{Mode, OFlags, ResolveFlags},
    path,
};
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
use std::{ffi::CStr, io};
#[cfg(libc)]
use {
    crate::negone_err,
    unsafe_io::os::posish::{AsRawFd, FromRawFd, RawFd},
};

#[cfg(all(libc, target_pointer_width = "32"))]
const SYS_OPENAT2: i32 = 437;
#[cfg(all(libc, target_pointer_width = "64"))]
const SYS_OPENAT2: i64 = 437;

#[cfg(libc)]
#[repr(C)]
#[derive(Debug)]
struct OpenHow {
    oflag: u64,
    mode: u64,
    resolve: u64,
}
#[cfg(libc)]
const SIZEOF_OPEN_HOW: usize = std::mem::size_of::<OpenHow>();

/// `openat2(dirfd, path, OpenHow { oflags, mode, resolve }, sizeof(OpenHow))`
#[inline]
pub fn openat2<'f, Fd: AsFd<'f>, P: path::Arg>(
    dirfd: Fd,
    path: P,
    oflags: OFlags,
    mode: Mode,
    resolve: ResolveFlags,
) -> io::Result<OwnedFd> {
    let dirfd = dirfd.as_fd();
    let path = path.as_c_str()?;
    _openat2(dirfd, &path, oflags, mode, resolve)
}

#[cfg(libc)]
fn _openat2(
    dirfd: BorrowedFd<'_>,
    path: &CStr,
    oflags: OFlags,
    mode: Mode,
    resolve: ResolveFlags,
) -> io::Result<OwnedFd> {
    let oflags: i32 = oflags.bits();
    let open_how = OpenHow {
        oflag: u64::from(oflags as u32),
        mode: u64::from(mode.bits()),
        resolve: resolve.bits(),
    };

    unsafe {
        let fd = negone_err(libc::syscall(
            SYS_OPENAT2,
            dirfd.as_raw_fd(),
            path.as_ptr(),
            &open_how,
            SIZEOF_OPEN_HOW,
        ))?;

        #[allow(clippy::useless_conversion)]
        Ok(OwnedFd::from_raw_fd(fd as RawFd))
    }
}

#[cfg(linux_raw)]
fn _openat2(
    dirfd: BorrowedFd<'_>,
    path: &CStr,
    oflags: OFlags,
    mode: Mode,
    resolve: ResolveFlags,
) -> io::Result<OwnedFd> {
    crate::linux_raw::openat2(
        dirfd,
        path,
        u64::from(oflags.bits()),
        u64::from(mode.bits()),
        u64::from(resolve.bits()),
    )
}
