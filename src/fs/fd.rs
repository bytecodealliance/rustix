//! Functions which operate on file descriptors.

#[cfg(not(target_os = "wasi"))]
use crate::fs::Mode;
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "wasi")))]
// not implemented in libc for netbsd yet
use crate::fs::StatFs;
use crate::{fs::Stat, io, time::Timespec};
use io_lifetimes::{AsFd, BorrowedFd};
#[cfg(all(
    libc,
    not(any(
        target_os = "android",
        target_os = "linux",
        target_os = "emscripten",
        target_os = "l4re",
    ))
))]
use libc::fstat as libc_fstat;
#[cfg(all(
    libc,
    not(any(
        target_os = "android",
        target_os = "linux",
        target_os = "emscripten",
        target_os = "l4re",
        target_os = "netbsd",
        target_os = "redox",
        target_os = "wasi"
    ))
))]
use libc::fstatfs as libc_fstatfs;
#[cfg(all(
    libc,
    not(any(
        target_os = "android",
        target_os = "linux",
        target_os = "emscripten",
        target_os = "l4re"
    ))
))]
use libc::lseek as libc_lseek;
#[cfg(all(
    libc,
    not(any(
        target_os = "ios",
        target_os = "freebsd",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "redox",
        target_os = "wasi",
    ))
))]
use libc::off64_t as libc_off_t;
#[cfg(all(
    libc,
    any(
        target_os = "ios",
        target_os = "freebsd",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "redox",
        target_os = "wasi",
    )
))]
use libc::off_t as libc_off_t;
#[cfg(all(
    libc,
    any(
        target_os = "android",
        target_os = "linux",
        target_os = "emscripten",
        target_os = "l4re"
    )
))]
use libc::{fstat64 as libc_fstat, fstatfs64 as libc_fstatfs, lseek64 as libc_lseek};
use std::{convert::TryInto, io::SeekFrom};
#[cfg(libc)]
use {
    crate::{negone_err, zero_ok},
    std::mem::MaybeUninit,
    unsafe_io::os::posish::AsRawFd,
};

/// `lseek(fd, offset, whence)`
#[inline]
pub fn seek<'f, Fd: AsFd<'f>>(fd: Fd, pos: SeekFrom) -> io::Result<u64> {
    let fd = fd.as_fd();
    _seek(fd, pos)
}

#[cfg(libc)]
fn _seek(fd: BorrowedFd<'_>, pos: SeekFrom) -> io::Result<u64> {
    let (whence, offset): (libc::c_int, libc_off_t) = match pos {
        SeekFrom::Start(pos) => (
            libc::SEEK_SET,
            pos.try_into().map_err(|_convert_err| io::Error::OVERFLOW)?,
        ),
        SeekFrom::End(offset) => (libc::SEEK_END, offset),
        SeekFrom::Current(offset) => (libc::SEEK_CUR, offset),
    };
    let offset = unsafe { negone_err(libc_lseek(fd.as_raw_fd() as libc::c_int, offset, whence))? };
    Ok(offset as u64)
}

#[cfg(linux_raw)]
#[inline]
fn _seek(fd: BorrowedFd<'_>, pos: SeekFrom) -> io::Result<u64> {
    let (whence, offset) = match pos {
        SeekFrom::Start(pos) => (
            linux_raw_sys::general::SEEK_SET,
            pos.try_into().map_err(|_convert_err| io::Error::OVERFLOW)?,
        ),
        SeekFrom::End(offset) => (linux_raw_sys::general::SEEK_END, offset),
        SeekFrom::Current(offset) => (linux_raw_sys::general::SEEK_CUR, offset),
    };
    crate::linux_raw::seek(fd, offset, whence)
}

/// `lseek(fd, 0, SEEK_CUR)`
#[inline]
pub fn tell<'f, Fd: AsFd<'f>>(fd: Fd) -> io::Result<u64> {
    let fd = fd.as_fd();
    _tell(fd)
}

#[cfg(libc)]
fn _tell(fd: BorrowedFd<'_>) -> io::Result<u64> {
    let offset =
        unsafe { negone_err(libc_lseek(fd.as_raw_fd() as libc::c_int, 0, libc::SEEK_CUR))? };
    Ok(offset as u64)
}

#[cfg(linux_raw)]
#[inline]
fn _tell(fd: BorrowedFd<'_>) -> io::Result<u64> {
    crate::linux_raw::seek(fd, 0, linux_raw_sys::general::SEEK_CUR).map(|x| x as u64)
}

/// `fchmod(fd)`.
///
/// Note that this implementation does not support `O_PATH` file descriptors,
/// even on platforms where the host libc emulates it.
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn fchmod<'f, Fd: AsFd<'f>>(fd: Fd, mode: Mode) -> io::Result<()> {
    let fd = fd.as_fd();
    _fchmod(fd, mode)
}

#[cfg(all(
    libc,
    not(any(target_os = "android", target_os = "linux", target_os = "wasi"))
))]
fn _fchmod(fd: BorrowedFd<'_>, mode: Mode) -> io::Result<()> {
    unsafe { zero_ok(libc::fchmod(fd.as_raw_fd() as libc::c_int, mode.bits())) }
}

#[cfg(all(libc, any(target_os = "android", target_os = "linux")))]
fn _fchmod(fd: BorrowedFd<'_>, mode: Mode) -> io::Result<()> {
    // Use `libc::syscall` rather than `libc::fchmod` because some libc
    // implementations, such as musl, add extra logic to `fchmod` to emulate
    // support for `O_PATH`, which uses `/proc` outside our control and
    // interferes with our own use of `O_PATH`.
    unsafe {
        zero_ok(libc::syscall(
            libc::SYS_fchmod,
            fd.as_raw_fd() as libc::c_int,
            mode.bits(),
        ))
    }
}

#[cfg(linux_raw)]
#[inline]
fn _fchmod(fd: BorrowedFd<'_>, mode: Mode) -> io::Result<()> {
    crate::linux_raw::fchmod(fd, mode.bits() as u16)
}

/// `fstat(fd)`
#[inline]
pub fn fstat<'f, Fd: AsFd<'f>>(fd: Fd) -> io::Result<Stat> {
    let fd = fd.as_fd();
    _fstat(fd)
}

#[cfg(libc)]
fn _fstat(fd: BorrowedFd<'_>) -> io::Result<Stat> {
    let mut stat = MaybeUninit::<Stat>::uninit();
    unsafe {
        zero_ok(libc_fstat(fd.as_raw_fd() as libc::c_int, stat.as_mut_ptr()))?;
        Ok(stat.assume_init())
    }
}

#[cfg(linux_raw)]
fn _fstat(fd: BorrowedFd<'_>) -> io::Result<Stat> {
    crate::linux_raw::fstat(fd)
}

/// `fstatfs(fd)`
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "wasi")))] // not implemented in libc for netbsd yet
#[inline]
pub fn fstatfs<'f, Fd: AsFd<'f>>(fd: Fd) -> io::Result<StatFs> {
    let fd = fd.as_fd();
    _fstatfs(fd)
}

#[cfg(all(
    libc,
    not(any(target_os = "netbsd", target_os = "redox", target_os = "wasi"))
))] // not implemented in libc for netbsd yet
fn _fstatfs(fd: BorrowedFd<'_>) -> io::Result<StatFs> {
    let mut statfs = MaybeUninit::<StatFs>::uninit();
    unsafe {
        zero_ok(libc_fstatfs(
            fd.as_raw_fd() as libc::c_int,
            statfs.as_mut_ptr(),
        ))?;
        Ok(statfs.assume_init())
    }
}

#[cfg(linux_raw)]
fn _fstatfs(fd: BorrowedFd<'_>) -> io::Result<StatFs> {
    crate::linux_raw::fstatfs(fd)
}

/// `futimens(fd, times)`
#[inline]
pub fn futimens<'f, Fd: AsFd<'f>>(fd: Fd, times: &[Timespec; 2]) -> io::Result<()> {
    let fd = fd.as_fd();
    _futimens(fd, times)
}

#[cfg(libc)]
fn _futimens(fd: BorrowedFd<'_>, times: &[Timespec; 2]) -> io::Result<()> {
    unsafe {
        zero_ok(libc::futimens(
            fd.as_raw_fd() as libc::c_int,
            times.as_ptr(),
        ))
    }
}

#[cfg(linux_raw)]
#[inline]
fn _futimens(fd: BorrowedFd<'_>, times: &[Timespec; 2]) -> io::Result<()> {
    crate::linux_raw::utimensat(fd, None, times, 0)
}

/// `posix_fallocate(fd, offset, len)`
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "openbsd")))] // not implemented in libc for netbsd yet
#[inline]
pub fn posix_fallocate<'f, Fd: AsFd<'f>>(fd: Fd, offset: u64, len: u64) -> io::Result<()> {
    let fd = fd.as_fd();
    _posix_fallocate(fd, offset, len)
}

#[cfg(all(
    libc,
    not(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "redox"
    ))
))]
fn _posix_fallocate(fd: BorrowedFd<'_>, offset: u64, len: u64) -> io::Result<()> {
    let offset = offset
        .try_into()
        .map_err(|_overflow_err| io::Error::OVERFLOW)?;
    let len = len
        .try_into()
        .map_err(|_overflow_err| io::Error::OVERFLOW)?;
    let err = unsafe { libc::posix_fallocate(fd.as_raw_fd() as libc::c_int, offset, len) };

    // `posix_fallocate` returns its error status rather than using `errno`.
    if err == 0 {
        Ok(())
    } else {
        Err(io::Error(err))
    }
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
fn _posix_fallocate(fd: BorrowedFd<'_>, offset: u64, len: u64) -> io::Result<()> {
    let offset: i64 = offset
        .try_into()
        .map_err(|_overflow_err| io::Error::OVERFLOW)?;
    let len = len
        .try_into()
        .map_err(|_overflow_err| io::Error::OVERFLOW)?;
    let new_len = offset.checked_add(len).ok_or_else(|| io::Error::FBIG)?;
    let mut store = libc::fstore_t {
        fst_flags: libc::F_ALLOCATECONTIG,
        fst_posmode: libc::F_PEOFPOSMODE,
        fst_offset: 0,
        fst_length: new_len,
        fst_bytesalloc: 0,
    };
    unsafe {
        let ret = libc::fcntl(fd.as_raw_fd() as libc::c_int, libc::F_PREALLOCATE, &store);
        if ret == -1 {
            store.fst_flags = libc::F_ALLOCATEALL;
            negone_err(libc::fcntl(fd.as_raw_fd(), libc::F_PREALLOCATE, &store))?;
        }
        zero_ok(libc::ftruncate(fd.as_raw_fd(), new_len))
    }
}

#[cfg(linux_raw)]
#[inline]
fn _posix_fallocate(fd: BorrowedFd<'_>, offset: u64, len: u64) -> io::Result<()> {
    let offset = offset
        .try_into()
        .map_err(|_overflow_err| io::Error::OVERFLOW)?;
    let len = len
        .try_into()
        .map_err(|_overflow_err| io::Error::OVERFLOW)?;
    crate::linux_raw::fallocate(fd, 0, offset, len)
}

/// `fcntl(fd, F_GETFL) & O_ACCMODE`. Returns a pair of booleans indicating
/// whether the file descriptor is readable and/or writeable, respectively.
/// This is only reliable on files; for example, it doesn't reflect whether
/// sockets have been shut down; for general I/O handle support, use
/// [`io::is_read_write`].
#[inline]
pub fn is_file_read_write<'f, Fd: AsFd<'f>>(fd: Fd) -> io::Result<(bool, bool)> {
    let fd = fd.as_fd();
    _is_file_read_write(fd)
}

pub(crate) fn _is_file_read_write(fd: BorrowedFd<'_>) -> io::Result<(bool, bool)> {
    let mode = crate::fs::fcntl::_fcntl_getfl(fd)?;

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

/// `fsync(fd)`
#[inline]
pub fn fsync<'f, Fd: AsFd<'f>>(fd: Fd) -> io::Result<()> {
    let fd = fd.as_fd();
    _fsync(fd)
}

#[cfg(libc)]
fn _fsync(fd: BorrowedFd<'_>) -> io::Result<()> {
    unsafe { zero_ok(libc::fsync(fd.as_raw_fd() as libc::c_int)) }
}

#[cfg(linux_raw)]
#[inline]
fn _fsync(fd: BorrowedFd<'_>) -> io::Result<()> {
    crate::linux_raw::fsync(fd)
}

/// `fdatasync(fd)`
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "redox")))]
#[inline]
pub fn fdatasync<'f, Fd: AsFd<'f>>(fd: Fd) -> io::Result<()> {
    let fd = fd.as_fd();
    _fdatasync(fd)
}

#[cfg(all(
    libc,
    not(any(target_os = "ios", target_os = "macos", target_os = "redox"))
))]
fn _fdatasync(fd: BorrowedFd<'_>) -> io::Result<()> {
    unsafe { zero_ok(libc::fdatasync(fd.as_raw_fd() as libc::c_int)) }
}

#[cfg(linux_raw)]
#[inline]
fn _fdatasync(fd: BorrowedFd<'_>) -> io::Result<()> {
    crate::linux_raw::fdatasync(fd)
}

/// `ftruncate(fd, length)`
#[inline]
pub fn ftruncate<'f, Fd: AsFd<'f>>(fd: Fd, length: u64) -> io::Result<()> {
    let fd = fd.as_fd();
    _ftruncate(fd, length)
}

#[cfg(libc)]
fn _ftruncate(fd: BorrowedFd<'_>, length: u64) -> io::Result<()> {
    let length = length.try_into().map_err(|_overflow_err| io::Error::FBIG)?;
    unsafe { zero_ok(libc::ftruncate(fd.as_raw_fd() as libc::c_int, length)) }
}

#[cfg(linux_raw)]
#[inline]
fn _ftruncate(fd: BorrowedFd<'_>, length: u64) -> io::Result<()> {
    crate::linux_raw::ftruncate(fd, length)
}
