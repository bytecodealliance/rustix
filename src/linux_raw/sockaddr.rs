use crate::{io, net::AddressFamily, path};
use std::ffi::CString;

/// `struct in_addr`
#[repr(transparent)]
#[derive(Clone)]
#[doc(alias("in_addr"))]
pub struct Ipv4Addr(pub(crate) linux_raw_sys::general::in_addr);

impl Ipv4Addr {
    /// Construct a new IPv4 address from 4 octets.
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self(linux_raw_sys::general::in_addr {
            s_addr: u32::from_ne_bytes([a, b, c, d]),
        })
    }
}

/// `struct in6_addr`
#[repr(transparent)]
#[derive(Clone)]
#[doc(alias("in6_addr"))]
pub struct Ipv6Addr(pub(crate) linux_raw_sys::general::in6_addr);

impl Ipv6Addr {
    /// Construct a new IPv6 address from eight 16-bit segments.
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
#[derive(Clone)]
#[doc(alias("sockaddr_in"))]
pub struct SocketAddrV4 {
    pub(crate) addr: Ipv4Addr,
    pub(crate) port: u16,
}

impl SocketAddrV4 {
    /// Construct a new IPv4 socket address from an address and a port.
    #[inline]
    pub const fn new(addr: Ipv4Addr, port: u16) -> Self {
        Self { addr, port }
    }

    /// Encode this socket address in the host format.
    #[inline]
    pub(crate) const fn encode(&self) -> linux_raw_sys::general::sockaddr_in {
        linux_raw_sys::general::sockaddr_in {
            sin_family: linux_raw_sys::general::AF_INET as _,
            sin_addr: self.addr.0,
            sin_port: self.port.to_be(),
            __pad: [0; 8_usize],
        }
    }

    /// Return the IPv4 address of this socket address.
    #[inline]
    pub const fn address(&self) -> &Ipv4Addr {
        &self.addr
    }

    /// Return the port of this address.
    #[inline]
    pub const fn port(&self) -> u16 {
        self.port
    }
}

/// `struct sockaddr_in6`
#[derive(Clone)]
#[doc(alias("sockaddr_in6"))]
pub struct SocketAddrV6 {
    pub(crate) addr: Ipv6Addr,
    pub(crate) port: u16,
    pub(crate) flowinfo: u32,
    pub(crate) scope_id: u32,
}

impl SocketAddrV6 {
    /// Construct a new IPv6 socket address from an address, port, flow info,
    /// and scope id.
    #[inline]
    pub const fn new(addr: Ipv6Addr, port: u16, flowinfo: u32, scope_id: u32) -> Self {
        Self {
            addr,
            port,
            flowinfo,
            scope_id,
        }
    }

    /// Encode this socket address in the host format.
    #[inline]
    pub(crate) const fn encode(&self) -> linux_raw_sys::general::sockaddr_in6 {
        linux_raw_sys::general::sockaddr_in6 {
            sin6_family: linux_raw_sys::general::AF_INET6 as _,
            sin6_addr: self.addr.0,
            sin6_port: self.port.to_be(),
            sin6_flowinfo: self.flowinfo,
            sin6_scope_id: self.scope_id,
        }
    }

    /// Return the IPv6 address of this socket address.
    #[inline]
    pub const fn address(&self) -> &Ipv6Addr {
        &self.addr
    }

    /// Return the port of this address.
    #[inline]
    pub const fn port(&self) -> u16 {
        self.port
    }

    /// Return the flowinfo of this address.
    #[inline]
    pub const fn flowinfo(&self) -> u32 {
        self.flowinfo
    }

    /// Return the scope_id of this address.
    #[inline]
    pub const fn scope_id(&self) -> u32 {
        self.scope_id
    }
}

/// `struct sockaddr_un`
#[derive(Clone)]
#[doc(alias("sockaddr_un"))]
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

    /// Encode this socket address in the host format.
    #[inline]
    pub(crate) fn encode(&self) -> linux_raw_sys::general::sockaddr_un {
        let mut encoded = linux_raw_sys::general::sockaddr_un {
            sun_family: linux_raw_sys::general::AF_UNIX as _,
            sun_path: [0; 108_usize],
        };
        let bytes = self.path.as_bytes();
        for (i, b) in bytes.iter().enumerate() {
            encoded.sun_path[i] = *b as std::os::raw::c_char;
        }
        encoded.sun_path[bytes.len()] = b'\0' as std::os::raw::c_char;
        encoded
    }
}

/// `struct sockaddr_storage`
#[derive(Clone)]
#[doc(alias("sockaddr"))]
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
}
