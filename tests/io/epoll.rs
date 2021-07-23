#![cfg(any(target_os = "android", target_os = "linux"))]

use posish::io::{
    epoll::{self, Epoll},
    ioctl_fionbio, read, write,
};
use posish::net::{
    accept, bind_v4, connect_v4, getsockname, listen, socket, AddressFamily, Ipv4Addr, Protocol,
    SocketAddr, SocketAddrV4, SocketType,
};
use std::os::unix::io::AsRawFd;
use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
};

const BUFFER_SIZE: usize = 20;

fn server(ready: Arc<(Mutex<u16>, Condvar)>) {
    let listen_sock = socket(AddressFamily::INET, SocketType::STREAM, Protocol::default()).unwrap();
    bind_v4(&listen_sock, &SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0)).unwrap();
    listen(&listen_sock, 1).unwrap();

    let who = match getsockname(&listen_sock).unwrap() {
        SocketAddr::V4(addr) => addr,
        _ => panic!(),
    };

    {
        let (lock, cvar) = &*ready;
        let mut port = lock.lock().unwrap();
        *port = who.port();
        cvar.notify_all();
    }

    let epoll = Epoll::new(epoll::CreateFlags::CLOEXEC, epoll::Owning::new()).unwrap();

    let raw_listen_sock = listen_sock.as_raw_fd();
    epoll.add(listen_sock, epoll::EventFlags::IN).unwrap();

    let mut event_list = epoll::EventVec::with_capacity(4);
    loop {
        epoll.wait(&mut event_list, -1).unwrap();
        for (_event_flags, target) in &event_list {
            if target.as_raw_fd() == raw_listen_sock {
                let conn_sock = accept(&*target).unwrap();
                ioctl_fionbio(&conn_sock, true).unwrap();
                epoll
                    .add(conn_sock, epoll::EventFlags::OUT | epoll::EventFlags::ET)
                    .unwrap();
            } else {
                write(&*target, b"hello\n").unwrap();
                let _ = epoll.del(target).unwrap();
            }
        }
    }
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

    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);
    let mut buffer = vec![0; BUFFER_SIZE];

    for _ in 0..16 {
        let data_socket =
            socket(AddressFamily::INET, SocketType::STREAM, Protocol::default()).unwrap();
        connect_v4(&data_socket, &addr).unwrap();

        let nread = read(&data_socket, &mut buffer).unwrap();
        assert_eq!(String::from_utf8_lossy(&buffer[..nread]), "hello\n");
    }
}

#[test]
fn test_epoll() {
    let ready = Arc::new((Mutex::new(0_u16), Condvar::new()));
    let ready_clone = Arc::clone(&ready);

    let _server = thread::spawn(move || {
        server(ready);
    });
    let client = thread::spawn(move || {
        client(ready_clone);
    });
    client.join().unwrap();
}
