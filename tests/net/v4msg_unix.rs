//! Test a simple IPv4 socket server and client. The client send a
//! message and the server sends one back, uses `sendmsg` and `recvmsg`.

#![cfg(not(any(target_os = "redox", target_os = "wasi")))]

use rustix::fs::{cwd, unlinkat, AtFlags};
use rustix::net::{
    bind_unix, connect_unix, recvmsg_unix, sendmsg_unix, socket, AddressFamily, Protocol,
    RecvFlags, SendFlags, SocketAddrUnix, SocketType,
};
use std::io::{IoSlice, IoSliceMut};
use std::path::Path;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

const BUFFER_SIZE: usize = 20;

fn server(ready: Arc<(Mutex<bool>, Condvar)>, path: &Path) {
    let connection_socket =
        socket(AddressFamily::UNIX, SocketType::DGRAM, Protocol::default()).unwrap();

    let name = SocketAddrUnix::new(path).unwrap();
    bind_unix(&connection_socket, &name).unwrap();
    {
        let (lock, cvar) = &*ready;
        let mut started = lock.lock().unwrap();
        *started = true;
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

    unlinkat(&cwd(), path, AtFlags::empty()).unwrap();
}

fn client(ready: Arc<(Mutex<bool>, Condvar)>, server_path: &Path, client_path: &Path) {
    {
        let (lock, cvar) = &*ready;
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }
    };

    let server_addr = SocketAddrUnix::new(server_path).unwrap();
    let client_addr = SocketAddrUnix::new(client_path).unwrap();

    let data_socket = socket(AddressFamily::UNIX, SocketType::DGRAM, Protocol::default()).unwrap();

    // bind client
    bind_unix(&data_socket, &client_addr).unwrap();

    // connect to the server
    connect_unix(&data_socket, &server_addr).unwrap();

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
    let ready = Arc::new((Mutex::new(false), Condvar::new()));
    let ready_clone = Arc::clone(&ready);

    let tmp = tempfile::tempdir().unwrap();
    let server_path = tmp.path().join("foo-server");
    let client_path = tmp.path().join("foo-client");

    let server_send_path = server_path.to_owned();
    let server = thread::Builder::new()
        .name("server".to_string())
        .spawn(move || {
            server(ready, &server_send_path);
        })
        .unwrap();

    let server_send_path = server_path.to_owned();
    let client_send_path = client_path.to_owned();
    let client = thread::Builder::new()
        .name("client".to_string())
        .spawn(move || {
            client(ready_clone, &server_send_path, &client_send_path);
        })
        .unwrap();
    client.join().unwrap();
    server.join().unwrap();
}
