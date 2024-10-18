//! A socket address for any kind of socket.
//!
//! This is similar to [`std::net::SocketAddr`], but also supports Unix-domain
//! socket addresses on Unix.
//!
//! # Safety
//!
//! The `read` and `write` functions allow decoding and encoding from and to
//! OS-specific socket address representations in memory.
#![allow(unsafe_code)]

#[cfg(target_os = "linux")]
use crate::net::xdp::SocketAddrXdp;
#[cfg(unix)]
use crate::net::SocketAddrUnix;
use crate::net::{AddressFamily, SocketAddr, SocketAddrV4, SocketAddrV6};
use crate::utils::{as_mut_ptr, as_ptr};
use crate::{backend, io};
#[cfg(feature = "std")]
use core::fmt;
use core::mem::zeroed;

pub use backend::net::addr::SocketAddrStorage;

/// `struct sockaddr_storage` as a Rust enum.
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[doc(alias = "sockaddr")]
#[non_exhaustive]
pub enum SocketAddrAny {
    /// `struct sockaddr_in`
    V4(SocketAddrV4),
    /// `struct sockaddr_in6`
    V6(SocketAddrV6),
    /// `struct sockaddr_un`
    #[cfg(unix)]
    Unix(SocketAddrUnix),
    /// `struct sockaddr_xdp`
    #[cfg(target_os = "linux")]
    Xdp(SocketAddrXdp),
}

impl From<SocketAddr> for SocketAddrAny {
    #[inline]
    fn from(from: SocketAddr) -> Self {
        match from {
            SocketAddr::V4(v4) => Self::V4(v4),
            SocketAddr::V6(v6) => Self::V6(v6),
        }
    }
}

impl From<SocketAddrV4> for SocketAddrAny {
    #[inline]
    fn from(from: SocketAddrV4) -> Self {
        Self::V4(from)
    }
}

impl From<SocketAddrV6> for SocketAddrAny {
    #[inline]
    fn from(from: SocketAddrV6) -> Self {
        Self::V6(from)
    }
}

#[cfg(unix)]
impl From<SocketAddrUnix> for SocketAddrAny {
    #[inline]
    fn from(from: SocketAddrUnix) -> Self {
        Self::Unix(from)
    }
}

impl SocketAddrAny {
    /// Return the address family of this socket address.
    #[inline]
    pub const fn address_family(&self) -> AddressFamily {
        match self {
            Self::V4(_) => AddressFamily::INET,
            Self::V6(_) => AddressFamily::INET6,
            #[cfg(unix)]
            Self::Unix(_) => AddressFamily::UNIX,
            #[cfg(target_os = "linux")]
            Self::Xdp(_) => AddressFamily::XDP,
        }
    }

    /// Creates a platform-specific encoding of this socket address,
    /// and returns it.
    pub fn to_raw(&self) -> RawSocketAddr {
        let mut raw = RawSocketAddr {
            storage: unsafe { zeroed() },
            len: 0,
        };

        raw.len = unsafe { self.write(raw.as_mut_ptr()) };
        raw
    }

    /// Reads a platform-specific encoding of a socket address.
    pub fn from_raw(raw: RawSocketAddr) -> io::Result<Self> {
        unsafe { Self::read(raw.as_ptr(), raw.len) }
    }

    /// Writes a platform-specific encoding of this socket address to
    /// the memory pointed to by `storage`, and returns the number of
    /// bytes used.
    ///
    /// # Safety
    ///
    /// `storage` must point to valid memory for encoding the socket
    /// address.
    pub unsafe fn write(&self, storage: *mut SocketAddrStorage) -> usize {
        backend::net::write_sockaddr::write_sockaddr(self, storage)
    }

    /// Reads a platform-specific encoding of a socket address from
    /// the memory pointed to by `storage`, which uses `len` bytes.
    ///
    /// # Safety
    ///
    /// `storage` must point to valid memory for decoding a socket
    /// address.
    pub unsafe fn read(storage: *const SocketAddrStorage, len: usize) -> io::Result<Self> {
        backend::net::read_sockaddr::read_sockaddr(storage, len)
    }
}

/// A raw sockaddr and its length.
#[repr(C)]
pub struct RawSocketAddr {
    pub(crate) storage: SocketAddrStorage,
    pub(crate) len: usize,
}

impl RawSocketAddr {
    /// Creates a raw encoded sockaddr from the given address.
    pub fn new(addr: impl Into<SocketAddrAny>) -> Self {
        addr.into().to_raw()
    }

    /// Returns a raw pointer to the sockaddr.
    pub fn as_ptr(&self) -> *const SocketAddrStorage {
        as_ptr(&self.storage)
    }

    /// Returns a raw mutable pointer to the sockaddr.
    pub fn as_mut_ptr(&mut self) -> *mut SocketAddrStorage {
        as_mut_ptr(&mut self.storage)
    }

    /// Returns the length of the encoded sockaddr.
    pub fn namelen(&self) -> usize {
        self.len
    }
}

#[cfg(feature = "std")]
impl fmt::Debug for SocketAddrAny {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::V4(v4) => v4.fmt(f),
            Self::V6(v6) => v6.fmt(f),
            #[cfg(unix)]
            Self::Unix(unix) => unix.fmt(f),
            #[cfg(target_os = "linux")]
            Self::Xdp(xdp) => xdp.fmt(f),
        }
    }
}
