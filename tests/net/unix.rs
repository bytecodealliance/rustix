//! Test a simple Unix-domain socket server and client.
//!
//! The client sends lists of integers and the server sends back sums.

// This test uses `AF_UNIX` with `SOCK_SEQPACKET` which is unsupported on macOS.
#![cfg(not(any(apple, target_os = "redox", target_os = "wasi")))]
// This test uses `DecInt`.
#![cfg(feature = "itoa")]
#![cfg(feature = "fs")]

use rustix::fs::{cwd, unlinkat, AtFlags};
use rustix::io::{read, write};
use rustix::net::{
    accept, bind_unix, connect_unix, listen, socket, AddressFamily, Protocol, SocketAddrUnix,
    SocketType,
};
use rustix::path::DecInt;
use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

const BUFFER_SIZE: usize = 20;

fn server(ready: Arc<(Mutex<bool>, Condvar)>, path: &Path) {
    let connection_socket = socket(
        AddressFamily::UNIX,
        SocketType::SEQPACKET,
        Protocol::default(),
    )
    .unwrap();

    let name = SocketAddrUnix::new(path).unwrap();
    bind_unix(&connection_socket, &name).unwrap();
    listen(&connection_socket, 1).unwrap();

    {
        let (lock, cvar) = &*ready;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_all();
    }

    let mut buffer = vec![0; BUFFER_SIZE];
    'exit: loop {
        let data_socket = accept(&connection_socket).unwrap();
        let mut sum = 0;
        loop {
            let nread = read(&data_socket, &mut buffer).unwrap();

            if &buffer[..nread] == b"exit" {
                break 'exit;
            }
            if &buffer[..nread] == b"sum" {
                break;
            }

            sum += i32::from_str(&String::from_utf8_lossy(&buffer[..nread])).unwrap();
        }

        write(&data_socket, DecInt::new(sum).as_bytes()).unwrap();
    }

    unlinkat(cwd(), path, AtFlags::empty()).unwrap();
}

fn client(ready: Arc<(Mutex<bool>, Condvar)>, path: &Path, runs: &[(&[&str], i32)]) {
    {
        let (lock, cvar) = &*ready;
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }
    }

    let addr = SocketAddrUnix::new(path).unwrap();
    let mut buffer = vec![0; BUFFER_SIZE];

    for (args, sum) in runs {
        let data_socket = socket(
            AddressFamily::UNIX,
            SocketType::SEQPACKET,
            Protocol::default(),
        )
        .unwrap();
        connect_unix(&data_socket, &addr).unwrap();

        for arg in *args {
            write(&data_socket, arg.as_bytes()).unwrap();
        }
        write(&data_socket, b"sum").unwrap();

        let nread = read(&data_socket, &mut buffer).unwrap();
        assert_eq!(
            i32::from_str(&String::from_utf8_lossy(&buffer[..nread])).unwrap(),
            *sum
        );
    }

    let data_socket = socket(
        AddressFamily::UNIX,
        SocketType::SEQPACKET,
        Protocol::default(),
    )
    .unwrap();
    connect_unix(&data_socket, &addr).unwrap();
    write(&data_socket, b"exit").unwrap();
}

#[test]
fn test_unix() {
    let ready = Arc::new((Mutex::new(false), Condvar::new()));
    let ready_clone = Arc::clone(&ready);

    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("soccer");
    let send_path = path.to_owned();
    let server = thread::Builder::new()
        .name("server".to_string())
        .spawn(move || {
            server(ready, &send_path);
        })
        .unwrap();
    let send_path = path.to_owned();
    let client = thread::Builder::new()
        .name("client".to_string())
        .spawn(move || {
            client(
                ready_clone,
                &send_path,
                &[
                    (&["1", "2"], 3),
                    (&["4", "77", "103"], 184),
                    (&["5", "78", "104"], 187),
                    (&[], 0),
                ],
            );
        })
        .unwrap();
    client.join().unwrap();
    server.join().unwrap();
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[test]
fn test_unix_msg() {
    use rustix::io::{IoSlice, IoSliceMut};
    use rustix::net::{recvmsg, sendmsg_noaddr, RecvFlags, SendFlags};
    use std::string::ToString;

    let tmpdir = tempfile::tempdir().unwrap();
    let path = tmpdir.path().join("scp_4804");
    let ready = Arc::new((Mutex::new(false), Condvar::new()));

    let server = {
        let ready = ready.clone();
        let path = path.clone();

        move || {
            let connection_socket = socket(
                AddressFamily::UNIX,
                SocketType::SEQPACKET,
                Protocol::default(),
            )
            .unwrap();

            let name = SocketAddrUnix::new(&path).unwrap();
            bind_unix(&connection_socket, &name).unwrap();
            listen(&connection_socket, 1).unwrap();

            {
                let (lock, cvar) = &*ready;
                let mut started = lock.lock().unwrap();
                *started = true;
                cvar.notify_all();
            }

            let mut buffer = vec![0; BUFFER_SIZE];
            'exit: loop {
                let data_socket = accept(&connection_socket).unwrap();
                let mut sum = 0;
                loop {
                    let nread = recvmsg(
                        &data_socket,
                        &mut [IoSliceMut::new(&mut buffer)],
                        &mut Default::default(),
                        RecvFlags::empty(),
                    )
                    .unwrap()
                    .bytes;

                    if &buffer[..nread] == b"exit" {
                        break 'exit;
                    }
                    if &buffer[..nread] == b"sum" {
                        break;
                    }

                    sum += i32::from_str(&String::from_utf8_lossy(&buffer[..nread])).unwrap();
                }

                let data = sum.to_string();
                sendmsg_noaddr(
                    &data_socket,
                    &[IoSlice::new(data.as_bytes())],
                    &mut Default::default(),
                    SendFlags::empty(),
                )
                .unwrap();
            }

            unlinkat(cwd(), path, AtFlags::empty()).unwrap();
        }
    };

    let client = move || {
        {
            let (lock, cvar) = &*ready;
            let mut started = lock.lock().unwrap();
            while !*started {
                started = cvar.wait(started).unwrap();
            }
        }

        let addr = SocketAddrUnix::new(path).unwrap();
        let mut buffer = vec![0; BUFFER_SIZE];
        let runs: &[(&[&str], i32)] = &[
            (&["1", "2"], 3),
            (&["4", "77", "103"], 184),
            (&["5", "78", "104"], 187),
            (&[], 0),
        ];

        for (args, sum) in runs {
            let data_socket = socket(
                AddressFamily::UNIX,
                SocketType::SEQPACKET,
                Protocol::default(),
            )
            .unwrap();
            connect_unix(&data_socket, &addr).unwrap();

            for arg in *args {
                sendmsg_noaddr(
                    &data_socket,
                    &[IoSlice::new(arg.as_bytes())],
                    &mut Default::default(),
                    SendFlags::empty(),
                )
                .unwrap();
            }
            sendmsg_noaddr(
                &data_socket,
                &[IoSlice::new(b"sum")],
                &mut Default::default(),
                SendFlags::empty(),
            )
            .unwrap();

            let nread = recvmsg(
                &data_socket,
                &mut [IoSliceMut::new(&mut buffer)],
                &mut Default::default(),
                RecvFlags::empty(),
            )
            .unwrap()
            .bytes;
            assert_eq!(
                i32::from_str(&String::from_utf8_lossy(&buffer[..nread])).unwrap(),
                *sum
            );
        }

        let data_socket = socket(
            AddressFamily::UNIX,
            SocketType::SEQPACKET,
            Protocol::default(),
        )
        .unwrap();
        connect_unix(&data_socket, &addr).unwrap();
        sendmsg_noaddr(
            &data_socket,
            &[IoSlice::new(b"exit")],
            &mut Default::default(),
            SendFlags::empty(),
        )
        .unwrap();
    };

    let server = thread::Builder::new()
        .name("server".to_string())
        .spawn(move || {
            server();
        })
        .unwrap();

    let client = thread::Builder::new()
        .name("client".to_string())
        .spawn(move || {
            client();
        })
        .unwrap();

    client.join().unwrap();
    server.join().unwrap();
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[test]
fn test_unix_msg_with_scm_rights() {
    use rustix::fd::AsFd;
    use rustix::io::{pipe, IoSlice, IoSliceMut};
    use rustix::net::{
        recvmsg, sendmsg_noaddr, RecvAncillaryBuffer, RecvAncillaryMessage, RecvFlags,
        SendAncillaryBuffer, SendAncillaryMessage, SendFlags,
    };
    use std::string::ToString;

    let tmpdir = tempfile::tempdir().unwrap();
    let path = tmpdir.path().join("scp_4804");
    let ready = Arc::new((Mutex::new(false), Condvar::new()));

    let server = {
        let ready = ready.clone();
        let path = path.clone();

        move || {
            let connection_socket = socket(
                AddressFamily::UNIX,
                SocketType::SEQPACKET,
                Protocol::default(),
            )
            .unwrap();
            let mut pipe_end = None;

            let name = SocketAddrUnix::new(&path).unwrap();
            bind_unix(&connection_socket, &name).unwrap();
            listen(&connection_socket, 1).unwrap();

            {
                let (lock, cvar) = &*ready;
                let mut started = lock.lock().unwrap();
                *started = true;
                cvar.notify_all();
            }

            let mut buffer = vec![0; BUFFER_SIZE];
            let mut cmsg_space = vec![0; rustix::cmsg_space!(ScmRights(1))];

            'exit: loop {
                let data_socket = accept(&connection_socket).unwrap();
                let mut sum = 0;
                loop {
                    let mut cmsg_buffer = RecvAncillaryBuffer::new(&mut cmsg_space);
                    let nread = recvmsg(
                        &data_socket,
                        &mut [IoSliceMut::new(&mut buffer)],
                        &mut cmsg_buffer,
                        RecvFlags::empty(),
                    )
                    .unwrap()
                    .bytes;

                    // Read out the pipe if we got it.
                    if let Some(end) = cmsg_buffer
                        .drain()
                        .filter_map(|msg| match msg {
                            RecvAncillaryMessage::ScmRights(rights) => Some(rights),
                            _ => None,
                        })
                        .flatten()
                        .next()
                    {
                        pipe_end = Some(end);
                    }

                    if &buffer[..nread] == b"exit" {
                        break 'exit;
                    }
                    if &buffer[..nread] == b"sum" {
                        break;
                    }

                    sum += i32::from_str(&String::from_utf8_lossy(&buffer[..nread])).unwrap();
                }

                let data = sum.to_string();
                sendmsg_noaddr(
                    &data_socket,
                    &[IoSlice::new(data.as_bytes())],
                    &mut Default::default(),
                    SendFlags::empty(),
                )
                .unwrap();
            }

            unlinkat(cwd(), path, AtFlags::empty()).unwrap();

            // Once we're done, send a message along the pipe.
            let pipe = pipe_end.unwrap();
            write(&pipe, b"pipe message!").unwrap();
        }
    };

    let client = move || {
        {
            let (lock, cvar) = &*ready;
            let mut started = lock.lock().unwrap();
            while !*started {
                started = cvar.wait(started).unwrap();
            }
        }

        let addr = SocketAddrUnix::new(path).unwrap();
        let (read_end, write_end) = pipe().unwrap();
        let mut buffer = vec![0; BUFFER_SIZE];
        let runs: &[(&[&str], i32)] = &[
            (&["1", "2"], 3),
            (&["4", "77", "103"], 184),
            (&["5", "78", "104"], 187),
            (&[], 0),
        ];

        for (args, sum) in runs {
            let data_socket = socket(
                AddressFamily::UNIX,
                SocketType::SEQPACKET,
                Protocol::default(),
            )
            .unwrap();
            connect_unix(&data_socket, &addr).unwrap();

            for arg in *args {
                sendmsg_noaddr(
                    &data_socket,
                    &[IoSlice::new(arg.as_bytes())],
                    &mut Default::default(),
                    SendFlags::empty(),
                )
                .unwrap();
            }
            sendmsg_noaddr(
                &data_socket,
                &[IoSlice::new(b"sum")],
                &mut Default::default(),
                SendFlags::empty(),
            )
            .unwrap();

            let nread = recvmsg(
                &data_socket,
                &mut [IoSliceMut::new(&mut buffer)],
                &mut Default::default(),
                RecvFlags::empty(),
            )
            .unwrap()
            .bytes;
            assert_eq!(
                i32::from_str(&String::from_utf8_lossy(&buffer[..nread])).unwrap(),
                *sum
            );
        }

        let data_socket = socket(
            AddressFamily::UNIX,
            SocketType::SEQPACKET,
            Protocol::default(),
        )
        .unwrap();

        // Format the CMSG.
        let we = [write_end.as_fd()];
        let msg = SendAncillaryMessage::ScmRights(&we);
        let mut space = vec![0; msg.size()];
        let mut cmsg_buffer = SendAncillaryBuffer::new(&mut space);
        assert!(cmsg_buffer.push(msg));

        connect_unix(&data_socket, &addr).unwrap();
        sendmsg_noaddr(
            &data_socket,
            &[IoSlice::new(b"exit")],
            &mut cmsg_buffer,
            SendFlags::empty(),
        )
        .unwrap();

        // Read a value from the pipe.
        let mut buffer = [0u8; 13];
        read(&read_end, &mut buffer).unwrap();
        assert_eq!(&buffer, b"pipe message!".as_ref());
    };

    let server = thread::Builder::new()
        .name("server".to_string())
        .spawn(move || {
            server();
        })
        .unwrap();

    let client = thread::Builder::new()
        .name("client".to_string())
        .spawn(move || {
            client();
        })
        .unwrap();

    client.join().unwrap();
    server.join().unwrap();
}
