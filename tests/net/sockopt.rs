use rustix::fd::OwnedFd;
use rustix::net::sockopt;
use rustix::net::{AddressFamily, SocketType};
use std::time::Duration;

// Test `socket` socket options.
fn test_sockopts_socket(s: &OwnedFd) {
    // On a new socket we shouldn't have a timeout yet.
    assert!(sockopt::get_socket_timeout(&s, sockopt::Timeout::Recv)
        .unwrap()
        .is_none());
    assert_eq!(sockopt::get_socket_type(&s).unwrap(), SocketType::STREAM);
    assert!(!sockopt::get_socket_reuseaddr(&s).unwrap());
    #[cfg(not(windows))]
    assert!(!sockopt::get_socket_broadcast(&s).unwrap());
    // On a new socket we shouldn't have a linger yet.
    assert!(sockopt::get_socket_linger(&s).unwrap().is_none());
    #[cfg(linux_kernel)]
    assert!(!sockopt::get_socket_passcred(&s).unwrap());

    // On a new socket we shouldn't have an error yet.
    assert_eq!(sockopt::get_socket_error(&s).unwrap(), Ok(()));
    assert!(!sockopt::get_socket_keepalive(&s).unwrap());
    assert_ne!(sockopt::get_socket_recv_buffer_size(&s).unwrap(), 0);
    assert_ne!(sockopt::get_socket_send_buffer_size(&s).unwrap(), 0);

    #[cfg(not(apple))]
    assert!(!sockopt::get_socket_acceptconn(&s).unwrap());

    // Set a timeout.
    sockopt::set_socket_timeout(&s, sockopt::Timeout::Recv, Some(Duration::new(1, 1))).unwrap();

    // Check that we have a timeout of at least the time we set.
    if cfg!(not(any(target_os = "freebsd", target_os = "netbsd"))) {
        assert!(
            sockopt::get_socket_timeout(&s, sockopt::Timeout::Recv)
                .unwrap()
                .unwrap()
                >= Duration::new(1, 1)
        );
    } else {
        // On FreeBSD <= 12 and NetBSD, it appears the system rounds the timeout down.
        assert!(
            sockopt::get_socket_timeout(&s, sockopt::Timeout::Recv)
                .unwrap()
                .unwrap()
                >= Duration::new(1, 0)
        );
    }

    // Set a timeout with more than a million nanoseconds.
    sockopt::set_socket_timeout(&s, sockopt::Timeout::Recv, Some(Duration::new(1, 10000000)))
        .unwrap();

    // Check that we have a timeout of at least the time we set.
    assert!(
        sockopt::get_socket_timeout(&s, sockopt::Timeout::Recv)
            .unwrap()
            .unwrap()
            >= Duration::new(1, 10000000)
    );

    // Set the reuse address flag
    sockopt::set_socket_reuseaddr(&s, true).unwrap();

    // Check that the reuse address flag is set.
    assert!(sockopt::get_socket_reuseaddr(&s).unwrap());

    #[cfg(not(windows))]
    {
        // Set the broadcast flag;
        sockopt::set_socket_broadcast(&s, true).unwrap();

        // Check that the broadcast flag is set. This has no effect on stream
        // sockets, and not all platforms even remember the value.
        #[cfg(not(bsd))]
        assert!(sockopt::get_socket_broadcast(&s).unwrap());
    }

    // Set the keepalive flag;
    sockopt::set_socket_keepalive(&s, true).unwrap();

    // Check that the keepalive flag is set.
    assert!(sockopt::get_socket_keepalive(&s).unwrap());

    // Set a linger.
    sockopt::set_socket_linger(&s, Some(Duration::new(1, 1))).unwrap();

    // Check that we have a linger of at least the time we set.
    assert!(dbg!(sockopt::get_socket_linger(&s).unwrap().unwrap()) >= Duration::new(1, 1));

    #[cfg(linux_kernel)]
    {
        // Set the passcred flag;
        sockopt::set_socket_passcred(&s, true).unwrap();

        // Check that the passcred flag is set.
        assert!(sockopt::get_socket_passcred(&s).unwrap());
    }

    // Set the receive buffer size.
    let size = sockopt::get_socket_recv_buffer_size(&s).unwrap();
    sockopt::set_socket_recv_buffer_size(&s, size * 2).unwrap();

    // Check that the receive buffer size is set.
    assert!(sockopt::get_socket_recv_buffer_size(&s).unwrap() >= size * 2);

    // Set the send buffer size.
    let size = sockopt::get_socket_send_buffer_size(&s).unwrap();
    sockopt::set_socket_send_buffer_size(&s, size * 4).unwrap();

    // Check that the send buffer size is set.
    assert!(sockopt::get_socket_send_buffer_size(&s).unwrap() >= size * 4);

    // Check that the oobinline flag is not initially set.
    assert!(!sockopt::get_socket_oobinline(&s).unwrap());

    // Set the oobinline flag;
    sockopt::set_socket_oobinline(&s, true).unwrap();

    // Check that the oobinline flag is set.
    assert!(sockopt::get_socket_oobinline(&s).unwrap());
}

// Test `tcp` socket options.
fn test_sockopts_tcp(s: &OwnedFd) {
    #[cfg(any(linux_like, taraget_os = "fuchsia"))]
    {
        assert_eq!(sockopt::get_tcp_user_timeout(&s).unwrap(), 0);
        sockopt::set_tcp_user_timeout(&s, 7).unwrap();
        assert_eq!(sockopt::get_tcp_user_timeout(&s).unwrap(), 7);
    }

    assert!(!sockopt::get_tcp_nodelay(&s).unwrap());

    #[cfg(not(any(target_os = "openbsd", target_os = "haiku", target_os = "nto")))]
    {
        assert!(sockopt::get_tcp_keepcnt(&s).is_ok());
        assert!(sockopt::get_tcp_keepidle(&s).is_ok());
        assert!(sockopt::get_tcp_keepintvl(&s).is_ok());
    }

    // Set the nodelay flag;
    sockopt::set_tcp_nodelay(&s, true).unwrap();

    // Check that the nodelay flag is set.
    assert!(sockopt::get_tcp_nodelay(&s).unwrap());

    #[cfg(not(any(target_os = "openbsd", target_os = "haiku", target_os = "nto")))]
    {
        // Set keepalive values:
        sockopt::set_tcp_keepcnt(&s, 42).unwrap();
        sockopt::set_tcp_keepidle(&s, Duration::from_secs(3601)).unwrap();
        sockopt::set_tcp_keepintvl(&s, Duration::from_secs(61)).unwrap();

        // Check keepalive values:
        assert_eq!(sockopt::get_tcp_keepcnt(&s).unwrap(), 42);
        assert_eq!(
            sockopt::get_tcp_keepidle(&s).unwrap(),
            Duration::from_secs(3601)
        );
        assert_eq!(
            sockopt::get_tcp_keepintvl(&s).unwrap(),
            Duration::from_secs(61)
        );
    }
}

#[test]
fn test_sockopts_ipv4() {
    let s = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();

    test_sockopts_socket(&s);

    #[cfg(not(any(
        apple,
        windows,
        target_os = "dragonfly",
        target_os = "emscripten",
        target_os = "espidf",
        target_os = "haiku",
        target_os = "netbsd",
        target_os = "nto",
    )))]
    assert_eq!(sockopt::get_socket_domain(&s).unwrap(), AddressFamily::INET);
    assert_ne!(sockopt::get_ip_ttl(&s).unwrap(), 0);
    assert_ne!(sockopt::get_ip_ttl(&s).unwrap(), 77);
    #[cfg(not(any(bsd, windows, target_os = "illumos")))]
    assert!(sockopt::get_ip_multicast_loop(&s).unwrap());
    #[cfg(not(any(bsd, windows, target_os = "illumos")))]
    assert_eq!(sockopt::get_ip_multicast_ttl(&s).unwrap(), 1);

    // Set the ip ttl.
    sockopt::set_ip_ttl(&s, 77).unwrap();

    // Check the ip ttl.
    assert_eq!(sockopt::get_ip_ttl(&s).unwrap(), 77);

    #[cfg(not(any(bsd, windows, target_os = "illumos")))]
    {
        // Set the multicast loop flag;
        sockopt::set_ip_multicast_loop(&s, false).unwrap();

        // Check that the multicast loop flag is set.
        assert!(!sockopt::get_ip_multicast_loop(&s).unwrap());
    }

    // Check the initial value of IP TOS, set it, and check it.
    #[cfg(linux_kernel)]
    {
        assert_eq!(sockopt::get_ip_tos(&s).unwrap(), 0);
        sockopt::set_ip_tos(&s, libc::IPTOS_THROUGHPUT).unwrap();
        assert_eq!(sockopt::get_ip_tos(&s).unwrap(), libc::IPTOS_THROUGHPUT);
    }

    // Check the initial value of IP RECVTOS, set it, and check it.
    #[cfg(any(apple, freebsdlike, linux_like, target_os = "fuchsia"))]
    {
        assert!(!sockopt::get_ip_recvtos(&s).unwrap());
        sockopt::set_ip_recvtos(&s, true).unwrap();
        assert!(sockopt::get_ip_recvtos(&s).unwrap());
    }

    test_sockopts_tcp(&s);
}

#[test]
fn test_sockopts_ipv6() {
    let s = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();

    test_sockopts_socket(&s);

    #[cfg(not(any(
        apple,
        windows,
        target_os = "dragonfly",
        target_os = "emscripten",
        target_os = "espidf",
        target_os = "haiku",
        target_os = "netbsd",
        target_os = "nto",
    )))]
    assert_eq!(
        sockopt::get_socket_domain(&s).unwrap(),
        AddressFamily::INET6
    );

    assert_ne!(sockopt::get_ipv6_unicast_hops(&s).unwrap(), 0);
    match sockopt::get_ipv6_multicast_loop(&s) {
        Ok(multicast_loop) => assert!(multicast_loop),
        Err(rustix::io::Errno::OPNOTSUPP) => (),
        Err(rustix::io::Errno::INVAL) => (),
        Err(rustix::io::Errno::NOPROTOOPT) => (),
        Err(err) => Err(err).unwrap(),
    }
    assert_ne!(sockopt::get_ipv6_unicast_hops(&s).unwrap(), 0);

    // On NetBSD, `get_ipv6_multicasthops` returns 1 here. It's not evident
    // why it differs from other OS's.
    #[cfg(not(target_os = "netbsd"))]
    match sockopt::get_ipv6_multicast_hops(&s) {
        Ok(hops) => assert_eq!(hops, 0),
        Err(rustix::io::Errno::NOPROTOOPT) => (),
        Err(rustix::io::Errno::INVAL) => (),
        Err(err) => Err(err).unwrap(),
    };

    // Set the IPV4 V6OONLY value.
    let v6only = rustix::net::sockopt::get_ipv6_v6only(&s).unwrap();
    sockopt::set_ipv6_v6only(&s, !v6only).unwrap();

    // Check that the IPV6 V6ONLY value is set.
    assert_eq!(sockopt::get_ipv6_v6only(&s).unwrap(), !v6only);

    // Set the IPV6 multicast loop value.
    match sockopt::set_ipv6_multicast_loop(&s, false) {
        Ok(()) => {
            // Check that the IPV6 multicast loop value is set.
            match sockopt::get_ipv6_multicast_loop(&s) {
                Ok(multicast_loop) => assert!(!multicast_loop),
                Err(err) => Err(err).unwrap(),
            }
        }
        Err(rustix::io::Errno::OPNOTSUPP) => (),
        Err(rustix::io::Errno::INVAL) => (),
        Err(rustix::io::Errno::NOPROTOOPT) => (),
        Err(err) => Err(err).unwrap(),
    }

    // Set the IPV6 unicast hops value to the default value.
    sockopt::set_ipv6_unicast_hops(&s, None).unwrap();

    // Check that the IPV6 unicast hops value is set.
    assert_ne!(sockopt::get_ipv6_unicast_hops(&s).unwrap(), 0);

    // Set the IPV6 unicast hops value to a specific value.
    sockopt::set_ipv6_unicast_hops(&s, Some(8)).unwrap();

    // Check that the IPV6 unicast hops value is set.
    assert_eq!(sockopt::get_ipv6_unicast_hops(&s).unwrap(), 8);

    // Check the initial value of IPV6 RECVTCLASS, set it, and check it.
    assert!(!sockopt::get_ipv6_recvtclass(&s).unwrap());
    sockopt::set_ipv6_recvtclass(&s, true).unwrap();
    assert!(sockopt::get_ipv6_recvtclass(&s).unwrap());

    test_sockopts_tcp(&s);
}
