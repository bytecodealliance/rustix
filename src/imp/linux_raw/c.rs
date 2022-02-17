//! Adapt the Linux API to resemble a POSIX-style libc API.
//!
//! The linux_raw backend doesn't use actual libc; this just
//! defines certain types that are convenient to have defined.

#![allow(unused_imports, dead_code)]
#![allow(unsafe_code)]
#![allow(missing_docs)]

pub(crate) use linux_raw_sys::general::{
    AF_DECnet, __kernel_sa_family_t as sa_family_t, cmsghdr, in6_addr, in6_pktinfo, in_addr,
    in_pktinfo, iovec, ip_mreq, ipv6_mreq, linger, msghdr, size_t, sockaddr, sockaddr_in,
    sockaddr_in6, sockaddr_un, socklen_t, ucred, AF_APPLETALK, AF_ASH, AF_ATMPVC, AF_ATMSVC,
    AF_AX25, AF_BLUETOOTH, AF_BRIDGE, AF_CAN, AF_ECONET, AF_IEEE802154, AF_INET, AF_INET6, AF_IPX,
    AF_IRDA, AF_ISDN, AF_IUCV, AF_KEY, AF_LLC, AF_NETBEUI, AF_NETLINK, AF_NETROM, AF_PACKET,
    AF_PHONET, AF_PPPOX, AF_RDS, AF_ROSE, AF_RXRPC, AF_SECURITY, AF_SNA, AF_TIPC, AF_UNIX,
    AF_UNSPEC, AF_WANPIPE, AF_X25, IPV6_ADD_MEMBERSHIP, IPV6_DROP_MEMBERSHIP, IPV6_MULTICAST_LOOP,
    IPV6_PKTINFO, IPV6_RECVERR, IPV6_V6ONLY, IP_ADD_MEMBERSHIP, IP_DROP_MEMBERSHIP,
    IP_MULTICAST_LOOP, IP_MULTICAST_TTL, IP_PKTINFO, IP_RECVERR, IP_TTL, MSG_CMSG_CLOEXEC,
    MSG_CONFIRM, MSG_CTRUNC, MSG_DONTROUTE, MSG_DONTWAIT, MSG_EOR, MSG_ERRQUEUE, MSG_MORE,
    MSG_NOSIGNAL, MSG_OOB, MSG_PEEK, MSG_TRUNC, MSG_WAITALL, O_CLOEXEC, O_NONBLOCK,
    SCM_CREDENTIALS, SCM_RIGHTS, SHUT_RD, SHUT_RDWR, SHUT_WR, SOCK_DGRAM, SOCK_RAW, SOCK_RDM,
    SOCK_SEQPACKET, SOCK_STREAM, SOL_SOCKET, SO_BROADCAST, SO_LINGER, SO_PASSCRED, SO_RCVTIMEO_NEW,
    SO_RCVTIMEO_OLD, SO_REUSEADDR, SO_SNDTIMEO_NEW, SO_SNDTIMEO_OLD, SO_TYPE, TCP_NODELAY,
};

pub(crate) use linux_raw_sys::ctypes::*;

pub(crate) use linux_raw_sys::errno::EINVAL;

use core::{mem, ptr};

/// The type of constants like `IPPROTO_IP`.
pub type IpConstantType = u32;

/// Given a length, returns it including the required alignment.
///
/// https://man7.org/linux/man-pages/man3/cmsg_align.3.html
#[allow(non_snake_case)]
pub const fn CMSG_ALIGN(len: size_t) -> size_t {
    len + mem::size_of::<usize>() as size_t - 1 & !(mem::size_of::<usize>() as size_t - 1)
}

/// Returns a pointer to the first `cmsghdr` in the ancillary data buffer
/// associated with the passed `msghdr`.
///
/// https://man7.org/linux/man-pages/man3/cmsg.3.html
/// Safety: `mhdr` must point to an initialized `msghdr`.
#[allow(non_snake_case)]
pub unsafe fn CMSG_FIRSTHDR(mhdr: *const msghdr) -> *mut cmsghdr {
    if (*mhdr).msg_controllen as usize >= mem::size_of::<cmsghdr>() {
        (*mhdr).msg_control as *mut cmsghdr
    } else {
        ptr::null_mut()
    }
}

/// Returns a pointer to the data portion of a cmsghdr.
///
/// https://man7.org/linux/man-pages/man3/cmsg.3.html
#[allow(non_snake_case)]
pub unsafe fn CMSG_DATA(cmsg: *const cmsghdr) -> *mut c_uchar {
    cmsg.offset(1) as *mut _
}

/// Returns the number of bytes an ancillary element with payload of the passed
/// data length occupies.
///
/// https://man7.org/linux/man-pages/man3/cmsg.3.html
#[allow(non_snake_case)]
pub const fn CMSG_SPACE(length: size_t) -> size_t {
    CMSG_ALIGN(length) + CMSG_ALIGN(mem::size_of::<cmsghdr>() as size_t)
}

/// Given a length, returns it including the required alignment.
///
/// https://man7.org/linux/man-pages/man3/cmsg.3.html
#[allow(non_snake_case)]
pub const fn CMSG_LEN(length: size_t) -> size_t {
    CMSG_ALIGN(mem::size_of::<cmsghdr>() as size_t) + length
}

/// Returns the next valid `cmsghdr` after the passed `cmsghdr`. It returns
/// NULL when there isn't enough space left in the buffer.
///
/// https://man7.org/linux/man-pages/man3/cmsg.3.html
#[allow(non_snake_case)]
pub unsafe fn CMSG_NXTHDR(mhdr: *const msghdr, cmsg: *const cmsghdr) -> *mut cmsghdr {
    if ((*cmsg).cmsg_len as usize) < mem::size_of::<cmsghdr>() {
        return ptr::null_mut();
    };
    let next = (cmsg as usize + CMSG_ALIGN((*cmsg).cmsg_len) as usize) as *mut cmsghdr;
    let max = (*mhdr).msg_control as usize + (*mhdr).msg_controllen as usize;
    if (next.offset(1)) as usize > max
        || next as usize + CMSG_ALIGN((*next).cmsg_len) as usize > max
    {
        0 as *mut cmsghdr
    } else {
        next as *mut cmsghdr
    }
}

// TODO: move back to linux-raw-sys
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone)]
pub struct sock_extended_err {
    pub ee_errno: u32,
    pub ee_origin: u8,
    pub ee_type: u8,
    pub ee_code: u8,
    pub ee_pad: u8,
    pub ee_info: u32,
    pub ee_daa: u32,
}

pub const SO_EE_ORIGIN_NONE: u8 = 0;
pub const SO_EE_ORIGIN_LOCAL: u8 = 1;
pub const SO_EE_ORIGIN_ICMP: u8 = 2;
pub const SO_EE_ORIGIN_ICMP6: u8 = 3;
pub const SO_EE_ORIGIN_TIMESTAMPING: u8 = 4;

#[allow(non_snake_case)]
pub unsafe fn SO_EE_OFFENDER(ee: *const sock_extended_err) -> *mut sockaddr {
    ee.offset(1) as *mut sockaddr
}

// avoid enum bindgen type
pub const IPPROTO_IP: IpConstantType = linux_raw_sys::general::IPPROTO_IP as IpConstantType;
pub const IPPROTO_IPV6: IpConstantType = linux_raw_sys::general::IPPROTO_IPV6 as IpConstantType;
pub const IPPROTO_AH: IpConstantType = linux_raw_sys::general::IPPROTO_AH as IpConstantType;
pub const IPPROTO_BEETPH: IpConstantType = linux_raw_sys::general::IPPROTO_BEETPH as IpConstantType;
pub const IPPROTO_COMP: IpConstantType = linux_raw_sys::general::IPPROTO_COMP as IpConstantType;
pub const IPPROTO_DCCP: IpConstantType = linux_raw_sys::general::IPPROTO_DCCP as IpConstantType;
pub const IPPROTO_EGP: IpConstantType = linux_raw_sys::general::IPPROTO_EGP as IpConstantType;
pub const IPPROTO_ENCAP: IpConstantType = linux_raw_sys::general::IPPROTO_ENCAP as IpConstantType;
pub const IPPROTO_ESP: IpConstantType = linux_raw_sys::general::IPPROTO_ESP as IpConstantType;
pub const IPPROTO_ETHERNET: IpConstantType =
    linux_raw_sys::general::IPPROTO_ETHERNET as IpConstantType;
pub const IPPROTO_FRAGMENT: IpConstantType =
    linux_raw_sys::general::IPPROTO_FRAGMENT as IpConstantType;
pub const IPPROTO_GRE: IpConstantType = linux_raw_sys::general::IPPROTO_GRE as IpConstantType;
pub const IPPROTO_ICMP: IpConstantType = linux_raw_sys::general::IPPROTO_ICMP as IpConstantType;
pub const IPPROTO_ICMPV6: IpConstantType = linux_raw_sys::general::IPPROTO_ICMPV6 as IpConstantType;
pub const IPPROTO_IDP: IpConstantType = linux_raw_sys::general::IPPROTO_IDP as IpConstantType;
pub const IPPROTO_IGMP: IpConstantType = linux_raw_sys::general::IPPROTO_IGMP as IpConstantType;
pub const IPPROTO_IPIP: IpConstantType = linux_raw_sys::general::IPPROTO_IPIP as IpConstantType;
pub const IPPROTO_MH: IpConstantType = linux_raw_sys::general::IPPROTO_MH as IpConstantType;
pub const IPPROTO_MPLS: IpConstantType = linux_raw_sys::general::IPPROTO_MPLS as IpConstantType;
pub const IPPROTO_MPTCP: IpConstantType = linux_raw_sys::general::IPPROTO_MPTCP as IpConstantType;
pub const IPPROTO_MTP: IpConstantType = linux_raw_sys::general::IPPROTO_MTP as IpConstantType;
pub const IPPROTO_PIM: IpConstantType = linux_raw_sys::general::IPPROTO_PIM as IpConstantType;
pub const IPPROTO_PUP: IpConstantType = linux_raw_sys::general::IPPROTO_PUP as IpConstantType;
pub const IPPROTO_RAW: IpConstantType = linux_raw_sys::general::IPPROTO_RAW as IpConstantType;
pub const IPPROTO_ROUTING: IpConstantType =
    linux_raw_sys::general::IPPROTO_ROUTING as IpConstantType;
pub const IPPROTO_RSVP: IpConstantType = linux_raw_sys::general::IPPROTO_RSVP as IpConstantType;
pub const IPPROTO_SCTP: IpConstantType = linux_raw_sys::general::IPPROTO_SCTP as IpConstantType;
pub const IPPROTO_TCP: IpConstantType = linux_raw_sys::general::IPPROTO_TCP as IpConstantType;
pub const IPPROTO_TP: IpConstantType = linux_raw_sys::general::IPPROTO_TP as IpConstantType;
pub const IPPROTO_UDP: IpConstantType = linux_raw_sys::general::IPPROTO_UDP as IpConstantType;
pub const IPPROTO_UDPLITE: IpConstantType =
    linux_raw_sys::general::IPPROTO_UDPLITE as IpConstantType;

// TODO: move these constants to linux-raw-sys

#[cfg(target_os = "linux")]
pub const SOL_UDP: IpConstantType = 17;
// Only available on kernel >= 5.0
#[cfg(target_os = "linux")]
pub const UDP_GRO: IpConstantType = 104;
// only available on kernel >= 4.18
#[cfg(target_os = "linux")]
pub const UDP_SEGMENT: IpConstantType = 103;
#[cfg(any(
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
))]
pub const IP_RECVDSTADDR: IpConstantType = 7;
#[cfg(any(target_os = "android", target_os = "fuchsia", target_os = "linux"))]
pub const SO_RXQ_OVFL: IpConstantType = 40;
#[cfg(any(
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
))]
pub const IP_RECVIF: IpConstantType = 20;
