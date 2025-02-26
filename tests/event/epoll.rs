use rustix::buffer::spare_capacity;
use rustix::event::epoll;
use rustix::io::{ioctl_fionbio, read, write};
use rustix::net::{
    accept, bind, connect, getsockname, listen, socket, AddressFamily, Ipv4Addr, SocketAddrV4,
    SocketType,
};
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

const BUFFER_SIZE: usize = 20;

fn server(ready: Arc<(Mutex<u16>, Condvar)>) -> ! {
    let listen_sock = socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    bind(&listen_sock, &SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0)).unwrap();
    listen(&listen_sock, 1).unwrap();

    let who = SocketAddrV4::try_from(getsockname(&listen_sock).unwrap()).unwrap();

    {
        let (lock, cvar) = &*ready;
        let mut port = lock.lock().unwrap();
        *port = who.port();
        cvar.notify_all();
    }

    let epoll = epoll::create(epoll::CreateFlags::CLOEXEC).unwrap();

    epoll::add(
        &epoll,
        &listen_sock,
        epoll::EventData::new_u64(1),
        epoll::EventFlags::IN,
    )
    .unwrap();

    let mut next_data = epoll::EventData::new_u64(2);
    let mut targets = HashMap::new();

    let mut event_list = Vec::with_capacity(4);
    loop {
        epoll::wait(&epoll, spare_capacity(&mut event_list), None).unwrap();
        for event in event_list.drain(..) {
            let target = event.data;
            if target.u64() == 1 {
                let conn_sock = accept(&listen_sock).unwrap();
                ioctl_fionbio(&conn_sock, true).unwrap();
                epoll::add(
                    &epoll,
                    &conn_sock,
                    next_data,
                    epoll::EventFlags::OUT | epoll::EventFlags::ET,
                )
                .unwrap();
                targets.insert(next_data, conn_sock);
                next_data = epoll::EventData::new_u64(next_data.u64() + 1);
            } else {
                let target = targets.remove(&target).unwrap();
                write(&target, b"hello\n").unwrap();
                epoll::delete(&epoll, &target).unwrap();
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
        let data_socket = socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
        connect(&data_socket, &addr).unwrap();

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

#[test]
fn test_epoll_event_data() {
    let d = epoll::EventData::new_u64(0);
    assert_eq!(d.u64(), 0);
    assert_eq!(d.ptr() as u64, 0);
    let d = epoll::EventData::new_u64(1);
    assert_eq!(d.u64(), 1);
    assert_eq!(d.ptr() as u64, 1);
    let d = epoll::EventData::new_u64(!5);
    assert_eq!(d.u64(), !5);
    assert_eq!(d.ptr() as u64, !5 as *mut c_void as u64);
    let d = epoll::EventData::new_ptr(core::ptr::null_mut());
    assert_eq!(d.u64(), 0);
    assert!(d.ptr().is_null());
    let d = epoll::EventData::new_ptr(3 as *mut c_void);
    assert_eq!(d.u64(), 3);
    assert_eq!(d.ptr() as u64, 3);
    let d = epoll::EventData::new_ptr(!3 as *mut c_void);
    assert_eq!(d.u64(), !3 as *mut c_void as u64);
    assert_eq!(d.ptr() as u64, !3 as *mut c_void as u64);
}
