#![allow(unsafe_code)]

#[cfg(unix)]
use crate::net::SocketAddrUnix;
use crate::{
    backend::{c, net::read_sockaddr},
    io::Errno,
    net::{
        addr::{SocketAddrArg, SocketAddrLen, SocketAddrOpaque, SocketAddrStorage},
        AddressFamily, SocketAddr, SocketAddrV4, SocketAddrV6,
    },
};
#[cfg(feature = "std")]
use core::fmt;
use core::{
    mem::{size_of, MaybeUninit},
    num::NonZeroU32,
};

/// Temporary buffer for creating a `SocketAddrAny` from a
/// syscall that writes to a `sockaddr_t` and `socklen_t`
pub(crate) struct SocketAddrBuf {
    pub(crate) len: c::socklen_t,
    pub(crate) storage: MaybeUninit<SocketAddrStorage>,
}

impl SocketAddrBuf {
    #[inline]
    pub(crate) fn new() -> Self {
        SocketAddrBuf {
            len: size_of::<SocketAddrStorage>().try_into().unwrap(),
            storage: MaybeUninit::<SocketAddrStorage>::uninit(),
        }
    }

    /// Convert the buffer into [`SocketAddrAny`].
    ///
    /// # Safety
    /// A valid address must have been written into `self.storage`
    /// and its length written into `self.len`.
    #[inline]
    pub(crate) unsafe fn into_any(self) -> SocketAddrAny {
        SocketAddrAny::new(self.storage, self.len.try_into().unwrap())
    }

    /// Convert the buffer into [`Option<SocketAddrAny>].
    ///
    /// This returns `None` if `len` is zero or other platform-specific
    /// conditions define the address as empty.
    ///
    /// # Safety
    /// Either valid address must have been written into `self.storage`
    /// and its length written into `self.len`, or `self.len` must
    /// have been set to 0.
    #[inline]
    pub(crate) unsafe fn into_any_option(self) -> Option<SocketAddrAny> {
        let len = self.len.try_into().unwrap();
        if read_sockaddr::sockaddr_nonempty(self.storage.as_ptr().cast(), len) {
            Some(SocketAddrAny::new(self.storage, len))
        } else {
            None
        }
    }
}

/// A type that can hold any kind of socket address, as a safe abstraction for
/// `sockaddr_storage`.
///
/// Socket addresses can be converted to `SocketAddrAny` via the [`From`] and
/// [`Into`] traits. `SocketAddrAny` can be converted back to a specific socket
/// address type with [`TryFrom`] and [`TryInto`]. These implementations return
/// [`Errno::AFNOSUPPORT`] if the address family does not match the requested
/// type.
#[derive(Clone)]
#[doc(alias = "sockaddr_storage")]
pub struct SocketAddrAny {
    // Invariants:
    // * `len` is at least `size_of::<backend::c::sa_family_t>()`
    // * `len` is at most `size_of::<SocketAddrStorage>()`
    // * The first `len` bytes of `storage` are initialized.
    pub(crate) len: NonZeroU32,
    pub(crate) storage: MaybeUninit<SocketAddrStorage>,
}

impl SocketAddrAny {
    /// Creates a socket address from `storage`, which is initialized for
    /// `len` bytes.
    ///
    /// # Panics
    ///
    /// if `len` is smaller than the sockaddr header or larger than
    /// `SocketAddrStorage`.
    ///
    /// # Safety
    ///
    /// * `storage` must contain a valid socket address.
    /// * `len` bytes must be initialized.
    #[inline]
    pub unsafe fn new(storage: MaybeUninit<SocketAddrStorage>, len: SocketAddrLen) -> Self {
        assert!(len as usize >= core::mem::size_of::<read_sockaddr::sockaddr_header>());
        assert!(len as usize <= core::mem::size_of::<SocketAddrStorage>());
        let len = NonZeroU32::new_unchecked(len);
        Self { storage, len }
    }

    /// Gets the initialized part of the storage as bytes.
    #[inline]
    fn bytes(&self) -> &[u8] {
        let len = self.len.get() as usize;
        unsafe { core::slice::from_raw_parts(self.storage.as_ptr().cast(), len) }
    }

    /// Gets the address family of this socket address.
    #[inline]
    pub fn address_family(&self) -> AddressFamily {
        unsafe {
            AddressFamily::from_raw(crate::backend::net::read_sockaddr::read_sa_family(
                self.storage.as_ptr().cast(),
            ))
        }
    }

    /// Returns a raw pointer to the sockaddr.
    #[inline]
    pub fn as_ptr(&self) -> *const SocketAddrStorage {
        self.storage.as_ptr()
    }

    /// Returns a raw mutable pointer to the sockaddr.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut SocketAddrStorage {
        self.storage.as_mut_ptr()
    }

    /// Returns the length of the encoded sockaddr.
    #[inline]
    pub fn len(&self) -> SocketAddrLen {
        self.len.get()
    }
}

impl PartialEq<SocketAddrAny> for SocketAddrAny {
    fn eq(&self, other: &SocketAddrAny) -> bool {
        self.bytes() == other.bytes()
    }
}

impl Eq for SocketAddrAny {}

impl PartialOrd<SocketAddrAny> for SocketAddrAny {
    fn partial_cmp(&self, other: &SocketAddrAny) -> Option<core::cmp::Ordering> {
        self.bytes().partial_cmp(other.bytes())
    }
}

impl Ord for SocketAddrAny {
    fn cmp(&self, other: &SocketAddrAny) -> core::cmp::Ordering {
        self.bytes().cmp(other.bytes())
    }
}

impl core::hash::Hash for SocketAddrAny {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.bytes().hash(state)
    }
}

#[cfg(feature = "std")]
impl fmt::Debug for SocketAddrAny {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.address_family() {
            AddressFamily::INET => {
                if let Ok(addr) = SocketAddrV4::try_from(self.clone()) {
                    return addr.fmt(f);
                }
            }
            AddressFamily::INET6 => {
                if let Ok(addr) = SocketAddrV6::try_from(self.clone()) {
                    return addr.fmt(f);
                }
            }
            #[cfg(unix)]
            AddressFamily::UNIX => {
                if let Ok(addr) = SocketAddrUnix::try_from(self.clone()) {
                    return addr.fmt(f);
                }
            }
            #[cfg(target_os = "linux")]
            AddressFamily::XDP => {
                if let Ok(addr) = crate::net::xdp::SocketAddrXdp::try_from(self.clone()) {
                    return addr.fmt(f);
                }
            }
            #[cfg(linux_kernel)]
            AddressFamily::NETLINK => {
                if let Ok(addr) = crate::net::netlink::SocketAddrNetlink::try_from(self.clone()) {
                    return addr.fmt(f);
                }
            }
            _ => {}
        }

        f.debug_struct("SocketAddrAny")
            .field("address_family", &self.address_family())
            .field("namelen", &self.len())
            .finish()
    }
}

unsafe impl SocketAddrArg for SocketAddrAny {
    fn with_sockaddr<R>(&self, f: impl FnOnce(*const SocketAddrOpaque, SocketAddrLen) -> R) -> R {
        f(self.as_ptr().cast(), self.len())
    }
}

impl From<SocketAddr> for SocketAddrAny {
    #[inline]
    fn from(from: SocketAddr) -> Self {
        from.as_any()
    }
}

impl TryFrom<SocketAddrAny> for SocketAddr {
    type Error = Errno;

    /// Convert if the address is an IPv4 or IPv6 address.
    ///
    /// Returns `Err(Errno::AFNOSUPPORT)` if the address family is not IPv4 or IPv6.
    #[inline]
    fn try_from(value: SocketAddrAny) -> Result<Self, Self::Error> {
        match value.address_family() {
            AddressFamily::INET => read_sockaddr::read_sockaddr_v4(&value).map(SocketAddr::V4),
            AddressFamily::INET6 => read_sockaddr::read_sockaddr_v6(&value).map(SocketAddr::V6),
            _ => Err(Errno::AFNOSUPPORT),
        }
    }
}

impl From<SocketAddrV4> for SocketAddrAny {
    #[inline]
    fn from(from: SocketAddrV4) -> Self {
        from.as_any()
    }
}

impl TryFrom<SocketAddrAny> for SocketAddrV4 {
    type Error = Errno;

    /// Convert if the address is an IPv4 address.
    ///
    /// Returns `Err(Errno::AFNOSUPPORT)` if the address family is not IPv4.
    #[inline]
    fn try_from(value: SocketAddrAny) -> Result<Self, Self::Error> {
        read_sockaddr::read_sockaddr_v4(&value)
    }
}

impl From<SocketAddrV6> for SocketAddrAny {
    #[inline]
    fn from(from: SocketAddrV6) -> Self {
        from.as_any()
    }
}

impl TryFrom<SocketAddrAny> for SocketAddrV6 {
    type Error = Errno;

    /// Convert if the address is an IPv6 address.
    ///
    /// Returns `Err(Errno::AFNOSUPPORT)` if the address family is not IPv6.
    #[inline]
    fn try_from(value: SocketAddrAny) -> Result<Self, Self::Error> {
        read_sockaddr::read_sockaddr_v6(&value)
    }
}

#[cfg(unix)]
impl From<SocketAddrUnix> for SocketAddrAny {
    #[inline]
    fn from(from: SocketAddrUnix) -> Self {
        from.as_any()
    }
}

#[cfg(unix)]
impl TryFrom<SocketAddrAny> for SocketAddrUnix {
    type Error = Errno;

    /// Convert if the address is a Unix socket address.
    ///
    /// Returns `Err(Errno::AFNOSUPPORT)` if the address family is not Unix.
    #[inline]
    fn try_from(value: SocketAddrAny) -> Result<Self, Self::Error> {
        read_sockaddr::read_sockaddr_unix(&value)
    }
}
