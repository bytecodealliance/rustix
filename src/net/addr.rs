// # Safety
//
// This file performs casts between various pointers to various `sockaddr_*`
// types in a manner similar to what C code does when working with sockets
// APIs. It uses `assert`s to check assumptions where feasible.
#![allow(unsafe_code)]

use crate::{as_ptr, io, net::AddressFamily};
#[cfg(all(
    libc,
    any(
        target_os = "netbsd",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd"
    )
))]
use std::mem::size_of;
use std::mem::{transmute, MaybeUninit};

/// `struct in_addr`
#[cfg(libc)]
#[repr(transparent)]
#[derive(Clone)]
#[doc(alias("in_addr"))]
pub struct Ipv4Addr(pub(crate) libc::in_addr);

/// `struct in_addr`
#[cfg(linux_raw)]
#[repr(transparent)]
#[derive(Clone)]
#[doc(alias("in_addr"))]
pub struct Ipv4Addr(pub(crate) linux_raw_sys::general::in_addr);

impl Ipv4Addr {
    /// Construct a new IPv4 address from 4 octets.
    #[cfg(libc)]
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self(libc::in_addr {
            s_addr: u32::from_ne_bytes([a, b, c, d]),
        })
    }

    /// Construct a new IPv4 address from 4 octets.
    #[cfg(linux_raw)]
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self(linux_raw_sys::general::in_addr {
            s_addr: u32::from_ne_bytes([a, b, c, d]),
        })
    }
}

/// `struct in6_addr`
#[cfg(libc)]
#[repr(transparent)]
#[derive(Clone)]
#[doc(alias("in6_addr"))]
pub struct Ipv6Addr(pub(crate) libc::in6_addr);

/// `struct in6_addr`
#[cfg(linux_raw)]
#[repr(transparent)]
#[derive(Clone)]
#[doc(alias("in6_addr"))]
pub struct Ipv6Addr(pub(crate) linux_raw_sys::general::in6_addr);

impl Ipv6Addr {
    /// Construct a new IPv address from eight 16-bit segments.
    #[cfg(libc)]
    pub const fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Self {
        Self(libc::in6_addr {
            s6_addr: [
                (a >> 8) as u8,
                (a & 0xff) as u8,
                (b >> 8) as u8,
                (b & 0xff) as u8,
                (c >> 8) as u8,
                (c & 0xff) as u8,
                (d >> 8) as u8,
                (d & 0xff) as u8,
                (e >> 8) as u8,
                (e & 0xff) as u8,
                (f >> 8) as u8,
                (f & 0xff) as u8,
                (g >> 8) as u8,
                (g & 0xff) as u8,
                (h >> 8) as u8,
                (h & 0xff) as u8,
            ],
        })
    }

    /// Construct a new IPv address from eight 16-bit segments.
    #[cfg(linux_raw)]
    pub const fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Self {
        Self(linux_raw_sys::general::in6_addr {
            in6_u: linux_raw_sys::general::in6_addr__bindgen_ty_1 {
                u6_addr16: [
                    a.to_be(),
                    b.to_be(),
                    c.to_be(),
                    d.to_be(),
                    e.to_be(),
                    f.to_be(),
                    g.to_be(),
                    h.to_be(),
                ],
            },
        })
    }
}

/// `struct sockaddr_in`
#[cfg(libc)]
#[repr(transparent)]
#[derive(Clone)]
#[doc(alias("sockaddr_in"))]
pub struct SocketAddrV4(pub(crate) libc::sockaddr_in);

/// `struct sockaddr_in`
#[cfg(linux_raw)]
#[repr(transparent)]
#[derive(Clone)]
#[doc(alias("sockaddr_in"))]
pub struct SocketAddrV4(pub(crate) linux_raw_sys::general::sockaddr_in);

impl SocketAddrV4 {
    /// Construct a new IPv4 socket address from an address and a port.
    #[cfg(libc)]
    #[inline]
    pub const fn new(addr: Ipv4Addr, port: u16) -> Self {
        Self(libc::sockaddr_in {
            #[cfg(any(
                target_os = "netbsd",
                target_os = "macos",
                target_os = "ios",
                target_os = "freebsd",
                target_os = "openbsd"
            ))]
            sin_len: size_of::<Self>() as u8,
            sin_family: AddressFamily::INET.0,
            sin_port: port.to_be(),
            sin_addr: addr.0,
            sin_zero: [0; 8_usize],
        })
    }

    /// Construct a new IPv4 socket address from an address and a port.
    #[cfg(linux_raw)]
    #[inline]
    pub const fn new(addr: Ipv4Addr, port: u16) -> Self {
        Self(linux_raw_sys::general::sockaddr_in {
            sin_family: AddressFamily::INET.0,
            sin_port: port.to_be(),
            sin_addr: addr.0,
            __pad: [0; 8_usize],
        })
    }
}

/// `struct sockaddr_in6`
#[cfg(libc)]
#[repr(transparent)]
#[derive(Clone)]
#[doc(alias("sockaddr_in6"))]
pub struct SocketAddrV6(pub(crate) libc::sockaddr_in6);

/// `struct sockaddr_in6`
#[cfg(linux_raw)]
#[repr(transparent)]
#[derive(Clone)]
#[doc(alias("sockaddr_in6"))]
pub struct SocketAddrV6(pub(crate) linux_raw_sys::general::sockaddr_in6);

impl SocketAddrV6 {
    /// Construct a new IPv6 socket address from an address, port, flow info, and scope id.
    #[cfg(libc)]
    #[inline]
    pub const fn new(addr: Ipv6Addr, port: u16, flowinfo: u32, scope_id: u32) -> Self {
        Self(libc::sockaddr_in6 {
            #[cfg(any(
                target_os = "netbsd",
                target_os = "macos",
                target_os = "ios",
                target_os = "freebsd",
                target_os = "openbsd"
            ))]
            sin6_len: size_of::<Self>() as u8,
            sin6_family: AddressFamily::INET6.0,
            sin6_port: port.to_be(),
            sin6_addr: addr.0,
            sin6_flowinfo: flowinfo,
            sin6_scope_id: scope_id,
        })
    }

    /// Construct a new IPv6 socket address from an address, port, flow info, and scope id.
    #[cfg(linux_raw)]
    #[inline]
    pub const fn new(addr: Ipv6Addr, port: u16, flowinfo: u32, scope_id: u32) -> Self {
        Self(linux_raw_sys::general::sockaddr_in6 {
            sin6_family: AddressFamily::INET6.0,
            sin6_port: port.to_be(),
            sin6_addr: addr.0,
            sin6_flowinfo: flowinfo,
            sin6_scope_id: scope_id,
        })
    }
}

/// `struct sockaddr_un`
#[cfg(libc)]
#[repr(transparent)]
#[derive(Clone)]
#[doc(alias("sockaddr_un"))]
pub struct SocketAddrUnix(pub(crate) libc::sockaddr_un);

/// `struct sockaddr_un`
#[cfg(linux_raw)]
#[repr(transparent)]
#[derive(Clone)]
#[doc(alias("sockaddr_un"))]
pub struct SocketAddrUnix(pub(crate) linux_raw_sys::general::sockaddr_un);

impl SocketAddrUnix {
    /// Construct a new Unix-domain address from a byte slice containing a
    /// filesystem path.
    #[cfg(libc)]
    #[inline]
    pub fn new(addr: &[u8]) -> io::Result<Self> {
        unsafe {
            let mut me = Self(libc::sockaddr_un {
                #[cfg(any(
                    target_os = "netbsd",
                    target_os = "macos",
                    target_os = "ios",
                    target_os = "freebsd",
                    target_os = "openbsd"
                ))]
                sun_len: size_of::<Self>() as u8,
                sun_family: AddressFamily::UNIX.0,
                #[cfg(any(
                    target_os = "netbsd",
                    target_os = "macos",
                    target_os = "ios",
                    target_os = "freebsd",
                    target_os = "openbsd"
                ))]
                sun_path: [0; 104_usize],
                #[cfg(not(any(
                    target_os = "netbsd",
                    target_os = "macos",
                    target_os = "ios",
                    target_os = "freebsd",
                    target_os = "openbsd"
                )))]
                sun_path: [0; 108_usize],
            });
            if addr.len() > me.0.sun_path.len() {
                return Err(io::Error::NAMETOOLONG);
            }
            me.0.sun_path[..addr.len()].copy_from_slice(transmute(addr));
            Ok(me)
        }
    }

    /// Construct a new Unix-domain address from a byte slice containing a
    /// filesystem path.
    #[cfg(linux_raw)]
    #[inline]
    pub fn new(addr: &[u8]) -> io::Result<Self> {
        unsafe {
            let mut me = Self(linux_raw_sys::general::sockaddr_un {
                sun_family: AddressFamily::UNIX.0,
                sun_path: [0; 108_usize],
            });
            if addr.len() > me.0.sun_path.len() {
                return Err(io::Error::NAMETOOLONG);
            }
            me.0.sun_path[..addr.len()].copy_from_slice(transmute(addr));
            Ok(me)
        }
    }
}

/// `struct sockaddr_storage`
#[cfg(libc)]
#[repr(transparent)]
#[derive(Clone)]
#[doc(alias("sockaddr"))]
pub struct SocketAddr(pub(crate) libc::sockaddr_storage);

/// `struct sockaddr_storage`
#[cfg(linux_raw)]
#[repr(transparent)]
#[derive(Clone)]
#[doc(alias("sockaddr"))]
pub struct SocketAddr(pub(crate) linux_raw_sys::general::sockaddr);

impl SocketAddr {
    /// Return the address family of this socket address.
    #[inline]
    #[cfg(libc)]
    pub const fn address_family(&self) -> AddressFamily {
        AddressFamily(self.0.ss_family)
    }

    /// Return the address family of this socket address.
    #[inline]
    #[cfg(linux_raw)]
    pub const fn address_family(&self) -> AddressFamily {
        AddressFamily(self.0.__storage.ss_family)
    }

    /// Construct a new IPv4 socket address.
    pub fn v4(addr: SocketAddrV4) -> Self {
        #[cfg(any(
            target_os = "netbsd",
            target_os = "macos",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "openbsd"
        ))]
        assert_eq!(usize::from(addr.0.sin_len), size_of::<SocketAddrV4>());
        assert_eq!(addr.0.sin_family, AddressFamily::INET.0);
        let mut me = MaybeUninit::<SocketAddr>::uninit();
        unsafe {
            *me.as_mut_ptr().cast::<SocketAddrV4>() = addr;
            me.assume_init()
        }
    }

    /// Construct a new IPv6 socket address.
    pub fn v6(addr: SocketAddrV6) -> Self {
        #[cfg(any(
            target_os = "netbsd",
            target_os = "macos",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "openbsd"
        ))]
        assert_eq!(usize::from(addr.0.sin6_len), size_of::<SocketAddrV6>());
        assert_eq!(addr.0.sin6_family, AddressFamily::INET6.0);
        let mut me = MaybeUninit::<SocketAddr>::uninit();
        unsafe {
            *me.as_mut_ptr().cast::<SocketAddrV6>() = addr;
            me.assume_init()
        }
    }

    /// Construct a new Unix-domain socket address.
    pub fn unix(addr: SocketAddrUnix) -> Self {
        #[cfg(any(
            target_os = "netbsd",
            target_os = "macos",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "openbsd"
        ))]
        assert_eq!(usize::from(addr.0.sun_len), size_of::<SocketAddrUnix>());
        assert_eq!(addr.0.sun_family, AddressFamily::UNIX.0);
        let mut me = MaybeUninit::<SocketAddr>::uninit();
        unsafe {
            *me.as_mut_ptr().cast::<SocketAddrUnix>() = addr;
            me.assume_init()
        }
    }

    /// If `self` holds an IPv4 socket address, return a reference to the more
    /// specific type.
    pub fn as_v4(&self) -> Option<&SocketAddrV4> {
        if self.address_family() == AddressFamily::INET {
            Some(unsafe { &*as_ptr(self).cast::<SocketAddrV4>() })
        } else {
            None
        }
    }

    /// If `self` holds an IPv6 socket address, return a reference to the more
    /// specific type.
    pub fn as_v6(&self) -> Option<&SocketAddrV6> {
        if self.address_family() == AddressFamily::INET6 {
            Some(unsafe { &*as_ptr(self).cast::<SocketAddrV6>() })
        } else {
            None
        }
    }

    /// If `self` holds an Unix-domain socket address, return a reference to
    /// the more specific type.
    pub fn as_unix(&self) -> Option<&SocketAddrUnix> {
        if self.address_family() == AddressFamily::UNIX {
            Some(unsafe { &*as_ptr(self).cast::<SocketAddrUnix>() })
        } else {
            None
        }
    }
}
