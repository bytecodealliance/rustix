//! Functions which operate on file descriptors.

#[cfg(not(target_os = "wasi"))]
use crate::fs::Mode;
use crate::{negone_err, zero_ok};
#[cfg(not(any(
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re",
    target_os = "netbsd",
    target_os = "redox",
    target_os = "wasi"
)))]
use libc::fstatfs as libc_fstatfs;
#[cfg(not(any(target_os = "linux", target_os = "emscripten", target_os = "l4re")))]
use libc::lseek as libc_lseek;
#[cfg(not(any(
    target_os = "ios",
    target_os = "freebsd",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
use libc::off64_t as libc_off_t;
#[cfg(any(
    target_os = "ios",
    target_os = "freebsd",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "redox",
    target_os = "wasi",
))]
use libc::off_t as libc_off_t;
#[cfg(any(target_os = "linux", target_os = "emscripten", target_os = "l4re"))]
use libc::{fstatfs64 as libc_fstatfs, lseek64 as libc_lseek};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, RawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, RawFd};
use std::{
    convert::TryInto,
    io::{self, SeekFrom},
};
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "wasi")))]
// not implemented in libc for netbsd yet
use {crate::fs::LibcStatFs, std::mem::MaybeUninit};

/// `lseek(fd, offset, whence)`
#[inline]
pub fn seek<Fd: AsRawFd>(fd: &Fd, pos: SeekFrom) -> io::Result<u64> {
    let fd = fd.as_raw_fd();
    unsafe { _seek(fd, pos) }
}

unsafe fn _seek(fd: RawFd, pos: SeekFrom) -> io::Result<u64> {
    let (whence, offset): (libc::c_int, libc_off_t) = match pos {
        SeekFrom::Start(pos) => (
            libc::SEEK_SET,
            pos.try_into()
                .map_err(|_| io::Error::from_raw_os_error(libc::EOVERFLOW))?,
        ),
        SeekFrom::End(offset) => (libc::SEEK_END, offset),
        SeekFrom::Current(offset) => (libc::SEEK_CUR, offset),
    };
    let offset = negone_err(libc_lseek(fd as libc::c_int, offset, whence))?;
    Ok(offset.try_into().unwrap())
}

/// `lseek(fd, 0, SEEK_CUR)`
#[inline]
pub fn tell<Fd: AsRawFd>(fd: &Fd) -> io::Result<u64> {
    let fd = fd.as_raw_fd();
    unsafe { _tell(fd) }
}

unsafe fn _tell(fd: RawFd) -> io::Result<u64> {
    let offset = negone_err(libc_lseek(fd as libc::c_int, 0, libc::SEEK_CUR))?;
    Ok(offset.try_into().unwrap())
}

/// `fchmod(fd)`.
///
/// Note that this implementation does not support `O_PATH` file descriptors,
/// even on platforms where the host libc emulates it.
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn fchmod<Fd: AsRawFd>(fd: &Fd, mode: Mode) -> io::Result<()> {
    let fd = fd.as_raw_fd();
    unsafe { _fchmod(fd, mode) }
}

#[cfg(not(any(target_os = "linux", target_os = "wasi")))]
unsafe fn _fchmod(fd: RawFd, mode: Mode) -> io::Result<()> {
    zero_ok(libc::fchmod(fd as libc::c_int, mode.bits()))
}

#[cfg(target_os = "linux")]
unsafe fn _fchmod(fd: RawFd, mode: Mode) -> io::Result<()> {
    // Use `libc::syscall` rather than `libc::fchmod` because some libc
    // implementations, such as musl, add extra logic to `fchmod` to emulate
    // support for `O_PATH`, which uses `/proc` outside our control and
    // interferes with our own use of `O_PATH`.
    zero_ok(libc::syscall(
        libc::SYS_fchmod,
        fd as libc::c_int,
        mode.bits(),
    ))
}

/// `fstatfs(fd)`
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "wasi")))] // not implemented in libc for netbsd yet
#[inline]
pub fn fstatfs<Fd: AsRawFd>(fd: &Fd) -> io::Result<LibcStatFs> {
    let fd = fd.as_raw_fd();
    unsafe { _fstatfs(fd) }
}

#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "wasi")))] // not implemented in libc for netbsd yet
unsafe fn _fstatfs(fd: RawFd) -> io::Result<LibcStatFs> {
    let mut statfs = MaybeUninit::<LibcStatFs>::uninit();
    zero_ok(libc_fstatfs(fd as libc::c_int, statfs.as_mut_ptr()))?;
    Ok(statfs.assume_init())
}

/// `futimens(fd, times)`
#[inline]
pub fn futimens<Fd: AsRawFd>(fd: &Fd, times: &[libc::timespec; 2]) -> io::Result<()> {
    let fd = fd.as_raw_fd();
    unsafe { _futimens(fd, times) }
}

unsafe fn _futimens(fd: RawFd, times: &[libc::timespec; 2]) -> io::Result<()> {
    zero_ok(libc::futimens(fd as libc::c_int, times.as_ptr()))
}
