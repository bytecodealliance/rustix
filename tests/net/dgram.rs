//! Tests similar to connect_bind_send.rs, but operating on datagram sockets.

use rustix::net::{
    AddressFamily, Ipv6Addr, RecvFlags, SendFlags, SocketAddrAny, SocketAddrV4, SocketAddrV6,
    SocketType,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

/// Test `connect_any`.
#[test]
fn net_dgram_v4_connect_any() {
    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");

    let local_addr = rustix::net::getsockname(&listener).unwrap();

    let sender = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let mut response = [0_u8; 128];
    let n = rustix::net::recv(&listener, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
}

/// Test `connect_any` using `accept_with`.
#[cfg(not(any(apple, windows, target_os = "haiku")))]
#[test]
fn net_dgram_v4_connect_any_accept_with() {
    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");

    let local_addr = rustix::net::getsockname(&listener).unwrap();

    let sender = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    let request = b"Hello, World, with flags!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let mut response = [0_u8; 128];
    let n = rustix::net::recv(&listener, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
}

/// Similar, but with V6.
#[test]
fn net_dgram_v6_connect_any() {
    let localhost = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::DGRAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");

    let local_addr = rustix::net::getsockname(&listener).unwrap();

    let sender = rustix::net::socket(AddressFamily::INET6, SocketType::DGRAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let mut response = [0_u8; 128];
    let n = rustix::net::recv(&listener, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
}

/// Similar, but with V6, using `accept_with`.
#[cfg(not(any(apple, windows, target_os = "haiku")))]
#[test]
fn net_dgram_v6_connect_any_accept_with() {
    let localhost = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::DGRAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");

    let local_addr = rustix::net::getsockname(&listener).unwrap();

    let sender = rustix::net::socket(AddressFamily::INET6, SocketType::DGRAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    let request = b"Hello, World, with flags!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let mut response = [0_u8; 128];
    let n = rustix::net::recv(&listener, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
}

/// Test `connect` with a `SocketAddr`.
#[test]
fn net_dgram_v4_connect() {
    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let local_addr = match local_addr {
        SocketAddrAny::V4(v4) => SocketAddr::V4(v4),
        other => panic!("unexpected socket address {:?}", other),
    };
    let sender = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();
    rustix::net::connect(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let mut response = [0_u8; 128];
    let n = rustix::net::recv(&listener, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
}

/// Similar, but use V6.
#[test]
fn net_dgram_v6_connect() {
    let localhost = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::DGRAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let local_addr = match local_addr {
        SocketAddrAny::V6(v6) => SocketAddr::V6(v6),
        other => panic!("unexpected socket address {:?}", other),
    };
    let sender = rustix::net::socket(AddressFamily::INET6, SocketType::DGRAM, None).unwrap();
    rustix::net::connect(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let mut response = [0_u8; 128];
    let n = rustix::net::recv(&listener, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
}

/// Test `connect_unspec`.
#[test]
fn net_dgram_v4_connect_unspec() {
    const SOME_PORT: u16 = 47;
    let localhost_addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, SOME_PORT);

    let socket = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();

    rustix::net::connect_v4(&socket, &localhost_addr).expect("connect_v4");
    assert_eq!(getsockname_v4(&socket).unwrap().ip(), &Ipv4Addr::LOCALHOST);
    assert_eq!(getpeername_v4(&socket).unwrap(), localhost_addr);

    match rustix::net::connect_unspec(&socket) {
        // BSD platforms return an error even if the socket was disconnected successfully.
        #[cfg(bsd)]
        Err(rustix::io::Errno::INVAL | rustix::io::Errno::AFNOSUPPORT) => {}
        r => r.expect("connect_unspec"),
    }
    assert_eq!(
        getsockname_v4(&socket).unwrap().ip(),
        &Ipv4Addr::UNSPECIFIED
    );
    assert_eq!(getpeername_v4(&socket), Err(rustix::io::Errno::NOTCONN));

    rustix::net::connect_v4(&socket, &localhost_addr).expect("connect_v4");
    assert_eq!(getsockname_v4(&socket).unwrap().ip(), &Ipv4Addr::LOCALHOST);
    assert_eq!(getpeername_v4(&socket).unwrap(), localhost_addr);

    fn getsockname_v4<Fd: rustix::fd::AsFd>(sockfd: Fd) -> rustix::io::Result<SocketAddrV4> {
        match rustix::net::getsockname(sockfd)? {
            SocketAddrAny::V4(addr_v4) => Ok(addr_v4),
            _ => Err(rustix::io::Errno::AFNOSUPPORT),
        }
    }

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
fn net_dgram_v6_connect_unspec() {
    const SOME_PORT: u16 = 47;
    let localhost_addr = SocketAddrV6::new(Ipv6Addr::LOCALHOST, SOME_PORT, 0, 0);

    let socket = rustix::net::socket(AddressFamily::INET6, SocketType::DGRAM, None).unwrap();

    rustix::net::connect_v6(&socket, &localhost_addr).expect("connect_v6");
    assert_eq!(getsockname_v6(&socket).unwrap().ip(), &Ipv6Addr::LOCALHOST);
    assert_eq!(getpeername_v6(&socket).unwrap(), localhost_addr);

    match rustix::net::connect_unspec(&socket) {
        // BSD platforms return an error even if the socket was disconnected successfully.
        #[cfg(bsd)]
        Err(rustix::io::Errno::INVAL | rustix::io::Errno::AFNOSUPPORT) => {}
        r => r.expect("connect_unspec"),
    }
    assert_eq!(
        getsockname_v6(&socket).unwrap().ip(),
        &Ipv6Addr::UNSPECIFIED
    );
    assert_eq!(getpeername_v6(&socket), Err(rustix::io::Errno::NOTCONN));

    rustix::net::connect_v6(&socket, &localhost_addr).expect("connect_v6");
    assert_eq!(getsockname_v6(&socket).unwrap().ip(), &Ipv6Addr::LOCALHOST);
    assert_eq!(getpeername_v6(&socket).unwrap(), localhost_addr);

    fn getsockname_v6<Fd: rustix::fd::AsFd>(sockfd: Fd) -> rustix::io::Result<SocketAddrV6> {
        match rustix::net::getsockname(sockfd)? {
            SocketAddrAny::V6(addr_v6) => Ok(addr_v6),
            _ => Err(rustix::io::Errno::AFNOSUPPORT),
        }
    }

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
fn net_dgram_v4_bind_any() {
    let localhost = Ipv4Addr::LOCALHOST;
    let addr = SocketAddrV4::new(localhost, 0).into();
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();
    rustix::net::bind_any(&listener, &addr).expect("bind");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let mut response = [0_u8; 128];
    let n = rustix::net::recv(&listener, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
}

/// Similar, but use V6.
#[test]
fn net_dgram_v6_bind_any() {
    let localhost = Ipv6Addr::LOCALHOST;
    let addr = SocketAddrAny::V6(SocketAddrV6::new(localhost, 0, 0, 0));
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::DGRAM, None).unwrap();
    rustix::net::bind_any(&listener, &addr).expect("bind");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET6, SocketType::DGRAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let mut response = [0_u8; 128];
    let n = rustix::net::recv(&listener, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
}

/// Test `sendto` with calling `connect`, on platforms which support that.
#[cfg(not(any(target_os = "freebsd", target_os = "illumos")))]
#[test]
fn net_dgram_v4_connect_sendto() {
    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();
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

    let mut response = [0_u8; 128];
    let (n, from) =
        rustix::net::recvfrom(&listener, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);

    let peer_addr = from.expect("peer address should be available");
    let peer_addr = match peer_addr {
        SocketAddrAny::V4(v4) => v4,
        other => panic!("unexpected socket address {:?}", other),
    };

    let local_addr = match local_addr {
        SocketAddr::V4(v4) => v4,
        other => panic!("unexpected socket address {:?}", other),
    };

    assert_eq!(peer_addr.ip(), local_addr.ip());
}

/// Test `sendto` without calling `connect`.
#[test]
fn net_dgram_v4_sendto() {
    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();
    let request = b"Hello, World!!!";
    let local_addr = match local_addr {
        SocketAddrAny::V4(v4) => SocketAddr::V4(v4),
        other => panic!("unexpected socket address {:?}", other),
    };
    let n = rustix::net::sendto(&sender, request, SendFlags::empty(), &local_addr).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let mut response = [0_u8; 128];
    let (n, from) =
        rustix::net::recvfrom(&listener, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);

    let peer_addr = from.expect("peer address should be available");
    let peer_addr = match peer_addr {
        SocketAddrAny::V4(v4) => v4,
        other => panic!("unexpected socket address {:?}", other),
    };

    let local_addr = match local_addr {
        SocketAddr::V4(v4) => v4,
        other => panic!("unexpected socket address {:?}", other),
    };

    assert_eq!(peer_addr.ip(), local_addr.ip());
}

/// Similar, but with V6.
#[cfg(not(any(target_os = "freebsd", target_os = "illumos")))]
#[test]
fn net_dgram_v6_connect_sendto() {
    let localhost = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::DGRAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET6, SocketType::DGRAM, None).unwrap();
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

    let mut response = [0_u8; 128];
    let (n, from) =
        rustix::net::recvfrom(&listener, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);

    let peer_addr = from.expect("peer address should be available");
    let peer_addr = match peer_addr {
        SocketAddrAny::V6(v6) => v6,
        other => panic!("unexpected socket address {:?}", other),
    };

    let local_addr = match local_addr {
        SocketAddr::V6(v6) => v6,
        other => panic!("unexpected socket address {:?}", other),
    };

    assert_eq!(peer_addr.ip(), local_addr.ip());
}

/// Similar, but with V6.
#[test]
fn net_dgram_v6_sendto() {
    let localhost = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::DGRAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET6, SocketType::DGRAM, None).unwrap();
    let request = b"Hello, World!!!";
    let local_addr = match local_addr {
        SocketAddrAny::V6(v6) => SocketAddr::V6(v6),
        other => panic!("unexpected socket address {:?}", other),
    };
    let n = rustix::net::sendto(&sender, request, SendFlags::empty(), &local_addr).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let mut response = [0_u8; 128];
    let (n, from) =
        rustix::net::recvfrom(&listener, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);

    let peer_addr = from.expect("peer address should be available");
    let peer_addr = match peer_addr {
        SocketAddrAny::V6(v6) => v6,
        other => panic!("unexpected socket address {:?}", other),
    };

    let local_addr = match local_addr {
        SocketAddr::V6(v6) => v6,
        other => panic!("unexpected socket address {:?}", other),
    };

    assert_eq!(peer_addr.ip(), local_addr.ip());
}

/// Test `sendto_any` with calling connect, on platforms which support that.
#[cfg(not(any(target_os = "freebsd", target_os = "illumos")))]
#[test]
fn net_dgram_v4_connect_sendto_any() {
    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n =
        rustix::net::sendto_any(&sender, request, SendFlags::empty(), &local_addr).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let mut response = [0_u8; 128];
    let (n, from) =
        rustix::net::recvfrom(&listener, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);

    let peer_addr = from.expect("peer address should be available");
    let peer_addr = match peer_addr {
        SocketAddrAny::V4(v4) => v4,
        other => panic!("unexpected socket address {:?}", other),
    };

    let local_addr = match local_addr {
        SocketAddrAny::V4(v4) => v4,
        other => panic!("unexpected socket address {:?}", other),
    };

    assert_eq!(peer_addr.ip(), local_addr.ip());
}

/// Test `sendto_any` without calling connect.
#[test]
fn net_dgram_v4_sendto_any() {
    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();
    let request = b"Hello, World!!!";
    let n =
        rustix::net::sendto_any(&sender, request, SendFlags::empty(), &local_addr).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let mut response = [0_u8; 128];
    let (n, from) =
        rustix::net::recvfrom(&listener, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);

    let peer_addr = from.expect("peer address should be available");
    let peer_addr = match peer_addr {
        SocketAddrAny::V4(v4) => v4,
        other => panic!("unexpected socket address {:?}", other),
    };

    let local_addr = match local_addr {
        SocketAddrAny::V4(v4) => v4,
        other => panic!("unexpected socket address {:?}", other),
    };

    assert_eq!(peer_addr.ip(), local_addr.ip());
}

/// Similar, but with V6.
#[cfg(not(any(target_os = "freebsd", target_os = "illumos")))]
#[test]
fn net_dgram_v6_connect_sendto_any() {
    let localhost = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::DGRAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET6, SocketType::DGRAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n =
        rustix::net::sendto_any(&sender, request, SendFlags::empty(), &local_addr).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let mut response = [0_u8; 128];
    let (n, from) =
        rustix::net::recvfrom(&listener, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);

    let peer_addr = from.expect("peer address should be available");
    let peer_addr = match peer_addr {
        SocketAddrAny::V6(v6) => v6,
        other => panic!("unexpected socket address {:?}", other),
    };

    let local_addr = match local_addr {
        SocketAddrAny::V6(v6) => v6,
        other => panic!("unexpected socket address {:?}", other),
    };

    assert_eq!(peer_addr.ip(), local_addr.ip());
}

/// Similar, but with V6.
#[test]
fn net_dgram_v6_sendto_any() {
    let localhost = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::DGRAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET6, SocketType::DGRAM, None).unwrap();
    let request = b"Hello, World!!!";
    let n =
        rustix::net::sendto_any(&sender, request, SendFlags::empty(), &local_addr).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let mut response = [0_u8; 128];
    let (n, from) =
        rustix::net::recvfrom(&listener, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);

    let peer_addr = from.expect("peer address should be available");
    let peer_addr = match peer_addr {
        SocketAddrAny::V6(v6) => v6,
        other => panic!("unexpected socket address {:?}", other),
    };

    let local_addr = match local_addr {
        SocketAddrAny::V6(v6) => v6,
        other => panic!("unexpected socket address {:?}", other),
    };

    assert_eq!(peer_addr.ip(), local_addr.ip());
}

/// Test `acceptfrom`.
#[test]
fn net_dgram_v4_acceptfrom() {
    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let mut response = [0_u8; 128];
    let n = rustix::net::recv(&listener, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
}

/// Similar, but with V6.
#[test]
fn net_dgram_v6_acceptfrom() {
    let localhost = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::DGRAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET6, SocketType::DGRAM, None).unwrap();
    rustix::net::connect_any(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let mut response = [0_u8; 128];
    let n = rustix::net::recv(&listener, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    assert_eq!(request, &response[..n]);
}
