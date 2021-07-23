//! Test a simple IPv6 socket server and client. The client send a
//! message and the server sends one back.

#![cfg(not(any(target_os = "redox", target_os = "wasi")))]

use posish::io::{read, write};
use posish::net::{
    accept, bind_v6, connect_v6, getsockname, listen, socket, AddressFamily, Ipv6Addr, Protocol,
    SocketAddr, SocketAddrV6, SocketType,
};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

const BUFFER_SIZE: usize = 20;

fn server(ready: Arc<(Mutex<u16>, Condvar)>) {
    let connection_socket = socket(
        AddressFamily::INET6,
        SocketType::STREAM,
        Protocol::default(),
    )
    .unwrap();

    let name = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 0, 0, 0);
    bind_v6(&connection_socket, &name).unwrap();

    let who = match getsockname(&connection_socket).unwrap() {
        SocketAddr::V6(addr) => addr,
        _ => panic!(),
    };

    listen(&connection_socket, 1).unwrap();

    {
        let (lock, cvar) = &*ready;
        let mut port = lock.lock().unwrap();
        *port = who.port();
        cvar.notify_all();
    }

    let mut buffer = vec![0; BUFFER_SIZE];
    let data_socket = accept(&connection_socket).unwrap();
    let nread = read(&data_socket, &mut buffer).unwrap();
    assert_eq!(String::from_utf8_lossy(&buffer[..nread]), "hello, world");

    write(&data_socket, b"goodnight, moon").unwrap();
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

    let data_socket = socket(
        AddressFamily::INET6,
        SocketType::STREAM,
        Protocol::default(),
    )
    .unwrap();
    connect_v6(&data_socket, &addr).unwrap();

    write(&data_socket, b"hello, world").unwrap();

    let nread = read(&data_socket, &mut buffer).unwrap();
    assert_eq!(String::from_utf8_lossy(&buffer[..nread]), "goodnight, moon");
}

#[test]
fn test_v6() {
    let ready = Arc::new((Mutex::new(0_u16), Condvar::new()));
    let ready_clone = Arc::clone(&ready);

    let server = thread::spawn(move || {
        server(ready);
    });
    let client = thread::spawn(move || {
        client(ready_clone);
    });
    client.join().unwrap();
    server.join().unwrap();
}
