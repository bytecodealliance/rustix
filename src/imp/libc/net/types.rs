use bitflags::bitflags;
use libc::c_int;

/// `SOCK_*` constants for [`socket`].
///
/// [`socket`]: crate::net::socket
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct SocketType(pub(crate) u32);

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
}

/// `AF_*` constants.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct AddressFamily(pub(crate) libc::sa_family_t);

impl AddressFamily {
    /// `AF_INET`
    pub const INET: Self = Self(libc::AF_INET as _);
    /// `AF_INET6`
    pub const INET6: Self = Self(libc::AF_INET6 as _);
    /// `AF_NETLINK`
    #[cfg(not(any(
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
}

/// `IPPROTO_*`
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(i32)]
#[non_exhaustive]
pub enum Protocol {
    /// `IPPROTO_IP`
    Ip = libc::IPPROTO_IP,
    /// `IPPROTO_ICMP`
    Icmp = libc::IPPROTO_ICMP,
    /// `IPPROTO_IGMP`
    Igmp = libc::IPPROTO_IGMP,
    /// `IPPROTO_IPIP`
    Ipip = libc::IPPROTO_IPIP,
    /// `IPPROTO_TCP`
    Tcp = libc::IPPROTO_TCP,
    /// `IPPROTO_EGP`
    Egp = libc::IPPROTO_EGP,
    /// `IPPROTO_PUP`
    Pup = libc::IPPROTO_PUP,
    /// `IPPROTO_UDP`
    Udp = libc::IPPROTO_UDP,
    /// `IPPROTO_IDP`
    Idp = libc::IPPROTO_IDP,
    /// `IPPROTO_TP`
    Tp = libc::IPPROTO_TP,
    /// `IPPROTO_DCCP`
    #[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "openbsd")))]
    Dccp = libc::IPPROTO_DCCP,
    /// `IPPROTO_IPV6`
    Ipv6 = libc::IPPROTO_IPV6,
    /// `IPPROTO_RSVP`
    Rsvp = libc::IPPROTO_RSVP,
    /// `IPPROTO_GRE`
    Gre = libc::IPPROTO_GRE,
    /// `IPPROTO_ESP`
    Esp = libc::IPPROTO_ESP,
    /// `IPPROTO_AH`
    Ah = libc::IPPROTO_AH,
    /// `IPPROTO_MTP`
    #[cfg(not(any(target_os = "netbsd", target_os = "openbsd")))]
    Mtp = libc::IPPROTO_MTP,
    /// `IPPROTO_BEETPH`
    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    Beetph = libc::IPPROTO_BEETPH,
    /// `IPPROTO_ENCAP`
    Encap = libc::IPPROTO_ENCAP,
    /// `IPPROTO_PIM`
    Pim = libc::IPPROTO_PIM,
    /// `IPPROTO_COMP`
    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    Comp = libc::IPPROTO_COMP,
    /// `IPPROTO_SCTP`
    #[cfg(not(target_os = "openbsd"))]
    Sctp = libc::IPPROTO_SCTP,
    /// `IPPROTO_UDPLITE`
    #[cfg(not(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    )))]
    Udplite = libc::IPPROTO_UDPLITE,
    /// `IPPROTO_MPLS`
    #[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "netbsd")))]
    Mpls = libc::IPPROTO_MPLS,
    /// `IPPROTO_RAW`
    Raw = libc::IPPROTO_RAW,
    /// `IPPROTO_MPTCP`
    #[cfg(not(any(
        target_os = "android",
        target_os = "emscripten",
        target_os = "freebsd",
        target_os = "fuchsia",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    Mptcp = libc::IPPROTO_MPTCP,
}

/// `SHUT_*` constants for [`shutdown`].
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
    pub struct AcceptFlags: c_int {
        /// `SOCK_NONBLOCK`
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        const NONBLOCK = libc::SOCK_NONBLOCK;

        /// `SOCK_CLOEXEC`
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        const CLOEXEC = libc::SOCK_CLOEXEC;
    }
}

bitflags! {
    /// `SOCK_*` constants for [`socket`].
    ///
    /// [`socket`]: crate::net::socket
    pub struct SocketFlags: c_int {
        /// `SOCK_NONBLOCK`
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        const NONBLOCK = libc::SOCK_NONBLOCK;

        /// `SOCK_CLOEXEC`
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        const CLOEXEC = libc::SOCK_CLOEXEC;
    }
}
