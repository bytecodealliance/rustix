//! `read` and `write`, optionally positioned, optionally vectored

use crate::io;
#[cfg(any(linux_raw, all(libc, target_os = "linux", target_env = "gnu")))]
use bitflags::bitflags;
use io_lifetimes::{AsFd, BorrowedFd};
#[cfg(all(
    libc,
    not(any(target_os = "android", target_os = "linux", target_os = "emscripten"))
))]
use libc::{pread as libc_pread, pwrite as libc_pwrite};
#[cfg(all(
    libc,
    any(target_os = "android", target_os = "linux", target_os = "emscripten")
))]
use libc::{
    pread64 as libc_pread, preadv64 as libc_preadv, pwrite64 as libc_pwrite,
    pwritev64 as libc_pwritev,
};
#[cfg(all(
    libc,
    not(any(
        target_os = "android",
        target_os = "linux",
        target_os = "emscripten",
        target_os = "redox"
    ))
))]
use libc::{preadv as libc_preadv, pwritev as libc_pwritev};
// `preadv64v2`/`pwritev64v2` submitted upstream here:
// <https://github.com/rust-lang/libc/pull/2257>
#[cfg(all(
    libc,
    target_pointer_width = "64",
    target_os = "linux",
    target_env = "gnu"
))]
use libc::{preadv2 as libc_preadv2, pwritev2 as libc_pwritev2};
#[cfg(libc)]
use libc::{read as libc_read, readv as libc_readv, write as libc_write, writev as libc_writev};
#[cfg(all(libc, not(any(target_os = "redox", target_env = "newlib"))))]
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{
    cmp::min,
    io::{IoSlice, IoSliceMut},
};
#[cfg(libc)]
use {
    crate::libc::conv::{borrowed_fd, ret_ssize_t},
    std::os::raw::c_int,
};

#[cfg(all(libc, target_os = "linux", target_env = "gnu"))]
bitflags! {
    /// `RWF_*` constants.
    pub struct ReadWriteFlags: std::os::raw::c_int {
        /// `RWF_DSYNC`
        const DSYNC = libc::RWF_DSYNC;
        /// `RWF_HIPRI`
        const HIPRI = libc::RWF_HIPRI;
        /// `RWF_SYNC`
        const SYNC = libc::RWF_SYNC;
        /// `RWF_NOWAIT`
        const NOWAIT = libc::RWF_NOWAIT;
        /// `RWF_APPEND`
        const APPEND = libc::RWF_APPEND;
    }
}

#[cfg(linux_raw)]
bitflags! {
    /// `RWF_*` constants.
    pub struct ReadWriteFlags: std::os::raw::c_uint {
        /// `RWF_DSYNC`
        const DSYNC = linux_raw_sys::general::RWF_DSYNC;
        /// `RWF_HIPRI`
        const HIPRI = linux_raw_sys::general::RWF_HIPRI;
        /// `RWF_SYNC`
        const SYNC = linux_raw_sys::general::RWF_SYNC;
        /// `RWF_NOWAIT`
        const NOWAIT = linux_raw_sys::general::RWF_NOWAIT;
        /// `RWF_APPEND`
        const APPEND = linux_raw_sys::general::RWF_APPEND;
    }
}

/// `read(fd, buf.as_ptr(), buf.len())`
#[inline]
pub fn read<Fd: AsFd>(fd: &Fd, buf: &mut [u8]) -> io::Result<usize> {
    let fd = fd.as_fd();
    _read(fd, buf)
}

#[cfg(libc)]
fn _read(fd: BorrowedFd<'_>, buf: &mut [u8]) -> io::Result<usize> {
    let nread = unsafe {
        ret_ssize_t(libc_read(
            borrowed_fd(fd),
            buf.as_mut_ptr().cast::<_>(),
            buf.len(),
        ))?
    };
    Ok(nread as usize)
}

#[cfg(linux_raw)]
#[inline]
fn _read(fd: BorrowedFd<'_>, buf: &mut [u8]) -> io::Result<usize> {
    crate::linux_raw::read(fd, buf)
}

/// `write(fd, buf.ptr(), buf.len())`
#[inline]
pub fn write<Fd: AsFd>(fd: &Fd, buf: &[u8]) -> io::Result<usize> {
    let fd = fd.as_fd();
    _write(fd, buf)
}

#[cfg(libc)]
fn _write(fd: BorrowedFd<'_>, buf: &[u8]) -> io::Result<usize> {
    let nwritten = unsafe {
        ret_ssize_t(libc_write(
            borrowed_fd(fd),
            buf.as_ptr().cast::<_>(),
            buf.len(),
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(linux_raw)]
#[inline]
fn _write(fd: BorrowedFd<'_>, buf: &[u8]) -> io::Result<usize> {
    crate::linux_raw::write(fd, buf)
}

/// `pread(fd, buf.as_ptr(), bufs.len(), offset)`
#[inline]
pub fn pread<Fd: AsFd>(fd: &Fd, buf: &mut [u8], offset: u64) -> io::Result<usize> {
    let fd = fd.as_fd();
    _pread(fd, buf, offset)
}

#[cfg(libc)]
fn _pread(fd: BorrowedFd<'_>, buf: &mut [u8], offset: u64) -> io::Result<usize> {
    // Silently cast; we'll get `EINVAL` if the value is negative.
    let offset = offset as i64;
    let nread = unsafe {
        ret_ssize_t(libc_pread(
            borrowed_fd(fd),
            buf.as_mut_ptr().cast::<_>(),
            buf.len(),
            offset,
        ))?
    };
    Ok(nread as usize)
}

#[cfg(linux_raw)]
#[inline]
fn _pread(fd: BorrowedFd<'_>, buf: &[u8], offset: u64) -> io::Result<usize> {
    crate::linux_raw::pread(fd, buf, offset)
}

/// `pwrite(fd, bufs.as_ptr(), bufs.len())`
#[inline]
pub fn pwrite<Fd: AsFd>(fd: &Fd, buf: &[u8], offset: u64) -> io::Result<usize> {
    let fd = fd.as_fd();
    _pwrite(fd, buf, offset)
}

#[cfg(libc)]
fn _pwrite(fd: BorrowedFd<'_>, buf: &[u8], offset: u64) -> io::Result<usize> {
    // Silently cast; we'll get `EINVAL` if the value is negative.
    let offset = offset as i64;
    let nwritten = unsafe {
        ret_ssize_t(libc_pwrite(
            borrowed_fd(fd),
            buf.as_ptr().cast::<_>(),
            buf.len(),
            offset,
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(linux_raw)]
#[inline]
fn _pwrite(fd: BorrowedFd<'_>, buf: &[u8], offset: u64) -> io::Result<usize> {
    crate::linux_raw::pwrite(fd, buf, offset)
}

/// `readv(fd, bufs.as_ptr(), bufs.len())`
#[inline]
pub fn readv<Fd: AsFd>(fd: &Fd, bufs: &[IoSliceMut]) -> io::Result<usize> {
    let fd = fd.as_fd();
    _readv(fd, bufs)
}

#[cfg(libc)]
fn _readv(fd: BorrowedFd<'_>, bufs: &[IoSliceMut]) -> io::Result<usize> {
    let nread = unsafe {
        ret_ssize_t(libc_readv(
            borrowed_fd(fd),
            bufs.as_ptr().cast::<libc::iovec>(),
            min(bufs.len(), max_iov()) as c_int,
        ))?
    };
    Ok(nread as usize)
}

#[cfg(linux_raw)]
#[inline]
fn _readv(fd: BorrowedFd<'_>, bufs: &[IoSliceMut]) -> io::Result<usize> {
    crate::linux_raw::readv(fd, &bufs[..min(bufs.len(), max_iov())])
}

/// `writev(fd, bufs.as_ptr(), bufs.len())`
#[inline]
pub fn writev<Fd: AsFd>(fd: &Fd, bufs: &[IoSlice]) -> io::Result<usize> {
    let fd = fd.as_fd();
    _writev(fd, bufs)
}

#[cfg(libc)]
fn _writev(fd: BorrowedFd<'_>, bufs: &[IoSlice]) -> io::Result<usize> {
    let nwritten = unsafe {
        ret_ssize_t(libc_writev(
            borrowed_fd(fd),
            bufs.as_ptr().cast::<libc::iovec>(),
            min(bufs.len(), max_iov()) as c_int,
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(linux_raw)]
#[inline]
fn _writev(fd: BorrowedFd<'_>, bufs: &[IoSlice]) -> io::Result<usize> {
    crate::linux_raw::writev(fd, &bufs[..min(bufs.len(), max_iov())])
}

/// `preadv(fd, bufs.as_ptr(), bufs.len(), offset)`
#[inline]
#[cfg(not(target_os = "redox"))]
pub fn preadv<Fd: AsFd>(fd: &Fd, bufs: &[IoSliceMut], offset: u64) -> io::Result<usize> {
    let fd = fd.as_fd();
    _preadv(fd, bufs, offset)
}

#[cfg(all(libc, not(target_os = "redox")))]
fn _preadv(fd: BorrowedFd<'_>, bufs: &[IoSliceMut], offset: u64) -> io::Result<usize> {
    // Silently cast; we'll get `EINVAL` if the value is negative.
    let offset = offset as i64;
    let nread = unsafe {
        ret_ssize_t(libc_preadv(
            borrowed_fd(fd),
            bufs.as_ptr().cast::<libc::iovec>(),
            min(bufs.len(), max_iov()) as c_int,
            offset,
        ))?
    };
    Ok(nread as usize)
}

#[cfg(linux_raw)]
#[inline]
fn _preadv(fd: BorrowedFd<'_>, bufs: &[IoSliceMut], offset: u64) -> io::Result<usize> {
    crate::linux_raw::preadv(fd, &bufs[..min(bufs.len(), max_iov())], offset)
}

/// `pwritev(fd, bufs.as_ptr(), bufs.len(), offset)`
#[cfg(not(target_os = "redox"))]
#[inline]
pub fn pwritev<Fd: AsFd>(fd: &Fd, bufs: &[IoSlice], offset: u64) -> io::Result<usize> {
    let fd = fd.as_fd();
    _pwritev(fd, bufs, offset)
}

#[cfg(all(libc, not(target_os = "redox")))]
fn _pwritev(fd: BorrowedFd<'_>, bufs: &[IoSlice], offset: u64) -> io::Result<usize> {
    // Silently cast; we'll get `EINVAL` if the value is negative.
    let offset = offset as i64;
    let nwritten = unsafe {
        ret_ssize_t(libc_pwritev(
            borrowed_fd(fd),
            bufs.as_ptr().cast::<libc::iovec>(),
            min(bufs.len(), max_iov()) as c_int,
            offset,
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(linux_raw)]
#[inline]
fn _pwritev(fd: BorrowedFd<'_>, bufs: &[IoSlice], offset: u64) -> io::Result<usize> {
    crate::linux_raw::pwritev(fd, &bufs[..min(bufs.len(), max_iov())], offset)
}

/// `preadv2(fd, bufs.as_ptr(), bufs.len(), offset, flags)`
#[cfg(any(
    linux_raw,
    all(
        libc,
        target_pointer_width = "64",
        target_os = "linux",
        target_env = "gnu"
    )
))]
#[inline]
pub fn preadv2<Fd: AsFd>(
    fd: &Fd,
    bufs: &[IoSliceMut],
    offset: u64,
    flags: ReadWriteFlags,
) -> io::Result<usize> {
    let fd = fd.as_fd();
    _preadv2(fd, bufs, offset, flags)
}

#[cfg(all(
    libc,
    target_pointer_width = "64",
    target_os = "linux",
    target_env = "gnu"
))]
fn _preadv2(
    fd: BorrowedFd<'_>,
    bufs: &[IoSliceMut],
    offset: u64,
    flags: ReadWriteFlags,
) -> io::Result<usize> {
    // Silently cast; we'll get `EINVAL` if the value is negative.
    let offset = offset as i64;
    let nread = unsafe {
        ret_ssize_t(libc_preadv2(
            borrowed_fd(fd),
            bufs.as_ptr().cast::<libc::iovec>(),
            min(bufs.len(), max_iov()) as c_int,
            offset,
            flags.bits(),
        ))?
    };
    Ok(nread as usize)
}

#[cfg(linux_raw)]
#[inline]
fn _preadv2(
    fd: BorrowedFd<'_>,
    bufs: &[IoSliceMut],
    offset: u64,
    flags: ReadWriteFlags,
) -> io::Result<usize> {
    crate::linux_raw::preadv2(
        fd,
        &bufs[..min(bufs.len(), max_iov())],
        offset,
        flags.bits(),
    )
}

/// `pwritev2(fd, bufs.as_ptr(), bufs.len(), offset, flags)`
#[cfg(any(
    linux_raw,
    all(
        libc,
        target_pointer_width = "64",
        target_os = "linux",
        target_env = "gnu"
    )
))]
#[inline]
pub fn pwritev2<Fd: AsFd>(
    fd: &Fd,
    bufs: &[IoSlice],
    offset: u64,
    flags: ReadWriteFlags,
) -> io::Result<usize> {
    let fd = fd.as_fd();
    _pwritev2(fd, bufs, offset, flags)
}

#[cfg(all(
    libc,
    target_pointer_width = "64",
    target_os = "linux",
    target_env = "gnu"
))]
fn _pwritev2(
    fd: BorrowedFd<'_>,
    bufs: &[IoSlice],
    offset: u64,
    flags: ReadWriteFlags,
) -> io::Result<usize> {
    // Silently cast; we'll get `EINVAL` if the value is negative.
    let offset = offset as i64;
    let nwritten = unsafe {
        ret_ssize_t(libc_pwritev2(
            borrowed_fd(fd),
            bufs.as_ptr().cast::<libc::iovec>(),
            min(bufs.len(), max_iov()) as c_int,
            offset,
            flags.bits(),
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(linux_raw)]
#[inline]
fn _pwritev2(
    fd: BorrowedFd<'_>,
    bufs: &[IoSlice],
    offset: u64,
    flags: ReadWriteFlags,
) -> io::Result<usize> {
    crate::linux_raw::pwritev2(
        fd,
        &bufs[..min(bufs.len(), max_iov())],
        offset,
        flags.bits(),
    )
}

// These functions are derived from Rust's library/std/src/sys/unix/fd.rs at
// revision 108e90ca78f052c0c1c49c42a22c85620be19712.

#[cfg(all(libc, not(any(target_os = "redox", target_env = "newlib"))))]
static LIM: AtomicUsize = AtomicUsize::new(0);

#[cfg(all(libc, not(any(target_os = "redox", target_env = "newlib"))))]
#[inline]
fn max_iov() -> usize {
    let mut lim = LIM.load(Ordering::Relaxed);
    if lim == 0 {
        lim = query_max_iov()
    }

    lim
}

#[cfg(all(libc, not(any(target_os = "redox", target_env = "newlib"))))]
fn query_max_iov() -> usize {
    let ret = unsafe { libc::sysconf(libc::_SC_IOV_MAX) };

    // 16 is the minimum value required by POSIX.
    let lim = if ret > 0 { ret as usize } else { 16 };
    LIM.store(lim, Ordering::Relaxed);
    lim
}

#[cfg(all(libc, any(target_os = "redox", target_env = "newlib")))]
#[inline]
fn max_iov() -> usize {
    16 // The minimum value required by POSIX.
}

#[cfg(linux_raw)]
#[inline]
fn max_iov() -> usize {
    linux_raw_sys::general::UIO_MAXIOV as usize
}
