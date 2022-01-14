#![allow(unsafe_code)]

use core::convert::TryFrom;
use core::marker::PhantomData;
use core::mem::{size_of, zeroed};
use core::ptr::{self, read_unaligned};
use core::slice;

use crate::imp::c;
use crate::imp::fd::{AsFd, AsRawFd, RawFd};
#[cfg(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "android",
    target_os = "ios",
))]
use crate::imp::net::ext::{in6_addr_new, in_addr_new};
#[cfg(any(target_os = "android", target_os = "linux",))]
use crate::imp::syscalls::{getgid, getpid, getuid};
use crate::io::OwnedFd;
#[cfg(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "android",
    target_os = "ios",
))]
use crate::net::SocketAddrV4;
#[cfg(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "android",
    target_os = "ios",
))]
use crate::net::SocketAddrV6;
#[cfg(any(target_os = "android", target_os = "linux",))]
use crate::process::{Gid, Pid, Uid};

/// Create a `[u8; N]` type buffer, where `N` is sized such that it fits the provided types
/// as control messages, as used by `sendmsg` and `recvmsg`.
///
/// # Examples
///
/// ```
/// # fn main() {
/// use rustix::{cmsg_buffer, net::SocketCred};
/// use rustix::io::OwnedFd;
/// // Create a buffer big enough for a `ScmRights` message with two file descriptors.
/// let _ = cmsg_buffer!([OwnedFd; 2]);
/// // Create a buffer big enough for a `ScmRights` message and a `ScmCredentials` message.
/// let _ = cmsg_buffer!(OwnedFd, SocketCred);
/// # }
/// ```
#[macro_export]
macro_rules! cmsg_buffer {
    ( $( $x:ty ),* ) => {{
        [0u8; 0 $(
            + $crate::net::CMSG_SPACE(core::mem::size_of::<$x>() as _) as usize
        )*]
    }}
}

#[doc(hidden)]
pub use c::CMSG_SPACE;

/// TODO: document
#[non_exhaustive]
pub enum SendAncillaryDataUnix<'a> {
    /// TODO: document
    ScmRights(ScmRights<'a>),
    /// TODO: document
    #[cfg(any(target_os = "android", target_os = "linux",))]
    ScmCredentials(ScmCredentials<'a>),
}

/// TODO: document
#[non_exhaustive]
pub enum RecvAncillaryDataUnix<'a> {
    /// TODO: document
    ScmRights(ScmRights<'a>),
    /// TODO: document
    #[cfg(any(target_os = "android", target_os = "linux",))]
    ScmCredentials(ScmCredentials<'a>),
}

/// Conversion trait for internal use.
#[doc(hidden)]
pub trait FromCmsghdr<'a>: Sized + private::SealFromCmsghdr {
    fn try_from(cmsg: &'a c::cmsghdr) -> Result<Self, AncillaryError>;
}

mod private {
    use super::*;

    /// Marker trait to seal FromCmsghdr.
    pub trait SealFromCmsghdr {}

    impl<'a> SealFromCmsghdr for SendAncillaryDataV4<'a> {}
    impl<'a> SealFromCmsghdr for SendAncillaryDataV6<'a> {}
    impl<'a> SealFromCmsghdr for SendAncillaryDataUnix<'a> {}
    impl<'a> SealFromCmsghdr for RecvAncillaryDataV4<'a> {}
    impl<'a> SealFromCmsghdr for RecvAncillaryDataV6<'a> {}
    impl<'a> SealFromCmsghdr for RecvAncillaryDataUnix<'a> {}
    impl<'a> SealFromCmsghdr for RecvAncillaryDataAny<'a> {}
}

impl<'a> FromCmsghdr<'a> for SendAncillaryDataUnix<'a> {
    fn try_from(cmsg: &'a c::cmsghdr) -> Result<Self, AncillaryError> {
        unsafe {
            let cmsg_len_zero = c::CMSG_LEN(0) as usize;
            let data_len = cmsg.cmsg_len as usize - cmsg_len_zero;
            let data = c::CMSG_DATA(cmsg).cast();
            let data: &[u8] = slice::from_raw_parts(data, data_len);

            match (
                cmsg.cmsg_level as c::IpConstantType,
                cmsg.cmsg_type as c::IpConstantType,
            ) {
                (c::SOL_SOCKET, c::SCM_RIGHTS) => Ok(SendAncillaryDataUnix::ScmRights(ScmRights(
                    AncillaryDataIter::new(data),
                ))),
                #[cfg(any(target_os = "android", target_os = "linux",))]
                (c::SOL_SOCKET, c::SCM_CREDENTIALS) => Ok(SendAncillaryDataUnix::ScmCredentials(
                    ScmCredentials(AncillaryDataIter::new(data)),
                )),
                (_, _) => Err(AncillaryError::from_cmsg(&*cmsg)),
            }
        }
    }
}

impl<'a> FromCmsghdr<'a> for RecvAncillaryDataUnix<'a> {
    fn try_from(cmsg: &'a c::cmsghdr) -> Result<Self, AncillaryError> {
        unsafe {
            let cmsg_len_zero = c::CMSG_LEN(0) as usize;
            let data_len = cmsg.cmsg_len as usize - cmsg_len_zero;
            let data = c::CMSG_DATA(cmsg).cast();
            let data: &[u8] = slice::from_raw_parts(data, data_len);

            match (
                cmsg.cmsg_level as c::IpConstantType,
                cmsg.cmsg_type as c::IpConstantType,
            ) {
                (c::SOL_SOCKET, c::SCM_RIGHTS) => Ok(RecvAncillaryDataUnix::ScmRights(ScmRights(
                    AncillaryDataIter::new(data),
                ))),
                #[cfg(any(target_os = "android", target_os = "linux",))]
                (c::SOL_SOCKET, c::SCM_CREDENTIALS) => Ok(RecvAncillaryDataUnix::ScmCredentials(
                    ScmCredentials(AncillaryDataIter::new(data)),
                )),
                (_, _) => Err(AncillaryError::from_cmsg(&*cmsg)),
            }
        }
    }
}

/// TODO: document
#[repr(transparent)]
pub struct ScmRights<'a>(AncillaryDataIter<'a, OwnedFd>);

impl<'a> Iterator for ScmRights<'a> {
    type Item = OwnedFd;

    fn next(&mut self) -> Option<OwnedFd> {
        self.0.next()
    }
}

/// TODO: document
#[cfg(any(target_os = "android", target_os = "linux",))]
#[repr(transparent)]
pub struct ScmCredentials<'a>(AncillaryDataIter<'a, c::ucred>);

#[cfg(any(doc, target_os = "android", target_os = "linux",))]
impl<'a> Iterator for ScmCredentials<'a> {
    type Item = SocketCred;

    fn next(&mut self) -> Option<SocketCred> {
        Some(SocketCred(self.0.next()?))
    }
}

/// TODO: document
#[non_exhaustive]
pub enum SendAncillaryDataV4<'a> {
    /// TODO: document
    #[cfg(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "android",
        target_os = "ios",
    ))]
    PacketInfos(Ipv4PacketInfos<'a>),
    /// TODO: document
    #[cfg(target_os = "linux")]
    UdpGsoSegments(UdpGsoSegments<'a>),
    /// Make the compiler happy if there are no variants present
    #[doc(hidden)]
    Other(PhantomData<&'a [u8]>),
}

/// TODO: document
pub enum RecvAncillaryDataAny<'a> {
    /// TODO: document
    V4(RecvAncillaryDataV4<'a>),
    /// TODO: document
    V6(RecvAncillaryDataV6<'a>),
    /// TODO: document
    Unix(RecvAncillaryDataUnix<'a>),
}

impl<'a> FromCmsghdr<'a> for RecvAncillaryDataAny<'a> {
    fn try_from(cmsg: &'a c::cmsghdr) -> Result<Self, AncillaryError> {
        if let Ok(v4) = <RecvAncillaryDataV4 as FromCmsghdr>::try_from(cmsg) {
            return Ok(RecvAncillaryDataAny::V4(v4));
        }
        if let Ok(v6) = <RecvAncillaryDataV6 as FromCmsghdr>::try_from(cmsg) {
            return Ok(RecvAncillaryDataAny::V6(v6));
        }
        if let Ok(unix) = <RecvAncillaryDataUnix as FromCmsghdr>::try_from(cmsg) {
            return Ok(RecvAncillaryDataAny::Unix(unix));
        }
        Err(AncillaryError::from_cmsg(&*cmsg))
    }
}

/// TODO: document
#[non_exhaustive]
pub enum RecvAncillaryDataV4<'a> {
    /// TODO: document
    #[cfg(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "android",
        target_os = "ios",
    ))]
    PacketInfos(Ipv4PacketInfos<'a>),
    /// TODO: document
    #[cfg(target_os = "linux")]
    UdpGroSegments(UdpGroSegments<'a>),
    /// TODO: document
    #[cfg(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    ))]
    RecvIf(Ipv4RecvIfs<'a>),
    /// TODO: document
    #[cfg(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    ))]
    RecvDstAddr(Ipv4RecvDstAddrs<'a>),
    /// Socket error queue control messages read with the `MSG_ERRQUEUE` flag.
    #[cfg(any(target_os = "android", target_os = "linux"))]
    RecvErr(Ipv4RecvErrs<'a>),
    /// TODO: document
    #[cfg(any(target_os = "android", target_os = "fuchsia", target_os = "linux"))]
    RxqOvfl(RxqOvfls<'a>),
    /// Make the compiler happy if there are no variants present
    #[doc(hidden)]
    Other(PhantomData<&'a [u8]>),
}

impl<'a> FromCmsghdr<'a> for SendAncillaryDataV4<'a> {
    #[cfg(not(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "android",
        target_os = "ios",
    )))]
    fn try_from(cmsg: &'a c::cmsghdr) -> Result<Self, AncillaryError> {
        Err(AncillaryError::from_cmsg(&*cmsg))
    }

    #[cfg(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "android",
        target_os = "ios",
    ))]
    fn try_from(cmsg: &'a c::cmsghdr) -> Result<Self, AncillaryError> {
        unsafe {
            let cmsg_len_zero = c::CMSG_LEN(0) as usize;
            let data_len = cmsg.cmsg_len as usize - cmsg_len_zero;
            let data = c::CMSG_DATA(cmsg).cast();
            let data: &[u8] = slice::from_raw_parts(data, data_len);

            match (
                cmsg.cmsg_level as c::IpConstantType,
                cmsg.cmsg_type as c::IpConstantType,
            ) {
                (c::IPPROTO_IP, c::IP_PKTINFO) => Ok(SendAncillaryDataV4::PacketInfos(
                    Ipv4PacketInfos(AncillaryDataIter::new(data)),
                )),
                #[cfg(target_os = "linux")]
                (c::SOL_UDP, c::UDP_SEGMENT) => Ok(SendAncillaryDataV4::UdpGsoSegments(
                    UdpGsoSegments(AncillaryDataIter::new(data)),
                )),
                (_, _) => Err(AncillaryError::from_cmsg(&*cmsg)),
            }
        }
    }
}

impl<'a> FromCmsghdr<'a> for RecvAncillaryDataV4<'a> {
    #[cfg(not(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "android",
        target_os = "ios",
    )))]
    fn try_from(cmsg: &'a c::cmsghdr) -> Result<Self, AncillaryError> {
        Err(AncillaryError::from_cmsg(&*cmsg))
    }

    #[cfg(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "android",
        target_os = "ios",
    ))]
    fn try_from(cmsg: &'a c::cmsghdr) -> Result<Self, AncillaryError> {
        unsafe {
            let cmsg_len_zero = c::CMSG_LEN(0) as usize;
            let data_len = cmsg.cmsg_len as usize - cmsg_len_zero;
            let data = c::CMSG_DATA(cmsg).cast();
            let data: &[u8] = slice::from_raw_parts(data, data_len);

            match (
                cmsg.cmsg_level as c::IpConstantType,
                cmsg.cmsg_type as c::IpConstantType,
            ) {
                (c::IPPROTO_IP, c::IP_PKTINFO) => Ok(RecvAncillaryDataV4::PacketInfos(
                    Ipv4PacketInfos(AncillaryDataIter::new(data)),
                )),
                #[cfg(target_os = "linux")]
                (c::SOL_UDP, c::UDP_GRO) => Ok(RecvAncillaryDataV4::UdpGroSegments(
                    UdpGroSegments(AncillaryDataIter::new(data)),
                )),
                #[cfg(any(
                    target_os = "freebsd",
                    target_os = "ios",
                    target_os = "macos",
                    target_os = "netbsd",
                    target_os = "openbsd",
                ))]
                (c::IPPROTO_IP, c::IP_RECVIF) => Ok(RecvAncillaryDataV4::RecvIf(Ipv4RecvIfs(
                    AncillaryDataIter::new(data),
                ))),
                #[cfg(any(
                    target_os = "freebsd",
                    target_os = "ios",
                    target_os = "macos",
                    target_os = "netbsd",
                    target_os = "openbsd",
                ))]
                (c::IPPROTO_IP, c::IP_RECVDSTADDR) => Ok(RecvAncillaryDataV4::RecvDstAddr(
                    Ipv4RecvDstAddrs(AncillaryDataIter::new(data)),
                )),
                #[cfg(any(target_os = "android", target_os = "linux"))]
                (c::IPPROTO_IP, c::IP_RECVERR) => Ok(RecvAncillaryDataV4::RecvErr(Ipv4RecvErrs(
                    AncillaryDataIter::new(data),
                ))),
                #[cfg(any(target_os = "android", target_os = "fuchsia", target_os = "linux"))]
                (c::SOL_SOCKET, c::SO_RXQ_OVFL) => Ok(RecvAncillaryDataV4::RxqOvfl(RxqOvfls(
                    AncillaryDataIter::new(data),
                ))),
                (_, _) => Err(AncillaryError::from_cmsg(&*cmsg)),
            }
        }
    }
}

/// TODO: document
#[cfg(any(
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
))]
#[repr(transparent)]
pub struct Ipv4RecvIfs<'a>(AncillaryDataIter<'a, c::sockaddr_dl>);

/// TODO: document
#[cfg(any(
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
))]
#[repr(transparent)]
pub struct Ipv4RecvDstAddrs<'a>(AncillaryDataIter<'a, libc::in_addr>);

/// TODO: document
#[cfg(any(target_os = "android", target_os = "linux"))]
#[repr(transparent)]
pub struct Ipv4RecvErrs<'a>(AncillaryDataIter<'a, (c::sock_extended_err, Option<c::sockaddr_in>)>);

/// TODO: document
#[cfg(any(target_os = "android", target_os = "fuchsia", target_os = "linux"))]
#[repr(transparent)]
pub struct RxqOvfls<'a>(AncillaryDataIter<'a, u32>);

/// TODO: document
#[non_exhaustive]
pub enum SendAncillaryDataV6<'a> {
    /// TODO: document
    #[cfg(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "android",
        target_os = "ios",
    ))]
    PacketInfos(Ipv6PacketInfos<'a>),
    /// TODO: document
    #[cfg(target_os = "linux")]
    UdpGsoSegments(UdpGsoSegments<'a>),
    /// Make the compiler happy if there are no variants present
    #[doc(hidden)]
    Other(PhantomData<&'a [u8]>),
}

impl<'a> FromCmsghdr<'a> for SendAncillaryDataV6<'a> {
    #[cfg(not(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "android",
        target_os = "ios",
    )))]
    fn try_from(cmsg: &'a c::cmsghdr) -> Result<Self, AncillaryError> {
        Err(AncillaryError::from_cmsg(&*cmsg))
    }

    #[cfg(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "android",
        target_os = "ios",
    ))]
    fn try_from(cmsg: &'a c::cmsghdr) -> Result<Self, AncillaryError> {
        unsafe {
            let cmsg_len_zero = c::CMSG_LEN(0) as usize;
            let data_len = cmsg.cmsg_len as usize - cmsg_len_zero;
            let data = c::CMSG_DATA(cmsg).cast();
            let data: &[u8] = slice::from_raw_parts(data, data_len);

            match (
                cmsg.cmsg_level as c::IpConstantType,
                cmsg.cmsg_type as c::IpConstantType,
            ) {
                (c::IPPROTO_IPV6, c::IPV6_PKTINFO) => Ok(SendAncillaryDataV6::PacketInfos(
                    Ipv6PacketInfos(AncillaryDataIter::new(data)),
                )),
                #[cfg(target_os = "linux")]
                (c::SOL_SOCKET, c::UDP_SEGMENT) => Ok(SendAncillaryDataV6::UdpGsoSegments(
                    UdpGsoSegments(AncillaryDataIter::new(data)),
                )),
                (_, _) => Err(AncillaryError::from_cmsg(&*cmsg)),
            }
        }
    }
}

/// TODO: document
#[non_exhaustive]
pub enum RecvAncillaryDataV6<'a> {
    /// TODO: document
    #[cfg(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "android",
        target_os = "ios",
    ))]
    PacketInfos(Ipv6PacketInfos<'a>),
    /// TODO: document
    #[cfg(target_os = "linux")]
    UdpGroSegments(UdpGroSegments<'a>),
    /// Socket error queue control messages read with the `MSG_ERRQUEUE` flag.
    #[cfg(any(target_os = "android", target_os = "linux"))]
    RecvErr(Ipv6RecvErrs<'a>),
    /// TODO: document
    #[cfg(any(target_os = "android", target_os = "fuchsia", target_os = "linux"))]
    RxqOvfl(RxqOvfls<'a>),
    /// Make the compiler happy if there are no variants present
    #[doc(hidden)]
    Other(PhantomData<&'a [u8]>),
}

impl<'a> FromCmsghdr<'a> for RecvAncillaryDataV6<'a> {
    #[cfg(not(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "android",
        target_os = "ios",
        target_os = "fuchsia",
    )))]
    fn try_from(cmsg: &'a c::cmsghdr) -> Result<Self, AncillaryError> {
        Err(AncillaryError::from_cmsg(&*cmsg))
    }

    #[cfg(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "android",
        target_os = "ios",
        target_os = "fuchsia",
    ))]
    fn try_from(cmsg: &'a c::cmsghdr) -> Result<Self, AncillaryError> {
        unsafe {
            let cmsg_len_zero = c::CMSG_LEN(0) as usize;
            let data_len = cmsg.cmsg_len as usize - cmsg_len_zero;
            let data = c::CMSG_DATA(cmsg).cast();
            let data: &[u8] = slice::from_raw_parts(data, data_len);

            match (
                cmsg.cmsg_level as c::IpConstantType,
                cmsg.cmsg_type as c::IpConstantType,
            ) {
                #[cfg(any(
                    target_os = "linux",
                    target_os = "macos",
                    target_os = "netbsd",
                    target_os = "android",
                    target_os = "ios",
                ))]
                (c::IPPROTO_IPV6, c::IPV6_PKTINFO) => Ok(RecvAncillaryDataV6::PacketInfos(
                    Ipv6PacketInfos(AncillaryDataIter::new(data)),
                )),
                #[cfg(target_os = "linux")]
                (c::SOL_UDP, c::UDP_GRO) => Ok(RecvAncillaryDataV6::UdpGroSegments(
                    UdpGroSegments(AncillaryDataIter::new(data)),
                )),
                #[cfg(any(target_os = "android", target_os = "linux"))]
                (c::IPPROTO_IPV6, c::IPV6_RECVERR) => Ok(RecvAncillaryDataV6::RecvErr(
                    Ipv6RecvErrs(AncillaryDataIter::new(data)),
                )),
                #[cfg(any(target_os = "android", target_os = "fuchsia", target_os = "linux"))]
                (c::SOL_SOCKET, c::SO_RXQ_OVFL) => Ok(RecvAncillaryDataV6::RxqOvfl(RxqOvfls(
                    AncillaryDataIter::new(data),
                ))),
                (_, _) => Err(AncillaryError::from_cmsg(&*cmsg)),
            }
        }
    }
}

/// TODO: document
#[cfg(any(target_os = "android", target_os = "linux"))]
#[repr(transparent)]
pub struct Ipv6RecvErrs<'a>(AncillaryDataIter<'a, (c::sock_extended_err, Option<c::sockaddr_in6>)>);

/// TODO: document
#[cfg(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "android",
    target_os = "ios",
))]
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Ipv4PacketInfo(c::in_pktinfo);

#[cfg(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "android",
    target_os = "ios",
))]
impl Default for Ipv4PacketInfo {
    #[cfg(target_os = "netbsd")]
    fn default() -> Self {
        let info = c::in_pktinfo {
            ipi_ifindex: 0,
            ipi_addr: in_addr_new(0),
        };
        Ipv4PacketInfo(info)
    }

    #[cfg(not(target_os = "netbsd"))]
    fn default() -> Self {
        let info = c::in_pktinfo {
            ipi_ifindex: 0,
            ipi_addr: c::in_addr { s_addr: 0 },
            ipi_spec_dst: in_addr_new(0),
        };
        Ipv4PacketInfo(info)
    }
}

#[cfg(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "android",
    target_os = "ios",
))]
impl Ipv4PacketInfo {
    /// Sets `ipi_spec_dst`, the local address to the provided `addr`.
    #[cfg(not(target_os = "netbsd"))]
    pub fn set_local_addr(&mut self, addr: &SocketAddrV4) {
        let sin_addr = in_addr_new(u32::from_ne_bytes(addr.ip().octets()));
        self.0.ipi_spec_dst = sin_addr;
    }

    /// Sets `ipi_ifindex`, the interface index to the provided `index`.
    pub fn set_interface_index(&mut self, index: u32) {
        self.0.ipi_ifindex = index as _;
    }
}

/// TODO: document
#[cfg(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "android",
    target_os = "ios",
))]
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Ipv6PacketInfo(c::in6_pktinfo);

#[cfg(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "android",
    target_os = "ios",
))]
impl Default for Ipv6PacketInfo {
    fn default() -> Self {
        let info = c::in6_pktinfo {
            ipi6_ifindex: 0,
            ipi6_addr: in6_addr_new([0u8; 16]),
        };
        Ipv6PacketInfo(info)
    }
}

#[cfg(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "android",
    target_os = "ios",
))]
impl Ipv6PacketInfo {
    /// Sets `ipi6_addr`, the source address to the provided `addr`.
    pub fn set_source_addr(&mut self, addr: &SocketAddrV6) {
        let sin_addr = in6_addr_new(addr.ip().octets());
        self.0.ipi6_addr = sin_addr;
    }

    /// Sets `ipi6_ifindex`, the interface index to the provided `index`.
    pub fn set_interface_index(&mut self, index: u32) {
        self.0.ipi6_ifindex = index as _;
    }
}

/// TODO: document
#[cfg(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "android",
    target_os = "ios",
))]
pub struct Ipv4PacketInfos<'a>(AncillaryDataIter<'a, c::in_pktinfo>);

/// TODO: document
#[cfg(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "android",
    target_os = "ios",
))]
pub struct Ipv6PacketInfos<'a>(AncillaryDataIter<'a, c::in6_pktinfo>);

/// TODO: document
#[cfg(target_os = "linux")]
pub struct UdpGsoSegments<'a>(AncillaryDataIter<'a, u16>);

/// TODO: document
#[cfg(target_os = "linux")]
pub struct UdpGroSegments<'a>(AncillaryDataIter<'a, u16>);

/// Unix credential.
#[cfg(any(target_os = "android", target_os = "linux",))]
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct SocketCred(c::ucred);

#[cfg(any(target_os = "android", target_os = "linux",))]
impl SocketCred {
    /// Create a Unix credential struct.
    ///
    /// PID, UID and GID is set to 0.
    #[must_use]
    pub fn new() -> Self {
        SocketCred(c::ucred {
            pid: 0,
            uid: 0,
            gid: 0,
        })
    }

    /// Creates a Unix credential struct from the currrent process.
    #[must_use]
    pub fn from_process() -> Self {
        SocketCred(c::ucred {
            pid: getpid().as_raw_nonzero().into(),
            uid: getuid().as_raw(),
            gid: getgid().as_raw(),
        })
    }

    /// Set the PID.
    pub fn set_pid(&mut self, pid: Pid) {
        self.0.pid = pid.as_raw_nonzero().into();
    }

    /// Get the current PID.
    pub fn get_pid(&self) -> Option<Pid> {
        unsafe { Pid::from_raw(self.0.pid) }
    }

    /// Set the UID.
    pub fn set_uid(&mut self, uid: Uid) {
        self.0.uid = uid.as_raw();
    }

    /// Get the current UID.
    pub fn get_uid(&self) -> Uid {
        unsafe { Uid::from_raw(self.0.uid) }
    }

    /// Set the GID.
    pub fn set_gid(&mut self, gid: Gid) {
        self.0.gid = gid.as_raw();
    }

    /// Get the current GID.
    pub fn get_gid(&self) -> Gid {
        unsafe { Gid::from_raw(self.0.gid) }
    }
}

// TODO: Find a way to use MaybeUninit as backing data.

// TODO: Should there exist a convenience wrapper that owns the buffer and potentially
// auto resizes?

// TODO: port tests from https://github.com/nix-rust/nix/blob/master/test/sys/test_socket.rs

/// TODO: document
#[derive(Debug)]
pub struct SocketAncillary<'a, T: FromCmsghdr<'a>> {
    buffer: &'a mut [u8],
    pub(crate) length: usize,
    pub(crate) truncated: bool,
    /// helper struct to easily operate the `CMSG_*` macros.
    msg: c::msghdr,
    _t: PhantomData<T>,
}

impl<'a, T: FromCmsghdr<'a>> SocketAncillary<'a, T> {
    /// Create an ancillary data with the given buffer.
    pub fn new(buffer: &'a mut [u8]) -> Self {
        let mut msg: c::msghdr = unsafe { zeroed() };
        msg.msg_control = buffer.as_mut_ptr().cast();
        msg.msg_controllen = 0;

        SocketAncillary {
            buffer,
            length: 0,
            truncated: false,
            msg,
            _t: Default::default(),
        }
    }

    /// Returns the capacity of the buffer.
    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    /// Returns `true` if the ancillary data is empty.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns the number of used bytes.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Is `true` if during a recv operation the ancillary was truncated.
    pub fn truncated(&self) -> bool {
        self.truncated
    }

    /// Clears the ancillary data, removing all values.
    pub fn clear(&mut self) {
        self.length = 0;
        self.truncated = false;
        self.msg.msg_controllen = 0;
    }

    /// Returns the iterator of the control messages.
    pub fn messages<'b: 'a>(&'b self) -> Messages<'a, T> {
        Messages {
            buffer: &self.buffer[..self.length],
            current: None,
            _t: Default::default(),
        }
    }

    pub(crate) unsafe fn buffer_mut_ptr(&mut self) -> *mut u8 {
        self.buffer.as_mut_ptr()
    }

    pub(crate) fn buffer_len(&self) -> usize {
        self.buffer.len()
    }

    /// Returns a pointer to the next `cmsgshdr` data section to write to, if enough space is available.
    unsafe fn get_cmsg_data(
        &mut self,
        source_len: Option<u32>,
        cmsg_level: u32,
        cmsg_type: u32,
    ) -> Option<(u32, &mut [u8])> {
        let source_len = source_len?;

        let additional_space = c::CMSG_SPACE(source_len as _) as usize;

        let new_length = additional_space.checked_add(self.length)?;

        if new_length > self.buffer.len() {
            return None;
        }

        for i in self.length..new_length {
            self.buffer[i] = 0;
        }
        self.length = new_length;
        self.msg.msg_control = self.buffer.as_mut_ptr().cast();
        self.msg.msg_controllen = self.length as _;

        let mut cmsg = c::CMSG_FIRSTHDR(&self.msg);
        let mut previous_cmsg = cmsg;
        while !cmsg.is_null() {
            previous_cmsg = cmsg;
            cmsg = c::CMSG_NXTHDR(&self.msg, cmsg);

            // Most operating systems, but not Linux or emscripten, return the previous pointer
            // when its length is zero. Therefore, check if the previous pointer is the same as
            // the current one.
            if ptr::eq(cmsg, previous_cmsg) {
                break;
            }
        }

        if previous_cmsg.is_null() {
            return None;
        }

        let cmsg_len = c::CMSG_LEN(source_len as _);
        (*previous_cmsg).cmsg_level = cmsg_level as _;
        (*previous_cmsg).cmsg_type = cmsg_type as _;
        (*previous_cmsg).cmsg_len = cmsg_len as _;

        let data: *mut u8 = c::CMSG_DATA(previous_cmsg).cast();
        let data_slice = slice::from_raw_parts_mut(data, usize::try_from(cmsg_len).ok()?);

        Some((source_len, data_slice))
    }
}

/// TODO: document
pub type SendSocketAncillaryUnix<'a> = SocketAncillary<'a, SendAncillaryDataUnix<'a>>;

impl<'a> SocketAncillary<'a, SendAncillaryDataUnix<'a>> {
    /// Add file descriptors to the ancillary data.
    ///
    /// The function returns `true` if there was enough space in the buffer.
    /// If there was not enough space then no file descriptors was appended.
    /// Technically, that means this operation adds a control message with the level `SOL_SOCKET`
    /// and type `SCM_RIGHTS`.
    pub fn add_fds<Fd: AsFd>(&mut self, fds: &[Fd]) -> bool {
        self.truncated = false;
        let size_single = size_of::<RawFd>();
        let size = fds
            .len()
            .checked_mul(size_single)
            .and_then(|v| u32::try_from(v).ok());

        unsafe {
            match self.get_cmsg_data(size, c::SOL_SOCKET as _, c::SCM_RIGHTS as _) {
                Some((_, data)) => {
                    for (fd, data_chunk) in fds.iter().zip(data.chunks_mut(size_single)) {
                        let raw = fd.as_fd().as_raw_fd();
                        data_chunk[..size_single].copy_from_slice(&raw.to_ne_bytes());
                    }
                    true
                }
                None => false,
            }
        }
    }

    /// Add credentials to the ancillary data.
    ///
    /// The function returns `true` if there was enough space in the buffer.
    /// If there was not enough space then no credentials was appended.
    /// Technically, that means this operation adds a control message with the level `SOL_SOCKET`
    /// and type `SCM_CREDENTIALS` or `SCM_CREDS`.
    ///
    #[cfg(any(target_os = "android", target_os = "linux",))]
    pub fn add_creds(&mut self, creds: &[SocketCred]) -> bool {
        self.truncated = false;
        let size = creds
            .len()
            .checked_mul(size_of::<c::ucred>())
            .and_then(|v| u32::try_from(v).ok());

        unsafe {
            match self.get_cmsg_data(size, c::SOL_SOCKET as _, c::SCM_CREDENTIALS as _) {
                Some((size, data)) => {
                    ptr::copy_nonoverlapping(
                        creds as *const _ as *mut c::ucred as *mut u8,
                        data.as_mut_ptr(),
                        size as usize,
                    );
                    true
                }
                None => false,
            }
        }
    }
}

/// TODO: document
pub type RecvSocketAncillaryAny<'a> = SocketAncillary<'a, RecvAncillaryDataAny<'a>>;

/// TODO: document
pub type RecvSocketAncillaryUnix<'a> = SocketAncillary<'a, RecvAncillaryDataUnix<'a>>;

/// TODO: document
pub type SendSocketAncillaryV4<'a> = SocketAncillary<'a, SendAncillaryDataV4<'a>>;

impl<'a> SocketAncillary<'a, SendAncillaryDataV4<'a>> {
    /// TODO
    #[cfg(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "android",
        target_os = "ios",
    ))]
    pub fn add_packet_info(&mut self, info: &Ipv4PacketInfo) -> bool {
        self.truncated = false;
        let size = u32::try_from(size_of::<c::in_pktinfo>()).ok();
        unsafe {
            match self.get_cmsg_data(size, c::IPPROTO_IP as _, c::IP_PKTINFO as _) {
                Some((size, data)) => {
                    ptr::copy_nonoverlapping(
                        info as *const _ as *mut c::in_pktinfo as *mut u8,
                        data.as_mut_ptr(),
                        size as usize,
                    );
                    true
                }
                None => false,
            }
        }
    }

    /// TODO: document
    #[cfg(target_os = "linux")]
    pub fn add_udp_gso_segment(&mut self, gso_size: u16) -> bool {
        self.truncated = false;
        let size = u32::try_from(size_of::<u16>()).ok();
        unsafe {
            match self.get_cmsg_data(size, c::SOL_UDP as _, c::UDP_SEGMENT as _) {
                Some((size, data)) => {
                    data[..size as usize].copy_from_slice(&gso_size.to_ne_bytes());
                    true
                }
                None => false,
            }
        }
    }
}

/// TODO: document
pub type RecvSocketAncillaryV4<'a> = SocketAncillary<'a, RecvAncillaryDataV4<'a>>;

/// TODO: document
pub type SendSocketAncillaryV6<'a> = SocketAncillary<'a, SendAncillaryDataV6<'a>>;

impl<'a> SocketAncillary<'a, SendAncillaryDataV6<'a>> {
    /// TODO
    #[cfg(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "android",
        target_os = "ios",
    ))]
    pub fn add_packet_info(&mut self, info: &Ipv6PacketInfo) -> bool {
        self.truncated = false;
        let size = u32::try_from(size_of::<c::in6_pktinfo>()).ok();
        unsafe {
            match self.get_cmsg_data(size, c::IPPROTO_IPV6 as _, c::IPV6_PKTINFO as _) {
                Some((size, data)) => {
                    ptr::copy_nonoverlapping(
                        info as *const _ as *mut c::in6_pktinfo as *mut u8,
                        data.as_mut_ptr(),
                        size as usize,
                    );
                    true
                }
                None => false,
            }
        }
    }

    /// TODO: document
    #[cfg(target_os = "linux")]
    pub fn add_udp_gso_segment(&mut self, gso_size: u16) -> bool {
        self.truncated = false;
        let size = u32::try_from(size_of::<u16>()).ok();
        unsafe {
            match self.get_cmsg_data(size, c::SOL_UDP as _, c::UDP_SEGMENT as _) {
                Some((size, data)) => {
                    data[..size as usize].copy_from_slice(&gso_size.to_ne_bytes());
                    true
                }
                None => false,
            }
        }
    }
}

/// TODO: document
pub type RecvSocketAncillaryV6<'a> = SocketAncillary<'a, RecvAncillaryDataV6<'a>>;

/// The error type which is returned from parsing the type a control message.
#[non_exhaustive]
#[derive(Debug)]
pub enum AncillaryError {
    /// TODO: document me
    Unknown {
        /// TODO: document me
        cmsg_level: i32,
        /// TODO: document me
        cmsg_type: i32,
    },
}

impl AncillaryError {
    fn from_cmsg(cmsg: &c::cmsghdr) -> Self {
        AncillaryError::Unknown {
            cmsg_level: cmsg.cmsg_level as _,
            cmsg_type: cmsg.cmsg_type as _,
        }
    }
}

/// This struct is used to iterate through the control messages.
pub struct Messages<'a, T: FromCmsghdr<'a>> {
    buffer: &'a [u8],
    current: Option<&'a c::cmsghdr>,
    _t: PhantomData<T>,
}

impl<'a, T: FromCmsghdr<'a>> Iterator for Messages<'a, T> {
    type Item = Result<T, AncillaryError>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut msg: c::msghdr = zeroed();
            msg.msg_control = self.buffer.as_ptr() as *mut _;
            msg.msg_controllen = self.buffer.len() as _;

            let cmsg = if let Some(current) = self.current {
                c::CMSG_NXTHDR(&msg, current)
            } else {
                c::CMSG_FIRSTHDR(&msg)
            };

            let cmsg = cmsg.as_ref()?;

            // Most operating systems, but not Linux or emscripten, return the previous pointer
            // when its length is zero. Therefore, check if the previous pointer is the same as
            // the current one.
            if let Some(current) = self.current {
                if ptr::eq(current, cmsg) {
                    return None;
                }
            }

            self.current = Some(cmsg);
            Some(T::try_from(cmsg))
        }
    }
}

/// TODO: document
struct AncillaryDataIter<'a, T> {
    data: &'a [u8],
    phantom: PhantomData<T>,
}

impl<'a, T> AncillaryDataIter<'a, T> {
    /// Create `AncillaryDataIter` struct to iterate through the data unit in the control message.
    ///
    /// # Safety
    ///
    /// `data` must contain a valid control message.
    unsafe fn new(data: &'a [u8]) -> AncillaryDataIter<'a, T> {
        AncillaryDataIter {
            data,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Iterator for AncillaryDataIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if size_of::<T>() <= self.data.len() {
            unsafe {
                let unit = read_unaligned(self.data.as_ptr().cast());
                self.data = &self.data[size_of::<T>()..];
                Some(unit)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::OwnedFd;

    #[test]
    fn test_cmsg_buffer() {
        let buf = cmsg_buffer!([OwnedFd; 2]);
        assert_eq!(
            buf.len(),
            c::CMSG_SPACE(core::mem::size_of::<[OwnedFd; 2]>() as _) as _
        );
    }
}
