#[cfg(windows)]
use super::libc;
use bitflags::bitflags;

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
    pub const STREAM: Self = Self(libc::SOCK_STREAM as u32);

    /// `SOCK_DGRAM`
    pub const DGRAM: Self = Self(libc::SOCK_DGRAM as u32);

    /// `SOCK_SEQPACKET`
    pub const SEQPACKET: Self = Self(libc::SOCK_SEQPACKET as u32);

    /// `SOCK_RAW`
    pub const RAW: Self = Self(libc::SOCK_RAW as u32);

    /// `SOCK_RDM`
    pub const RDM: Self = Self(libc::SOCK_RDM as u32);

    /// Constructs a `SocketType` from a raw integer.
    #[inline]
    pub const fn from_raw(raw: RawSocketType) -> Self {
        Self(raw)
    }

    /// Returns the raw integer for this `SocketType`.
    #[inline]
    pub const fn as_raw(self) -> RawSocketType {
        self.0
    }
}

/// A type for holding raw integer address families.
#[doc(hidden)]
pub type RawAddressFamily = libc::sa_family_t;

/// `AF_*` constants.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct AddressFamily(pub(crate) RawAddressFamily);

#[rustfmt::skip]
impl AddressFamily {
    /// `AF_UNSPEC`
    pub const UNSPEC: Self = Self(libc::AF_UNSPEC as _);
    /// `AF_INET`
    pub const INET: Self = Self(libc::AF_INET as _);
    /// `AF_INET6`
    pub const INET6: Self = Self(libc::AF_INET6 as _);
    /// `AF_NETLINK`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const NETLINK: Self = Self(libc::AF_NETLINK as _);
    /// `AF_UNIX`, aka `AF_LOCAL`
    #[doc(alias = "LOCAL")]
    pub const UNIX: Self = Self(libc::AF_UNIX as _);
    /// `AF_AX25`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const AX25: Self = Self(libc::AF_AX25 as _);
    /// `AF_IPX`
    pub const IPX: Self = Self(libc::AF_IPX as _);
    /// `AF_APPLETALK`
    pub const APPLETALK: Self = Self(libc::AF_APPLETALK as _);
    /// `AF_NETROM`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const NETROM: Self = Self(libc::AF_NETROM as _);
    /// `AF_BRIDGE`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const BRIDGE: Self = Self(libc::AF_BRIDGE as _);
    /// `AF_ATMPVC`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const ATMPVC: Self = Self(libc::AF_ATMPVC as _);
    /// `AF_X25`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const X25: Self = Self(libc::AF_X25 as _);
    /// `AF_ROSE`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const ROSE: Self = Self(libc::AF_ROSE as _);
    /// `AF_DECnet`
    #[allow(non_upper_case_globals)]
    pub const DECnet: Self = Self(libc::AF_DECnet as _);
    /// `AF_NETBEUI`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const NETBEUI: Self = Self(libc::AF_NETBEUI as _);
    /// `AF_SECURITY`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const SECURITY: Self = Self(libc::AF_SECURITY as _);
    /// `AF_KEY`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const KEY: Self = Self(libc::AF_KEY as _);
    /// `AF_PACKET`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const PACKET: Self = Self(libc::AF_PACKET as _);
    /// `AF_ASH`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const ASH: Self = Self(libc::AF_ASH as _);
    /// `AF_ECONET`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const ECONET: Self = Self(libc::AF_ECONET as _);
    /// `AF_ATMSVC`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const ATMSVC: Self = Self(libc::AF_ATMSVC as _);
    /// `AF_RDS`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const RDS: Self = Self(libc::AF_RDS as _);
    /// `AF_SNA`
    pub const SNA: Self = Self(libc::AF_SNA as _);
    /// `AF_IRDA`
    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const IRDA: Self = Self(libc::AF_IRDA as _);
    /// `AF_PPPOX`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const PPPOX: Self = Self(libc::AF_PPPOX as _);
    /// `AF_WANPIPE`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const WANPIPE: Self = Self(libc::AF_WANPIPE as _);
    /// `AF_LLC`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const LLC: Self = Self(libc::AF_LLC as _);
    /// `AF_CAN`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const CAN: Self = Self(libc::AF_CAN as _);
    /// `AF_TIPC`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const TIPC: Self = Self(libc::AF_TIPC as _);
    /// `AF_BLUETOOTH`
    #[cfg(not(any(windows, target_os = "ios", target_os = "macos",)))]
    pub const BLUETOOTH: Self = Self(libc::AF_BLUETOOTH as _);
    /// `AF_IUCV`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const IUCV: Self = Self(libc::AF_IUCV as _);
    /// `AF_RXRPC`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const RXRPC: Self = Self(libc::AF_RXRPC as _);
    /// `AF_ISDN`
    #[cfg(not(windows))]
    pub const ISDN: Self = Self(libc::AF_ISDN as _);
    /// `AF_PHONET`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const PHONET: Self = Self(libc::AF_PHONET as _);
    /// `AF_IEEE802154`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const IEEE802154: Self = Self(libc::AF_IEEE802154 as _);

    /// Constructs a `AddressFamily` from a raw integer.
    #[inline]
    pub const fn from_raw(raw: RawAddressFamily) -> Self {
        Self(raw)
    }

    /// Returns the raw integer for this `AddressFamily`.
    #[inline]
    pub const fn as_raw(self) -> RawAddressFamily {
        self.0
    }
}

/// A type for holding raw integer protocols.
#[doc(hidden)]
pub type RawProtocol = i32;

/// `IPPROTO_*`
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Protocol(pub(crate) RawProtocol);

#[rustfmt::skip]
impl Protocol {
    /// `IPPROTO_IP`
    pub const IP: Self = Self(libc::IPPROTO_IP as _);
    /// `IPPROTO_ICMP`
    pub const ICMP: Self = Self(libc::IPPROTO_ICMP as _);
    /// `IPPROTO_IGMP`
    pub const IGMP: Self = Self(libc::IPPROTO_IGMP as _);
    /// `IPPROTO_IPIP`
    #[cfg(not(windows))]
    pub const IPIP: Self = Self(libc::IPPROTO_IPIP as _);
    /// `IPPROTO_TCP`
    pub const TCP: Self = Self(libc::IPPROTO_TCP as _);
    /// `IPPROTO_EGP`
    pub const EGP: Self = Self(libc::IPPROTO_EGP as _);
    /// `IPPROTO_PUP`
    pub const PUP: Self = Self(libc::IPPROTO_PUP as _);
    /// `IPPROTO_UDP`
    pub const UDP: Self = Self(libc::IPPROTO_UDP as _);
    /// `IPPROTO_IDP`
    pub const IDP: Self = Self(libc::IPPROTO_IDP as _);
    /// `IPPROTO_TP`
    #[cfg(not(windows))]
    pub const TP: Self = Self(libc::IPPROTO_TP as _);
    /// `IPPROTO_DCCP`
    #[cfg(not(any(windows, target_os = "ios", target_os = "macos", target_os = "openbsd")))]
    pub const DCCP: Self = Self(libc::IPPROTO_DCCP as _);
    /// `IPPROTO_IPV6`
    pub const IPV6: Self = Self(libc::IPPROTO_IPV6 as _);
    /// `IPPROTO_RSVP`
    #[cfg(not(windows))]
    pub const RSVP: Self = Self(libc::IPPROTO_RSVP as _);
    /// `IPPROTO_GRE`
    #[cfg(not(windows))]
    pub const GRE: Self = Self(libc::IPPROTO_GRE as _);
    /// `IPPROTO_ESP`
    pub const ESP: Self = Self(libc::IPPROTO_ESP as _);
    /// `IPPROTO_AH`
    pub const AH: Self = Self(libc::IPPROTO_AH as _);
    /// `IPPROTO_MTP`
    #[cfg(not(any(windows, target_os = "netbsd", target_os = "openbsd")))]
    pub const MTP: Self = Self(libc::IPPROTO_MTP as _);
    /// `IPPROTO_BEETPH`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const BEETPH: Self = Self(libc::IPPROTO_BEETPH as _);
    /// `IPPROTO_ENCAP`
    #[cfg(not(windows))]
    pub const ENCAP: Self = Self(libc::IPPROTO_ENCAP as _);
    /// `IPPROTO_PIM`
    pub const PIM: Self = Self(libc::IPPROTO_PIM as _);
    /// `IPPROTO_COMP`
    #[cfg(not(any(
        windows,
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const COMP: Self = Self(libc::IPPROTO_COMP as _);
    /// `IPPROTO_SCTP`
    #[cfg(not(target_os = "openbsd"))]
    pub const SCTP: Self = Self(libc::IPPROTO_SCTP as _);
    /// `IPPROTO_UDPLITE`
    #[cfg(not(any(
        windows,
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    )))]
    pub const UDPLITE: Self = Self(libc::IPPROTO_UDPLITE as _);
    /// `IPPROTO_MPLS`
    #[cfg(not(any(windows, target_os = "ios", target_os = "macos", target_os = "netbsd")))]
    pub const MPLS: Self = Self(libc::IPPROTO_MPLS as _);
    /// `IPPROTO_RAW`
    pub const RAW: Self = Self(libc::IPPROTO_RAW as _);
    /// `IPPROTO_MPTCP`
    #[cfg(not(any(
        windows,
        target_os = "android",
        target_os = "emscripten",
        target_os = "freebsd",
        target_os = "fuchsia",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const MPTCP: Self = Self(libc::IPPROTO_MPTCP as _);
    /// `IPPROTO_FRAGMENT`
    pub const FRAGMENT: Self = Self(libc::IPPROTO_FRAGMENT as _);
    /// `IPPROTO_ICMPV6`
    pub const ICMPV6: Self = Self(libc::IPPROTO_ICMPV6 as _);
    /// `IPPROTO_MH`
    #[cfg(not(any(
        windows,
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    pub const MH: Self = Self(libc::IPPROTO_MH as _);
    /// `IPPROTO_ROUTING`
    pub const ROUTING: Self = Self(libc::IPPROTO_ROUTING as _);

    /// Constructs a `Protocol` from a raw integer.
    #[inline]
    pub const fn from_raw(raw: RawProtocol) -> Self {
        Self(raw)
    }

    /// Returns the raw integer for this `Protocol`.
    #[inline]
    pub const fn as_raw(self) -> RawProtocol {
        self.0
    }
}

/// `SHUT_*` constants for [`shutdown`].
///
/// [`shutdown`]: crate::net::shutdown
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Shutdown {
    /// `SHUT_RD`
    Read = libc::SHUT_RD,
    /// `SHUT_WR`
    Write = libc::SHUT_WR,
    /// `SHUT_RDWR`
    ReadWrite = libc::SHUT_RDWR,
}

bitflags! {
    /// `SOCK_*` constants for [`accept_with`] and [`acceptfrom_with`].
    ///
    /// [`accept_with`]: crate::net::accept_with
    /// [`acceptfrom_with`]: crate::net::acceptfrom_with
    pub struct AcceptFlags: libc::c_int {
        /// `SOCK_NONBLOCK`
        #[cfg(not(any(windows, target_os = "ios", target_os = "macos")))]
        const NONBLOCK = libc::SOCK_NONBLOCK;

        /// `SOCK_CLOEXEC`
        #[cfg(not(any(windows, target_os = "ios", target_os = "macos")))]
        const CLOEXEC = libc::SOCK_CLOEXEC;
    }
}

bitflags! {
    /// `SOCK_*` constants for [`socket`].
    ///
    /// [`socket`]: crate::net::socket
    pub struct SocketFlags: libc::c_int {
        /// `SOCK_NONBLOCK`
        #[cfg(not(any(windows, target_os = "ios", target_os = "macos")))]
        const NONBLOCK = libc::SOCK_NONBLOCK;

        /// `SOCK_CLOEXEC`
        #[cfg(not(any(windows, target_os = "ios", target_os = "macos")))]
        const CLOEXEC = libc::SOCK_CLOEXEC;
    }
}

/// Timeout identifier for use with [`set_socket_timeout`] and
/// [`get_socket_timeout`].
///
/// [`set_socket_timeout`]: crate::net::sockopt::set_socket_timeout.
/// [`get_socket_timeout`]: crate::net::sockopt::get_socket_timeout.
#[repr(i32)]
pub enum Timeout {
    /// `SO_RCVTIMEO`—Timeout for receiving.
    Recv = libc::SO_RCVTIMEO,

    /// `SO_SNDTIMEO`—Timeout for sending.
    Send = libc::SO_SNDTIMEO,
}
