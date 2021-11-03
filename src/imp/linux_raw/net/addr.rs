//! IPv4, IPv6, and Socket addresses.
//!
//! # Safety
//!
//! Linux's IPv6 type contains a union.
#![allow(unsafe_code)]

use crate::ffi::{ZStr, ZString};
use crate::{io, path};
use core::fmt;

/// `struct sockaddr_un`
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[doc(alias = "sockaddr_un")]
pub struct SocketAddrUnix {
    path: ZString,
}

impl SocketAddrUnix {
    /// Construct a new Unix-domain address from a byte slice.
    /// filesystem path.
    #[inline]
    pub fn new<P: path::Arg>(path: P) -> io::Result<Self> {
        let path = path.into_z_str()?.into_owned();
        Self::_new(path)
    }

    #[inline]
    fn _new(path: ZString) -> io::Result<Self> {
        let bytes = path.as_bytes();
        let z = linux_raw_sys::general::sockaddr_un {
            sun_family: 0,
            sun_path: [0; 108_usize],
        };
        if bytes.len() + 1 > z.sun_path.len() {
            return Err(io::Error::NAMETOOLONG);
        }
        Ok(Self { path })
    }

    /// Returns a reference to the contained path.
    #[inline]
    pub fn path(&self) -> &ZStr {
        &self.path
    }
}

impl fmt::Debug for SocketAddrUnix {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.path.fmt(fmt)
    }
}

/// `struct sockaddr_storage` as a raw struct.
pub type SocketAddrStorage = linux_raw_sys::general::sockaddr;
