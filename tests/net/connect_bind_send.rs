#[cfg(not(any(apple, windows, target_os = "haiku")))]
use rustix::net::SocketFlags;
use rustix::net::{
    AddressFamily, Ipv6Addr, RecvFlags, SendFlags, SocketAddrAny, SocketAddrV4, SocketAddrV6,
    SocketType,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

/// Test `connect` with `SocketAddrAny`.
#[test]
fn net_v4_connect_any() {
    crate::init();

    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();

    let sender = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::connect(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept(&listener).expect("accept");
    let mut response = [0_u8; 128];
    let (n, actual) =
        rustix::net::recv(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());
    assert_eq!(actual, request.len());

    assert_eq!(request, &response[..n]);
}

/// Test `connect` with `SocketAddrAny` using `accept_with`.
#[cfg(not(any(apple, windows, target_os = "haiku")))]
#[test]
fn net_v4_connect_any_accept_with() {
    crate::init();

    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();

    let sender = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::connect(&sender, &local_addr).expect("connect");
    let request = b"Hello, World, with flags!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept_with(&listener, SocketFlags::CLOEXEC).expect("accept_with");
    let mut response = [0_u8; 128];
    let (n, actual) =
        rustix::net::recv(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());
    assert_eq!(actual, request.len());

    assert_eq!(request, &response[..n]);
}

/// Similar, but with V6.
#[test]
fn net_v6_connect_any() {
    crate::init();

    let localhost = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();

    let sender = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::connect(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept(&listener).expect("accept");
    let mut response = [0_u8; 128];
    let (n, actual) =
        rustix::net::recv(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());
    assert_eq!(actual, request.len());

    assert_eq!(request, &response[..n]);
}

/// Similar, but with V6, using `accept_with`.
#[cfg(not(any(apple, windows, target_os = "haiku")))]
#[test]
fn net_v6_connect_any_accept_with() {
    crate::init();

    let localhost = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();

    let sender = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::connect(&sender, &local_addr).expect("connect");
    let request = b"Hello, World, with flags!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept_with(&listener, SocketFlags::CLOEXEC).expect("accept_with");
    let mut response = [0_u8; 128];
    let (n, actual) =
        rustix::net::recv(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());
    assert_eq!(actual, request.len());

    assert_eq!(request, &response[..n]);
}

/// Test `connect` with a `SocketAddr`.
#[test]
fn net_v4_connect() {
    crate::init();

    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let local_addr = SocketAddrV4::try_from(local_addr).expect("unexpected socket address");
    let sender = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::connect(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept(&listener).expect("accept");
    let mut response = [0_u8; 128];
    let (n, actual) =
        rustix::net::recv(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());
    assert_eq!(actual, request.len());

    assert_eq!(request, &response[..n]);
}

/// Similar, but use V6.
#[test]
fn net_v6_connect() {
    crate::init();

    let localhost = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let local_addr = match SocketAddrV6::try_from(local_addr) {
        Ok(v6) => SocketAddr::V6(v6),
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
    let (n, actual) =
        rustix::net::recv(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());
    assert_eq!(actual, request.len());

    assert_eq!(request, &response[..n]);
}

/// Test `connect_unspec`.
#[test]
fn net_v4_connect_unspec() {
    crate::init();

    const SOME_PORT: u16 = 47;
    let localhost_addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, SOME_PORT);

    let socket = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();

    rustix::net::connect(&socket, &localhost_addr).expect("connect_v4");
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

    rustix::net::connect(&socket, &localhost_addr).expect("connect_v4");
    assert_eq!(getsockname_v4(&socket).unwrap().ip(), &Ipv4Addr::LOCALHOST);
    assert_eq!(getpeername_v4(&socket).unwrap(), localhost_addr);

    fn getsockname_v4<Fd: rustix::fd::AsFd>(sockfd: Fd) -> rustix::io::Result<SocketAddrV4> {
        rustix::net::getsockname(sockfd)?.try_into()
    }

    fn getpeername_v4<Fd: rustix::fd::AsFd>(sockfd: Fd) -> rustix::io::Result<SocketAddrV4> {
        match rustix::net::getpeername(sockfd)? {
            Some(addr) => addr.try_into(),
            None => Err(rustix::io::Errno::NOTCONN),
        }
    }
}

/// Test `connect_unspec`.
#[test]
fn net_v6_connect_unspec() {
    crate::init();

    const SOME_PORT: u16 = 47;
    let localhost_addr = SocketAddrV6::new(Ipv6Addr::LOCALHOST, SOME_PORT, 0, 0);

    let socket = rustix::net::socket(AddressFamily::INET6, SocketType::DGRAM, None).unwrap();

    rustix::net::connect(&socket, &localhost_addr).expect("connect_v6");
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

    rustix::net::connect(&socket, &localhost_addr).expect("connect_v6");
    assert_eq!(getsockname_v6(&socket).unwrap().ip(), &Ipv6Addr::LOCALHOST);
    assert_eq!(getpeername_v6(&socket).unwrap(), localhost_addr);

    fn getsockname_v6<Fd: rustix::fd::AsFd>(sockfd: Fd) -> rustix::io::Result<SocketAddrV6> {
        rustix::net::getsockname(sockfd)?.try_into()
    }

    fn getpeername_v6<Fd: rustix::fd::AsFd>(sockfd: Fd) -> rustix::io::Result<SocketAddrV6> {
        match rustix::net::getpeername(sockfd)? {
            Some(addr) => addr.try_into(),
            None => Err(rustix::io::Errno::NOTCONN),
        }
    }
}

/// Test `bind` with `SocketAddrAny`.
#[test]
fn net_v4_bind_any() {
    crate::init();

    let localhost = Ipv4Addr::LOCALHOST;
    let addr = SocketAddrAny::from(SocketAddrV4::new(localhost, 0));
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::connect(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept(&listener).expect("accept");
    let mut response = [0_u8; 128];
    let (n, actual) =
        rustix::net::recv(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());
    assert_eq!(actual, request.len());

    assert_eq!(request, &response[..n]);
}

/// Similar, but use V6.
#[test]
fn net_v6_bind_any() {
    crate::init();

    let localhost = Ipv6Addr::LOCALHOST;
    let addr = SocketAddrAny::from(SocketAddrV6::new(localhost, 0, 0, 0));
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::connect(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept(&listener).expect("accept");
    let mut response = [0_u8; 128];
    let (n, actual) =
        rustix::net::recv(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());
    assert_eq!(actual, request.len());

    assert_eq!(request, &response[..n]);
}

/// Test `sendto`.
#[test]
fn net_v4_sendto() {
    crate::init();

    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::connect(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let local_addr = SocketAddrV4::try_from(local_addr).expect("unexpected socket address");
    let n = rustix::net::sendto(&sender, request, SendFlags::empty(), &local_addr).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept(&listener).expect("accept");
    let mut response = [0_u8; 128];
    let (n, actual, from) =
        rustix::net::recvfrom(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());
    assert_eq!(actual, request.len());

    assert_eq!(request, &response[..n]);
    assert!(from.is_none());
}

/// Similar, but with V6.
#[test]
fn net_v6_sendto() {
    crate::init();

    let localhost = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::connect(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let local_addr = SocketAddrV6::try_from(local_addr).expect("unexpected socket address");
    let n = rustix::net::sendto(&sender, request, SendFlags::empty(), &local_addr).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept(&listener).expect("accept");
    let mut response = [0_u8; 128];
    let (n, actual, from) =
        rustix::net::recvfrom(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());
    assert_eq!(actual, request.len());

    assert_eq!(request, &response[..n]);
    assert!(from.is_none());
}

/// Test `sendto` with `SocketAddrAny`.
#[test]
fn net_v4_sendto_any() {
    crate::init();

    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::connect(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::sendto(&sender, request, SendFlags::empty(), &local_addr).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept(&listener).expect("accept");
    let mut response = [0_u8; 128];
    let (n, actual, from) =
        rustix::net::recvfrom(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());
    assert_eq!(actual, request.len());

    assert_eq!(request, &response[..n]);
    assert!(from.is_none());
}

/// Similar, but with V6.
#[test]
fn net_v6_sendto_any() {
    crate::init();

    let localhost = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::connect(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::sendto(&sender, request, SendFlags::empty(), &local_addr).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let accepted = rustix::net::accept(&listener).expect("accept");
    let mut response = [0_u8; 128];
    let (n, actual, from) =
        rustix::net::recvfrom(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());
    assert_eq!(actual, request.len());

    assert_eq!(request, &response[..n]);
    assert!(from.is_none());
}

/// Test `acceptfrom`.
#[test]
fn net_v4_acceptfrom() {
    crate::init();

    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::connect(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let (accepted, from) = rustix::net::acceptfrom(&listener).expect("accept");

    assert_ne!(from.clone().unwrap(), local_addr);

    let from = SocketAddrV4::try_from(from.unwrap()).unwrap();
    let local_addr = SocketAddrV4::try_from(local_addr).unwrap();

    assert_eq!(from.clone().ip(), local_addr.ip());
    assert_ne!(from.clone().port(), local_addr.port());

    let peer_addr = rustix::net::getpeername(&accepted).expect("getpeername");
    let peer_addr = peer_addr.expect("peer address should be available");
    let peer_addr = SocketAddrV4::try_from(peer_addr).unwrap();

    assert_eq!(from.clone().ip(), peer_addr.ip());
    assert_eq!(from.clone().port(), peer_addr.port());

    let mut response = [0_u8; 128];
    let (n, actual) =
        rustix::net::recv(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());
    assert_eq!(actual, request.len());

    assert_eq!(request, &response[..n]);
}

/// Similar, but with V6.
#[test]
fn net_v6_acceptfrom() {
    crate::init();

    let localhost = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let sender = rustix::net::socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    rustix::net::connect(&sender, &local_addr).expect("connect");
    let request = b"Hello, World!!!";
    let n = rustix::net::send(&sender, request, SendFlags::empty()).expect("send");
    drop(sender);

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());

    let (accepted, from) = rustix::net::acceptfrom(&listener).expect("accept");

    assert_ne!(from.clone().unwrap(), local_addr);

    let from = SocketAddrV6::try_from(from.unwrap()).unwrap();
    let local_addr = SocketAddrV6::try_from(local_addr).unwrap();

    assert_eq!(from.clone().ip(), local_addr.ip());
    assert_ne!(from.clone().port(), local_addr.port());

    let peer_addr = rustix::net::getpeername(&accepted).expect("getpeername");
    let peer_addr = peer_addr.expect("peer address should be available");
    let peer_addr = SocketAddrV6::try_from(peer_addr).unwrap();

    assert_eq!(from.clone().ip(), peer_addr.ip());
    assert_eq!(from.clone().port(), peer_addr.port());

    let mut response = [0_u8; 128];
    let (n, actual) =
        rustix::net::recv(&accepted, &mut response, RecvFlags::empty()).expect("recv");

    // Not strictly required, but it makes the test simpler.
    assert_eq!(n, request.len());
    assert_eq!(actual, request.len());

    assert_eq!(request, &response[..n]);
}

/// Test `shutdown`.
#[test]
fn net_shutdown() {
    crate::init();

    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");

    let local_addr = rustix::net::getsockname(&listener).unwrap();

    let sender = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::connect(&sender, &local_addr).expect("connect");
    rustix::net::shutdown(&sender, rustix::net::Shutdown::Write).expect("shutdown");

    let accepted = rustix::net::accept(&listener).expect("accept");
    let mut response = [0_u8; 128];
    let (n, actual) =
        rustix::net::recv(&accepted, &mut response, RecvFlags::empty()).expect("recv");
    assert_eq!(n, 0);
    assert_eq!(actual, 0);

    drop(sender);
}
