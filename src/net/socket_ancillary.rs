// TODO: remove
#![allow(dead_code, unused_variables)]
#![allow(unsafe_code)]

use core::marker::PhantomData;
use core::mem::{size_of, zeroed};
use core::ops::{Deref, DerefMut};
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
pub enum UnixAncillaryData<'a> {
    /// TODO: document
    ScmRights(ScmRights<'a>),
    /// TODO: document
    #[cfg(any(target_os = "android", target_os = "linux",))]
    ScmCredentials(ScmCredentials<'a>),
    /// Catch-all variant for unimplemented cmsg types.
    #[doc(hidden)]
    Unknown(UnknownCmsgs<'a>),
}

impl<'a> UnixAncillaryData<'a> {
    /// Create an `UnixAncillaryData::ScmRights` variant.
    ///
    /// # Safety
    ///
    /// `data` must contain a valid control message and the control message must be type of
    /// `SOL_SOCKET` and level of `SCM_RIGHTS`.
    unsafe fn as_rights(data: &'a [u8]) -> Self {
        let ancillary_data_iter = AncillaryDataIter::new(data);
        let scm_rights = ScmRights(ancillary_data_iter);
        UnixAncillaryData::ScmRights(scm_rights)
    }

    /// Create an `UnixAncillaryData::ScmCredentials` variant.
    ///
    /// # Safety
    ///
    /// `data` must contain a valid control message and the control message must be type of
    /// `SOL_SOCKET` and level of `SCM_CREDENTIALS` or `SCM_CREDENTIALS`.
    #[cfg(any(doc, target_os = "android", target_os = "linux",))]
    unsafe fn as_credentials(data: &'a [u8]) -> Self {
        let ancillary_data_iter = AncillaryDataIter::new(data);
        let scm_credentials = ScmCredentials(ancillary_data_iter);
        UnixAncillaryData::ScmCredentials(scm_credentials)
    }

    fn try_from_cmsghdr(cmsg: &'a c::cmsghdr) -> Result<Self, AncillaryError> {
        unsafe {
            let cmsg_len_zero = c::CMSG_LEN(0) as usize;
            let data_len = (*cmsg).cmsg_len as usize - cmsg_len_zero;
            let data = c::CMSG_DATA(cmsg).cast();
            let data = slice::from_raw_parts(data, data_len);

            match (*cmsg).cmsg_level as _ {
                c::SOL_SOCKET => match (*cmsg).cmsg_type as _ {
                    c::SCM_RIGHTS => Ok(Self::as_rights(data)),
                    #[cfg(any(target_os = "android", target_os = "linux",))]
                    c::SCM_CREDENTIALS => Ok(Self::as_credentials(data)),
                    cmsg_type => Err(AncillaryError::Unknown {
                        cmsg_level: c::SOL_SOCKET as _,
                        cmsg_type: cmsg_type as _,
                    }),
                },
                cmsg_level => Err(AncillaryError::Unknown {
                    cmsg_level: (*cmsg).cmsg_level,
                    cmsg_type: (*cmsg).cmsg_type,
                }),
            }
        }
    }
}

/// An opaque structure to capture unknown cmsgs.
#[doc(hidden)]
pub struct UnknownCmsgs<'a>(AncillaryDataIter<'a, &'a [u8]>);

impl<'a> Iterator for UnknownCmsgs<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<&'a [u8]> {
        self.0.next()
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
pub enum Ipv4AncillaryData<'a> {
    /// TODO: document
    PacketInfos(Ipv4PacketInfos<'a>),
    /// TODO: document
    #[cfg(target_os = "linux")]
    UdpGsoSegments(UdpGsoSegments<'a>),
    /// TODO: document
    #[cfg(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    ))]
    Ipv4RecvIf(libc::sockaddr_dl),
    /// TODO: document
    #[cfg(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    ))]
    Ipv4RecvDstAddr(libc::in_addr),
    // /// Socket error queue control messages read with the `MSG_ERRQUEUE` flag.
    // #[cfg(any(target_os = "android", target_os = "linux"))]
    // Ipv4RecvErr(libc::sock_extended_err, Option<sockaddr_in>),
    // #[cfg(any(target_os = "android", target_os = "fuchsia", target_os = "linux"))]
    // RxqOvfl(u32),
    /// Catch-all variant for unimplemented cmsg types.
    #[doc(hidden)]
    Unknown(UnknownCmsgs<'a>),
}

/// TODO: document
#[non_exhaustive]
pub enum Ipv6AncillaryData<'a> {
    /// TODO: document
    PacketInfos(Ipv6PacketInfos<'a>),
    /// TODO: document
    #[cfg(target_os = "linux")]
    UdpGsoSegments(UdpGsoSegments<'a>),
    /// Socket error queue control messages read with the `MSG_ERRQUEUE` flag.
    // #[cfg(any(target_os = "android", target_os = "linux"))]
    // Ipv6RecvErr(libc::sock_extended_err, Option<sockaddr_in6>),
    // #[cfg(any(target_os = "android", target_os = "fuchsia", target_os = "linux"))]
    RxqOvfl(u32),
    /// Catch-all variant for unimplemented cmsg types.
    #[doc(hidden)]
    Unknown(UnknownCmsgs<'a>),
}

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
pub struct SocketAncillary<'a> {
    pub(crate) buffer: &'a mut [u8],
    pub(crate) length: usize,
    pub(crate) truncated: bool,
}

impl<'a> SocketAncillary<'a> {
    /// Create an ancillary data with the given buffer.
    pub fn new(buffer: &'a mut [u8]) -> Self {
        SocketAncillary {
            buffer,
            length: 0,
            truncated: false,
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
}

/// TODO: document
#[derive(Debug)]
pub struct UnixSocketAncillary<'a>(SocketAncillary<'a>);

impl<'a> Deref for UnixSocketAncillary<'a> {
    type Target = SocketAncillary<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for UnixSocketAncillary<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> UnixSocketAncillary<'a> {
    /// Create an ancillary data with the given buffer.
    pub fn new(buffer: &'a mut [u8]) -> Self {
        UnixSocketAncillary(SocketAncillary::new(buffer))
    }

    /// Add file descriptors to the ancillary data.
    ///
    /// The function returns `true` if there was enough space in the buffer.
    /// If there was not enough space then no file descriptors was appended.
    /// Technically, that means this operation adds a control message with the level `SOL_SOCKET`
    /// and type `SCM_RIGHTS`.
    pub fn add_fds<Fd: AsFd>(&mut self, fds: &[Fd]) -> bool {
        self.truncated = false;
        add_to_ancillary_data(
            &mut self.0.buffer,
            &mut self.0.length,
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
            &mut self.0.buffer,
            &mut self.0.length,
            creds,
            c::SOL_SOCKET as _,
            c::SCM_CREDENTIALS as _,
        )
    }

    /// Returns the iterator of the control messages.
    pub fn messages(&self) -> UnixMessages<'_> {
        UnixMessages {
            buffer: &self.0.buffer[..self.0.length],
            current: None,
        }
    }
}

/// TODO: document
#[derive(Debug)]
pub struct Ipv4SocketAncillary<'a>(SocketAncillary<'a>);

impl<'a> Deref for Ipv4SocketAncillary<'a> {
    type Target = SocketAncillary<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Ipv4SocketAncillary<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> Ipv4SocketAncillary<'a> {
    /// Create an ancillary data with the given buffer.
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Ipv4SocketAncillary(SocketAncillary::new(buffer))
    }

    /// TODO
    pub fn add_packet_info(&mut self, info: &Ipv4PacketInfo) -> bool {
        todo!()
    }
}

/// TODO: document
#[derive(Debug)]
pub struct Ipv6SocketAncillary<'a>(SocketAncillary<'a>);

impl<'a> Deref for Ipv6SocketAncillary<'a> {
    type Target = SocketAncillary<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Ipv6SocketAncillary<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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

/// This struct is used to iterate through the control messages.
pub struct UnixMessages<'a> {
    buffer: &'a [u8],
    current: Option<&'a c::cmsghdr>,
}

impl<'a> Iterator for UnixMessages<'a> {
    type Item = Result<UnixAncillaryData<'a>, AncillaryError>;

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
            let ancillary_result = UnixAncillaryData::try_from_cmsghdr(cmsg);
            Some(ancillary_result)
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
    use core::convert::TryFrom;
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
