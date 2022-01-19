//! Safe (where possible) wrappers around system calls.
//!
//! # Safety
//!
//! This file performs system calls by calling libc functions, and sometimes
//! passes them uninitialized memory buffers.
//!
//! Some of this could be auto-generated from the libc bindings, but we often
//! need more information than they provide, such as which pointers are array
//! slices, out parameters, or in-out parameters, which integers are owned or
//! borrowed file descriptors, etc.

// There are a lot of system calls, so they're split out into separate files.
#[cfg(not(windows))]
pub(crate) use super::fs::syscalls::*;
#[cfg(not(windows))]
pub(crate) use super::io::syscalls::*;
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) use super::net::syscalls::*;
#[cfg(not(windows))]
pub(crate) use super::process::syscalls::*;
#[cfg(target_os = "linux")]
pub(crate) use super::rand::syscalls::*;
#[cfg(not(windows))]
pub(crate) use super::thread::syscalls::*;
#[cfg(not(windows))]
pub(crate) use super::time::syscalls::*;

#[cfg(windows)]
use super::c;
#[cfg(windows)]
use super::conv::{borrowed_fd, ret};
#[cfg(windows)]
use super::fd::{BorrowedFd, LibcFd, RawFd};
#[cfg(windows)]
use crate::io;

#[cfg(windows)]
pub(crate) fn ioctl_fionbio(fd: BorrowedFd<'_>, value: bool) -> io::Result<()> {
    unsafe {
        let mut data = value as c::c_uint;
        ret(c::ioctl(borrowed_fd(fd), c::FIONBIO, &mut data))
    }
}

#[cfg(windows)]
pub(crate) unsafe fn close(raw_fd: RawFd) {
    let _ = c::close(raw_fd as LibcFd);
}
