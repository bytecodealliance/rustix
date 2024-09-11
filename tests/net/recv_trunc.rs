use rustix::net::{AddressFamily, RecvFlags, SendFlags, SocketAddrUnix, SocketType};
use std::mem::MaybeUninit;

/// Test `recv_uninit` with the `RecvFlags::Trunc` flag.
#[test]
fn net_recv_uninit_trunc() {
    crate::init();

    let tmpdir = tempfile::tempdir().unwrap();
    let path = tmpdir.path().join("recv_uninit_trunc");
    let name = SocketAddrUnix::new(&path).unwrap();

    let receiver = rustix::net::socket(AddressFamily::UNIX, SocketType::DGRAM, None).unwrap();
    rustix::net::bind_unix(&receiver, &name).expect("bind");

    let sender = rustix::net::socket(AddressFamily::UNIX, SocketType::DGRAM, None).unwrap();
    let request = b"Hello, World!!!";
    let n = rustix::net::sendto_unix(&sender, request, SendFlags::empty(), &name).expect("send");
    assert_eq!(n, request.len());
    drop(sender);

    let mut response = [MaybeUninit::<u8>::zeroed(); 5];
    let (init, uninit) =
        rustix::net::recv_uninit(&receiver, &mut response, RecvFlags::TRUNC).expect("recv_uninit");

    // We used the `TRUNC` flag, so we should have only gotten 5 bytes.
    assert_eq!(init, b"Hello");
    assert!(uninit.is_empty());
}
