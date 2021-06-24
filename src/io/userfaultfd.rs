//! The Linux `userfaultfd` API.
//!
//! # Safety
//!
//! Calling `userfaultfd` is safe, but the returned file descriptor lets users
//! observe and manipulate process memory in magical ways.
#![allow(unsafe_code)]

use crate::io;
#[cfg(libc)]
use crate::libc::conv::syscall_ret_owned_fd;
use bitflags::bitflags;
use io_lifetimes::OwnedFd;

#[cfg(libc)]
bitflags! {
    /// The `O_*` flags accepted by `userfaultfd`.
    pub struct UserFaultFdFlags: std::os::raw::c_int {
        /// `O_CLOEXEC`
        const CLOEXEC = libc::O_CLOEXEC;
        /// `O_NONBLOCK`
        const NONBLOCK = libc::O_NONBLOCK;
    }
}

#[cfg(linux_raw)]
bitflags! {
    /// The `O_*` flags accepted by `userfaultfd`.
    pub struct UserFaultFdFlags: std::os::raw::c_uint {
        /// `O_CLOEXEC`
        const CLOEXEC = linux_raw_sys::general::O_CLOEXEC;
        /// `O_NONBLOCK`
        const NONBLOCK = linux_raw_sys::general::O_NONBLOCK;
    }
}

/// `userfaultfd(flags)`
///
/// # Safety
///
/// The call itself is safe, but the returned file descriptor lets users
/// observe and manipuate process memory in magical ways.
#[inline]
pub unsafe fn userfaultfd(flags: UserFaultFdFlags) -> io::Result<OwnedFd> {
    _userfaultfd(flags)
}

#[cfg(libc)]
unsafe fn _userfaultfd(flags: UserFaultFdFlags) -> io::Result<OwnedFd> {
    syscall_ret_owned_fd(libc::syscall(libc::SYS_userfaultfd, flags.bits()))
}

#[cfg(linux_raw)]
#[inline]
unsafe fn _userfaultfd(flags: UserFaultFdFlags) -> io::Result<OwnedFd> {
    crate::linux_raw::userfaultfd(flags.bits())
}
