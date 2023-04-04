#[test]
fn test_sockopts_ipv4() {
    use rustix::net::{AddressFamily, Protocol, SocketType};
    use std::time::Duration;

    let s =
        rustix::net::socket(AddressFamily::INET, SocketType::STREAM, Protocol::default()).unwrap();

    // On a new socket we shouldn't have a timeout yet.
    assert!(
        rustix::net::sockopt::get_socket_timeout(&s, rustix::net::sockopt::Timeout::Recv)
            .unwrap()
            .is_none()
    );
    assert_eq!(
        rustix::net::sockopt::get_socket_type(&s).unwrap(),
        SocketType::STREAM
    );
    #[cfg(not(windows))]
    assert!(!rustix::net::sockopt::get_socket_broadcast(&s).unwrap());
    // On a new socket we shouldn't have a linger yet.
    assert!(rustix::net::sockopt::get_socket_linger(&s)
        .unwrap()
        .is_none());
    #[cfg(any(target_os = "android", target_os = "linux"))]
    assert!(!rustix::net::sockopt::get_socket_passcred(&s).unwrap());
    assert_ne!(rustix::net::sockopt::get_ip_ttl(&s).unwrap(), 0);
    assert_ne!(rustix::net::sockopt::get_ip_ttl(&s).unwrap(), 77);
    #[cfg(not(any(bsd, windows)))]
    assert!(rustix::net::sockopt::get_ip_multicast_loop(&s).unwrap());
    #[cfg(not(any(bsd, windows)))]
    assert_eq!(rustix::net::sockopt::get_ip_multicast_ttl(&s).unwrap(), 1);
    assert!(!rustix::net::sockopt::get_tcp_nodelay(&s).unwrap());
    // On a new socket we shouldn't have an error yet.
    assert_eq!(rustix::net::sockopt::get_socket_error(&s).unwrap(), Ok(()));
    assert!(!rustix::net::sockopt::get_socket_keepalive(&s).unwrap());
    assert_ne!(
        rustix::net::sockopt::get_socket_recv_buffer_size(&s).unwrap(),
        0
    );
    assert_ne!(
        rustix::net::sockopt::get_socket_send_buffer_size(&s).unwrap(),
        0
    );

    // Set a timeout.
    rustix::net::sockopt::set_socket_timeout(
        &s,
        rustix::net::sockopt::Timeout::Recv,
        Some(Duration::new(1, 1)),
    )
    .unwrap();

    // Check that we have a timeout of at least the time we set.
    if cfg!(not(target_os = "freebsd")) {
        assert!(
            rustix::net::sockopt::get_socket_timeout(&s, rustix::net::sockopt::Timeout::Recv)
                .unwrap()
                .unwrap()
                >= Duration::new(1, 1)
        );
    } else {
        // On FreeBSD <= 12, it appears the system rounds the timeout down.
        assert!(
            rustix::net::sockopt::get_socket_timeout(&s, rustix::net::sockopt::Timeout::Recv)
                .unwrap()
                .unwrap()
                >= Duration::new(1, 0)
        );
    }

    #[cfg(not(windows))]
    {
        // Set the broadcast flag;
        rustix::net::sockopt::set_socket_broadcast(&s, true).unwrap();

        // Check that the broadcast flag is set. This has no effect on stream
        // sockets, and not all platforms even remember the value.
        #[cfg(not(bsd))]
        assert!(rustix::net::sockopt::get_socket_broadcast(&s).unwrap());
    }

    // Set the keepalive flag;
    rustix::net::sockopt::set_socket_keepalive(&s, true).unwrap();

    // Check that the keepalive flag is set.
    assert!(rustix::net::sockopt::get_socket_keepalive(&s).unwrap());

    // Set a linger.
    rustix::net::sockopt::set_socket_linger(&s, Some(Duration::new(1, 1))).unwrap();

    // Check that we have a linger of at least the time we set.
    assert!(
        dbg!(rustix::net::sockopt::get_socket_linger(&s)
            .unwrap()
            .unwrap())
            >= Duration::new(1, 1)
    );

    #[cfg(any(target_os = "android", target_os = "linux"))]
    {
        // Set the passcred flag;
        rustix::net::sockopt::set_socket_passcred(&s, true).unwrap();

        // Check that the passcred flag is set.
        assert!(rustix::net::sockopt::get_socket_passcred(&s).unwrap());
    }

    // Set the ip ttl.
    rustix::net::sockopt::set_ip_ttl(&s, 77).unwrap();

    // Check the ip ttl.
    assert_eq!(rustix::net::sockopt::get_ip_ttl(&s).unwrap(), 77);

    #[cfg(not(any(bsd, windows)))]
    {
        // Set the multicast loop flag;
        rustix::net::sockopt::set_ip_multicast_loop(&s, false).unwrap();

        // Check that the multicast loop flag is set.
        assert!(!rustix::net::sockopt::get_ip_multicast_loop(&s).unwrap());
    }

    // Set the nodelay flag;
    rustix::net::sockopt::set_tcp_nodelay(&s, true).unwrap();

    // Check that the nodelay flag is set.
    assert!(rustix::net::sockopt::get_tcp_nodelay(&s).unwrap());

    // Set the receive buffer size.
    let size = rustix::net::sockopt::get_socket_recv_buffer_size(&s).unwrap();
    rustix::net::sockopt::set_socket_recv_buffer_size(&s, size * 2).unwrap();

    // Check that the receive buffer size is set.
    assert!(rustix::net::sockopt::get_socket_recv_buffer_size(&s).unwrap() >= size * 2);

    // Set the send buffer size.
    let size = rustix::net::sockopt::get_socket_send_buffer_size(&s).unwrap();
    rustix::net::sockopt::set_socket_send_buffer_size(&s, size * 4).unwrap();

    // Check that the send buffer size is set.
    assert!(rustix::net::sockopt::get_socket_send_buffer_size(&s).unwrap() >= size * 4);
}

#[test]
fn test_sockopts_ipv6() {
    use rustix::net::{AddressFamily, Protocol, SocketType};

    let s = rustix::net::socket(
        AddressFamily::INET6,
        SocketType::STREAM,
        Protocol::default(),
    )
    .unwrap();

    assert_ne!(rustix::net::sockopt::get_ipv6_unicast_hops(&s).unwrap(), 0);
    assert!(rustix::net::sockopt::get_ipv6_multicast_loop(&s).unwrap());
    assert_ne!(rustix::net::sockopt::get_ipv6_unicast_hops(&s).unwrap(), 0);
    assert_eq!(
        rustix::net::sockopt::get_ipv6_multicast_hops(&s).unwrap(),
        0
    );

    // Set the IPV4 V6OONLY value.
    let v6only = rustix::net::sockopt::get_ipv6_v6only(&s).unwrap();
    rustix::net::sockopt::set_ipv6_v6only(&s, !v6only).unwrap();

    // Check that the IPV6 V6ONLY value is set.
    assert_eq!(rustix::net::sockopt::get_ipv6_v6only(&s).unwrap(), !v6only);

    // Set the IPV6 multicast loop value.
    rustix::net::sockopt::set_ipv6_multicast_loop(&s, false).unwrap();

    // Check that the IPV6 multicast loop value is set.
    assert!(!rustix::net::sockopt::get_ipv6_multicast_loop(&s).unwrap());

    // Set the IPV6 unicast hops value to the default value.
    rustix::net::sockopt::set_ipv6_unicast_hops(&s, None).unwrap();

    // Check that the IPV6 unicast hops value is set.
    assert_ne!(rustix::net::sockopt::get_ipv6_unicast_hops(&s).unwrap(), 0);

    // Set the IPV6 unicast hops value to a specific value.
    rustix::net::sockopt::set_ipv6_unicast_hops(&s, Some(8)).unwrap();

    // Check that the IPV6 unicast hops value is set.
    assert_eq!(rustix::net::sockopt::get_ipv6_unicast_hops(&s).unwrap(), 8);
}
