//! IPv4, IPv6, and Socket addresses.

use super::super::c;
#[cfg(not(windows))]
use crate::io;
#[cfg(not(windows))]
use crate::path;
#[cfg(not(windows))]
use std::ffi::{CStr, CString};
#[cfg(not(windows))]
use std::fmt;

/// `struct sockaddr_un`
#[cfg(not(windows))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[doc(alias = "sockaddr_un")]
pub struct SocketAddrUnix {
    path: CString,
}

#[cfg(not(windows))]
impl SocketAddrUnix {
    /// Construct a new Unix-domain address from a byte slice.
    /// filesystem path.
    #[inline]
    pub fn new<P: path::Arg>(path: P) -> io::Result<Self> {
        let path = path.into_c_str()?.into_owned();
        Self::_new(path)
    }

    #[inline]
    fn _new(path: CString) -> io::Result<Self> {
        let bytes = path.as_bytes();

        let z = c::sockaddr_un {
            #[cfg(any(
                target_os = "netbsd",
                target_os = "macos",
                target_os = "ios",
                target_os = "freebsd",
                target_os = "openbsd"
            ))]
            sun_len: 0,
            sun_family: 0,
            #[cfg(any(
                target_os = "netbsd",
                target_os = "macos",
                target_os = "ios",
                target_os = "freebsd",
                target_os = "openbsd"
            ))]
            sun_path: [0; 104],
            #[cfg(not(any(
                target_os = "netbsd",
                target_os = "macos",
                target_os = "ios",
                target_os = "freebsd",
                target_os = "openbsd"
            )))]
            sun_path: [0; 108],
        };
        if bytes.len() + 1 > z.sun_path.len() {
            return Err(io::Error::NAMETOOLONG);
        }
        Ok(Self { path })
    }

    /// Returns a reference to the contained path.
    #[inline]
    pub fn path(&self) -> &CStr {
        &self.path
    }
}

#[cfg(not(windows))]
impl fmt::Debug for SocketAddrUnix {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.path.fmt(fmt)
    }
}

/// `struct sockaddr_storage` as a raw struct.
pub type SocketAddrStorage = c::sockaddr_storage;
