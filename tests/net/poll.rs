//! The same as v6.rs, but with `poll` calls.

#![cfg(not(target_os = "wasi"))]

use rustix::event::{poll, PollFd, PollFlags};
use rustix::net::{
    accept, bind_v6, connect_v6, getsockname, listen, recv, send, socket, AddressFamily, Ipv6Addr,
    RecvFlags, SendFlags, SocketAddrAny, SocketAddrV6, SocketType,
};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

const BUFFER_SIZE: usize = 20;

fn server(ready: Arc<(Mutex<u16>, Condvar)>) {
    let connection_socket = socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();

    let name = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 0, 0, 0);
    bind_v6(&connection_socket, &name).unwrap();

    let who = match getsockname(&connection_socket).unwrap() {
        SocketAddrAny::V6(addr) => addr,
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

    let mut fds = [PollFd::new(&data_socket, PollFlags::IN)];
    assert_eq!(poll(&mut fds, -1).unwrap(), 1);
    assert!(fds[0].revents().intersects(PollFlags::IN));
    assert!(!fds[0].revents().intersects(PollFlags::OUT));

    #[cfg(not(target_os = "espidf"))]
    {
        let expected_nread = rustix::io::ioctl_fionread(&data_socket).unwrap();
        let nread = recv(&data_socket, &mut buffer, RecvFlags::empty()).unwrap();
        assert_eq!(String::from_utf8_lossy(&buffer[..nread]), "hello, world");
        assert_eq!(expected_nread, nread as u64);
    }

    let mut fds = [PollFd::new(&data_socket, PollFlags::OUT)];
    assert_eq!(poll(&mut fds, -1).unwrap(), 1);
    assert!(!fds[0].revents().intersects(PollFlags::IN));
    assert!(fds[0].revents().intersects(PollFlags::OUT));

    send(&data_socket, b"goodnight, moon", SendFlags::empty()).unwrap();
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

    let data_socket = socket(AddressFamily::INET6, SocketType::STREAM, None).unwrap();
    connect_v6(&data_socket, &addr).unwrap();

    let mut fds = [PollFd::new(&data_socket, PollFlags::OUT)];
    assert_eq!(poll(&mut fds, -1).unwrap(), 1);
    assert!(!fds[0].revents().intersects(PollFlags::IN));
    assert!(fds[0].revents().intersects(PollFlags::OUT));

    send(&data_socket, b"hello, world", SendFlags::empty()).unwrap();

    let mut fds = [PollFd::new(&data_socket, PollFlags::IN)];
    assert_eq!(poll(&mut fds, -1).unwrap(), 1);
    assert!(fds[0].revents().intersects(PollFlags::IN));
    assert!(!fds[0].revents().intersects(PollFlags::OUT));

    #[cfg(not(target_os = "espidf"))]
    {
        let expected_nread = rustix::io::ioctl_fionread(&data_socket).unwrap();
        let nread = recv(&data_socket, &mut buffer, RecvFlags::empty()).unwrap();
        assert_eq!(String::from_utf8_lossy(&buffer[..nread]), "goodnight, moon");
        assert_eq!(expected_nread, nread as u64);
    }
}

#[test]
fn test_poll() {
    crate::init();

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
}
