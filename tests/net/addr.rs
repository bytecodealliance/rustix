#[test]
fn encode_decode() {
    use rsix::net::{
        Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrStorage, SocketAddrUnix, SocketAddrV4,
        SocketAddrV6,
    };

    unsafe {
        let orig = SocketAddrV4::new(Ipv4Addr::new(2, 3, 5, 6), 33);
        let mut encoded = std::mem::MaybeUninit::<SocketAddrStorage>::uninit();
        let len = SocketAddr::V4(orig.clone()).write(encoded.as_mut_ptr());
        let decoded = SocketAddr::read(encoded.as_ptr(), len).unwrap();
        assert_eq!(decoded, SocketAddr::V4(orig));

        let orig = SocketAddrV6::new(Ipv6Addr::new(2, 3, 5, 6, 8, 9, 11, 12), 33, 34, 36);
        let mut encoded = std::mem::MaybeUninit::<SocketAddrStorage>::uninit();
        let len = SocketAddr::V6(orig.clone()).write(encoded.as_mut_ptr());
        let decoded = SocketAddr::read(encoded.as_ptr(), len).unwrap();
        assert_eq!(decoded, SocketAddr::V6(orig));

        let orig = SocketAddrUnix::new("/path/to/socket").unwrap();
        let mut encoded = std::mem::MaybeUninit::<SocketAddrStorage>::uninit();
        let len = SocketAddr::Unix(orig.clone()).write(encoded.as_mut_ptr());
        let decoded = SocketAddr::read(encoded.as_ptr(), len).unwrap();
        assert_eq!(decoded, SocketAddr::Unix(orig));
    }
}
