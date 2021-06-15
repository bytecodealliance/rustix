//! Positioned *and* vectored I/O: `preadv` and `pwritev`.

use io_lifetimes::{AsFd, BorrowedFd};
#[cfg(all(
    libc,
    not(any(target_os = "android", target_os = "linux", target_os = "emscripten"))
))]
use libc::{preadv as libc_preadv, pwritev as libc_pwritev};
#[cfg(all(
    libc,
    any(target_os = "android", target_os = "linux", target_os = "emscripten")
))]
use libc::{preadv64 as libc_preadv, pwritev64 as libc_pwritev};
#[cfg(all(libc, not(any(target_os = "redox", target_env = "newlib"))))]
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{
    cmp::min,
    convert::TryInto,
    io::{self, IoSlice, IoSliceMut},
};
#[cfg(libc)]
use {crate::negone_err, unsafe_io::os::posish::AsRawFd};

/// `preadv(fd, bufs.as_ptr(), bufs.len(), offset)`
#[inline]
pub fn preadv<'f, Fd: AsFd<'f>>(fd: Fd, bufs: &[IoSliceMut], offset: u64) -> io::Result<usize> {
    let fd = fd.as_fd();
    _preadv(fd, bufs, offset)
}

#[cfg(libc)]
fn _preadv(fd: BorrowedFd<'_>, bufs: &[IoSliceMut], offset: u64) -> io::Result<usize> {
    let offset = offset
        .try_into()
        .map_err(|_overflow_err| io::Error::from_raw_os_error(libc::EOVERFLOW))?;
    let nread = unsafe {
        negone_err(libc_preadv(
            fd.as_raw_fd() as libc::c_int,
            bufs.as_ptr().cast::<libc::iovec>(),
            min(bufs.len(), max_iov()).try_into().unwrap(),
            offset,
        ))?
    };
    Ok(nread.try_into().unwrap())
}

#[cfg(linux_raw)]
#[inline]
fn _preadv(fd: BorrowedFd<'_>, bufs: &[IoSliceMut], offset: u64) -> io::Result<usize> {
    let offset = offset.try_into().map_err(|_overflow_err| {
        io::Error::from_raw_os_error(linux_raw_sys::errno::EOVERFLOW as i32)
    })?;
    crate::linux_raw::preadv(fd, &bufs[..min(bufs.len(), max_iov())], offset)
}

/// `pwritev(fd, bufs.as_ptr(), bufs.len(), offset)`
#[inline]
pub fn pwritev<'f, Fd: AsFd<'f>>(fd: Fd, bufs: &[IoSlice], offset: u64) -> io::Result<usize> {
    let fd = fd.as_fd();
    _pwritev(fd, bufs, offset)
}

#[cfg(libc)]
fn _pwritev(fd: BorrowedFd<'_>, bufs: &[IoSlice], offset: u64) -> io::Result<usize> {
    let offset = offset
        .try_into()
        .map_err(|_overflow_err| io::Error::from_raw_os_error(libc::EOVERFLOW))?;
    let nwritten = unsafe {
        negone_err(libc_pwritev(
            fd.as_raw_fd() as libc::c_int,
            bufs.as_ptr().cast::<libc::iovec>(),
            min(bufs.len(), max_iov()).try_into().unwrap(),
            offset,
        ))?
    };
    Ok(nwritten.try_into().unwrap())
}

#[cfg(linux_raw)]
#[inline]
fn _pwritev(fd: BorrowedFd<'_>, bufs: &[IoSlice], offset: u64) -> io::Result<usize> {
    let offset = offset.try_into().map_err(|_overflow_err| {
        io::Error::from_raw_os_error(linux_raw_sys::errno::EOVERFLOW as i32)
    })?;
    crate::linux_raw::pwritev(fd, &bufs[..min(bufs.len(), max_iov())], offset)
}

// These functions are derived from Rust's library/std/src/sys/unix/fd.rs at
// revision 108e90ca78f052c0c1c49c42a22c85620be19712.

#[cfg(all(libc, not(any(target_os = "redox", target_env = "newlib"))))]
fn max_iov() -> usize {
    static LIM: AtomicUsize = AtomicUsize::new(0);

    let mut lim = LIM.load(Ordering::Relaxed);
    if lim == 0 {
        let ret = unsafe { libc::sysconf(libc::_SC_IOV_MAX) };

        // 16 is the minimum value required by POSIX.
        lim = if ret > 0 { ret as usize } else { 16 };
        LIM.store(lim, Ordering::Relaxed);
    }

    lim
}

#[cfg(all(libc, any(target_os = "redox", target_env = "newlib")))]
fn max_iov() -> usize {
    16 // The minimum value required by POSIX.
}

#[cfg(linux_raw)]
#[inline]
fn max_iov() -> usize {
    linux_raw_sys::general::UIO_MAXIOV as usize
}
