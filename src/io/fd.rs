//! Functions which operate on file descriptors.

#[cfg(not(target_os = "wasi"))]
use crate::negone_err;
#[cfg(not(any(target_os = "wasi", target_os = "fuchsia")))]
use std::ffi::OsString;
use std::io;
#[cfg(all(unix, not(any(target_os = "wasi", target_os = "fuchsia"))))]
use std::os::unix::ffi::OsStringExt;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
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
            #[cfg(not(any(target_os = "android", target_os = "linux")))]
            Some(libc::ENOTTY) => false,

            // Old Linux versions reportedly return `EINVAL`.
            // https://man7.org/linux/man-pages/man3/isatty.3.html#ERRORS
            #[cfg(any(target_os = "android", target_os = "linux"))]
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

/// `dup(fd)`
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn dup<Fd: AsRawFd + FromRawFd>(fd: &Fd) -> io::Result<Fd> {
    let fd = fd.as_raw_fd();
    unsafe { _dup(fd).map(|raw_fd| Fd::from_raw_fd(raw_fd)) }
}

#[cfg(not(target_os = "wasi"))]
unsafe fn _dup(fd: RawFd) -> io::Result<RawFd> {
    negone_err(libc::dup(fd as libc::c_int))
}

/// `ttyname_r(fd)`
///
/// If `reuse` is non-empty, reuse its buffer to store the result if possible.
#[cfg(not(any(target_os = "wasi", target_os = "fuchsia")))]
#[inline]
pub fn ttyname<Fd: AsRawFd>(dirfd: &Fd, reuse: OsString) -> io::Result<OsString> {
    let dirfd = dirfd.as_raw_fd();
    unsafe { _ttyname(dirfd, reuse) }
}

#[cfg(not(any(target_os = "wasi", target_os = "fuchsia")))]
unsafe fn _ttyname(dirfd: RawFd, reuse: OsString) -> io::Result<OsString> {
    let mut buffer = reuse.into_vec();

    // Start with a buffer big enough for the vast majority of paths.
    // This and the `reserve` below would be a good candidate for `try_reserve`.
    // https://github.com/rust-lang/rust/issues/48043
    buffer.clear();
    buffer.reserve(256);

    loop {
        match libc::ttyname_r(
            dirfd as libc::c_int,
            buffer.as_mut_ptr() as *mut libc::c_char,
            buffer.capacity(),
        ) {
            // Use `Vec`'s builtin capacity-doubling strategy.
            libc::ERANGE => buffer.reserve(1),
            0 => {
                buffer.set_len(libc::strlen(buffer.as_ptr() as *const libc::c_char));
                return Ok(OsString::from_vec(buffer));
            }
            errno => return Err(io::Error::from_raw_os_error(errno)),
        }
    }
}
