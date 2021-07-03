//! The `mmap` API.
//!
//! # Safety
//!
//! `mmap` manipulates raw pointers and has special semantics and is
//! wildly unsafe.
#![allow(unsafe_code)]

use crate::{imp, io};
use io_lifetimes::AsFd;
use std::os::raw::c_void;

pub use imp::io::{MapFlags, ProtFlags};

/// `mmap(fd, len, prot, flags, fd, offset)`
///
/// # Safety
///
/// Raw pointers and lots of special semantics.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/mmap.html
/// [Linux]: https://man7.org/linux/man-pages/man2/mmap.2.html
#[inline]
pub unsafe fn mmap<Fd: AsFd>(
    ptr: *mut c_void,
    len: usize,
    prot: ProtFlags,
    flags: MapFlags,
    fd: &Fd,
    offset: u64,
) -> io::Result<*mut c_void> {
    let fd = fd.as_fd();
    imp::syscalls::mmap(ptr, len, prot, flags, fd, offset)
}

/// `munmap(ptr, len)`
///
/// # Safety
///
/// Raw pointers and lots of special semantics.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/munmap.html
/// [Linux]: https://man7.org/linux/man-pages/man2/munmap.2.html
#[inline]
pub unsafe fn munmap(ptr: *mut c_void, len: usize) -> io::Result<()> {
    imp::syscalls::munmap(ptr, len)
}
