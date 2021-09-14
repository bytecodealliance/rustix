//! IPv4, IPv6, and Socket addresses.
//!
//! # Safety
//!
//! Linux's IPv6 type contains a union.
#![allow(unsafe_code)]

use super::{read_sockaddr, write_sockaddr, AddressFamily};
use crate::{io, path};
use std::ffi::{CStr, CString};
use std::fmt;
use std::net::{SocketAddrV4, SocketAddrV6};

/// `struct sockaddr_un`
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[doc(alias = "sockaddr_un")]
pub struct SocketAddrUnix {
    path: CString,
}

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
    pub fn path(&self) -> &CStr {
        &self.path
    }
}

impl fmt::Debug for SocketAddrUnix {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.path.fmt(fmt)
    }
}

/// `struct sockaddr_storage` as a Rust enum.
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[doc(alias = "sockaddr")]
#[non_exhaustive]
pub enum SocketAddr {
    /// `struct sockaddr_in`
    V4(SocketAddrV4),
    /// `struct sockaddr_in6`
    V6(SocketAddrV6),
    /// `struct sockaddr_un`
    Unix(SocketAddrUnix),
}

impl SocketAddr {
    /// Return the address family of this socket address.
    #[inline]
    pub const fn address_family(&self) -> AddressFamily {
        match self {
            SocketAddr::V4(_) => AddressFamily::INET,
            SocketAddr::V6(_) => AddressFamily::INET6,
            SocketAddr::Unix(_) => AddressFamily::UNIX,
        }
    }

    /// Writes a platform-specific encoding of this socket address to
    /// the memory pointed to by `storage`, and returns the number of
    /// bytes used.
    pub unsafe fn write(&self, storage: *mut SocketAddrStorage) -> usize {
        write_sockaddr(self, storage)
    }

    /// Reads a platform-specific encoding of a socket address from
    /// the memory pointed to by `storage`, which uses `len` bytes.
    pub unsafe fn read(storage: *const SocketAddrStorage, len: usize) -> io::Result<Self> {
        read_sockaddr(storage, len)
    }
}

impl fmt::Debug for SocketAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SocketAddr::V4(v4) => v4.fmt(fmt),
            SocketAddr::V6(v6) => v6.fmt(fmt),
            SocketAddr::Unix(unix) => unix.fmt(fmt),
        }
    }
}

/// `struct sockaddr_storage` as a raw struct.
pub type SocketAddrStorage = linux_raw_sys::general::sockaddr;
