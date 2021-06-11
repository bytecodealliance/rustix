//! Functions which operate on file descriptors.

#[cfg(not(target_os = "wasi"))]
use crate::fs::Mode;
use crate::{negone_err, time::Timespec, zero_ok};
#[cfg(not(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re",
    target_os = "netbsd",
    target_os = "redox",
    target_os = "wasi"
)))]
use libc::fstatfs as libc_fstatfs;
#[cfg(not(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re"
)))]
use libc::lseek as libc_lseek;
#[cfg(not(any(
    target_os = "ios",
    target_os = "freebsd",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
use libc::off64_t as libc_off_t;
#[cfg(any(
    target_os = "ios",
    target_os = "freebsd",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi",
))]
use libc::off_t as libc_off_t;
#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re"
))]
use libc::{fstatfs64 as libc_fstatfs, lseek64 as libc_lseek};
use std::{
    convert::TryInto,
    io::{self, SeekFrom},
};
use unsafe_io::{os::posish::AsRawFd, AsUnsafeHandle, UnsafeHandle};
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "wasi")))]
// not implemented in libc for netbsd yet
use {crate::fs::StatFs, std::mem::MaybeUninit};

/// `lseek(fd, offset, whence)`
#[inline]
pub fn seek<Fd: AsUnsafeHandle>(fd: &Fd, pos: SeekFrom) -> io::Result<u64> {
    let fd = fd.as_unsafe_handle();
    unsafe { _seek(fd, pos) }
}

unsafe fn _seek(fd: UnsafeHandle, pos: SeekFrom) -> io::Result<u64> {
    let (whence, offset): (libc::c_int, libc_off_t) = match pos {
        SeekFrom::Start(pos) => (
            libc::SEEK_SET,
            pos.try_into()
                .map_err(|_convert_err| io::Error::from_raw_os_error(libc::EOVERFLOW))?,
        ),
        SeekFrom::End(offset) => (libc::SEEK_END, offset),
        SeekFrom::Current(offset) => (libc::SEEK_CUR, offset),
    };
    let offset = negone_err(libc_lseek(fd.as_raw_fd() as libc::c_int, offset, whence))?;
    Ok(offset.try_into().unwrap())
}

/// `lseek(fd, 0, SEEK_CUR)`
#[inline]
pub fn tell<Fd: AsUnsafeHandle>(fd: &Fd) -> io::Result<u64> {
    let fd = fd.as_unsafe_handle();
    unsafe { _tell(fd) }
}

unsafe fn _tell(fd: UnsafeHandle) -> io::Result<u64> {
    let offset = negone_err(libc_lseek(fd.as_raw_fd() as libc::c_int, 0, libc::SEEK_CUR))?;
    Ok(offset.try_into().unwrap())
}

/// `fchmod(fd)`.
///
/// Note that this implementation does not support `O_PATH` file descriptors,
/// even on platforms where the host libc emulates it.
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn fchmod<Fd: AsUnsafeHandle>(fd: &Fd, mode: Mode) -> io::Result<()> {
    let fd = fd.as_unsafe_handle();
    unsafe { _fchmod(fd, mode) }
}

#[cfg(not(any(target_os = "android", target_os = "linux", target_os = "wasi")))]
unsafe fn _fchmod(fd: UnsafeHandle, mode: Mode) -> io::Result<()> {
    zero_ok(libc::fchmod(fd.as_raw_fd() as libc::c_int, mode.bits()))
}

#[cfg(any(target_os = "android", target_os = "linux"))]
unsafe fn _fchmod(fd: UnsafeHandle, mode: Mode) -> io::Result<()> {
    // Use `libc::syscall` rather than `libc::fchmod` because some libc
    // implementations, such as musl, add extra logic to `fchmod` to emulate
    // support for `O_PATH`, which uses `/proc` outside our control and
    // interferes with our own use of `O_PATH`.
    zero_ok(libc::syscall(
        libc::SYS_fchmod,
        fd.as_raw_fd() as libc::c_int,
        mode.bits(),
    ))
}

/// `fstatfs(fd)`
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "wasi")))] // not implemented in libc for netbsd yet
#[inline]
pub fn fstatfs<Fd: AsUnsafeHandle>(fd: &Fd) -> io::Result<StatFs> {
    let fd = fd.as_unsafe_handle();
    unsafe { _fstatfs(fd) }
}

#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "wasi")))] // not implemented in libc for netbsd yet
unsafe fn _fstatfs(fd: UnsafeHandle) -> io::Result<StatFs> {
    let mut statfs = MaybeUninit::<StatFs>::uninit();
    zero_ok(libc_fstatfs(
        fd.as_raw_fd() as libc::c_int,
        statfs.as_mut_ptr(),
    ))?;
    Ok(statfs.assume_init())
}

/// `futimens(fd, times)`
#[inline]
pub fn futimens<Fd: AsUnsafeHandle>(fd: &Fd, times: &[Timespec; 2]) -> io::Result<()> {
    let fd = fd.as_unsafe_handle();
    unsafe { _futimens(fd, times) }
}

unsafe fn _futimens(fd: UnsafeHandle, times: &[Timespec; 2]) -> io::Result<()> {
    zero_ok(libc::futimens(
        fd.as_raw_fd() as libc::c_int,
        times.as_ptr(),
    ))
}

/// `posix_fallocate(fd, offset, len)`
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "openbsd")))] // not implemented in libc for netbsd yet
#[inline]
pub fn posix_fallocate<Fd: AsUnsafeHandle>(fd: &Fd, offset: u64, len: u64) -> io::Result<()> {
    let fd = fd.as_unsafe_handle();
    unsafe { _posix_fallocate(fd, offset, len) }
}

#[cfg(not(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox"
)))]
unsafe fn _posix_fallocate(fd: UnsafeHandle, offset: u64, len: u64) -> io::Result<()> {
    let offset = offset
        .try_into()
        .map_err(|_overflow_err| io::Error::from_raw_os_error(libc::EOVERFLOW))?;
    let len = len
        .try_into()
        .map_err(|_overflow_err| io::Error::from_raw_os_error(libc::EOVERFLOW))?;
    let err = libc::posix_fallocate(fd.as_raw_fd() as libc::c_int, offset, len);

    // `posix_fallocate` returns its error status rather than using `errno`.
    if err == 0 {
        Ok(())
    } else {
        Err(std::io::Error::from_raw_os_error(err))
    }
}

#[cfg(any(target_os = "ios", target_os = "macos",))]
unsafe fn _posix_fallocate(fd: UnsafeHandle, offset: u64, len: u64) -> io::Result<()> {
    let offset: i64 = offset
        .try_into()
        .map_err(|_overflow_err| io::Error::from_raw_os_error(libc::EOVERFLOW))?;
    let len = len
        .try_into()
        .map_err(|_overflow_err| io::Error::from_raw_os_error(libc::EOVERFLOW))?;
    let new_len = offset.checked_add(len).ok_or_else(|| {
        io::Error::new(io::ErrorKind::Other, "overflow while allocating file space")
    })?;
    let mut store = libc::fstore_t {
        fst_flags: libc::F_ALLOCATECONTIG,
        fst_posmode: libc::F_PEOFPOSMODE,
        fst_offset: 0,
        fst_length: new_len,
        fst_bytesalloc: 0,
    };
    let ret = libc::fcntl(fd.as_raw_fd() as libc::c_int, libc::F_PREALLOCATE, &store);
    if ret == -1 {
        store.fst_flags = libc::F_ALLOCATEALL;
        negone_err(libc::fcntl(fd.as_raw_fd(), libc::F_PREALLOCATE, &store))?;
    }
    zero_ok(libc::ftruncate(fd.as_raw_fd(), new_len))
}

/// `fcntl(fd, F_GETFL) & O_ACCMODE`. Returns a pair of booleans indicating
/// whether the file descriptor is readable and/or writeable, respectively.
/// This is only reliable on files; for example, it doesn't reflect whether
/// sockets have been shut down; for general I/O handle support, use
/// [`crate::io::is_read_write`].
pub fn is_file_read_write<Fd: AsUnsafeHandle>(fd: &Fd) -> io::Result<(bool, bool)> {
    let fd = fd.as_unsafe_handle();
    unsafe { _is_file_read_write(fd) }
}

pub(crate) unsafe fn _is_file_read_write(fd: UnsafeHandle) -> io::Result<(bool, bool)> {
    let mode = crate::fs::fcntl::_getfl(fd)?;

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
