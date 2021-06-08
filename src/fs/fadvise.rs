use io_lifetimes::{AsFd, BorrowedFd};
#[cfg(all(
    libc,
    not(any(
        target_os = "android",
        target_os = "linux",
        target_os = "emscripten",
        target_os = "l4re"
    ))
))]
use libc::posix_fadvise as libc_posix_fadvise;
#[cfg(all(
    libc,
    any(
        target_os = "android",
        target_os = "linux",
        target_os = "emscripten",
        target_os = "l4re"
    )
))]
use libc::posix_fadvise64 as libc_posix_fadvise;
use std::{convert::TryInto, io};
#[cfg(libc)]
use {crate::zero_ok, unsafe_io::os::posish::AsRawFd};

/// `POSIX_FADV_*` constants.
#[cfg(libc)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum Advice {
    /// `POSIX_FADV_NORMAL`
    Normal = libc::POSIX_FADV_NORMAL as libc::c_uint,

    /// `POSIX_FADV_SEQUENTIAL`
    Sequential = libc::POSIX_FADV_SEQUENTIAL as libc::c_uint,

    /// `POSIX_FADV_RANDOM`
    Random = libc::POSIX_FADV_RANDOM as libc::c_uint,

    /// `POSIX_FADV_NOREUSE`
    NoReuse = libc::POSIX_FADV_NOREUSE as libc::c_uint,

    /// `POSIX_FADV_WILLNEED`
    WillNeed = libc::POSIX_FADV_WILLNEED as libc::c_uint,

    /// `POSIX_FADV_DONTNEED`
    DontNeed = libc::POSIX_FADV_DONTNEED as libc::c_uint,
}

/// `POSIX_FADV_*` constants.
#[cfg(linux_raw)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum Advice {
    /// `POSIX_FADV_NORMAL`
    Normal = linux_raw_sys::general::POSIX_FADV_NORMAL,

    /// `POSIX_FADV_SEQUENTIAL`
    Sequential = linux_raw_sys::general::POSIX_FADV_SEQUENTIAL,

    /// `POSIX_FADV_RANDOM`
    Random = linux_raw_sys::general::POSIX_FADV_RANDOM,

    /// `POSIX_FADV_NOREUSE`
    NoReuse = linux_raw_sys::general::POSIX_FADV_NOREUSE,

    /// `POSIX_FADV_WILLNEED`
    WillNeed = linux_raw_sys::general::POSIX_FADV_WILLNEED,

    /// `POSIX_FADV_DONTNEED`
    DontNeed = linux_raw_sys::general::POSIX_FADV_DONTNEED,
}

/// `posix_fadvise(fd, offset, len, advice)`
#[inline]
pub fn fadvise<Fd: AsFd>(fd: &Fd, offset: u64, len: u64, advice: Advice) -> io::Result<()> {
    let fd = fd.as_fd();
    _fadvise(fd, offset, len, advice)
}

#[cfg(libc)]
fn _fadvise(fd: BorrowedFd<'_>, offset: u64, len: u64, advice: Advice) -> io::Result<()> {
    if let (Ok(offset), Ok(len)) = (offset.try_into(), len.try_into()) {
        unsafe {
            zero_ok(libc_posix_fadvise(
                fd.as_raw_fd() as libc::c_int,
                offset,
                len,
                advice as libc::c_int,
            ))?;
        }
    }

    // If the offset or length can't be converted, ignore the advice, as it
    // isn't likely to be useful in that case.
    Ok(())
}

#[cfg(linux_raw)]
#[inline]
fn _fadvise(fd: BorrowedFd<'_>, offset: u64, len: u64, advice: Advice) -> io::Result<()> {
    if let (Ok(offset), Ok(len)) = (offset.try_into(), len.try_into()) {
        crate::linux_raw::fadvise(fd, offset, len, advice as i32)?;
    }

    // If the offset or length can't be converted, ignore the advice, as it
    // isn't likely to be useful in that case.
    Ok(())
}
