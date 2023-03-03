use rustix::io::{epoll, ioctl_fionbio, read, write};
use rustix::net::{
    accept, bind_v4, connect_v4, getsockname, listen, socket, AddressFamily, Ipv4Addr, Protocol,
    SocketAddrAny, SocketAddrV4, SocketType,
};
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

const BUFFER_SIZE: usize = 20;

fn server(ready: Arc<(Mutex<u16>, Condvar)>) {
    let listen_sock = socket(AddressFamily::INET, SocketType::STREAM, Protocol::default()).unwrap();
    bind_v4(&listen_sock, &SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0)).unwrap();
    listen(&listen_sock, 1).unwrap();

    let who = match getsockname(&listen_sock).unwrap() {
        SocketAddrAny::V4(addr) => addr,
        _ => panic!(),
    };

    {
        let (lock, cvar) = &*ready;
        let mut port = lock.lock().unwrap();
        *port = who.port();
        cvar.notify_all();
    }

    let epoll = epoll::epoll_create(epoll::CreateFlags::CLOEXEC).unwrap();

    epoll::epoll_add(&epoll, &listen_sock, 1, epoll::EventFlags::IN).unwrap();

    let mut next_data = 2;
    let mut targets = HashMap::new();

    let mut event_list = epoll::EventVec::with_capacity(4);
    loop {
        epoll::epoll_wait(&epoll, &mut event_list, -1).unwrap();
        for (_event_flags, target) in &event_list {
            if target == 1 {
                let conn_sock = accept(&listen_sock).unwrap();
                ioctl_fionbio(&conn_sock, true).unwrap();
                epoll::epoll_add(
                    &epoll,
                    &conn_sock,
                    next_data,
                    epoll::EventFlags::OUT | epoll::EventFlags::ET,
                )
                .unwrap();
                targets.insert(next_data, conn_sock);
                next_data += 1;
            } else {
                let target = targets.remove(&target).unwrap();
                write(&target, b"hello\n").unwrap();
                epoll::epoll_del(&epoll, &target).unwrap();
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

    let _server = thread::Builder::new()
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
}
