#[cfg(not(target_os = "redox"))]
use rustix::io::IoSliceMut;
#[cfg(not(target_os = "redox"))]
use rustix::net::ReturnFlags;
use rustix::net::{AddressFamily, RecvFlags, SendFlags, SocketAddrUnix, SocketType};
use std::mem::MaybeUninit;

/// Test `recv` with `&mut [MaybeUninit<u8>]` with the `RecvFlags::Trunc` flag.
#[test]
fn net_recv_uninit_trunc() {
    crate::init();

    let tmpdir = tempfile::tempdir().unwrap();
    let path = tmpdir.path().join("recv_uninit_trunc");
    let name = SocketAddrUnix::new(&path).unwrap();

    let receiver = rustix::net::socket(AddressFamily::UNIX, SocketType::DGRAM, None).unwrap();
    rustix::net::bind(&receiver, &name).expect("bind");

    let sender = rustix::net::socket(AddressFamily::UNIX, SocketType::DGRAM, None).unwrap();
    let request = b"Hello, World!!!";
    let n = rustix::net::sendto(&sender, request, SendFlags::empty(), &name).expect("send");
    assert_eq!(n, request.len());

    // Test with `RecvFlags::TRUNC`, which is not supported on Apple, illumos, or
    // NetBSD.
    #[cfg(not(any(apple, solarish, target_os = "netbsd")))]
    {
        let mut response = [MaybeUninit::<u8>::zeroed(); 5];
        let ((init, uninit), length) =
            rustix::net::recv(&receiver, &mut response, RecvFlags::TRUNC).expect("recv_uninit");

        // We used the `TRUNC` flag, so we should have only gotten 5 bytes.
        assert_eq!(init, b"Hello");
        assert!(uninit.is_empty());

        // Send the message again.
        let n = rustix::net::sendto(&sender, request, SendFlags::empty(), &name).expect("send");
        assert_eq!(n, request.len());

        // Check the `length`.
        assert_eq!(length, 15);
    }

    // This time receive it without `TRUNC`. This should fail.
    let mut response = [MaybeUninit::<u8>::zeroed(); 5];
    let ((init, uninit), length) =
        rustix::net::recv(&receiver, &mut response, RecvFlags::empty()).expect("recv_uninit");

    // We didn't use the `TRUNC` flag, so we should have received 15 bytes,
    // truncated to 5 bytes.
    assert_eq!(init, b"Hello");
    assert!(uninit.is_empty());
    assert_eq!(length, 5);
}

/// Test `recvmsg` with the `RecvFlags::Trunc` flag.
#[cfg(not(target_os = "redox"))]
#[test]
fn net_recvmsg_trunc() {
    crate::init();

    let tmpdir = tempfile::tempdir().unwrap();
    let path = tmpdir.path().join("recvmsg_trunc");
    let name = SocketAddrUnix::new(&path).unwrap();

    let receiver = rustix::net::socket(AddressFamily::UNIX, SocketType::DGRAM, None).unwrap();
    rustix::net::bind(&receiver, &name).expect("bind");

    let sender = rustix::net::socket(AddressFamily::UNIX, SocketType::DGRAM, None).unwrap();
    let request = b"Hello, World!!!";

    // Test with `RecvFlags::TRUNC`, which is not supported on Apple, illumos, or
    // NetBSD.
    #[cfg(not(any(apple, solarish, target_os = "netbsd")))]
    {
        let n = rustix::net::sendto(&sender, request, SendFlags::empty(), &name).expect("send");
        assert_eq!(n, request.len());

        let mut response = [0_u8; 5];
        let result = rustix::net::recvmsg(
            &receiver,
            &mut [IoSliceMut::new(&mut response)],
            &mut Default::default(),
            RecvFlags::TRUNC,
        )
        .expect("recvmsg");

        // We used the `TRUNC` flag, so we should have received 15 bytes,
        // truncated to 5 bytes, and the `TRUNC` flag should have been returned.
        assert_eq!(&response, b"Hello");
        assert_eq!(result.bytes, 15);
        assert_eq!(result.flags, ReturnFlags::TRUNC);

        // Send the message again.
        let n = rustix::net::sendto(&sender, request, SendFlags::empty(), &name).expect("send");
        assert_eq!(n, request.len());

        // This time receive it with `TRUNC` and a big enough buffer.
        let mut response = [0_u8; 30];
        let result = rustix::net::recvmsg(
            &receiver,
            &mut [IoSliceMut::new(&mut response)],
            &mut Default::default(),
            RecvFlags::TRUNC,
        )
        .expect("recvmsg");

        // We used the `TRUNC` flag, so we should have received 15 bytes
        // and the buffer was big enough so the `TRUNC` flag should not have
        // been returned.
        assert_eq!(&response[..result.bytes], request);
        assert_eq!(result.flags, ReturnFlags::empty());
    }

    // Send the message again.
    let n = rustix::net::sendto(&sender, request, SendFlags::empty(), &name).expect("send");
    assert_eq!(n, request.len());

    // This time receive it without `TRUNC` but a big enough buffer.
    let mut response = [0_u8; 30];
    let result = rustix::net::recvmsg(
        &receiver,
        &mut [IoSliceMut::new(&mut response)],
        &mut Default::default(),
        RecvFlags::empty(),
    )
    .expect("recvmsg");

    // We didn't use the `TRUNC` flag, but the buffer was big enough, so we
    // should have received 15 bytes, and the `TRUNC` flag should have been
    // returned.
    assert_eq!(&response[..result.bytes], request);
    assert_eq!(result.flags, ReturnFlags::empty());

    // Send the message again.
    let n = rustix::net::sendto(&sender, request, SendFlags::empty(), &name).expect("send");
    assert_eq!(n, request.len());

    // This time receive it without `TRUNC` and a small buffer.
    let mut response = [0_u8; 5];
    let result = rustix::net::recvmsg(
        &receiver,
        &mut [IoSliceMut::new(&mut response)],
        &mut Default::default(),
        RecvFlags::empty(),
    )
    .expect("recvmsg");

    // We didn't use the `TRUNC` flag, so we should have received 15 bytes,
    // truncated to 5 bytes, and the `TRUNC` flag should have been returned.
    assert_eq!(&response[..result.bytes], b"Hello");
    assert_eq!(result.flags, ReturnFlags::TRUNC);
}
