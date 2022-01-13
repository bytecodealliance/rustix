//! Test a simple IPv6 socket server and client. The client send a
//! message and the server sends one back.

#![cfg(not(any(target_os = "redox", target_os = "wasi")))]

use rustix::net::{
    bind_v6, connect_v6, getsockname, recvmsg_v6, sendmsg_v6, socket, AddressFamily, Ipv6Addr,
    Protocol, RecvFlags, SendFlags, SocketAddrAny, SocketAddrV6, SocketType,
};
use std::io::{IoSlice, IoSliceMut};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

const BUFFER_SIZE: usize = 20;

fn server(ready: Arc<(Mutex<u16>, Condvar)>) {
    let connection_socket = socket(AddressFamily::INET6, SocketType::DGRAM, Protocol::UDP).unwrap();

    let name = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 0, 0, 0);
    bind_v6(&connection_socket, &name).unwrap();

    let who = match getsockname(&connection_socket).unwrap() {
        SocketAddrAny::V6(addr) => addr,
        _ => panic!(),
    };

    {
        let (lock, cvar) = &*ready;
        let mut port = lock.lock().unwrap();
        *port = who.port();
        cvar.notify_all();
    }

    let mut buffer = vec![0u8; BUFFER_SIZE];
    // no accept for UDP
    let data_socket = connection_socket;
    let res = recvmsg_v6(
        &data_socket,
        &mut [IoSliceMut::new(&mut buffer)],
        RecvFlags::empty(),
    )
    .unwrap();
    assert!(res.addr.is_some());
    assert_eq!(
        String::from_utf8_lossy(&buffer[..res.bytes]),
        "hello, world"
    );

    sendmsg_v6(
        &data_socket,
        &[IoSlice::new(b"goodnight, moon")],
        res.addr.as_ref(),
        SendFlags::empty(),
    )
    .unwrap();
}

fn client(ready: Arc<(Mutex<u16>, Condvar)>) {
    let port = {
        let (lock, cvar) = &*ready;
        let mut port = lock.lock().unwrap();
        while *port == 0 {
            port = cvar.wait(port).unwrap();
        }
        *port
    };

    let addr = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), port, 0, 0);
    let mut buffer = vec![0; BUFFER_SIZE];

    let data_socket = socket(AddressFamily::INET6, SocketType::DGRAM, Protocol::UDP).unwrap();
    connect_v6(&data_socket, &addr).unwrap();

    sendmsg_v6(
        &data_socket,
        &[IoSlice::new(b"hello, world")],
        None,
        SendFlags::empty(),
    )
    .unwrap();

    let res = recvmsg_v6(
        &data_socket,
        &mut [IoSliceMut::new(&mut buffer)],
        RecvFlags::empty(),
    )
    .unwrap();
    assert!(res.addr.is_some());
    assert_eq!(
        String::from_utf8_lossy(&buffer[..res.bytes]),
        "goodnight, moon"
    );
}

#[test]
fn test_v6_msg_udp() {
    #[cfg(windows)]
    rustix::net::wsa_startup().unwrap();

    let ready = Arc::new((Mutex::new(0_u16), Condvar::new()));
    let ready_clone = Arc::clone(&ready);

    let server = thread::Builder::new()
        .name("server".to_string())
        .spawn(move || {
            server(ready);
        })
        .unwrap();
    let client = thread::Builder::new()
        .name("client".to_string())
        .spawn(move || {
            client(ready_clone);
        })
        .unwrap();
    client.join().unwrap();
    server.join().unwrap();

    #[cfg(windows)]
    rustix::net::wsa_cleanup().unwrap();
}

// Ported from https://github.com/nix-rust/nix/blob/master/test/sys/test_socket.rs
// Original License: MIT
//
// Verify `Ipv6PacketInfo` for `sendmsg`.
// This creates a (udp) socket bound to localhost, then sends a message to
// itself but uses Ipv6PacketInfo to force the source address to be
// ip6-localhost.
#[cfg(any(target_os = "linux", target_os = "macos", target_os = "netbsd"))]
#[test]
pub fn test_v6_msg_ipv6packetinfo() {
    use rustix::{
        cmsg_buffer,
        net::{sendmsg_v6_with_ancillary, Ipv6PacketInfo, SendSocketAncillaryV6},
    };

    let connection_socket = socket(AddressFamily::INET6, SocketType::DGRAM, Protocol::default())
        .expect("socket failed");

    let name = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 6000, 0, 0);
    bind_v6(&connection_socket, &name).expect("bind failed");

    let slice = [1u8, 2, 3, 4, 5, 6, 7, 8];
    let iovs = [IoSlice::new(&slice)];

    let mut pi = Ipv6PacketInfo::default();
    pi.set_source_addr(&name);

    let mut cmsg_buffer = cmsg_buffer!(Ipv6PacketInfo);
    let mut cmsg = SendSocketAncillaryV6::new(&mut cmsg_buffer);
    cmsg.add_packet_info(&pi);
    sendmsg_v6_with_ancillary(
        &connection_socket,
        &iovs,
        Some(&name),
        &mut cmsg,
        SendFlags::empty(),
    )
    .expect("sendmsg");
}
