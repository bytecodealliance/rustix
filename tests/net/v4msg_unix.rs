//! Test a simple IPv4 socket server and client. The client send a
//! message and the server sends one back, uses `sendmsg` and `recvmsg`.

#![cfg(not(any(target_os = "redox", target_os = "wasi")))]

use rustix::net::{
    bind_unix, connect_unix, recvmsg_unix, sendmsg_unix, socket, AddressFamily, Protocol,
    RecvFlags, SendFlags, SocketAddrUnix, SocketType,
};
use std::io::{IoSlice, IoSliceMut};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

const BUFFER_SIZE: usize = 20;

const SOCKET_NAME: &str = "test-socket";

fn server(ready: Arc<(Mutex<bool>, Condvar)>) {
    let connection_socket =
        socket(AddressFamily::UNIX, SocketType::DGRAM, Protocol::default()).unwrap();

    let name = SocketAddrUnix::new(SOCKET_NAME).unwrap();
    bind_unix(&connection_socket, &name).unwrap();
    {
        let (lock, cvar) = &*ready;
        let mut ready = lock.lock().unwrap();
        *ready = true;
        cvar.notify_all();
    }

    let data_socket = connection_socket;
    let mut buffer = vec![0u8; BUFFER_SIZE];

    let res = recvmsg_unix(
        &data_socket,
        &[IoSliceMut::new(&mut buffer)],
        RecvFlags::empty(),
    )
    .unwrap();
    dbg!(&res, &buffer);

    assert!(res.addr.is_some());
    assert_eq!(
        String::from_utf8_lossy(&buffer[..res.bytes]),
        "hello, world"
    );

    sendmsg_unix(
        &data_socket,
        &[IoSlice::new(b"goodnight, moon")],
        res.addr.as_ref(),
        SendFlags::empty(),
    )
    .unwrap();
}

fn client(ready: Arc<(Mutex<bool>, Condvar)>) {
    {
        let (lock, cvar) = &*ready;
        let mut ready = lock.lock().unwrap();
        while !*ready {
            ready = cvar.wait(ready).unwrap();
        }
    };

    let addr = SocketAddrUnix::new(SOCKET_NAME).unwrap();

    let data_socket = socket(AddressFamily::UNIX, SocketType::DGRAM, Protocol::default()).unwrap();
    connect_unix(&data_socket, &addr).unwrap();

    sendmsg_unix(
        &data_socket,
        &[IoSlice::new(b"hello, world")],
        None,
        SendFlags::empty(),
    )
    .unwrap();

    let mut buffer = vec![0u8; BUFFER_SIZE];
    let res = recvmsg_unix(
        &data_socket,
        &[IoSliceMut::new(&mut buffer)],
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
fn test_unix_msg() {
    // clear socket
    std::fs::remove_file(SOCKET_NAME).unwrap();

    let ready = Arc::new((Mutex::new(false), Condvar::new()));
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
}
