#[cfg(not(any(apple, windows, target_os = "haiku")))]
use rustix::net::SocketFlags;
use rustix::net::{
    AddressFamily, Ipv6Addr, RecvFlags, SendFlags, SocketAddrAny, SocketAddrV4, SocketAddrV6,
    SocketType,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

/// Test `connect_any`.
#[test]
fn net_v4_connect_any() {
    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();

    let sender = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept(&listener).expect("accept");
    let mut response = [0_u8; 128];
    let n = rustix::net::recv(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
}

/// Test `connect_any` using `accept_with`.
#[cfg(not(any(apple, windows, target_os = "haiku")))]
#[test]
fn net_v4_connect_any_accept_with() {
    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();

    let sender = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    let request = b"Hello, World, with flags!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept_with(&listener, SocketFlags::CLOEXEC).expect("accept_with");
    let mut response = [0_u8; 128];
    let n = rustix::net::recv(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
}

/// Similar, but with V6.
#[test]
fn net_v6_connect_any() {
    let localhost = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();

    let sender = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept(&listener).expect("accept");
    let mut response = [0_u8; 128];
    let n = rustix::net::recv(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
}

/// Similar, but with V6, using `accept_with`.
#[cfg(not(any(apple, windows, target_os = "haiku")))]
#[test]
fn net_v6_connect_any_accept_with() {
    let localhost = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();

    let sender = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    let request = b"Hello, World, with flags!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept_with(&listener, SocketFlags::CLOEXEC).expect("accept_with");
    let mut response = [0_u8; 128];
    let n = rustix::net::recv(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
}

/// Test `connect` with a `SocketAddr`.
#[test]
fn net_v4_connect() {
    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let local_addr = match local_addr {
        SocketAddrAny::V4(v4) => SocketAddr::V4(v4),
        other => panic!("unexpected socket address {:?}", other),
    };
    let sender = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::connect(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept(&listener).expect("accept");
    let mut response = [0_u8; 128];
    let n = rustix::net::recv(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
}

/// Similar, but use V6.
#[test]
fn net_v6_connect() {
    let localhost = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let local_addr = match local_addr {
        SocketAddrAny::V6(v6) => SocketAddr::V6(v6),
        other => panic!("unexpected socket address {:?}", other),
    };
    let sender = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::connect(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept(&listener).expect("accept");
    let mut response = [0_u8; 128];
    let n = rustix::net::recv(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
}

/// Test `connect_unspec`.
#[test]
fn net_v4_connect_unspec() {
    const SOME_PORT: u16 = 47;
    let localhost_addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, SOME_PORT);

    let socket = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();

    rustix::net::connect_v4(&socket, &localhost_addr).expect("connect_v4");
    let peer = getpeername_v4(&socket).unwrap();
    assert_eq!(peer.ip().to_owned(), Ipv4Addr::LOCALHOST);

    match rustix::net::connect_unspec(&socket) {
        #[cfg(apple)]
        Err(rustix::io::Errno::INVAL) => {} // Apple platforms return an error even when the call succeeded.
        r => r.expect("connect_unspec"),
    }
    let peer_result = getpeername_v4(&socket);
    assert_eq!(peer_result, Err(rustix::io::Errno::NOTCONN));

    rustix::net::connect_v4(&socket, &localhost_addr).expect("connect_v4");
    let peer = getpeername_v4(&socket).unwrap();
    assert_eq!(peer.ip().to_owned(), Ipv4Addr::LOCALHOST);

    fn getpeername_v4<Fd: rustix::fd::AsFd>(sockfd: Fd) -> rustix::io::Result<SocketAddrV4> {
        match rustix::net::getpeername(sockfd)? {
            Some(SocketAddrAny::V4(addr_v4)) => Ok(addr_v4),
            None => Err(rustix::io::Errno::NOTCONN),
            _ => Err(rustix::io::Errno::AFNOSUPPORT),
        }
    }
}

/// Test `connect_unspec`.
#[test]
fn net_v6_connect_unspec() {
    const SOME_PORT: u16 = 47;
    let localhost_addr = SocketAddrV6::new(Ipv6Addr::LOCALHOST, SOME_PORT, 0, 0);

    let socket = rustix::net::socket(AddressFamily::INET6, SocketType::DGRAM, None).unwrap();

    rustix::net::connect_v6(&socket, &localhost_addr).expect("connect_v6");
    let peer = getpeername_v6(&socket).unwrap();
    assert_eq!(peer.ip().to_owned(), Ipv6Addr::LOCALHOST);

    match rustix::net::connect_unspec(&socket) {
        #[cfg(apple)]
        Err(rustix::io::Errno::AFNOSUPPORT) => {} // Apple platforms return an error even when the call succeeded.
        r => r.expect("connect_unspec"),
    }
    let peer_result = getpeername_v6(&socket);
    assert_eq!(peer_result, Err(rustix::io::Errno::NOTCONN));

    rustix::net::connect_v6(&socket, &localhost_addr).expect("connect_v6");
    let peer = getpeername_v6(&socket).unwrap();
    assert_eq!(peer.ip().to_owned(), Ipv6Addr::LOCALHOST);

    fn getpeername_v6<Fd: rustix::fd::AsFd>(sockfd: Fd) -> rustix::io::Result<SocketAddrV6> {
        match rustix::net::getpeername(sockfd)? {
            Some(SocketAddrAny::V6(addr_v6)) => Ok(addr_v6),
            None => Err(rustix::io::Errno::NOTCONN),
            _ => Err(rustix::io::Errno::AFNOSUPPORT),
        }
    }
}

/// Test `bind_any`.
#[test]
fn net_v4_bind_any() {
    let localhost = Ipv4Addr::LOCALHOST;
    let addr = SocketAddrV4::new(localhost, 0).into();
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::bind_any(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept(&listener).expect("accept");
    let mut response = [0_u8; 128];
    let n = rustix::net::recv(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
}

/// Similar, but use V6.
#[test]
fn net_v6_bind_any() {
    let localhost = Ipv6Addr::LOCALHOST;
    let addr = SocketAddrAny::V6(SocketAddrV6::new(localhost, 0, 0, 0));
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::bind_any(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept(&listener).expect("accept");
    let mut response = [0_u8; 128];
    let n = rustix::net::recv(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
}

/// Test `sendto`.
#[test]
fn net_v4_sendto() {
    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let local_addr = match local_addr {
        SocketAddrAny::V4(v4) => SocketAddr::V4(v4),
        other => panic!("unexpected socket address {:?}", other),
    };
    let n = rustix::net::sendto(&sender, request, SendFlags::empty(), &local_addr).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept(&listener).expect("accept");
    let mut response = [0_u8; 128];
    let (n, from) =
        rustix::net::recvfrom(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
    assert!(from.is_none());
}

/// Similar, but with V6.
#[test]
fn net_v6_sendto() {
    let localhost = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let local_addr = match local_addr {
        SocketAddrAny::V6(v6) => SocketAddr::V6(v6),
        other => panic!("unexpected socket address {:?}", other),
    };
    let n = rustix::net::sendto(&sender, request, SendFlags::empty(), &local_addr).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept(&listener).expect("accept");
    let mut response = [0_u8; 128];
    let (n, from) =
        rustix::net::recvfrom(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
    assert!(from.is_none());
}

/// Test `sendto_any`.
#[test]
fn net_v4_sendto_any() {
    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n =
        rustix::net::sendto_any(&sender, request, SendFlags::empty(), &local_addr).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept(&listener).expect("accept");
    let mut response = [0_u8; 128];
    let (n, from) =
        rustix::net::recvfrom(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
    assert!(from.is_none());
}

/// Similar, but with V6.
#[test]
fn net_v6_sendto_any() {
    let localhost = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n =
        rustix::net::sendto_any(&sender, request, SendFlags::empty(), &local_addr).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept(&listener).expect("accept");
    let mut response = [0_u8; 128];
    let (n, from) =
        rustix::net::recvfrom(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
    assert!(from.is_none());
}

/// Test `acceptfrom`.
#[test]
fn net_v4_acceptfrom() {
    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let (accepted, from) = rustix::net::acceptfrom(&listener).expect("accept");

    assert_ne!(from.clone().unwrap(), local_addr);

    let from = match from.unwrap() {
        SocketAddrAny::V4(v4) => v4,
        other => panic!("unexpected socket address {:?}", other),
    };
    let local_addr = match local_addr {
        SocketAddrAny::V4(v4) => v4,
        other => panic!("unexpected socket address {:?}", other),
    };

    assert_eq!(from.clone().ip(), local_addr.ip());
    assert_ne!(from.clone().port(), local_addr.port());

    let peer_addr = rustix::net::getpeername(&accepted).expect("getpeername");
    let peer_addr = peer_addr.expect("peer address should be available");
    let peer_addr = match peer_addr {
        SocketAddrAny::V4(v4) => v4,
        other => panic!("unexpected socket address {:?}", other),
    };

    assert_eq!(from.clone().ip(), peer_addr.ip());
    assert_eq!(from.clone().port(), peer_addr.port());

    let mut response = [0_u8; 128];
    let n = rustix::net::recv(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
}

/// Similar, but with V6.
#[test]
fn net_v6_acceptfrom() {
    let localhost = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let (accepted, from) = rustix::net::acceptfrom(&listener).expect("accept");

    assert_ne!(from.clone().unwrap(), local_addr);

    let from = match from.unwrap() {
        SocketAddrAny::V6(v6) => v6,
        other => panic!("unexpected socket address {:?}", other),
    };
    let local_addr = match local_addr {
        SocketAddrAny::V6(v6) => v6,
        other => panic!("unexpected socket address {:?}", other),
    };

    assert_eq!(from.clone().ip(), local_addr.ip());
    assert_ne!(from.clone().port(), local_addr.port());

    let peer_addr = rustix::net::getpeername(&accepted).expect("getpeername");
    let peer_addr = peer_addr.expect("peer address should be available");
    let peer_addr = match peer_addr {
        SocketAddrAny::V6(v6) => v6,
        other => panic!("unexpected socket address {:?}", other),
    };

    assert_eq!(from.clone().ip(), peer_addr.ip());
    assert_eq!(from.clone().port(), peer_addr.port());

    let mut response = [0_u8; 128];
    let n = rustix::net::recv(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
}

/// Test `shutdown`.
#[test]
fn net_shutdown() {
    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();

    let sender = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    rustix::net::shutdown(&sender, rustix::net::Shutdown::Write).expect("shutdown");

    let accepted = rustix::net::accept(&listener).expect("accept");
    let mut response = [0_u8; 128];
    let n = rustix::net::recv(&accepted, &mut response, RecvFlags::empty()).expect("recv");
    assert_eq!(n, 0);

    drop(sender);
}
