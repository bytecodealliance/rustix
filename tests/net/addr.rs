#[test]
fn encode_decode() {
    #[cfg(not(windows))]
    use rustix::net::SocketAddrUnix;
    use rustix::net::{
        Ipv4Addr, Ipv6Addr, SocketAddrAny, SocketAddrStorage, SocketAddrV4, SocketAddrV6,
    };

    unsafe {
        let orig = SocketAddrV4::new(Ipv4Addr::new(2, 3, 5, 6), 33);
        let mut encoded = std::mem::MaybeUninit::<SocketAddrStorage>::uninit();
        let len = SocketAddrAny::V4(orig.clone()).write(encoded.as_mut_ptr());
        let decoded = SocketAddrAny::read(encoded.as_ptr(), len).unwrap();
        assert_eq!(decoded, SocketAddrAny::V4(orig));

        let orig = SocketAddrV6::new(Ipv6Addr::new(2, 3, 5, 6, 8, 9, 11, 12), 33, 34, 36);
        let mut encoded = std::mem::MaybeUninit::<SocketAddrStorage>::uninit();
        let len = SocketAddrAny::V6(orig.clone()).write(encoded.as_mut_ptr());
        let decoded = SocketAddrAny::read(encoded.as_ptr(), len).unwrap();
        assert_eq!(decoded, SocketAddrAny::V6(orig));

        #[cfg(not(windows))]
        {
            let orig = SocketAddrUnix::new("/path/to/socket").unwrap();
            let mut encoded = std::mem::MaybeUninit::<SocketAddrStorage>::uninit();
            let len = SocketAddrAny::Unix(orig.clone()).write(encoded.as_mut_ptr());
            let decoded = SocketAddrAny::read(encoded.as_ptr(), len).unwrap();
            assert_eq!(decoded, SocketAddrAny::Unix(orig));
        }
    }
}
