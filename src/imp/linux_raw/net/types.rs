use bitflags::bitflags;
use std::os::raw::c_uint;

/// A type for holding raw integer socket types.
#[doc(hidden)]
pub type RawSocketType = u32;

/// `SOCK_*` constants for [`socket`].
///
/// [`socket`]: crate::net::socket
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct SocketType(pub(crate) RawSocketType);

#[rustfmt::skip]
impl SocketType {
    /// `SOCK_STREAM`
    pub const STREAM: Self = Self(linux_raw_sys::general::SOCK_STREAM);

    /// `SOCK_DGRAM`
    pub const DGRAM: Self = Self(linux_raw_sys::general::SOCK_DGRAM);

    /// `SOCK_SEQPACKET`
    pub const SEQPACKET: Self = Self(linux_raw_sys::general::SOCK_SEQPACKET);

    /// `SOCK_RAW`
    pub const RAW: Self = Self(linux_raw_sys::general::SOCK_RAW);

    /// `SOCK_RDM`
    pub const RDM: Self = Self(linux_raw_sys::general::SOCK_RDM);

    /// Constructs a `SocketType` from a raw integer.
    #[inline]
    pub fn from_raw(raw: RawSocketType) -> Self {
        Self(raw)
    }

    /// Returns the raw integer for this `SocketType`.
    #[inline]
    pub fn as_raw(self) -> RawSocketType {
        self.0
    }
}

/// A type for holding raw integer address families.
#[doc(hidden)]
pub type RawAddressFamily = linux_raw_sys::general::__kernel_sa_family_t;

/// `AF_*` constants.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct AddressFamily(pub(crate) RawAddressFamily);

impl AddressFamily {
    /// `AF_INET`
    pub const INET: Self = Self(linux_raw_sys::general::AF_INET as _);
    /// `AF_INET6`
    pub const INET6: Self = Self(linux_raw_sys::general::AF_INET6 as _);
    /// `AF_NETLINK`
    pub const NETLINK: Self = Self(linux_raw_sys::general::AF_NETLINK as _);
    /// `AF_UNIX`, aka `AF_LOCAL`
    #[doc(alias = "LOCAL")]
    pub const UNIX: Self = Self(linux_raw_sys::general::AF_UNIX as _);

    /// Constructs a `AddressFamily` from a raw integer.
    #[inline]
    pub fn from_raw(raw: RawAddressFamily) -> Self {
        Self(raw)
    }

    /// Returns the raw integer for this `AddressFamily`.
    #[inline]
    pub fn as_raw(self) -> RawAddressFamily {
        self.0
    }
}

/// A type for holding raw integer protocols.
#[doc(hidden)]
pub type RawProtocol = u32;

/// `IPPROTO_*`
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Protocol(pub(crate) RawProtocol);

impl Protocol {
    /// `IPPROTO_IP`
    pub const IP: Self = Self(linux_raw_sys::general::IPPROTO_IP as _);
    /// `IPPROTO_ICMP`
    pub const ICMP: Self = Self(linux_raw_sys::general::IPPROTO_ICMP as _);
    /// `IPPROTO_IGMP`
    pub const IGMP: Self = Self(linux_raw_sys::general::IPPROTO_IGMP as _);
    /// `IPPROTO_IPIP`
    pub const IPIP: Self = Self(linux_raw_sys::general::IPPROTO_IPIP as _);
    /// `IPPROTO_TCP`
    pub const TCP: Self = Self(linux_raw_sys::general::IPPROTO_TCP as _);
    /// `IPPROTO_EGP`
    pub const EGP: Self = Self(linux_raw_sys::general::IPPROTO_EGP as _);
    /// `IPPROTO_PUP`
    pub const PUP: Self = Self(linux_raw_sys::general::IPPROTO_PUP as _);
    /// `IPPROTO_UDP`
    pub const UDP: Self = Self(linux_raw_sys::general::IPPROTO_UDP as _);
    /// `IPPROTO_IDP`
    pub const IDP: Self = Self(linux_raw_sys::general::IPPROTO_IDP as _);
    /// `IPPROTO_TP`
    pub const TP: Self = Self(linux_raw_sys::v5_4::general::IPPROTO_TP as _);
    /// `IPPROTO_DCCP`
    pub const DCCP: Self = Self(linux_raw_sys::general::IPPROTO_DCCP as _);
    /// `IPPROTO_IPV6`
    pub const IPV6: Self = Self(linux_raw_sys::general::IPPROTO_IPV6 as _);
    /// `IPPROTO_RSVP`
    pub const RSVP: Self = Self(linux_raw_sys::general::IPPROTO_RSVP as _);
    /// `IPPROTO_GRE`
    pub const GRE: Self = Self(linux_raw_sys::general::IPPROTO_GRE as _);
    /// `IPPROTO_ESP`
    pub const ESP: Self = Self(linux_raw_sys::general::IPPROTO_ESP as _);
    /// `IPPROTO_AH`
    pub const AH: Self = Self(linux_raw_sys::general::IPPROTO_AH as _);
    /// `IPPROTO_MTP`
    pub const MTP: Self = Self(linux_raw_sys::v5_4::general::IPPROTO_MTP as _);
    /// `IPPROTO_BEETPH`
    pub const BEETPH: Self = Self(linux_raw_sys::general::IPPROTO_BEETPH as _);
    /// `IPPROTO_ENCAP`
    pub const ENCAP: Self = Self(linux_raw_sys::v5_4::general::IPPROTO_ENCAP as _);
    /// `IPPROTO_PIM`
    pub const PIM: Self = Self(linux_raw_sys::general::IPPROTO_PIM as _);
    /// `IPPROTO_COMP`
    pub const COMP: Self = Self(linux_raw_sys::general::IPPROTO_COMP as _);
    /// `IPPROTO_SCTP`
    pub const SCTP: Self = Self(linux_raw_sys::general::IPPROTO_SCTP as _);
    /// `IPPROTO_UDPLITE`
    pub const UDPLITE: Self = Self(linux_raw_sys::general::IPPROTO_UDPLITE as _);
    /// `IPPROTO_MPLS`
    pub const MPLS: Self = Self(linux_raw_sys::v5_4::general::IPPROTO_MPLS as _);
    /// `IPPROTO_ETHERNET`
    pub const ETHERNET: Self = Self(linux_raw_sys::v5_11::general::IPPROTO_ETHERNET as _);
    /// `IPPROTO_RAW`
    pub const RAW: Self = Self(linux_raw_sys::general::IPPROTO_RAW as _);
    /// `IPPROTO_MPTCP`
    pub const MPTCP: Self = Self(linux_raw_sys::v5_11::general::IPPROTO_MPTCP as _);
    /// `IPPROTO_FRAGMENT`
    pub const FRAGMENT: Self = Self(linux_raw_sys::general::IPPROTO_FRAGMENT as _);
    /// `IPPROTO_ICMPV6`
    pub const ICMPV6: Self = Self(linux_raw_sys::general::IPPROTO_ICMPV6 as _);
    /// `IPPROTO_MH`
    pub const MH: Self = Self(linux_raw_sys::general::IPPROTO_MH as _);
    /// `IPPROTO_ROUTING`
    pub const ROUTING: Self = Self(linux_raw_sys::general::IPPROTO_ROUTING as _);

    /// Constructs a `Protocol` from a raw integer.
    #[inline]
    pub fn from_raw(raw: RawProtocol) -> Self {
        Self(raw)
    }

    /// Returns the raw integer for this `Protocol`.
    #[inline]
    pub fn as_raw(self) -> RawProtocol {
        self.0
    }
}

/// `SHUT_*` constants for [`shutdown`].
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum Shutdown {
    /// `SHUT_RD`
    Read = linux_raw_sys::general::SHUT_RD,
    /// `SHUT_WR`
    Write = linux_raw_sys::general::SHUT_WR,
    /// `SHUT_RDWR`
    ReadWrite = linux_raw_sys::general::SHUT_RDWR,
}

bitflags! {
    /// `SOCK_*` constants for [`accept_with`] and [`acceptfrom_with`].
    ///
    /// [`accept_with`]: crate::net::accept_with
    /// [`acceptfrom_with`]: crate::net::acceptfrom_with
    pub struct AcceptFlags: c_uint {
        /// `SOCK_NONBLOCK`
        const NONBLOCK = linux_raw_sys::general::O_NONBLOCK;
        /// `SOCK_CLOEXEC`
        const CLOEXEC = linux_raw_sys::general::O_CLOEXEC;
    }
}

bitflags! {
    /// `SOCK_*` constants for [`socket`].
    ///
    /// [`socket`]: crate::net::socket
    pub struct SocketFlags: c_uint {
        /// `SOCK_NONBLOCK`
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        const NONBLOCK = linux_raw_sys::general::O_NONBLOCK;

        /// `SOCK_CLOEXEC`
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        const CLOEXEC = linux_raw_sys::general::O_CLOEXEC;
    }
}
