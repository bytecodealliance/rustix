//! Uname support.
//!
//! # Safety
//!
//! This function converts from `struct utsname` fields provided from the
//! kernel into `&str` references, which assumes that they're NUL-terminated.
#![allow(unsafe_code)]

use crate::imp;
use std::ffi::{CStr, OsStr};
use std::fmt;
use std::os::raw::c_char;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
#[cfg(target_os = "wasi")]
use std::os::wasi::ffi::OsStrExt;

/// `uname()`—Returns high-level information about the runtime OS and
/// hardware.
#[inline]
pub fn uname() -> Uname {
    Uname(imp::syscalls::uname())
}

/// `struct utsname`—Return type for [`uname`].
#[doc(alias = "utsname")]
pub struct Uname(imp::process::RawUname);

impl Uname {
    /// `sysname`—Operating system release name
    #[inline]
    pub fn sysname(&self) -> &OsStr {
        Self::to_os_str(self.0.sysname.as_ptr())
    }

    /// `nodename`—Name with vague meaning
    ///
    /// This is intended to be a network name, however it's unable to convey
    /// information about hosts that have multiple names, or any information
    /// about where the names are visible.
    #[inline]
    pub fn nodename(&self) -> &OsStr {
        Self::to_os_str(self.0.nodename.as_ptr())
    }

    /// `release`—Operating system release version string
    #[inline]
    pub fn release(&self) -> &OsStr {
        Self::to_os_str(self.0.release.as_ptr())
    }

    /// `version`—Operating system build identifiers
    #[inline]
    pub fn version(&self) -> &OsStr {
        Self::to_os_str(self.0.version.as_ptr())
    }

    /// `machine`—Hardware architecture identifier
    #[inline]
    pub fn machine(&self) -> &OsStr {
        Self::to_os_str(self.0.machine.as_ptr())
    }

    /// `domainname`—NIS or YP domain identifer
    #[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
    #[inline]
    pub fn domainname(&self) -> &OsStr {
        Self::to_os_str(self.0.domainname.as_ptr())
    }

    #[inline]
    fn to_os_str<'a>(ptr: *const c_char) -> &'a OsStr {
        // Safety: Strings returned from the kernel are always NUL-terminated.
        OsStr::from_bytes(unsafe { CStr::from_ptr(ptr) }.to_bytes())
    }
}

impl fmt::Debug for Uname {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(not(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux")))))]
        {
            write!(
                fmt,
                "{} {} {} {} {}",
                self.sysname().to_string_lossy(),
                self.nodename().to_string_lossy(),
                self.release().to_string_lossy(),
                self.version().to_string_lossy(),
                self.machine().to_string_lossy(),
            )
        }
        #[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
        {
            write!(
                fmt,
                "{} {} {} {} {} {}",
                self.sysname().to_string_lossy(),
                self.nodename().to_string_lossy(),
                self.release().to_string_lossy(),
                self.version().to_string_lossy(),
                self.machine().to_string_lossy(),
                self.domainname().to_string_lossy(),
            )
        }
    }
}
