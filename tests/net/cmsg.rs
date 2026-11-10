#[cfg(feature = "pipe")]
#[test]
fn test_empty_buffers() {
    use rustix::fd::AsFd as _;
    use rustix::net::{RecvAncillaryBuffer, SendAncillaryBuffer, SendAncillaryMessage};
    use rustix::pipe::pipe;

    let (_read_end, write_end) = pipe().unwrap();
    let we = [write_end.as_fd()];

    let mut cmsg_buffer = SendAncillaryBuffer::new(&mut []);
    let msg = SendAncillaryMessage::ScmRights(&we);
    assert!(!cmsg_buffer.push(msg));

    let mut cmsg_buffer = SendAncillaryBuffer::default();
    let msg = SendAncillaryMessage::ScmRights(&we);
    assert!(!cmsg_buffer.push(msg));

    let mut cmsg_buffer = RecvAncillaryBuffer::new(&mut []);
    assert!(cmsg_buffer.drain().next().is_none());

    let mut cmsg_buffer = RecvAncillaryBuffer::default();
    assert!(cmsg_buffer.drain().next().is_none());
}

#[test]
fn test_buffer_sizes() {
    use rustix::cmsg_space;

    assert!(cmsg_space!(ScmRights(0)) > 0);
    assert!(cmsg_space!(ScmRights(1)) >= cmsg_space!(ScmRights(0)));
    assert!(cmsg_space!(ScmRights(2)) < cmsg_space!(ScmRights(1), ScmRights(1)));
    assert!(cmsg_space!(ScmRights(1)) * 2 >= cmsg_space!(ScmRights(1), ScmRights(1)));
    assert!(cmsg_space!(ScmRights(1), ScmRights(0)) >= cmsg_space!(ScmRights(1)));
}

/// Test that receiving more `SCM_RIGHTS` file descriptors than fit in the
/// ancillary buffer doesn't panic and doesn't produce descriptors from
/// outside the buffer.
///
/// Linux truncates the control data and adjusts `cmsg_len` to match, but
/// macOS truncates the data and leaves `cmsg_len` holding the untruncated
/// length, so the parsing code has to be prepared for a `cmsg_len` that runs
/// past the end of the buffer.
///
/// Platforms report the truncation with `CTRUNC`, but not all of the
/// environments rustix's CI runs in do, so this doesn't check for it.
#[test]
fn test_truncated_scm_rights() {
    use rustix::fd::{AsFd, OwnedFd};
    use rustix::io::{IoSlice, IoSliceMut};
    use rustix::net::{
        recvmsg, sendmsg, socket, socketpair, AddressFamily, RecvAncillaryBuffer,
        RecvAncillaryMessage, RecvFlags, SendAncillaryBuffer, SendAncillaryMessage, SendFlags,
        SocketFlags, SocketType,
    };
    use std::mem::MaybeUninit;

    /// The number of file descriptors to send.
    const NUM_FDS: usize = 16;
    /// The number of file descriptors the receive buffer has room for.
    const NUM_SLOTS: usize = 5;

    crate::init();

    let (send_sock, recv_sock) = socketpair(
        AddressFamily::UNIX,
        SocketType::STREAM,
        SocketFlags::empty(),
        None,
    )
    .unwrap();

    // Make some file descriptors to send.
    let fds: Vec<OwnedFd> = (0..NUM_FDS)
        .map(|_| socket(AddressFamily::UNIX, SocketType::STREAM, None).unwrap())
        .collect();
    let borrowed: Vec<_> = fds.iter().map(AsFd::as_fd).collect();

    let mut space = [MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(NUM_FDS))];
    let mut cmsg_buffer = SendAncillaryBuffer::new(space.as_mut_slice());
    assert!(cmsg_buffer.push(SendAncillaryMessage::ScmRights(&borrowed)));

    sendmsg(
        &send_sock,
        &[IoSlice::new(b"hello")],
        &mut cmsg_buffer,
        SendFlags::empty(),
    )
    .unwrap();

    // Receive into a buffer with room for only `NUM_SLOTS` file descriptors.
    let mut cmsg_space = [MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(NUM_SLOTS))];
    let mut cmsg_buffer = RecvAncillaryBuffer::new(cmsg_space.as_mut_slice());

    let mut buffer = [0_u8; 5];
    let result = recvmsg(
        &recv_sock,
        &mut [IoSliceMut::new(&mut buffer)],
        &mut cmsg_buffer,
        RecvFlags::empty(),
    )
    .unwrap();

    assert_eq!(result.bytes, 5);
    assert_eq!(&buffer, b"hello");

    // Draining the buffer shouldn't panic.
    let mut received = Vec::new();
    for msg in cmsg_buffer.drain() {
        match msg {
            RecvAncillaryMessage::ScmRights(rights) => received.extend(rights),
            _ => panic!("unexpected ancillary message"),
        }
    }

    // Platforms deliver differing amounts of a truncated control message —
    // Linux fills the buffer, FreeBSD delivers none of it — so don't assume
    // how many descriptors come back. Do require that the message was
    // truncated, and that every descriptor that did come back is one of the
    // sockets that was sent, rather than something read from past the end of
    // the buffer.
    assert!(received.len() < NUM_FDS);
    for fd in &received {
        assert_eq!(
            rustix::net::sockopt::socket_type(fd).unwrap(),
            SocketType::STREAM
        );
    }

    // Dropping the buffer drains it again; that shouldn't panic either.
    drop(cmsg_buffer);
}
