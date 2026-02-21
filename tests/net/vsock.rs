//! Test a simple IPv4 socket server and client.
//!
//! The client send a message and the server sends one back.

#![cfg(any(linux_kernel, apple))]

use rustix::net::vsock::{SocketAddrVSock, VMADDR_CID_LOCAL};
use rustix::net::{
    accept, bind, connect, getsockname, listen, recv, send, socket, AddressFamily, RecvFlags,
    ReturnFlags, SendFlags, SocketType,
};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

const BUFFER_SIZE: usize = 20;

/// Only run vsock tests if it is supported on the current machine.
fn vsock_supported() -> bool {
    let sock = match socket(AddressFamily::VSOCK, SocketType::STREAM, None) {
        Ok(sock) => sock,
        Err(rustix::io::Errno::AFNOSUPPORT)
        | Err(rustix::io::Errno::NOTSUP)
        | Err(rustix::io::Errno::NODEV) => return false,
        Err(_) => return true,
    };

    match bind(&sock, &SocketAddrVSock::new(VMADDR_CID_LOCAL, 0x1230)) {
        Ok(_) => true,
        Err(rustix::io::Errno::ADDRNOTAVAIL) => false,
        Err(_) => true,
    }
}

fn server(ready: Arc<(Mutex<u32>, Condvar)>) {
    let connection_socket = socket(AddressFamily::VSOCK, SocketType::STREAM, None).unwrap();

    let name = SocketAddrVSock::new(VMADDR_CID_LOCAL, 0x1234);
    bind(&connection_socket, &name).unwrap();

    let who = getsockname(&connection_socket).unwrap();
    let who = SocketAddrVSock::try_from(who).unwrap();

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

fn client(ready: Arc<(Mutex<u32>, Condvar)>) {
    let port = {
        let (lock, cvar) = &*ready;
        let mut port = lock.lock().unwrap();
        while *port == 0 {
            port = cvar.wait(port).unwrap();
        }
        *port
    };

    let addr = SocketAddrVSock::new(VMADDR_CID_LOCAL, port);
    let mut buffer = vec![0; BUFFER_SIZE];

    let data_socket = socket(AddressFamily::VSOCK, SocketType::STREAM, None).unwrap();
    connect(&data_socket, &addr).unwrap();

    send(&data_socket, b"hello, world", SendFlags::empty()).unwrap();

    let (nread, actual) = recv(&data_socket, &mut buffer, RecvFlags::empty()).unwrap();
    assert_eq!(String::from_utf8_lossy(&buffer[..nread]), "goodnight, moon");
    assert_eq!(actual, nread);
}

#[test]
fn test_vsock() {
    crate::init();

    if !vsock_supported() {
        return;
    }

    let ready = Arc::new((Mutex::new(0_u32), Condvar::new()));
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
fn test_vsock_msg() {
    crate::init();

    if !vsock_supported() {
        return;
    }

    use rustix::io::{IoSlice, IoSliceMut};
    use rustix::net::{recvmsg, sendmsg};

    fn server(ready: Arc<(Mutex<u32>, Condvar)>) {
        let connection_socket = socket(AddressFamily::VSOCK, SocketType::STREAM, None).unwrap();

        let name = SocketAddrVSock::new(VMADDR_CID_LOCAL, 0x1238);
        bind(&connection_socket, &name).unwrap();

        let who = getsockname(&connection_socket).unwrap();
        let who = SocketAddrVSock::try_from(who).unwrap();

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

    fn client(ready: Arc<(Mutex<u32>, Condvar)>) {
        let port = {
            let (lock, cvar) = &*ready;
            let mut port = lock.lock().unwrap();
            while *port == 0 {
                port = cvar.wait(port).unwrap();
            }
            *port
        };

        let addr = SocketAddrVSock::new(VMADDR_CID_LOCAL, port);
        let mut buffer = vec![0; BUFFER_SIZE];

        let data_socket = socket(AddressFamily::VSOCK, SocketType::STREAM, None).unwrap();
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

    let ready = Arc::new((Mutex::new(0_u32), Condvar::new()));
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

#[cfg(target_os = "linux")]
#[test]
// TODO(notgull): Figure out why this test keeps failing.
#[ignore]
fn test_vsock_sendmmsg() {
    crate::init();

    if !vsock_supported() {
        return;
    }

    use std::net::TcpStream;

    use rustix::io::IoSlice;
    use rustix::net::addr::SocketAddrArg as _;
    use rustix::net::{sendmmsg, MMsgHdr};

    fn server(ready: Arc<(Mutex<u32>, Condvar)>) {
        let connection_socket = socket(AddressFamily::VSOCK, SocketType::STREAM, None).unwrap();

        let name = SocketAddrVSock::new(VMADDR_CID_LOCAL, 0x1236);
        bind(&connection_socket, &name).unwrap();

        let who = getsockname(&connection_socket).unwrap();
        let who = SocketAddrVSock::try_from(who).unwrap();

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

    fn client(ready: Arc<(Mutex<u32>, Condvar)>) {
        let port = {
            let (lock, cvar) = &*ready;
            let mut port = lock.lock().unwrap();
            while *port == 0 {
                port = cvar.wait(port).unwrap();
            }
            *port
        };

        let addr = SocketAddrVSock::new(VMADDR_CID_LOCAL, port);
        let data_socket = socket(AddressFamily::VSOCK, SocketType::STREAM, None).unwrap();
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

    let ready = Arc::new((Mutex::new(0_u32), Condvar::new()));
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
