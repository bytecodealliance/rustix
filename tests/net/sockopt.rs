use rustix::fd::OwnedFd;
use rustix::io;
#[cfg(any(
    linux_kernel,
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "openbsd",
    target_os = "redox",
    target_env = "newlib"
))]
use rustix::net::ipproto;
use rustix::net::{sockopt, AddressFamily, SocketType};
use std::time::Duration;

// Test `socket` socket options.
fn test_sockopts_socket(s: &OwnedFd) {
    // On a new socket we shouldn't have a timeout yet.
    assert!(sockopt::socket_timeout(s, sockopt::Timeout::Recv)
        .unwrap()
        .is_none());
    assert_eq!(sockopt::socket_type(s).unwrap(), SocketType::STREAM);
    #[cfg(any(
        linux_kernel,
        target_os = "freebsd",
        target_os = "fuchsia",
        target_os = "openbsd",
        target_os = "redox",
        target_env = "newlib"
    ))]
    {
        assert_eq!(sockopt::socket_protocol(s).unwrap(), Some(ipproto::TCP));
    }
    assert!(!sockopt::socket_reuseaddr(s).unwrap());
    #[cfg(not(windows))]
    assert!(!sockopt::socket_broadcast(s).unwrap());
    // On a new socket we shouldn't have a linger yet.
    assert!(sockopt::socket_linger(s).unwrap().is_none());
    #[cfg(linux_kernel)]
    assert!(!sockopt::socket_passcred(s).unwrap());

    // On a new socket we shouldn't have an error yet.
    assert_eq!(sockopt::socket_error(s).unwrap(), Ok(()));
    assert!(!sockopt::socket_keepalive(s).unwrap());
    assert_ne!(sockopt::socket_recv_buffer_size(s).unwrap(), 0);
    assert_ne!(sockopt::socket_send_buffer_size(s).unwrap(), 0);

    #[cfg(not(apple))]
    assert!(!sockopt::socket_acceptconn(s).unwrap());

    // Set a timeout.
    sockopt::set_socket_timeout(s, sockopt::Timeout::Recv, Some(Duration::new(1, 1))).unwrap();

    // Check that we have a timeout of at least the time we set.
    if cfg!(not(any(target_os = "freebsd", target_os = "netbsd"))) {
        assert!(
            sockopt::socket_timeout(s, sockopt::Timeout::Recv)
                .unwrap()
                .unwrap()
                >= Duration::new(1, 1)
        );
    } else {
        // On FreeBSD ≤ 12 and NetBSD, it appears the system rounds the timeout
        // down.
        assert!(
            sockopt::socket_timeout(s, sockopt::Timeout::Recv)
                .unwrap()
                .unwrap()
                >= Duration::new(1, 0)
        );
    }

    // Set a timeout with more than a million nanoseconds.
    sockopt::set_socket_timeout(s, sockopt::Timeout::Recv, Some(Duration::new(1, 10000000)))
        .unwrap();

    // Check that we have a timeout of at least the time we set.
    assert!(
        sockopt::socket_timeout(s, sockopt::Timeout::Recv)
            .unwrap()
            .unwrap()
            >= Duration::new(1, 10000000)
    );

    // Set the reuse address flag
    sockopt::set_socket_reuseaddr(s, true).unwrap();

    // Check that the reuse address flag is set.
    assert!(sockopt::socket_reuseaddr(s).unwrap());

    #[cfg(not(windows))]
    {
        // Set the broadcast flag;
        sockopt::set_socket_broadcast(s, true).unwrap();

        // Check that the broadcast flag is set. This has no effect on stream
        // sockets, and not all platforms even remember the value.
        #[cfg(not(bsd))]
        assert!(sockopt::socket_broadcast(s).unwrap());
    }

    // Set the keepalive flag;
    sockopt::set_socket_keepalive(s, true).unwrap();

    // Check that the keepalive flag is set.
    assert!(sockopt::socket_keepalive(s).unwrap());

    // Set a linger.
    sockopt::set_socket_linger(s, Some(Duration::new(1, 1))).unwrap();

    // Check that we have a linger of at least the time we set.
    assert!(sockopt::socket_linger(s).unwrap().unwrap() >= Duration::new(1, 1));

    #[cfg(linux_kernel)]
    {
        // Set the passcred flag;
        sockopt::set_socket_passcred(s, true).unwrap();

        // Check that the passcred flag is set.
        assert!(sockopt::socket_passcred(s).unwrap());
    }

    // Set the receive buffer size.
    let size = sockopt::socket_recv_buffer_size(s).unwrap();
    sockopt::set_socket_recv_buffer_size(s, size * 2).unwrap();

    // Check that the receive buffer size is set.
    assert!(sockopt::socket_recv_buffer_size(s).unwrap() >= size * 2);

    // Set the send buffer size.
    let size = sockopt::socket_send_buffer_size(s).unwrap();
    sockopt::set_socket_send_buffer_size(s, size * 2).unwrap();

    // Check that the send buffer size is set.
    assert!(sockopt::socket_send_buffer_size(s).unwrap() >= size * 2);

    // Check that the oobinline flag is not initially set.
    assert!(!sockopt::socket_oobinline(s).unwrap());

    // Set the oobinline flag;
    sockopt::set_socket_oobinline(s, true).unwrap();

    // Check that the oobinline flag is set.
    assert!(sockopt::socket_oobinline(s).unwrap());

    // Check the initial value of `SO_REUSEPORT`, set it, and check it.
    #[cfg(not(any(solarish, windows)))]
    {
        assert!(!sockopt::socket_reuseport(s).unwrap());
        sockopt::set_socket_reuseport(s, true).unwrap();
        assert!(sockopt::socket_reuseport(s).unwrap());
    }

    // Check the initial value of `SO_REUSEPORT_LB`, set it, and check it.
    #[cfg(target_os = "freebsd")]
    {
        assert!(!sockopt::socket_reuseport_lb(s).unwrap());
        sockopt::set_socket_reuseport_lb(s, true).unwrap();
        assert!(sockopt::socket_reuseport_lb(s).unwrap());
    }

    // Not much we can check with `get_socket_cookie`, but make sure we can
    // call it and that it returns the same value if called twice.
    #[cfg(target_os = "linux")]
    {
        assert_eq!(
            sockopt::socket_cookie(s).unwrap(),
            sockopt::socket_cookie(s).unwrap()
        );
    }

    // Check the initial value of `SO_INCOMING_CPU`, set it, and check it.
    #[cfg(target_os = "linux")]
    {
        assert_eq!(sockopt::socket_incoming_cpu(s).unwrap(), u32::MAX);
        sockopt::set_socket_incoming_cpu(s, 3).unwrap();
        assert_eq!(sockopt::socket_incoming_cpu(s).unwrap(), 3);
    }

    // Check the initial value of `SO_NOSIGPIPE`, set it, and check it.
    #[cfg(any(apple, freebsdlike, target_os = "netbsd"))]
    {
        assert_eq!(sockopt::socket_nosigpipe(s).unwrap(), false);
        sockopt::set_socket_nosigpipe(s, true).unwrap();
        assert_eq!(sockopt::socket_nosigpipe(s).unwrap(), true);
    }
}

// Test `tcp` socket options.
fn test_sockopts_tcp(s: &OwnedFd) {
    #[cfg(any(linux_like, target_os = "fuchsia"))]
    {
        assert_eq!(sockopt::tcp_user_timeout(s).unwrap(), 0);
        sockopt::set_tcp_user_timeout(s, 7).unwrap();
        assert_eq!(sockopt::tcp_user_timeout(s).unwrap(), 7);
    }

    assert!(!sockopt::tcp_nodelay(s).unwrap());

    #[cfg(not(any(target_os = "openbsd", target_os = "haiku", target_os = "nto")))]
    {
        assert!(sockopt::tcp_keepcnt(s).is_ok());
        assert!(sockopt::tcp_keepidle(s).is_ok());
        assert!(sockopt::tcp_keepintvl(s).is_ok());
    }

    // Set the nodelay flag.
    sockopt::set_tcp_nodelay(s, true).unwrap();

    // Check that the nodelay flag is set.
    assert!(sockopt::tcp_nodelay(s).unwrap());

    // Clear the nodelay flag.
    sockopt::set_tcp_nodelay(s, false).unwrap();

    // Check that the nodelay flag is cleared.
    assert!(!sockopt::tcp_nodelay(s).unwrap());

    #[cfg(not(any(target_os = "openbsd", target_os = "haiku", target_os = "nto")))]
    {
        // Set keepalive values:
        sockopt::set_tcp_keepcnt(s, 42).unwrap();
        sockopt::set_tcp_keepidle(s, Duration::from_secs(3601)).unwrap();
        sockopt::set_tcp_keepintvl(s, Duration::from_secs(60)).unwrap();

        // Check keepalive values:
        assert_eq!(sockopt::tcp_keepcnt(s).unwrap(), 42);
        assert_eq!(sockopt::tcp_keepidle(s).unwrap(), Duration::from_secs(3601));
        assert_eq!(sockopt::tcp_keepintvl(s).unwrap(), Duration::from_secs(60));

        #[cfg(not(target_os = "illumos"))]
        {
            sockopt::set_tcp_keepintvl(s, Duration::from_secs(61)).unwrap();
            assert_eq!(sockopt::tcp_keepintvl(s).unwrap(), Duration::from_secs(61));
        }
    }

    // Check the initial value of `TCP_QUICKACK`, set it, and check it.
    #[cfg(any(linux_like, target_os = "fuchsia"))]
    {
        assert!(sockopt::tcp_quickack(s).unwrap());
        sockopt::set_tcp_quickack(s, false).unwrap();
        assert!(!sockopt::tcp_quickack(s).unwrap());
    }

    // Check the initial value of `TCP_CONGESTION`, set it, and check it.
    //
    // Temporarily disable this test on non-x86 as qemu isn't yet aware of
    // TCP_CONGESTION.
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[cfg(any(
        linux_like,
        target_os = "freebsd",
        target_os = "fuchsia",
        target_os = "illumos"
    ))]
    #[cfg(feature = "alloc")]
    {
        let algo = sockopt::tcp_congestion(s).unwrap();
        assert!(!algo.is_empty());
        #[cfg(linux_like)]
        {
            sockopt::set_tcp_congestion(s, "reno").unwrap();
            assert_eq!(sockopt::tcp_congestion(s).unwrap(), "reno");
        }
    }

    // Check the initial value of `TCP_THIN_LINEAR_TIMEOUTS`, set it, and check
    // it.
    #[cfg(any(linux_like, target_os = "fuchsia"))]
    {
        assert!(!sockopt::tcp_thin_linear_timeouts(s).unwrap());
        sockopt::set_tcp_thin_linear_timeouts(s, true).unwrap();
        assert!(sockopt::tcp_thin_linear_timeouts(s).unwrap());
    }

    // Check the initial value of `TCP_CORK`, set it, and check it.
    #[cfg(any(linux_like, solarish, target_os = "fuchsia"))]
    {
        assert!(!sockopt::tcp_cork(s).unwrap());
        sockopt::set_tcp_cork(s, true).unwrap();
        assert!(sockopt::tcp_cork(s).unwrap());
    }
}

#[test]
fn test_sockopts_ipv4() {
    crate::init();

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
    assert_eq!(sockopt::socket_domain(&s).unwrap(), AddressFamily::INET);
    assert_ne!(sockopt::ip_ttl(&s).unwrap(), 0);
    assert_ne!(sockopt::ip_ttl(&s).unwrap(), 77);
    #[cfg(not(any(bsd, windows, solarish)))]
    assert!(sockopt::ip_multicast_loop(&s).unwrap());
    #[cfg(not(any(bsd, windows, solarish)))]
    assert_eq!(sockopt::ip_multicast_ttl(&s).unwrap(), 1);

    // Set the ip ttl.
    sockopt::set_ip_ttl(&s, 77).unwrap();

    // Check the ip ttl.
    assert_eq!(sockopt::ip_ttl(&s).unwrap(), 77);

    #[cfg(not(any(bsd, windows, solarish)))]
    {
        // Set the multicast loop flag;
        sockopt::set_ip_multicast_loop(&s, false).unwrap();

        // Check that the multicast loop flag is set.
        assert!(!sockopt::ip_multicast_loop(&s).unwrap());
    }

    // Check the initial value of `IP_TOS`, set it, and check it.
    #[cfg(any(
        bsd,
        linux_like,
        target_os = "aix",
        target_os = "fuchsia",
        target_os = "haiku",
        target_os = "nto",
        target_env = "newlib"
    ))]
    {
        assert_eq!(sockopt::ip_tos(&s).unwrap(), 0);

        #[cfg(any(linux_like, target_os = "aix", target_os = "nto"))]
        {
            sockopt::set_ip_tos(&s, libc::IPTOS_THROUGHPUT).unwrap();
            assert_eq!(sockopt::ip_tos(&s).unwrap(), libc::IPTOS_THROUGHPUT);
        }
    }

    // Check the initial value of `IP_RECVTOS`, set it, and check it.
    #[cfg(any(apple, linux_like, target_os = "freebsd", target_os = "fuchsia"))]
    {
        assert!(!sockopt::ip_recvtos(&s).unwrap());
        sockopt::set_ip_recvtos(&s, true).unwrap();
        assert!(sockopt::ip_recvtos(&s).unwrap());
    }

    // Check the initial value of `IP_FREEBIND`, set it, and check it.
    #[cfg(any(linux_kernel, target_os = "fuchsia"))]
    {
        assert!(!sockopt::ip_freebind(&s).unwrap());
        sockopt::set_ip_freebind(&s, true).unwrap();
        assert!(sockopt::ip_freebind(&s).unwrap());
    }

    // Check that we can query `SO_ORIGINAL_DST`.
    #[cfg(any(linux_kernel, target_os = "fuchsia"))]
    {
        assert!(matches!(
            sockopt::ip_original_dst(&s),
            Err(io::Errno::NOENT | io::Errno::NOPROTOOPT)
        ));
    }

    test_sockopts_tcp(&s);
}

#[test]
fn test_sockopts_ipv6() {
    crate::init();

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
    assert_eq!(sockopt::socket_domain(&s).unwrap(), AddressFamily::INET6);

    assert_ne!(sockopt::ipv6_unicast_hops(&s).unwrap(), 0);
    match sockopt::ipv6_multicast_loop(&s) {
        Ok(multicast_loop) => assert!(multicast_loop),
        Err(io::Errno::OPNOTSUPP) => (),
        Err(io::Errno::INVAL) => (),
        Err(io::Errno::NOPROTOOPT) => (),
        Err(err) => panic!("{:?}", err),
    }
    assert_ne!(sockopt::ipv6_unicast_hops(&s).unwrap(), 0);

    // On NetBSD, `get_ipv6_multicasthops` returns 1 here. It's not evident
    // why it differs from other OS's.
    #[cfg(not(target_os = "netbsd"))]
    match sockopt::ipv6_multicast_hops(&s) {
        Ok(hops) => assert_eq!(hops, 0),
        Err(io::Errno::NOPROTOOPT) => (),
        Err(io::Errno::INVAL) => (),
        Err(err) => panic!("{:?}", err),
    };

    // Set the IPV4 V6OONLY value.
    let v6only = rustix::net::sockopt::ipv6_v6only(&s).unwrap();
    sockopt::set_ipv6_v6only(&s, !v6only).unwrap();

    // Check that the IPV6 V6ONLY value is set.
    assert_eq!(sockopt::ipv6_v6only(&s).unwrap(), !v6only);

    // Set the IPV6 multicast loop value.
    match sockopt::set_ipv6_multicast_loop(&s, false) {
        Ok(()) => {
            // Check that the IPV6 multicast loop value is set.
            match sockopt::ipv6_multicast_loop(&s) {
                Ok(multicast_loop) => assert!(!multicast_loop),
                Err(err) => panic!("{:?}", err),
            }
        }
        Err(io::Errno::OPNOTSUPP) => (),
        Err(io::Errno::INVAL) => (),
        Err(io::Errno::NOPROTOOPT) => (),
        Err(err) => panic!("{:?}", err),
    }

    // Set the IPV6 unicast hops value to the default value.
    sockopt::set_ipv6_unicast_hops(&s, None).unwrap();

    // Check that the IPV6 unicast hops value is set.
    assert_ne!(sockopt::ipv6_unicast_hops(&s).unwrap(), 0);

    // Set the IPV6 unicast hops value to a specific value.
    sockopt::set_ipv6_unicast_hops(&s, Some(8)).unwrap();

    // Check that the IPV6 unicast hops value is set.
    assert_eq!(sockopt::ipv6_unicast_hops(&s).unwrap(), 8);

    // Check the initial value of `IPV6_RECVTCLASS`, set it, and check it.
    #[cfg(any(
        bsd,
        linux_like,
        target_os = "aix",
        target_os = "fuchsia",
        target_os = "nto"
    ))]
    {
        assert!(!sockopt::ipv6_recvtclass(&s).unwrap());
        sockopt::set_ipv6_recvtclass(&s, true).unwrap();
        assert!(sockopt::ipv6_recvtclass(&s).unwrap());
    }

    // Check the initial value of `IPV6_FREEBIND`, set it, and check it.
    #[cfg(linux_kernel)]
    {
        assert!(!sockopt::ipv6_freebind(&s).unwrap());
        sockopt::set_ipv6_freebind(&s, true).unwrap();
        assert!(sockopt::ipv6_freebind(&s).unwrap());
    }

    // Check the initial value of `IPV6_TCLASS`, set it, and check it.
    #[cfg(not(any(solarish, windows, target_os = "espidf", target_os = "haiku")))]
    {
        assert_eq!(sockopt::ipv6_tclass(&s).unwrap(), 0);
        sockopt::set_ipv6_tclass(&s, 12).unwrap();
        assert_eq!(sockopt::ipv6_tclass(&s).unwrap(), 12);
    }

    // Check that we can query `IP6T_SO_ORIGINAL_DST`.
    #[cfg(linux_kernel)]
    {
        assert!(matches!(
            sockopt::ipv6_original_dst(&s),
            Err(io::Errno::NOENT | io::Errno::NOPROTOOPT)
        ));
    }

    test_sockopts_tcp(&s);
}
