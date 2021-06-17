use crate::io;
use bitflags::bitflags;
use io_lifetimes::OwnedFd;
#[cfg(libc)]
use {
    crate::negone_err,
    unsafe_io::os::posish::{FromRawFd, RawFd},
};

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
    let fd = negone_err(libc::syscall(libc::SYS_userfaultfd, flags.bits()))?;
    Ok(OwnedFd::from_raw_fd(fd as RawFd))
}

#[cfg(linux_raw)]
unsafe fn _userfaultfd(flags: UserFaultFdFlags) -> io::Result<OwnedFd> {
    crate::linux_raw::userfaultfd(flags.bits())
}
