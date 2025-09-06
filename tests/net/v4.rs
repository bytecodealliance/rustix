//! Test a simple IPv4 socket server and client.
//!
//! The client send a message and the server sends one back.

#![cfg(not(target_os = "wasi"))]

#[cfg(not(any(windows, target_os = "espidf", target_os = "redox", target_os = "wasi")))]
use rustix::net::ReturnFlags;
use rustix::net::{
    accept, bind, connect, getsockname, listen, recv, send, socket, AddressFamily, Ipv4Addr,
    RecvFlags, SendFlags, SocketAddrV4, SocketType,
};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

const BUFFER_SIZE: usize = 20;

fn server(ready: Arc<(Mutex<u16>, Condvar)>) {
    let connection_socket = socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();

    let name = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0);
    bind(&connection_socket, &name).unwrap();

    let who = getsockname(&connection_socket).unwrap();
    let who = SocketAddrV4::try_from(who).unwrap();

    listen(&connection_socket, 1).unwrap();

    {
        let (lock, cvar) = &*ready;
        let mut port = lock.lock().unwrap();
        *port = who.port();
        cvar.notify_all();
    }

    let mut buffer = vec![0; BUFFER_SIZE];
    let data_socket = accept(&connection_socket).unwrap();
    let (nread, actual) = recv(&data_socket, &mut buffer, RecvFlags::empty()).unwrap();
    assert_eq!(String::from_utf8_lossy(&buffer[..nread]), "hello, world");
    assert_eq!(actual, nread);

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

    let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port);
    let mut buffer = vec![0; BUFFER_SIZE];

    let data_socket = socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    connect(&data_socket, &addr).unwrap();

    send(&data_socket, b"hello, world", SendFlags::empty()).unwrap();

    let (nread, actual) = recv(&data_socket, &mut buffer, RecvFlags::empty()).unwrap();
    assert_eq!(String::from_utf8_lossy(&buffer[..nread]), "goodnight, moon");
    assert_eq!(actual, nread);
}

#[test]
fn test_v4() {
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

#[cfg(not(any(windows, target_os = "espidf", target_os = "redox", target_os = "wasi")))]
#[test]
fn test_v4_msg() {
    crate::init();

    use rustix::io::{IoSlice, IoSliceMut};
    use rustix::net::{recvmsg, sendmsg};

    fn server(ready: Arc<(Mutex<u16>, Condvar)>) {
        let connection_socket = socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();

        let name = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0);
        bind(&connection_socket, &name).unwrap();

        let who = getsockname(&connection_socket).unwrap();
        let who = SocketAddrV4::try_from(who).unwrap();

        listen(&connection_socket, 1).unwrap();

        {
            let (lock, cvar) = &*ready;
            let mut port = lock.lock().unwrap();
            *port = who.port();
            cvar.notify_all();
        }

        let mut buffer = vec![0; BUFFER_SIZE];
        let data_socket = accept(&connection_socket).unwrap();
        let res = recvmsg(
            &data_socket,
            &mut [IoSliceMut::new(&mut buffer)],
            &mut Default::default(),
            RecvFlags::empty(),
        )
        .unwrap();
        assert_eq!(
            String::from_utf8_lossy(&buffer[..res.bytes]),
            "hello, world"
        );
        assert_eq!(res.flags, ReturnFlags::empty());

        sendmsg(
            &data_socket,
            &[IoSlice::new(b"goodnight, moon")],
            &mut Default::default(),
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
        let mut buffer = vec![0; BUFFER_SIZE];

        let data_socket = socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
        connect(&data_socket, &addr).unwrap();

        sendmsg(
            &data_socket,
            &[IoSlice::new(b"hello, world")],
            &mut Default::default(),
            SendFlags::empty(),
        )
        .unwrap();

        let res = recvmsg(
            &data_socket,
            &mut [IoSliceMut::new(&mut buffer)],
            &mut Default::default(),
            RecvFlags::empty(),
        )
        .unwrap();
        assert_eq!(
            String::from_utf8_lossy(&buffer[..res.bytes]),
            "goodnight, moon"
        );
        assert_eq!(res.flags, ReturnFlags::empty());
    }

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

#[test]
#[cfg(target_os = "linux")]
fn test_v4_sendmmsg() {
    crate::init();

    use std::net::TcpStream;

    use rustix::io::IoSlice;
    use rustix::net::addr::SocketAddrArg as _;
    use rustix::net::{sendmmsg, MMsgHdr};

    fn server(ready: Arc<(Mutex<u16>, Condvar)>) {
        let connection_socket = socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();

        let name = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0);
        bind(&connection_socket, &name).unwrap();

        let who = getsockname(&connection_socket).unwrap();
        let who = SocketAddrV4::try_from(who).unwrap();

        listen(&connection_socket, 1).unwrap();

        {
            let (lock, cvar) = &*ready;
            let mut port = lock.lock().unwrap();
            *port = who.port();
            cvar.notify_all();
        }

        let mut buffer = vec![0; 13];
        let mut data_socket: TcpStream = accept(&connection_socket).unwrap().into();

        std::io::Read::read_exact(&mut data_socket, &mut buffer).unwrap();
        assert_eq!(String::from_utf8_lossy(&buffer), "hello...world");
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
        let data_socket = socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
        connect(&data_socket, &addr).unwrap();

        let mut off = 0;
        while off < 2 {
            let sent = sendmmsg(
                &data_socket,
                &mut [
                    MMsgHdr::new(&[IoSlice::new(b"hello")], &mut Default::default()),
                    MMsgHdr::new_with_addr(
                        &addr.as_any(),
                        &[IoSlice::new(b"...world")],
                        &mut Default::default(),
                    ),
                ][off..],
                SendFlags::empty(),
            )
            .unwrap();

            off += sent;
        }
    }

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

#[test]
#[cfg(all(target_os = "linux", feature = "time"))]
fn test_v4_txtime() {
    crate::init();

    use std::mem::MaybeUninit;
    use std::time;

    use rustix::io::IoSlice;
    use rustix::net::{sendmsg, sockopt, SendAncillaryBuffer, SendAncillaryMessage, TxTimeFlags};
    use rustix::time::ClockId;

    let data_socket = socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();
    let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 12345);
    connect(&data_socket, &addr).unwrap();

    match sockopt::set_txtime(&data_socket, ClockId::Monotonic, TxTimeFlags::empty()) {
        Ok(_) => (),
        // Skip on unsupported platforms.
        Err(e) if e.to_string().contains("Protocol not available") => return,
        Err(e) => panic!("{e}"),
    }

    let mut space = [MaybeUninit::uninit(); rustix::cmsg_space!(TxTime(1))];
    let mut cmsg_buffer = SendAncillaryBuffer::new(&mut space);

    let t = time::UNIX_EPOCH.elapsed().unwrap() + time::Duration::from_millis(100);
    cmsg_buffer.push(SendAncillaryMessage::TxTime(t.as_nanos() as u64));

    sendmsg(
        &data_socket,
        &[IoSlice::new(b"hello, world")],
        &mut cmsg_buffer,
        SendFlags::empty(),
    )
    .unwrap();
}
