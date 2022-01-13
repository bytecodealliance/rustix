//! Test a simple IPv4 socket server and client. The client send a
//! message and the server sends one back, uses `sendmsg` and `recvmsg`.

#![cfg(not(any(target_os = "redox", target_os = "wasi")))]

use rustix::net::{
    accept, bind_v4, connect_v4, getsockname, listen, recvmsg_v4, sendmsg_v4, socket,
    AddressFamily, Ipv4Addr, Protocol, RecvFlags, SendFlags, SocketAddrAny, SocketAddrV4,
    SocketType,
};
use std::io::{IoSlice, IoSliceMut};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

const BUFFER_SIZE: usize = 20;

fn server(ready: Arc<(Mutex<u16>, Condvar)>) {
    let connection_socket = socket(AddressFamily::INET, SocketType::STREAM, Protocol::TCP).unwrap();

    let name = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0);
    bind_v4(&connection_socket, &name).unwrap();

    let who = match getsockname(&connection_socket).unwrap() {
        SocketAddrAny::V4(addr) => addr,
        _ => panic!(),
    };

    listen(&connection_socket, 1).unwrap();
    {
        let (lock, cvar) = &*ready;
        let mut port = lock.lock().unwrap();
        *port = who.port();
        cvar.notify_all();
    }

    let data_socket = accept(&connection_socket).unwrap();

    let mut buffer = vec![0u8; BUFFER_SIZE];

    let res = recvmsg_v4(
        &data_socket,
        &mut [IoSliceMut::new(&mut buffer)],
        RecvFlags::empty(),
    )
    .unwrap();
    assert!(res.addr.is_none());
    assert_eq!(
        String::from_utf8_lossy(&buffer[..res.bytes]),
        "hello, world"
    );

    sendmsg_v4(
        &data_socket,
        &[IoSlice::new(b"goodnight, moon")],
        None,
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

    let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port);

    let data_socket = socket(AddressFamily::INET, SocketType::STREAM, Protocol::TCP).unwrap();
    connect_v4(&data_socket, &addr).unwrap();

    sendmsg_v4(
        &data_socket,
        &[IoSlice::new(b"hello, world")],
        None,
        SendFlags::empty(),
    )
    .unwrap();

    let mut buffer = vec![0u8; BUFFER_SIZE];
    let res = recvmsg_v4(
        &data_socket,
        &mut [IoSliceMut::new(&mut buffer)],
        RecvFlags::empty(),
    )
    .unwrap();
    assert!(res.addr.is_none());
    assert_eq!(
        String::from_utf8_lossy(&buffer[..res.bytes]),
        "goodnight, moon"
    );
}

#[test]
fn test_v4_msg_tcp() {
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
