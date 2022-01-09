// TODO: remove
#![allow(dead_code, unused_variables)]

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

use crate::imp::c;
use crate::imp::fd::{AsRawFd, RawFd};

/// TODO: document
#[non_exhaustive]
pub enum UnixSendAncillaryData<'a> {
    /// TODO: document
    ScmRights(ScmRights<'a>),
    /// TODO: document
    #[cfg(any(target_os = "android", target_os = "linux",))]
    ScmCredentials(ScmCredentials<'a>),
}

/// TODO: document
#[repr(transparent)]
pub struct ScmRights<'a>(AncillaryDataIter<'a, RawFd>);

/// TODO: document
#[repr(transparent)]
pub struct ScmCredentials<'a>(AncillaryDataIter<'a, c::ucred>);

/// TODO: document
#[non_exhaustive]
pub enum Ipv4SendAncillaryData<'a> {
    /// TODO: document
    PacketInfos(Ipv4PacketInfos<'a>),
    /// TODO: document
    UdpGsoSegments(UdpGsoSegments<'a>),
}

/// TODO: document
#[non_exhaustive]
pub enum Ipv6SendAncillaryData<'a> {
    /// TODO: document
    PacketInfos(Ipv6PacketInfos<'a>),
    /// TODO: document
    UdpGsoSegments(UdpGsoSegments<'a>),
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
pub struct UdpGsoSegments<'a>(AncillaryDataIter<'a, u16>);

/// Unix credential.
#[cfg(any(target_os = "android", target_os = "linux",))]
#[derive(Copy, Clone)]
pub struct SocketCred(c::ucred);

/// TODO: document
pub struct AncillaryDataIter<'a, T> {
    data: &'a [u8],
    _t: PhantomData<T>,
}

/// A type-safe zero-copy wrapper around a list of [`SendControlMessage`s].
pub struct SendControlMessages<'a> {
    bytes: &'a [u8],
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
    buffer: &'a mut [u8],
    length: usize,
    truncated: bool,
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

    /// Returns the iterator of the control messages.
    pub fn messages(&self) -> Messages<'_> {
        Messages {
            buffer: &self.buffer[..self.length],
            current: None,
        }
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
    pub fn add_fds<Fd: AsRawFd>(&mut self, fds: &[Fd]) -> bool {
        self.truncated = false;
        /*add_to_ancillary_data(
                &mut self.buffer,
                &mut self.length,
                fds,
                c::SOL_SOCKET,
                c::SCM_RIGHTS,
        )*/
        todo!()
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
        /*add_to_ancillary_data(
                &mut self.buffer,
                &mut self.length,
                creds,
                c::SOL_SOCKET,
                c::SCM_CREDENTIALS,
        )*/
        todo!()
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
    pub fn add_packet_info<Fd: AsRawFd>(&mut self, info: &Ipv4PacketInfo) -> bool {
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

/// This struct is used to iterate through the control messages.
pub struct Messages<'a> {
    buffer: &'a [u8],
    current: Option<&'a c::cmsghdr>,
}
