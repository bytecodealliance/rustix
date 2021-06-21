use crate::{
    io,
    net::{SocketAddr, SocketAddrUnix, SocketAddrV4, SocketAddrV6},
};
use bitflags::bitflags;
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
use std::os::raw::c_int;
#[cfg(linux_raw)]
use std::os::raw::c_uint;
#[cfg(libc)]
use {
    super::sockaddr_header::decode_sockaddr,
    crate::{as_ptr, negone_err, zero_ok},
    libc::{sockaddr_storage, socklen_t},
    std::mem::{size_of, MaybeUninit},
    unsafe_io::os::posish::{AsRawFd, FromRawFd},
};

/// `SOCK_*` constants for `socket`.
#[cfg(libc)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct SocketType(pub(crate) u32);

#[cfg(cfg)]
impl SocketType {
    /// `SOCK_STREAM`.
    pub const STREAM: Self = Self(libc::SOCK_STREAM as u32);

    /// `SOCK_DGRAM`.
    pub const DGRAM: Self = Self(libc::SOCK_DGRAM as u32);

    /// `SOCK_SEQPACKET`.
    pub const SEQPACKET: Self = Self(libc::SOCK_SEQPACKET as u32);

    /// `SOCK_RAW`.
    pub const RAW: Self = Self(libc::SOCK_RAW as u32);

    /// `SOCK_RDM`.
    pub const RDM: Self = Self(libc::SOCK_RDM as u32);
}

/// `SOCK_*` constants for `socket`.
#[cfg(linux_raw)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct SocketType(pub(crate) u32);

#[cfg(linux_raw)]
impl SocketType {
    /// `SOCK_STREAM`.
    pub const STREAM: Self = Self(linux_raw_sys::general::SOCK_STREAM);

    /// `SOCK_DGRAM`.
    pub const DGRAM: Self = Self(linux_raw_sys::general::SOCK_DGRAM);

    /// `SOCK_SEQPACKET`.
    pub const SEQPACKET: Self = Self(linux_raw_sys::general::SOCK_SEQPACKET);

    /// `SOCK_RAW`.
    pub const RAW: Self = Self(linux_raw_sys::general::SOCK_RAW);

    /// `SOCK_RDM`.
    pub const RDM: Self = Self(linux_raw_sys::general::SOCK_RDM);
}

/// `AF_*` constants.
#[cfg(libc)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct AddressFamily(pub(crate) libc::sa_family_t);

/// `AF_*` constants.
#[cfg(linux_raw)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
// Older Linux versions didn't export this typedef.
pub struct AddressFamily(pub(crate) linux_raw_sys::v5_4::general::__kernel_sa_family_t);

#[cfg(libc)]
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
        target_os = "netbsd"
    )))]
    pub const NETLINK: Self = Self(libc::AF_NETLINK as _);
    /// `AF_UNIX`, aka `AF_LOCAL`
    #[doc(alias = "Local")]
    pub const UNIX: Self = Self(libc::AF_UNIX as _);
}

#[cfg(linux_raw)]
impl AddressFamily {
    /// `AF_INET`
    pub const INET: Self = Self(linux_raw_sys::general::AF_INET as _);
    /// `AF_INET6`
    pub const INET6: Self = Self(linux_raw_sys::general::AF_INET6 as _);
    /// `AF_NETLINK`
    pub const NETLINK: Self = Self(linux_raw_sys::general::AF_NETLINK as _);
    /// `AF_UNIX`, aka `AF_LOCAL`
    #[doc(alias = "Local")]
    pub const UNIX: Self = Self(linux_raw_sys::general::AF_LOCAL as _);
}

/// `IPPROTO_*`
#[cfg(libc)]
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
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
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
    #[cfg(not(target_os = "netbsd"))]
    Mtp = libc::IPPROTO_MTP,
    /// `IPPROTO_BEETPH`
    #[cfg(not(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "freebsd"
    )))]
    Beetph = libc::IPPROTO_BEETPH,
    /// `IPPROTO_ENCAP`
    Encap = libc::IPPROTO_ENCAP,
    /// `IPPROTO_PIM`
    Pim = libc::IPPROTO_PIM,
    /// `IPPROTO_COMP`
    #[cfg(not(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "freebsd"
    )))]
    Comp = libc::IPPROTO_COMP,
    /// `IPPROTO_SCTP`
    Sctp = libc::IPPROTO_SCTP,
    /// `IPPROTO_UDPLITE`
    #[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "netbsd")))]
    Udplite = libc::IPPROTO_UDPLITE,
    /// `IPPROTO_MPLS`
    #[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "netbsd")))]
    Mpls = libc::IPPROTO_MPLS,
    /// `IPPROTO_RAW`
    Raw = libc::IPPROTO_RAW,
    /// `IPPROTO_MPTCP`
    #[cfg(not(any(
        target_os = "android",
        target_os = "netbsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "freebsd",
        target_os = "emscripten",
        target_os = "fuchsia"
    )))]
    Mptcp = libc::IPPROTO_MPTCP,
}

/// `IPPROTO_*`
#[cfg(linux_raw)]
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
}

/// `SHUT_*`
#[cfg(libc)]
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

/// `SHUT_*`
#[cfg(linux_raw)]
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

#[cfg(libc)]
bitflags! {
    /// `SOCK_*` constants for `accept`.
    pub struct AcceptFlags: c_int {
        /// `SOCK_NONBLOCK`
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        const NONBLOCK = libc::SOCK_NONBLOCK;

        /// `SOCK_CLOEXEC`
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        const CLOEXEC = libc::SOCK_CLOEXEC;
    }
}

#[cfg(linux_raw)]
bitflags! {
    /// `SOCK_*` constants for `accept`.
    pub struct AcceptFlags: c_uint {
        /// `SOCK_NONBLOCK`
        const NONBLOCK = linux_raw_sys::general::O_NONBLOCK;
        /// `SOCK_CLOEXEC`
        const CLOEXEC = linux_raw_sys::general::O_CLOEXEC;
    }
}

/// `socket(domain, type_, protocol)`
#[inline]
pub fn socket(domain: AddressFamily, type_: SocketType, protocol: Protocol) -> io::Result<OwnedFd> {
    _socket(domain, type_, protocol)
}

#[cfg(libc)]
fn _socket(domain: AddressFamily, type_: SocketType, protocol: Protocol) -> io::Result<OwnedFd> {
    unsafe {
        let raw_fd = negone_err(libc::socket(
            domain.0 as c_int,
            type_.0 as c_int,
            protocol as c_int,
        ))?;
        Ok(OwnedFd::from_raw_fd(raw_fd))
    }
}

#[cfg(linux_raw)]
fn _socket(domain: AddressFamily, type_: SocketType, protocol: Protocol) -> io::Result<OwnedFd> {
    crate::linux_raw::socket(domain.0.into(), type_.0, protocol as c_uint)
}

/// `bind(sockfd, addr, sizeof(struct sockaddr_in))`
#[inline]
#[doc(alias("bind"))]
pub fn bind_v4<'f, Fd: AsFd<'f>>(sockfd: Fd, addr: &SocketAddrV4) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    _bind_v4(sockfd, addr)
}

#[cfg(libc)]
fn _bind_v4(sockfd: BorrowedFd<'_>, addr: &SocketAddrV4) -> io::Result<()> {
    unsafe {
        zero_ok(libc::bind(
            sockfd.as_raw_fd(),
            as_ptr(addr).cast::<_>(),
            size_of::<SocketAddrV4>() as socklen_t,
        ))
    }
}

#[cfg(linux_raw)]
fn _bind_v4(sockfd: BorrowedFd<'_>, addr: &SocketAddrV4) -> io::Result<()> {
    crate::linux_raw::bind_in(sockfd, addr)
}

/// `bind(sockfd, addr, sizeof(struct sockaddr_in6))`
#[inline]
#[doc(alias("bind"))]
pub fn bind_v6<'f, Fd: AsFd<'f>>(sockfd: Fd, addr: &SocketAddrV6) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    _bind_v6(sockfd, addr)
}

#[cfg(libc)]
fn _bind_v6(sockfd: BorrowedFd<'_>, addr: &SocketAddrV6) -> io::Result<()> {
    unsafe {
        zero_ok(libc::bind(
            sockfd.as_raw_fd(),
            as_ptr(addr).cast::<_>(),
            size_of::<SocketAddrV6>() as socklen_t,
        ))
    }
}

#[cfg(linux_raw)]
fn _bind_v6(sockfd: BorrowedFd<'_>, addr: &SocketAddrV6) -> io::Result<()> {
    crate::linux_raw::bind_in6(sockfd, addr)
}

/// `bind(sockfd, addr, sizeof(struct sockaddr_un))`
#[inline]
#[doc(alias("bind"))]
pub fn bind_unix<'f, Fd: AsFd<'f>>(sockfd: Fd, addr: &SocketAddrUnix) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    _bind_unix(sockfd, addr)
}

#[cfg(libc)]
fn _bind_unix(sockfd: BorrowedFd<'_>, addr: &SocketAddrUnix) -> io::Result<()> {
    unsafe {
        zero_ok(libc::bind(
            sockfd.as_raw_fd(),
            as_ptr(addr).cast::<_>(),
            size_of::<SocketAddrUnix>() as socklen_t,
        ))
    }
}

#[cfg(linux_raw)]
fn _bind_unix(sockfd: BorrowedFd<'_>, addr: &SocketAddrUnix) -> io::Result<()> {
    crate::linux_raw::bind_un(sockfd, addr)
}

/// `connect(sockfd, addr, sizeof(struct sockaddr_in))`
#[inline]
#[doc(alias("connect"))]
pub fn connect_v4<'f, Fd: AsFd<'f>>(sockfd: Fd, addr: &SocketAddrV4) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    _connect_v4(sockfd, addr)
}

#[cfg(libc)]
fn _connect_v4(sockfd: BorrowedFd<'_>, addr: &SocketAddrV4) -> io::Result<()> {
    unsafe {
        zero_ok(libc::connect(
            sockfd.as_raw_fd(),
            as_ptr(addr).cast::<_>(),
            size_of::<SocketAddrV4>() as socklen_t,
        ))
    }
}

#[cfg(linux_raw)]
fn _connect_v4(sockfd: BorrowedFd<'_>, addr: &SocketAddrV4) -> io::Result<()> {
    crate::linux_raw::connect_in(sockfd, addr)
}

/// `connect(sockfd, addr, sizeof(struct sockaddr_in6))`
#[inline]
#[doc(alias("connect"))]
pub fn connect_v6<'f, Fd: AsFd<'f>>(sockfd: Fd, addr: &SocketAddrV6) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    _connect_v6(sockfd, addr)
}

#[cfg(libc)]
fn _connect_v6(sockfd: BorrowedFd<'_>, addr: &SocketAddrV6) -> io::Result<()> {
    unsafe {
        zero_ok(libc::connect(
            sockfd.as_raw_fd(),
            as_ptr(addr).cast::<_>(),
            size_of::<SocketAddrV6>() as socklen_t,
        ))
    }
}

#[cfg(linux_raw)]
fn _connect_v6(sockfd: BorrowedFd<'_>, addr: &SocketAddrV6) -> io::Result<()> {
    crate::linux_raw::connect_in6(sockfd, addr)
}

/// `connect(sockfd, addr, sizeof(struct sockaddr_un))`
#[doc(alias("connect"))]
#[inline]
pub fn connect_unix<'f, Fd: AsFd<'f>>(sockfd: Fd, addr: &SocketAddrUnix) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    _connect_unix(sockfd, addr)
}

#[cfg(libc)]
fn _connect_unix(sockfd: BorrowedFd<'_>, addr: &SocketAddrUnix) -> io::Result<()> {
    unsafe {
        zero_ok(libc::connect(
            sockfd.as_raw_fd(),
            as_ptr(addr).cast::<_>(),
            size_of::<SocketAddrUnix>() as socklen_t,
        ))
    }
}

#[cfg(linux_raw)]
fn _connect_unix(sockfd: BorrowedFd<'_>, addr: &SocketAddrUnix) -> io::Result<()> {
    crate::linux_raw::connect_un(sockfd, addr)
}

/// `listen(fd, backlog)`
#[inline]
pub fn listen<'f, Fd: AsFd<'f>>(sockfd: Fd, backlog: c_int) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    _listen(sockfd, backlog)
}

#[cfg(libc)]
fn _listen(sockfd: BorrowedFd<'_>, backlog: c_int) -> io::Result<()> {
    unsafe { zero_ok(libc::listen(sockfd.as_raw_fd(), backlog)) }
}

#[cfg(linux_raw)]
#[inline]
fn _listen(sockfd: BorrowedFd<'_>, backlog: c_int) -> io::Result<()> {
    crate::linux_raw::listen(sockfd, backlog)
}

/// `accept4(fd, addr, len, flags)`
#[inline]
#[doc(alias = "accept4")]
pub fn accept<'f, Fd: AsFd<'f>>(
    sockfd: Fd,
    flags: AcceptFlags,
) -> io::Result<(OwnedFd, SocketAddr)> {
    let sockfd = sockfd.as_fd();
    _accept(sockfd, flags)
}

#[cfg(all(libc, not(any(target_os = "ios", target_os = "macos"))))]
fn _accept(sockfd: BorrowedFd<'_>, flags: AcceptFlags) -> io::Result<(OwnedFd, SocketAddr)> {
    unsafe {
        let mut storage = MaybeUninit::<sockaddr_storage>::uninit();
        let mut len = size_of::<sockaddr_storage>() as socklen_t;
        let raw_fd = negone_err(libc::accept4(
            sockfd.as_raw_fd(),
            storage.as_mut_ptr().cast::<_>(),
            &mut len,
            flags.bits(),
        ))?;
        let owned_fd = OwnedFd::from_raw_fd(raw_fd);
        Ok((owned_fd, decode_sockaddr(storage.as_ptr())))
    }
}

/// Darwin lacks `accept4`, but does have `accept`. We define
/// `AcceptFlags` to have no flags, so we can discard it here.
#[cfg(all(libc, any(target_os = "ios", target_os = "macos")))]
fn _accept(sockfd: BorrowedFd<'_>, _flags: AcceptFlags) -> io::Result<(OwnedFd, SocketAddr)> {
    unsafe {
        let mut storage = MaybeUninit::<sockaddr_storage>::uninit();
        let mut len = size_of::<sockaddr_storage>() as socklen_t;
        let raw_fd = negone_err(libc::accept(
            sockfd.as_raw_fd(),
            storage.as_mut_ptr().cast::<_>(),
            &mut len,
        ))?;
        let owned_fd = OwnedFd::from_raw_fd(raw_fd);
        Ok((owned_fd, decode_sockaddr(storage.as_ptr())))
    }
}

#[cfg(linux_raw)]
#[inline]
fn _accept(sockfd: BorrowedFd<'_>, flags: AcceptFlags) -> io::Result<(OwnedFd, SocketAddr)> {
    crate::linux_raw::accept(sockfd, flags.bits())
}

/// `shutdown(fd, how)`
#[inline]
pub fn shutdown<'f, Fd: AsFd<'f>>(sockfd: Fd, how: Shutdown) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    _shutdown(sockfd, how)
}

#[cfg(libc)]
fn _shutdown(sockfd: BorrowedFd<'_>, how: Shutdown) -> io::Result<()> {
    unsafe { zero_ok(libc::shutdown(sockfd.as_raw_fd(), how as c_int)) }
}

#[cfg(linux_raw)]
#[inline]
fn _shutdown(sockfd: BorrowedFd<'_>, how: Shutdown) -> io::Result<()> {
    crate::linux_raw::shutdown(sockfd, how as c_uint)
}

/// `getsockopt(fd, SOL_SOCKET, SO_TYPE)`
#[inline]
pub fn getsockopt_socket_type<'f, Fd: AsFd<'f>>(fd: Fd) -> io::Result<SocketType> {
    let fd = fd.as_fd();
    _getsockopt_socket_type(fd)
}

#[cfg(libc)]
fn _getsockopt_socket_type(fd: BorrowedFd<'_>) -> io::Result<SocketType> {
    let mut buffer = MaybeUninit::<SocketType>::uninit();
    let mut out_len = size_of::<SocketType>() as socklen_t;
    unsafe {
        zero_ok(libc::getsockopt(
            fd.as_raw_fd(),
            libc::SOL_SOCKET,
            libc::SO_TYPE,
            buffer.as_mut_ptr().cast::<libc::c_void>(),
            &mut out_len,
        ))?;
        assert_eq!(
            out_len as usize,
            size_of::<SocketType>(),
            "unexpected SocketType size"
        );
        Ok(buffer.assume_init())
    }
}

#[cfg(linux_raw)]
fn _getsockopt_socket_type(fd: BorrowedFd<'_>) -> io::Result<SocketType> {
    crate::linux_raw::getsockopt_socket_type(fd).map(SocketType)
}

/// `getsockname(fd, addr, len)`
#[inline]
pub fn getsockname<'f, Fd: AsFd<'f>>(sockfd: Fd) -> io::Result<SocketAddr> {
    let sockfd = sockfd.as_fd();
    _getsockname(sockfd)
}

#[cfg(libc)]
fn _getsockname(sockfd: BorrowedFd<'_>) -> io::Result<SocketAddr> {
    unsafe {
        let mut storage = MaybeUninit::<sockaddr_storage>::uninit();
        let mut len = size_of::<sockaddr_storage>() as socklen_t;
        zero_ok(libc::getsockname(
            sockfd.as_raw_fd(),
            storage.as_mut_ptr().cast::<_>(),
            &mut len,
        ))?;
        Ok(decode_sockaddr(storage.as_ptr()))
    }
}

#[cfg(linux_raw)]
#[inline]
fn _getsockname(sockfd: BorrowedFd<'_>) -> io::Result<SocketAddr> {
    crate::linux_raw::getsockname(sockfd)
}

/// `getpeername(fd, addr, len)`
#[inline]
pub fn getpeername<'f, Fd: AsFd<'f>>(sockfd: Fd) -> io::Result<SocketAddr> {
    let sockfd = sockfd.as_fd();
    _getpeername(sockfd)
}

#[cfg(libc)]
fn _getpeername(sockfd: BorrowedFd<'_>) -> io::Result<SocketAddr> {
    unsafe {
        let mut storage = MaybeUninit::<sockaddr_storage>::uninit();
        let mut len = size_of::<sockaddr_storage>() as socklen_t;
        zero_ok(libc::getpeername(
            sockfd.as_raw_fd(),
            storage.as_mut_ptr().cast::<_>(),
            &mut len,
        ))?;
        Ok(decode_sockaddr(storage.as_ptr()))
    }
}

#[cfg(linux_raw)]
#[inline]
fn _getpeername(sockfd: BorrowedFd<'_>) -> io::Result<SocketAddr> {
    crate::linux_raw::getpeername(sockfd)
}
