#[cfg(libc)]
use crate::libc::conv::{c_str, syscall_ret_owned_fd};
use crate::{io, path};
use bitflags::bitflags;
use io_lifetimes::OwnedFd;
use std::ffi::CStr;

#[cfg(libc)]
bitflags! {
    /// `MFD_*` constants for use with [`memfd_create`].
    pub struct MemfdFlags: std::os::raw::c_uint {
        /// `MFD_CLOEXEC`
        const CLOEXEC = libc::MFD_CLOEXEC;

        /// `MFD_ALLOW_SEALING`
        const ALLOW_SEALING = libc::MFD_ALLOW_SEALING;
    }
}

#[cfg(linux_raw)]
bitflags! {
    /// `MFD_*` constants for use with [`memfd_create`].
    pub struct MemfdFlags: std::os::raw::c_uint {
        /// `MFD_CLOEXEC`
        const CLOEXEC = linux_raw_sys::v5_4::general::MFD_CLOEXEC;

        /// `MFD_ALLOW_SEALING`
        const ALLOW_SEALING = linux_raw_sys::v5_4::general::MFD_ALLOW_SEALING;
    }
}

/// `memfd_create(path, flags)`
#[inline]
pub fn memfd_create<P: path::Arg>(path: P, flags: MemfdFlags) -> io::Result<OwnedFd> {
    path.into_with_c_str(|path| _memfd_create(&path, flags))
}

#[cfg(libc)]
fn _memfd_create(path: &CStr, flags: MemfdFlags) -> io::Result<OwnedFd> {
    unsafe {
        syscall_ret_owned_fd(libc::syscall(
            libc::SYS_memfd_create,
            c_str(path),
            flags.bits(),
        ))
    }
}

#[cfg(linux_raw)]
#[inline]
fn _memfd_create(path: &CStr, flags: MemfdFlags) -> io::Result<OwnedFd> {
    crate::linux_raw::memfd_create(path, flags.bits())
}
