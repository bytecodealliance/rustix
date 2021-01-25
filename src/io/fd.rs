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
use {crate::zero_ok, std::convert::TryInto, std::mem::MaybeUninit};

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

            // Darwin mysteriously returns `EOPNOTSUPP` sometimes.
            #[cfg(any(target_os = "ios", target_os = "macos"))]
            Some(libc::EOPNOTSUPP) => false,

            _ => panic!("unexpected error from isatty: {}", err),
        }
    } else {
        true
    }
}

/// Returns a pair of booleans indicating whether the file descriptor is readable
/// and/or writeable, respectively. Unlike [`is_file_read_write`], this correctly
/// detects whether sockets have been shutdown, partially or completely.
#[cfg(not(target_os = "redox"))]
#[inline]
pub fn is_read_write<Fd: AsRawFd>(fd: &Fd) -> io::Result<(bool, bool)> {
    let fd = fd.as_raw_fd();
    unsafe { _is_read_write(fd) }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
unsafe fn _is_read_write(fd: RawFd) -> io::Result<(bool, bool)> {
    let (mut read, mut write) = crate::fs::is_file_read_write(&fd)?;
    let mut not_socket = false;
    if read {
        // Do a `recv` with `PEEK` and `DONTWAIT` for 1 byte. A 0 indicates
        // the read side is shut down; an `EWOULDBLOCK` indicates the read
        // side is still open.
        match libc::recv(
            fd,
            MaybeUninit::<[u8; 1]>::uninit()
                .as_mut_ptr()
                .cast::<libc::c_void>(),
            1,
            libc::MSG_PEEK | libc::MSG_DONTWAIT,
        ) {
            0 => read = false,
            -1 => {
                let err = io::Error::last_os_error();
                #[allow(unreachable_patterns)] // EAGAIN may equal EWOULDBLOCK
                match err.raw_os_error() {
                    Some(libc::EAGAIN) | Some(libc::EWOULDBLOCK) => (),
                    Some(libc::ENOTSOCK) => not_socket = true,
                    _ => return Err(err),
                }
            }
            _ => (),
        }
    }
    if write && !not_socket {
        // Do a `send` with `DONTWAIT` for 0 bytes. An `EPIPE` indicates
        // the write side is shut down.
        match libc::send(fd, [].as_ptr(), 0, libc::MSG_DONTWAIT) {
            -1 => {
                let err = io::Error::last_os_error();
                #[allow(unreachable_patterns)] // EAGAIN may equal EWOULDBLOCK
                match err.raw_os_error() {
                    Some(libc::EAGAIN) | Some(libc::EWOULDBLOCK) => (),
                    Some(libc::ENOTSOCK) => (),
                    Some(libc::EPIPE) => write = false,
                    _ => return Err(err),
                }
            }
            _ => (),
        }
    }
    Ok((read, write))
}

#[cfg(target_os = "wasi")]
unsafe fn _is_read_write(_fd: RawFd) -> io::Result<(bool, bool)> {
    todo!("Implement is_read_write in terms of fd_fdstat_get");
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
            buffer.as_mut_ptr().cast::<libc::c_char>(),
            buffer.capacity(),
        ) {
            // Use `Vec`'s builtin capacity-doubling strategy.
            libc::ERANGE => buffer.reserve(1),
            0 => {
                buffer.set_len(libc::strlen(buffer.as_ptr().cast::<libc::c_char>()));
                return Ok(OsString::from_vec(buffer));
            }
            errno => return Err(io::Error::from_raw_os_error(errno)),
        }
    }
}
