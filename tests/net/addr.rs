#[test]
fn encode_decode() {
    #[cfg(unix)]
    use rustix::net::SocketAddrUnix;
    use rustix::net::{Ipv4Addr, Ipv6Addr, SocketAddrAny, SocketAddrV4, SocketAddrV6};

    let orig = SocketAddrV4::new(Ipv4Addr::new(2, 3, 5, 6), 33);
    let encoded = SocketAddrAny::from(orig);
    let decoded = SocketAddrV4::try_from(encoded).unwrap();
    assert_eq!(decoded, orig);

    let orig = SocketAddrV6::new(Ipv6Addr::new(2, 3, 5, 6, 8, 9, 11, 12), 33, 34, 36);
    let encoded = SocketAddrAny::from(orig.clone());
    let decoded = SocketAddrV6::try_from(encoded).unwrap();
    assert_eq!(decoded, orig);

    #[cfg(not(windows))]
    {
        let orig = SocketAddrUnix::new("/path/to/socket").unwrap();
        let encoded = SocketAddrAny::from(orig.clone());
        let decoded = SocketAddrUnix::try_from(encoded).unwrap();
        assert_eq!(decoded, orig);
    }
}

#[cfg(not(windows))]
#[test]
fn test_unix_addr() {
    use rustix::cstr;
    use rustix::net::SocketAddrUnix;
    use std::borrow::Cow;

    assert_eq!(
        SocketAddrUnix::new("/").unwrap().path().unwrap(),
        Cow::from(cstr!("/"))
    );
    assert_eq!(
        SocketAddrUnix::new("//").unwrap().path().unwrap(),
        Cow::from(cstr!("//"))
    );
    assert_eq!(
        SocketAddrUnix::new("/foo/bar").unwrap().path().unwrap(),
        Cow::from(cstr!("/foo/bar"))
    );
    assert_eq!(
        SocketAddrUnix::new("foo").unwrap().path().unwrap(),
        Cow::from(cstr!("foo"))
    );
    SocketAddrUnix::new("/foo\0/bar").unwrap_err();
    assert!(SocketAddrUnix::new("").unwrap().path().is_none());

    #[cfg(linux_kernel)]
    {
        assert!(SocketAddrUnix::new("foo")
            .unwrap()
            .abstract_name()
            .is_none());

        assert_eq!(
            SocketAddrUnix::new_abstract_name(b"test")
                .unwrap()
                .abstract_name()
                .unwrap(),
            b"test"
        );
        assert_eq!(
            SocketAddrUnix::new_abstract_name(b"")
                .unwrap()
                .abstract_name()
                .unwrap(),
            b""
        );
        assert_eq!(
            SocketAddrUnix::new_abstract_name(b"this\0that")
                .unwrap()
                .abstract_name()
                .unwrap(),
            b"this\0that"
        );
        SocketAddrUnix::new_abstract_name(&[b'a'; 500]).unwrap_err();
        assert!(SocketAddrUnix::new_abstract_name(b"test")
            .unwrap()
            .path()
            .is_none());
    }
}
