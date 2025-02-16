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
