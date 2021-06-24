#[cfg(libc)]
use crate::libc::conv::{borrowed_fd, c_str, syscall_ret_owned_fd};
use crate::{
    fs::{Mode, OFlags, ResolveFlags},
    io, path,
};
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
use std::ffi::CStr;

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
pub fn openat2<Fd: AsFd, P: path::Arg>(
    dirfd: &Fd,
    path: P,
    oflags: OFlags,
    mode: Mode,
    resolve: ResolveFlags,
) -> io::Result<OwnedFd> {
    let dirfd = dirfd.as_fd();
    let path = path.into_c_str()?;
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
        syscall_ret_owned_fd(libc::syscall(
            SYS_OPENAT2,
            borrowed_fd(dirfd),
            c_str(path),
            &open_how,
            SIZEOF_OPEN_HOW,
        ))
    }
}

#[cfg(linux_raw)]
#[inline]
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
