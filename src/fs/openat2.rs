use crate::{
    fs::{Mode, OFlags, ResolveFlags},
    negone_err, path,
};
use std::{convert::TryInto, ffi::CStr, fs, io};
use unsafe_io::{
    os::posish::{AsRawFd, FromRawFd},
    AsUnsafeHandle, UnsafeHandle,
};

#[cfg(target_pointer_width = "32")]
const SYS_OPENAT2: i32 = 437;
#[cfg(target_pointer_width = "64")]
const SYS_OPENAT2: i64 = 437;

#[repr(C)]
#[derive(Debug)]
struct OpenHow {
    oflag: u64,
    mode: u64,
    resolve: u64,
}
const SIZEOF_OPEN_HOW: usize = std::mem::size_of::<OpenHow>();

/// `openat2(dirfd, path, OpenHow { oflags, mode, resolve }, sizeof(OpenHow))`
#[inline]
pub fn openat2<Fd: AsUnsafeHandle, P: path::Arg>(
    dirfd: &Fd,
    path: P,
    oflags: OFlags,
    mode: Mode,
    resolve: ResolveFlags,
) -> io::Result<fs::File> {
    let dirfd = dirfd.as_unsafe_handle();
    let path = path.as_c_str()?;
    unsafe { _openat2(dirfd, &path, oflags, mode, resolve) }
}

unsafe fn _openat2(
    dirfd: UnsafeHandle,
    path: &CStr,
    oflags: OFlags,
    mode: Mode,
    resolve: ResolveFlags,
) -> io::Result<fs::File> {
    let oflags: i32 = oflags.bits();
    let open_how = OpenHow {
        oflag: u64::from(oflags as u32),
        mode: u64::from(mode.bits()),
        resolve: resolve.bits(),
    };

    let fd = negone_err(libc::syscall(
        SYS_OPENAT2,
        dirfd.as_raw_fd(),
        path.as_ptr(),
        &open_how,
        SIZEOF_OPEN_HOW,
    ))?;

    #[allow(clippy::useless_conversion)]
    Ok(fs::File::from_raw_fd(fd.try_into().unwrap()))
}
