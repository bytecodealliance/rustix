use crate::path;
use bitflags::bitflags;
use io_lifetimes::OwnedFd;
use std::{ffi::CStr, io};
#[cfg(libc)]
use {crate::negone_err, std::convert::TryInto, unsafe_io::os::posish::FromRawFd};

#[cfg(libc)]
bitflags! {
    /// `MFD_*` constants.
    pub struct MemfdFlags: std::os::raw::c_uint {
        /// `MFD_CLOEXEC`
        const CLOEXEC = libc::MFD_CLOEXEC;

        /// `MFD_ALLOW_SEALING`
        const ALLOW_SEALING = libc::MFD_ALLOW_SEALING;
    }
}

#[cfg(linux_raw)]
bitflags! {
    /// `MFD_*` constants.
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
    let path = path.as_c_str()?;
    _memfd_create(&path, flags)
}

#[cfg(libc)]
fn _memfd_create(path: &CStr, flags: MemfdFlags) -> io::Result<OwnedFd> {
    unsafe {
        let fd = negone_err(libc::syscall(
            libc::SYS_memfd_create,
            path.as_ptr(),
            flags.bits(),
        ))?;

        #[allow(clippy::useless_conversion)]
        Ok(OwnedFd::from_raw_fd(fd.try_into().unwrap()))
    }
}

#[cfg(linux_raw)]
#[inline]
fn _memfd_create(path: &CStr, flags: MemfdFlags) -> io::Result<OwnedFd> {
    crate::linux_raw::memfd_create(path, flags.bits())
}
