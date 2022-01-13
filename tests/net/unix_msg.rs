#![cfg(not(any(target_os = "redox", target_os = "wasi")))]

use rustix::fs::{cwd, unlinkat, AtFlags};
use rustix::net::{
    bind_unix, connect_unix, recvmsg_unix, sendmsg_unix, socket, socketpair, AddressFamily,
    Protocol, RecvFlags, SendFlags, SocketAddrUnix, SocketType,
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
        &mut [IoSliceMut::new(&mut buffer)],
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
        &mut [IoSliceMut::new(&mut buffer)],
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

// Ported from https://github.com/nix-rust/nix/blob/master/test/sys/test_socket.rs
// Original License: MIT
#[cfg(any(target_os = "android", target_os = "linux"))]
#[test]
fn test_scm_credentials_and_rights() {
    use io_lifetimes::AsFd;
    use rustix::cmsg_buffer;
    use rustix::io::{pipe, read, write, OwnedFd};
    use rustix::net::{
        recvmsg_unix_with_ancillary, sendmsg_unix_with_ancillary, RecvAncillaryDataUnix,
        RecvSocketAncillaryUnix, SendSocketAncillaryUnix, SocketCred,
    };
    use rustix::net::{sockopt::set_socket_passcred, SocketFlags};
    use rustix::process::{getgid, getpid, getuid};

    let mut space = cmsg_buffer!(OwnedFd, SocketCred);

    let (send, recv) = socketpair(
        AddressFamily::UNIX,
        SocketType::STREAM,
        SocketFlags::empty(),
        Protocol::default(),
    )
    .unwrap();
    set_socket_passcred(&recv, true).unwrap();

    let (r, w) = pipe().unwrap();
    let mut received_r = None;

    {
        let iovs = [IoSlice::new(b"hello")];
        let mut cmsgs = SendSocketAncillaryUnix::new(&mut space);
        let cred = SocketCred::from_process();
        assert!(cmsgs.add_creds(&[cred]));

        cmsgs.add_fds(&[r.as_fd()]);
        assert_eq!(
            sendmsg_unix_with_ancillary(&send, &iovs, None, &mut cmsgs, SendFlags::empty())
                .unwrap(),
            5
        );
        drop(r);
        drop(send);
    }

    {
        let mut buf = [0u8; 5];
        let mut iovs = [IoSliceMut::new(&mut buf[..])];
        let mut cmsgs = RecvSocketAncillaryUnix::new(&mut space);
        let msg =
            recvmsg_unix_with_ancillary(&recv, &mut iovs, &mut cmsgs, RecvFlags::empty()).unwrap();

        assert_eq!(cmsgs.messages().count(), 2, "expected 2 cmsgs");
        let mut received_cred = None;
        for cmsg in cmsgs.messages() {
            match cmsg.unwrap() {
                RecvAncillaryDataUnix::ScmRights(fds) => {
                    assert!(received_r.is_none(), "already received fd");
                    let fds = fds.collect::<Vec<_>>();
                    assert_eq!(fds.len(), 1);
                    received_r = Some(fds);
                }
                RecvAncillaryDataUnix::ScmCredentials(creds) => {
                    assert!(received_cred.is_none());
                    let creds = creds.collect::<Vec<_>>();
                    assert_eq!(creds.len(), 1);
                    assert_eq!(creds[0].get_pid(), Some(getpid()));
                    assert_eq!(creds[0].get_uid(), getuid());
                    assert_eq!(creds[0].get_gid(), getgid());
                    received_cred = Some(creds);
                }
                _ => panic!("unexpected cmsg"),
            }
        }
        received_cred.expect("no creds received");
        assert_eq!(msg.bytes, 5);
        assert!(!msg.flags.contains(RecvFlags::TRUNC));
        assert!(!msg.flags.contains(RecvFlags::CTRUNC));

        drop(recv);
    }

    let received_r = received_r.expect("Did not receive passed fd");
    // Ensure that the received file descriptor works
    write(&w, b"world").unwrap();
    let mut buf = [0u8; 5];
    read(&received_r[0], &mut buf).unwrap();
    assert_eq!(&buf[..], b"world");
}
