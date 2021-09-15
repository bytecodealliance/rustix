use bitflags::bitflags;
use std::os::raw::c_uint;

/// `SOCK_*` constants for [`socket`].
///
/// [`socket`]: crate::net::socket
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct SocketType(pub(crate) u32);

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
}

/// `AF_*` constants.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct AddressFamily(pub(crate) linux_raw_sys::general::__kernel_sa_family_t);

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
}

/// `IPPROTO_*`
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u32)]
#[non_exhaustive]
pub enum Protocol {
    /// `IPPROTO_IP`
    Ip = linux_raw_sys::general::IPPROTO_IP as u32,
    /// `IPPROTO_ICMP`
    Icmp = linux_raw_sys::general::IPPROTO_ICMP as u32,
    /// `IPPROTO_IGMP`
    Igmp = linux_raw_sys::general::IPPROTO_IGMP as u32,
    /// `IPPROTO_IPIP`
    Ipip = linux_raw_sys::general::IPPROTO_IPIP as u32,
    /// `IPPROTO_TCP`
    Tcp = linux_raw_sys::general::IPPROTO_TCP as u32,
    /// `IPPROTO_EGP`
    Egp = linux_raw_sys::general::IPPROTO_EGP as u32,
    /// `IPPROTO_PUP`
    Pup = linux_raw_sys::general::IPPROTO_PUP as u32,
    /// `IPPROTO_UDP`
    Udp = linux_raw_sys::general::IPPROTO_UDP as u32,
    /// `IPPROTO_IDP`
    Idp = linux_raw_sys::general::IPPROTO_IDP as u32,
    /// `IPPROTO_TP`
    Tp = linux_raw_sys::v5_4::general::IPPROTO_TP as u32,
    /// `IPPROTO_DCCP`
    Dccp = linux_raw_sys::general::IPPROTO_DCCP as u32,
    /// `IPPROTO_IPV6`
    Ipv6 = linux_raw_sys::general::IPPROTO_IPV6 as u32,
    /// `IPPROTO_RSVP`
    Rsvp = linux_raw_sys::general::IPPROTO_RSVP as u32,
    /// `IPPROTO_GRE`
    Gre = linux_raw_sys::general::IPPROTO_GRE as u32,
    /// `IPPROTO_ESP`
    Esp = linux_raw_sys::general::IPPROTO_ESP as u32,
    /// `IPPROTO_AH`
    Ah = linux_raw_sys::general::IPPROTO_AH as u32,
    /// `IPPROTO_MTP`
    Mtp = linux_raw_sys::v5_4::general::IPPROTO_MTP as u32,
    /// `IPPROTO_BEETPH`
    Beetph = linux_raw_sys::general::IPPROTO_BEETPH as u32,
    /// `IPPROTO_ENCAP`
    Encap = linux_raw_sys::v5_4::general::IPPROTO_ENCAP as u32,
    /// `IPPROTO_PIM`
    Pim = linux_raw_sys::general::IPPROTO_PIM as u32,
    /// `IPPROTO_COMP`
    Comp = linux_raw_sys::general::IPPROTO_COMP as u32,
    /// `IPPROTO_SCTP`
    Sctp = linux_raw_sys::general::IPPROTO_SCTP as u32,
    /// `IPPROTO_UDPLITE`
    Udplite = linux_raw_sys::general::IPPROTO_UDPLITE as u32,
    /// `IPPROTO_MPLS`
    Mpls = linux_raw_sys::v5_4::general::IPPROTO_MPLS as u32,
    /// `IPPROTO_ETHERNET`
    Ethernet = linux_raw_sys::v5_11::general::IPPROTO_ETHERNET as u32,
    /// `IPPROTO_RAW`
    Raw = linux_raw_sys::general::IPPROTO_RAW as u32,
    /// `IPPROTO_MPTCP`
    Mptcp = linux_raw_sys::v5_11::general::IPPROTO_MPTCP as u32,
    /// `IPPROTO_FRAGMENT`
    Fragment = linux_raw_sys::general::IPPROTO_FRAGMENT as u32,
    /// `IPPROTO_ICMPV6`
    Icmpv6 = linux_raw_sys::general::IPPROTO_ICMPV6 as u32,
    /// `IPPROTO_MH`
    Mh = linux_raw_sys::general::IPPROTO_MH as u32,
    /// `IPPROTO_ROUTING`
    Routing = linux_raw_sys::general::IPPROTO_ROUTING as u32,
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
