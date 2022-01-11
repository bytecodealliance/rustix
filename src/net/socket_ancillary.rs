// TODO: remove
#![allow(dead_code, unused_variables)]
#![allow(unsafe_code)]

use core::convert::TryFrom;
use core::marker::PhantomData;
use core::mem::{size_of, zeroed};
use core::ptr::{self, read_unaligned};
use core::slice;

use crate::imp::c;
use crate::imp::fd::AsFd;
use crate::imp::syscalls::{getgid, getpid, getuid};
use crate::io::OwnedFd;
use crate::process::{Gid, Pid, Uid};

/// Create a buffer large enough for storing some control messages as returned by `recvmsg`.
///
/// # Examples
///
/// ```
/// # fn main() {
/// use rustix::{cmsg_space, net::SocketCred};
/// use rustix::io::OwnedFd;
/// // Create a buffer big enough for a `ScmRights` message with two file descriptors.
/// let _ = cmsg_space!([OwnedFd; 2]);
/// // Create a buffer big enough for a `ScmRights` message and a `ScmCredentials` message.
/// let _ = cmsg_space!(OwnedFd, SocketCred);
/// # }
/// ```
#[macro_export]
macro_rules! cmsg_space {
    ( $( $x:ty ),* ) => {
        {
            [0u8; 0 $(
                + $crate::net::CMSG_SPACE(core::mem::size_of::<$x>() as _) as usize
            )*]
        }
    }
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

impl<'a> TryFrom<&'a c::cmsghdr> for SendAncillaryDataUnix<'a> {
    type Error = AncillaryError;

    fn try_from(cmsg: &'a c::cmsghdr) -> Result<Self, AncillaryError> {
        unsafe {
            let cmsg_len_zero = c::CMSG_LEN(0) as usize;
            let data_len = (*cmsg).cmsg_len as usize - cmsg_len_zero;
            let data = c::CMSG_DATA(cmsg).cast();
            let data = slice::from_raw_parts(data, data_len);

            match ((*cmsg).cmsg_level as _, (*cmsg).cmsg_type as _) {
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

impl<'a> TryFrom<&'a c::cmsghdr> for RecvAncillaryDataUnix<'a> {
    type Error = AncillaryError;

    fn try_from(cmsg: &'a c::cmsghdr) -> Result<Self, AncillaryError> {
        unsafe {
            let cmsg_len_zero = c::CMSG_LEN(0) as usize;
            let data_len = (*cmsg).cmsg_len as usize - cmsg_len_zero;
            let data = c::CMSG_DATA(cmsg).cast();
            let data = slice::from_raw_parts(data, data_len);

            match ((*cmsg).cmsg_level as _, (*cmsg).cmsg_type as _) {
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
    PacketInfos(Ipv4PacketInfos<'a>),
    /// TODO: document
    #[cfg(target_os = "linux")]
    UdpGsoSegments(UdpGsoSegments<'a>),
}

/// TODO: document
#[non_exhaustive]
pub enum RecvAncillaryDataV4<'a> {
    /// TODO: document
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
}

impl<'a> TryFrom<&'a c::cmsghdr> for SendAncillaryDataV4<'a> {
    type Error = AncillaryError;

    fn try_from(cmsg: &'a c::cmsghdr) -> Result<Self, AncillaryError> {
        unsafe {
            let cmsg_len_zero = c::CMSG_LEN(0) as usize;
            let data_len = (*cmsg).cmsg_len as usize - cmsg_len_zero;
            let data = c::CMSG_DATA(cmsg).cast();
            let data = slice::from_raw_parts(data, data_len);

            match ((*cmsg).cmsg_level as _, (*cmsg).cmsg_type as _) {
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

impl<'a> TryFrom<&'a c::cmsghdr> for RecvAncillaryDataV4<'a> {
    type Error = AncillaryError;

    fn try_from(cmsg: &'a c::cmsghdr) -> Result<Self, AncillaryError> {
        unsafe {
            let cmsg_len_zero = c::CMSG_LEN(0) as usize;
            let data_len = (*cmsg).cmsg_len as usize - cmsg_len_zero;
            let data = c::CMSG_DATA(cmsg).cast();
            let data = slice::from_raw_parts(data, data_len);

            match ((*cmsg).cmsg_level as _, (*cmsg).cmsg_type as _) {
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
    PacketInfos(Ipv6PacketInfos<'a>),
    /// TODO: document
    #[cfg(target_os = "linux")]
    UdpGsoSegments(UdpGsoSegments<'a>),
}

impl<'a> TryFrom<&'a c::cmsghdr> for SendAncillaryDataV6<'a> {
    type Error = AncillaryError;

    fn try_from(cmsg: &'a c::cmsghdr) -> Result<Self, AncillaryError> {
        unsafe {
            let cmsg_len_zero = c::CMSG_LEN(0) as usize;
            let data_len = (*cmsg).cmsg_len as usize - cmsg_len_zero;
            let data = c::CMSG_DATA(cmsg).cast();
            let data = slice::from_raw_parts(data, data_len);

            match ((*cmsg).cmsg_level as _, (*cmsg).cmsg_type as _) {
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
}

impl<'a> TryFrom<&'a c::cmsghdr> for RecvAncillaryDataV6<'a> {
    type Error = AncillaryError;

    fn try_from(cmsg: &'a c::cmsghdr) -> Result<Self, AncillaryError> {
        unsafe {
            let cmsg_len_zero = c::CMSG_LEN(0) as usize;
            let data_len = (*cmsg).cmsg_len as usize - cmsg_len_zero;
            let data = c::CMSG_DATA(cmsg).cast();
            let data = slice::from_raw_parts(data, data_len);

            match ((*cmsg).cmsg_level as _, (*cmsg).cmsg_type as _) {
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
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Ipv4PacketInfo(c::in_pktinfo);

/// TODO: document
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Ipv6PacketInfo(c::in6_pktinfo);

/// TODO: document
pub struct Ipv4PacketInfos<'a>(AncillaryDataIter<'a, c::in_pktinfo>);

/// TODO: document
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

// TODO: Provide way of sizing the buffer for SocketAncillary upfront, like in
// https://docs.rs/nix/latest/nix/macro.cmsg_space.html

// TODO: Find a way to use MaybeUninit as backing data.

// TODO: Should there exist a convenience wrapper that owns the buffer and potentially
// auto resizes?

// TODO: port tests from https://github.com/nix-rust/nix/blob/master/test/sys/test_socket.rs

/// TODO: document
#[derive(Debug)]
pub struct SocketAncillary<'a, T: TryFrom<&'a c::cmsghdr>> {
    pub(crate) buffer: &'a mut [u8],
    pub(crate) length: usize,
    pub(crate) truncated: bool,
    _t: PhantomData<T>,
}

impl<'a, T: TryFrom<&'a c::cmsghdr>> SocketAncillary<'a, T> {
    /// Create an ancillary data with the given buffer.
    pub fn new(buffer: &'a mut [u8]) -> Self {
        SocketAncillary {
            buffer,
            length: 0,
            truncated: false,
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
    }

    /// Returns the iterator of the control messages.
    pub fn messages<'b: 'a>(&'b self) -> Messages<'a, T> {
        Messages {
            buffer: &self.buffer[..self.length],
            current: None,
            _t: Default::default(),
        }
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
        add_to_ancillary_data(
            &mut self.buffer,
            &mut self.length,
            fds,
            c::SOL_SOCKET as _,
            c::SCM_RIGHTS as _,
        )
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
        add_to_ancillary_data(
            &mut self.buffer,
            &mut self.length,
            creds,
            c::SOL_SOCKET as _,
            c::SCM_CREDENTIALS as _,
        )
    }
}

/// TODO: document
pub type RecvSocketAncillaryUnix<'a> = SocketAncillary<'a, RecvAncillaryDataUnix<'a>>;

/// TODO: document
pub type SendSocketAncillaryV4<'a> = SocketAncillary<'a, SendAncillaryDataV4<'a>>;

impl<'a> SocketAncillary<'a, SendAncillaryDataV4<'a>> {
    /// TODO
    pub fn add_packet_info(&mut self, info: &Ipv4PacketInfo) -> bool {
        todo!()
    }
}

/// TODO: document
pub type RecvSocketAncillaryV4<'a> = SocketAncillary<'a, RecvAncillaryDataV4<'a>>;

/// TODO: document
pub type SendSocketAncillaryV6<'a> = SocketAncillary<'a, SendAncillaryDataV6<'a>>;

impl<'a> SocketAncillary<'a, SendAncillaryDataV6<'a>> {
    /// TODO
    pub fn add_packet_info(&mut self, info: &Ipv6PacketInfo) -> bool {
        todo!()
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
pub struct Messages<'a, T: TryFrom<&'a c::cmsghdr>> {
    buffer: &'a [u8],
    current: Option<&'a c::cmsghdr>,
    _t: PhantomData<T>,
}

impl<'a, T: TryFrom<&'a c::cmsghdr>> Iterator for Messages<'a, T> {
    type Item = Result<T, T::Error>;

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

fn add_to_ancillary_data<T>(
    buffer: &mut [u8],
    length: &mut usize,
    source: &[T],
    cmsg_level: c::c_uint,
    cmsg_type: c::c_uint,
) -> bool {
    let source_len = if let Some(source_len) = source.len().checked_mul(size_of::<T>()) {
        if let Ok(source_len) = u32::try_from(source_len) {
            source_len
        } else {
            return false;
        }
    } else {
        return false;
    };

    unsafe {
        let additional_space = c::CMSG_SPACE(source_len as _) as usize;

        let new_length = if let Some(new_length) = additional_space.checked_add(*length) {
            new_length
        } else {
            return false;
        };

        if new_length > buffer.len() {
            return false;
        }

        buffer[*length..new_length].fill(0);

        *length = new_length;

        let mut msg: c::msghdr = zeroed();
        msg.msg_control = buffer.as_mut_ptr().cast();
        msg.msg_controllen = *length as _;

        let mut cmsg = c::CMSG_FIRSTHDR(&msg);
        let mut previous_cmsg = cmsg;
        while !cmsg.is_null() {
            previous_cmsg = cmsg;
            cmsg = c::CMSG_NXTHDR(&msg, cmsg);

            // Most operating systems, but not Linux or emscripten, return the previous pointer
            // when its length is zero. Therefore, check if the previous pointer is the same as
            // the current one.
            if ptr::eq(cmsg, previous_cmsg) {
                break;
            }
        }

        if previous_cmsg.is_null() {
            return false;
        }

        (*previous_cmsg).cmsg_level = cmsg_level as _;
        (*previous_cmsg).cmsg_type = cmsg_type as _;
        (*previous_cmsg).cmsg_len = c::CMSG_LEN(source_len as _) as _;

        let data: *mut T = c::CMSG_DATA(previous_cmsg).cast();

        ptr::copy_nonoverlapping(source.as_ptr().cast(), data, source.len());
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::OwnedFd;

    #[test]
    fn test_cmsg_space() {
        let buf = cmsg_space!([OwnedFd; 2]);
        assert_eq!(
            buf.len(),
            c::CMSG_SPACE(core::mem::size_of::<[OwnedFd; 2]>() as _) as _
        );
    }
}
