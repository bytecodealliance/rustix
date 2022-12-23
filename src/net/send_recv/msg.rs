//! `recvmsg` and `sendmsg` functions.

use crate::backend::{self, c};
use crate::fd::{AsFd, BorrowedFd, OwnedFd};
use crate::io;

use core::convert::TryFrom;
use core::marker::PhantomData;
use core::mem::size_of;
use core::ptr::NonNull;

use super::{RecvFlags, SendFlags, SocketAddrAny, SocketAddrV4, SocketAddrV6};

/// Ancillary message for `sendmsg`.
#[non_exhaustive]
pub enum SendAncillaryMessage<'slice, 'fd> {
    /// Send one or more file descriptors.
    ScmRights(&'slice [BorrowedFd<'fd>]),
}

impl SendAncillaryMessage<'_, '_> {
    /// Get the maximum size of an ancillary message.
    ///
    /// This can be helpful in determining the size of the buffer you allocate.
    #[allow(unsafe_code)]
    pub fn size(&self) -> usize {
        let total_bytes = match self {
            Self::ScmRights(slice) => slice.len() * size_of::<BorrowedFd<'static>>(),
        };

        unsafe { c::CMSG_LEN(total_bytes as _) as usize }
    }
}

/// Ancillary message for `recvmsg`.
#[non_exhaustive]
pub enum RecvAncillaryMessage {
    /// Received one or more file descriptors.
    ScmRights(Vec<OwnedFd>),
}

/// Buffer for sending ancillary messages.
pub struct SendAncillaryBuffer<'buf, 'slice, 'fd> {
    /// Raw byte buffer for messages.
    buffer: &'buf mut [u8],

    /// The amount of the buffer that is used.
    length: usize,

    /// Phantom data for lifetime of `&'slice [BorrowedFd<'fd>]`.
    _phantom: PhantomData<&'slice [BorrowedFd<'fd>]>,
}

impl<'buf> From<&'buf mut [u8]> for SendAncillaryBuffer<'buf, '_, '_> {
    fn from(buffer: &'buf mut [u8]) -> Self {
        Self::new(buffer)
    }
}

impl<'buf, 'slice, 'fd> SendAncillaryBuffer<'buf, 'slice, 'fd> {
    /// Create a new, empty `SendAncillaryBuffer` from a raw byte buffer.
    pub fn new(buffer: &'buf mut [u8]) -> Self {
        Self {
            buffer,
            length: 0,
            _phantom: PhantomData,
        }
    }

    /// Returns a pointer to the message data.
    pub(crate) fn as_control_ptr(&mut self) -> *mut u8 {
        if self.length > 0 {
            core::ptr::null_mut()
        } else {
            self.buffer.as_mut_ptr()
        }
    }

    /// Returns the length of the message data.
    pub(crate) fn control_len(&self) -> usize {
        self.length
    }

    /// Delete all messages from the buffer.
    pub fn clear(&mut self) {
        self.length = 0;
    }

    /// Add an ancillary message to the buffer.
    ///
    /// Returns `true` if the message was added successfully.
    #[allow(unsafe_code)]
    pub fn push(&mut self, msg: SendAncillaryMessage<'slice, 'fd>) -> bool {
        match msg {
            SendAncillaryMessage::ScmRights(fds) => {
                let fds_bytes = unsafe {
                    core::slice::from_raw_parts(
                        fds.as_ptr() as *const u8,
                        fds.len() * core::mem::size_of::<c::c_int>(),
                    )
                };
                self.push_ancillary(fds_bytes, c::SOL_SOCKET as _, c::SCM_RIGHTS as _)
            }
        }
    }

    /// Pushes an ancillary message to the buffer.
    #[allow(unsafe_code)]
    fn push_ancillary(&mut self, source: &[u8], cmsg_level: c::c_int, cmsg_type: c::c_int) -> bool {
        macro_rules! leap {
            ($e:expr) => {{
                match ($e) {
                    Some(x) => x,
                    None => return false,
                }
            }};
        }

        // Calculate the length of the message.
        let source_len = leap!(u32::try_from(source.len()).ok());

        // Calculate the new length of the buffer.
        let additional_space = unsafe { c::CMSG_SPACE(source_len) };
        let new_length = leap!(self.length.checked_add(additional_space as usize));
        if new_length > self.buffer.len() {
            return false;
        }

        // Fill the new part of the buffer with zeroes.
        self.buffer[self.length..new_length].fill(0);
        self.length = new_length;

        // Get the last header in the buffer.
        let mut last_header = leap!(messages::Messages::new(self.buffer, self.length).last());

        // Set the header fields.
        unsafe {
            let last_header = last_header.as_mut();
            last_header.cmsg_len = c::CMSG_LEN(source_len) as _;
            last_header.cmsg_level = cmsg_level;
            last_header.cmsg_type = cmsg_type;
        }

        // Get the pointer to the payload and copy the data.
        unsafe {
            let payload = c::CMSG_DATA(last_header.as_ptr());
            core::ptr::copy_nonoverlapping(source.as_ptr(), payload, source_len as _);
        }

        true
    }
}

impl<'slice, 'fd> Extend<SendAncillaryMessage<'slice, 'fd>>
    for SendAncillaryBuffer<'_, 'slice, 'fd>
{
    fn extend<T: IntoIterator<Item = SendAncillaryMessage<'slice, 'fd>>>(&mut self, iter: T) {
        // TODO: This could be optimized to add every message in one go.
        iter.into_iter().all(|msg| self.push(msg));
    }
}

/// Buffer for receiving ancillary messages.
pub struct RecvAncillaryBuffer<'buf> {
    /// Raw byte buffer for messages.
    buffer: &'buf mut [u8],

    /// The portion of the buffer we've read from already.
    read: usize,

    /// The amount of the buffer that is used.
    length: usize,
}

impl<'buf> From<&'buf mut [u8]> for RecvAncillaryBuffer<'buf> {
    fn from(buffer: &'buf mut [u8]) -> Self {
        Self::new(buffer)
    }
}

impl<'buf> RecvAncillaryBuffer<'buf> {
    /// Create a new, empty `RecvAncillaryBuffer` from a raw byte buffer.
    pub fn new(buffer: &'buf mut [u8]) -> Self {
        Self {
            buffer,
            read: 0,
            length: 0,
        }
    }

    /// Returns a pointer to the message data.
    pub(crate) fn as_control_ptr(&mut self) -> *mut u8 {
        self.buffer.as_mut_ptr()
    }

    /// Returns the length of the message data.
    pub(crate) fn control_len(&self) -> usize {
        self.length
    }

    /// Drain all messages from the buffer.
    #[allow(unsafe_code)]
    pub fn drain(&mut self) -> AncillaryDrain<'_> {
        AncillaryDrain {
            messages: messages::Messages::new(
                &mut self.buffer[self.read..self.read + self.length],
                self.length,
            ),
            read: &mut self.read,
        }
    }
}

impl Drop for RecvAncillaryBuffer<'_> {
    fn drop(&mut self) {
        self.drain().for_each(drop);
    }
}

/// An iterator that drains messages from a `RecvAncillaryBuffer`.
pub struct AncillaryDrain<'buf> {
    /// Inner iterator over messages.
    messages: messages::Messages<'buf>,

    /// Increment the number of messages we've read.
    read: &'buf mut usize,
}

impl AncillaryDrain<'_> {
    /// A closure that converts a message into a `RecvAncillaryMessage`.
    #[allow(unsafe_code)]
    fn cvt_msg(read: &mut usize, msg: NonNull<c::cmsghdr>) -> Option<RecvAncillaryMessage> {
        unsafe {
            let msg = msg.as_ref();

            // Advance the "read" pointer.
            let msg_len = msg.cmsg_len as usize;
            *read += msg_len;

            // Get a pointer to the payload.
            let payload = c::CMSG_DATA(msg as *const _ as *const _);
            let payload_len = msg.cmsg_len as usize - c::CMSG_LEN(0) as usize;

            // Determine what type it is.
            let (level, msg_type) = (msg.cmsg_level, msg.cmsg_type);
            match (level as _, msg_type as _) {
                (c::SOL_SOCKET, c::SCM_RIGHTS) => {
                    // Create a buffer for FDs and copy the data.
                    let fd_size = core::mem::size_of::<OwnedFd>();
                    let mut fds = Vec::with_capacity(payload_len / fd_size);

                    core::ptr::copy_nonoverlapping(
                        payload as *const u8,
                        fds.as_mut_ptr() as *mut _,
                        payload_len,
                    );

                    Some(RecvAncillaryMessage::ScmRights(fds))
                }
                _ => None,
            }
        }
    }
}

impl Iterator for AncillaryDrain<'_> {
    type Item = RecvAncillaryMessage;

    fn next(&mut self) -> Option<Self::Item> {
        let read = &mut self.read;
        self.messages.find_map(|ev| Self::cvt_msg(read, ev))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, max) = self.messages.size_hint();
        (0, max)
    }

    fn fold<B, F>(mut self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        let read = &mut self.read;
        self.messages
            .filter_map(|ev| Self::cvt_msg(read, ev))
            .fold(init, f)
    }

    fn count(mut self) -> usize {
        let read = &mut self.read;
        self.messages
            .filter_map(|ev| Self::cvt_msg(read, ev))
            .count()
    }

    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        let read = &mut self.read;
        self.messages
            .filter_map(|ev| Self::cvt_msg(read, ev))
            .last()
    }

    fn collect<B: core::iter::FromIterator<Self::Item>>(mut self) -> B
    where
        Self: Sized,
    {
        let read = &mut self.read;
        self.messages
            .filter_map(|ev| Self::cvt_msg(read, ev))
            .collect()
    }
}

impl core::iter::FusedIterator for AncillaryDrain<'_> {}

/// `sendmsg(msghdr)`- Sends a message on a socket.
pub fn sendmsg_noaddr(
    socket: impl AsFd,
    iov: &[io::IoSlice<'_>],
    control: &mut SendAncillaryBuffer<'_, '_, '_>,
    flags: SendFlags,
) -> io::Result<usize> {
    backend::net::syscalls::sendmsg_noaddr(socket.as_fd(), iov, control, flags)
}

/// `sendmsg(msghdr)`- Sends a message on a socket.
pub fn sendmsg_v4(
    socket: impl AsFd,
    addr: &SocketAddrV4,
    iov: &[io::IoSlice<'_>],
    control: &mut SendAncillaryBuffer<'_, '_, '_>,
    flags: SendFlags,
) -> io::Result<usize> {
    backend::net::syscalls::sendmsg_v4(socket.as_fd(), addr, iov, control, flags)
}

/// `sendmsg(msghdr)`- Sends a message on a socket.
pub fn sendmsg_v6(
    socket: impl AsFd,
    addr: &SocketAddrV6,
    iov: &[io::IoSlice<'_>],
    control: &mut SendAncillaryBuffer<'_, '_, '_>,
    flags: SendFlags,
) -> io::Result<usize> {
    backend::net::syscalls::sendmsg_v6(socket.as_fd(), addr, iov, control, flags)
}

/// `sendmsg(msghdr)`- Sends a message on a socket.
#[cfg(unix)]
pub fn sendmsg_unix(
    socket: impl AsFd,
    addr: &super::SocketAddrUnix,
    iov: &[io::IoSlice<'_>],
    control: &mut SendAncillaryBuffer<'_, '_, '_>,
    flags: SendFlags,
) -> io::Result<usize> {
    backend::net::syscalls::sendmsg_unix(socket.as_fd(), addr, iov, control, flags)
}

/// `sendmsg(msghdr)`- Sends a message on a socket.
pub fn sendmsg_any(
    socket: impl AsFd,
    addr: Option<&SocketAddrAny>,
    iov: &[io::IoSlice<'_>],
    control: &mut SendAncillaryBuffer<'_, '_, '_>,
    flags: SendFlags,
) -> io::Result<usize> {
    match addr {
        None => backend::net::syscalls::sendmsg_noaddr(socket.as_fd(), iov, control, flags),
        Some(SocketAddrAny::V4(addr)) => {
            backend::net::syscalls::sendmsg_v4(socket.as_fd(), addr, iov, control, flags)
        }
        Some(SocketAddrAny::V6(addr)) => {
            backend::net::syscalls::sendmsg_v6(socket.as_fd(), addr, iov, control, flags)
        }
        #[cfg(unix)]
        Some(SocketAddrAny::Unix(addr)) => {
            backend::net::syscalls::sendmsg_unix(socket.as_fd(), addr, iov, control, flags)
        }
    }
}

/// `recvmsg(msghdr)`- Receives a message from a socket.
pub fn recvmsg(
    socket: impl AsFd,
    iov: &mut [io::IoSliceMut<'_>],
    control: &mut RecvAncillaryBuffer<'_>,
    flags: RecvFlags,
) -> io::Result<RecvMsgResult> {
    backend::net::syscalls::recvmsg(socket.as_fd(), iov, control, flags)
}

/// The result of a `recvmsg` call.
#[non_exhaustive]
pub struct RecvMsgResult {
    /// The number of bytes received.
    pub bytes: usize,

    /// The flags received.
    pub flags: RecvFlags,

    /// The address of the socket we received from, if any.
    pub address: Option<SocketAddrAny>,
}

#[allow(unsafe_code)]
mod messages {
    use crate::backend::c;
    use core::marker::PhantomData;
    use core::ptr::NonNull;

    /// An iterator over the messages in an ancillary buffer.
    pub(super) struct Messages<'buf> {
        /// The message header we're using to iterator over the messages.
        msghdr: c::msghdr,

        /// The current pointer to the next message header to return.
        ///
        /// This has a lifetime of `'buf`.
        header: Option<NonNull<c::cmsghdr>>,

        /// Capture the original lifetime of the buffer.
        _buffer: PhantomData<&'buf mut [u8]>,
    }

    impl<'buf> Messages<'buf> {
        /// Create a new iterator over messages from a byte buffer.
        pub(super) fn new(buf: &'buf mut [u8], len: usize) -> Self {
            let msghdr = c::msghdr {
                msg_control: buf.as_mut_ptr() as *mut _,
                msg_controllen: len as _,
                ..unsafe { core::mem::zeroed() }
            };

            // Get the first header.
            let header = NonNull::new(unsafe { c::CMSG_FIRSTHDR(&msghdr) });

            Self {
                msghdr,
                header,
                _buffer: PhantomData,
            }
        }
    }

    impl Iterator for Messages<'_> {
        type Item = NonNull<c::cmsghdr>;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            // Get the current header.
            let header = self.header?;

            // Get the next header.
            self.header = NonNull::new(unsafe { c::CMSG_NXTHDR(&self.msghdr, header.as_ptr()) });

            // If the headers are equal, we're done.
            if Some(header) == self.header {
                self.header = None;
            }

            Some(header)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            if self.header.is_some() {
                // The remaining buffer *could* be filled with zero-length messages.
                let max_size = unsafe { c::CMSG_LEN(0) } as usize;
                let remaining_count = self.msghdr.msg_controllen as usize / max_size;
                (1, Some(remaining_count))
            } else {
                (0, Some(0))
            }
        }
    }

    impl core::iter::FusedIterator for Messages<'_> {}
}
