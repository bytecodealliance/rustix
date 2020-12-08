//! Functions which operate on file descriptors.

use std::io;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, RawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, RawFd};
#[cfg(not(target_os = "redox"))]
use {
    crate::zero_ok,
    std::{convert::TryInto, mem::MaybeUninit},
};

/// `ioctl(fd, FIONREAD)`.
#[cfg(not(target_os = "redox"))]
#[inline]
pub fn fionread<Fd: AsRawFd>(fd: &Fd) -> io::Result<u64> {
    let fd = fd.as_raw_fd();
    unsafe { _fionread(fd) }
}

#[cfg(not(target_os = "redox"))]
unsafe fn _fionread(fd: RawFd) -> io::Result<u64> {
    let mut nread = MaybeUninit::<libc::c_int>::uninit();
    zero_ok(libc::ioctl(
        fd as libc::c_int,
        libc::FIONREAD,
        nread.as_mut_ptr(),
    ))?;
    Ok(nread.assume_init().try_into().unwrap())
}

/// `isatty(fd)`
#[inline]
pub fn isatty<Fd: AsRawFd>(fd: &Fd) -> bool {
    let fd = fd.as_raw_fd();
    unsafe { _isatty(fd) }
}

unsafe fn _isatty(fd: RawFd) -> bool {
    let res = libc::isatty(fd as libc::c_int);
    if res == 0 {
        let err = io::Error::last_os_error();
        match err.raw_os_error() {
            #[cfg(not(target_os = "linux"))]
            Some(libc::ENOTTY) => false,

            // Old Linux versions reportedly return `EINVAL`.
            // https://man7.org/linux/man-pages/man3/isatty.3.html#ERRORS
            #[cfg(target_os = "linux")]
            Some(libc::ENOTTY) | Some(libc::EINVAL) => false,

            _ => panic!("unexpected error from isatty: {}", err),
        }
    } else {
        true
    }
}

/// `fcntl(fd, F_GETFL) & O_ACCMODE`. Returns a pair of booleans indicating
/// whether the file descriptor is readable and/or writeable, respectively.
pub fn is_read_write<Fd: AsRawFd>(fd: &Fd) -> io::Result<(bool, bool)> {
    let mode = crate::fs::getfl(fd)?;

    // Check for `O_PATH`.
    #[cfg(any(
        target_os = "android",
        target_os = "fuchsia",
        target_os = "linux",
        target_os = "emscripten"
    ))]
    if mode.contains(crate::fs::OFlags::PATH) {
        return Ok((false, false));
    }

    // Use `RWMODE` rather than `ACCMODE` as `ACCMODE` may include `O_PATH`.
    // We handled `O_PATH` above.
    match mode & crate::fs::OFlags::RWMODE {
        crate::fs::OFlags::RDONLY => Ok((true, false)),
        crate::fs::OFlags::RDWR => Ok((true, true)),
        crate::fs::OFlags::WRONLY => Ok((false, true)),
        _ => unreachable!(),
    }
}
