//! Like unix.rs, but uses `Vec`s for the buffers.

// This test uses `AF_UNIX` with `SOCK_SEQPACKET` which is unsupported on
// macOS.
#![cfg(not(any(apple, target_os = "espidf", target_os = "redox", target_os = "wasi")))]
#![cfg(feature = "fs")]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use rustix::fs::{unlinkat, AtFlags, CWD};
use rustix::io::{read, write};
use rustix::net::{
    accept, bind, connect, listen, socket, AddressFamily, SocketAddrUnix, SocketType,
};
use rustix::path::DecInt;
use std::path::Path;
use std::str::FromStr as _;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

const BUFFER_SIZE: usize = 20;

fn server(ready: Arc<(Mutex<bool>, Condvar)>, path: &Path) {
    let connection_socket = socket(AddressFamily::UNIX, SocketType::SEQPACKET, None).unwrap();

    let name = SocketAddrUnix::new(path).unwrap();
    bind(&connection_socket, &name).unwrap();
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

    unlinkat(CWD, path, AtFlags::empty()).unwrap();
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
        let data_socket = socket(AddressFamily::UNIX, SocketType::SEQPACKET, None).unwrap();
        connect(&data_socket, &addr).unwrap();

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

    let data_socket = socket(AddressFamily::UNIX, SocketType::SEQPACKET, None).unwrap();
    connect(&data_socket, &addr).unwrap();
    write(&data_socket, b"exit").unwrap();
}

#[test]
#[cfg(not(target_os = "freebsd"))] // TODO: Investigate why these tests fail on FreeBSD.
fn test_unix() {
    crate::init();

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

#[cfg(not(any(target_os = "espidf", target_os = "redox", target_os = "wasi")))]
#[cfg(not(target_os = "freebsd"))] // TODO: Investigate why these tests fail on FreeBSD.
fn do_test_unix_msg(addr: SocketAddrUnix) {
    use rustix::io::{IoSlice, IoSliceMut};
    use rustix::net::{recvmsg, sendmsg, RecvFlags, ReturnFlags, SendFlags};

    let server = {
        let connection_socket = socket(AddressFamily::UNIX, SocketType::SEQPACKET, None).unwrap();
        bind(&connection_socket, &addr).unwrap();
        listen(&connection_socket, 1).unwrap();

        move || {
            let mut buffer = vec![0; BUFFER_SIZE];
            'exit: loop {
                let data_socket = accept(&connection_socket).unwrap();
                let mut sum = 0;
                loop {
                    let result = recvmsg(
                        &data_socket,
                        &mut [IoSliceMut::new(&mut buffer)],
                        &mut Default::default(),
                        RecvFlags::empty(),
                    )
                    .unwrap();
                    let nread = result.bytes;

                    assert_eq!(result.flags, ReturnFlags::empty());

                    if &buffer[..nread] == b"exit" {
                        break 'exit;
                    }
                    if &buffer[..nread] == b"sum" {
                        break;
                    }

                    sum += i32::from_str(&String::from_utf8_lossy(&buffer[..nread])).unwrap();
                }

                let data = sum.to_string();
                sendmsg(
                    &data_socket,
                    &[IoSlice::new(data.as_bytes())],
                    &mut Default::default(),
                    SendFlags::empty(),
                )
                .unwrap();
            }
        }
    };

    let client = move || {
        let mut buffer = vec![0; BUFFER_SIZE];
        let runs: &[(&[&str], i32)] = &[
            (&["1", "2"], 3),
            (&["4", "77", "103"], 184),
            (&["5", "78", "104"], 187),
            (&[], 0),
        ];

        for (args, sum) in runs {
            let data_socket = socket(AddressFamily::UNIX, SocketType::SEQPACKET, None).unwrap();
            connect(&data_socket, &addr).unwrap();

            for arg in *args {
                sendmsg(
                    &data_socket,
                    &[IoSlice::new(arg.as_bytes())],
                    &mut Default::default(),
                    SendFlags::empty(),
                )
                .unwrap();
            }
            sendmsg(
                &data_socket,
                &[IoSlice::new(b"sum")],
                &mut Default::default(),
                SendFlags::empty(),
            )
            .unwrap();

            let result = recvmsg(
                &data_socket,
                &mut [IoSliceMut::new(&mut buffer)],
                &mut Default::default(),
                RecvFlags::empty(),
            )
            .unwrap();
            let nread = result.bytes;
            assert_eq!(
                i32::from_str(&String::from_utf8_lossy(&buffer[..nread])).unwrap(),
                *sum
            );
            // Don't ask me why, but this was seen to fail on FreeBSD.
            // `SocketAddrUnix::path()` returned `None` for some reason.
            // illumos and NetBSD too.
            #[cfg(not(any(solarish, target_os = "freebsd", target_os = "netbsd")))]
            assert_eq!(Some(addr.clone().into()), result.address);
        }

        let data_socket = socket(AddressFamily::UNIX, SocketType::SEQPACKET, None).unwrap();
        connect(&data_socket, &addr).unwrap();
        sendmsg(
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

/// Similar to `do_test_unix_msg` but uses an unconnected socket and
/// `sendmsg_addr` instead of `sendmsg`.
#[cfg(not(any(target_os = "espidf", target_os = "redox", target_os = "wasi")))]
fn do_test_unix_msg_unconnected(addr: SocketAddrUnix) {
    use rustix::io::{IoSlice, IoSliceMut};
    use rustix::net::{recvmsg, sendmsg_addr, RecvFlags, ReturnFlags, SendFlags};

    let server = {
        let runs: &[i32] = &[3, 184, 187, 0];
        let data_socket = socket(AddressFamily::UNIX, SocketType::DGRAM, None).unwrap();
        bind(&data_socket, &addr).unwrap();

        move || {
            let mut buffer = vec![0; BUFFER_SIZE];
            for expected_sum in runs {
                let mut sum = 0;
                loop {
                    let result = recvmsg(
                        &data_socket,
                        &mut [IoSliceMut::new(&mut buffer)],
                        &mut Default::default(),
                        RecvFlags::empty(),
                    )
                    .unwrap();
                    let nread = result.bytes;

                    assert_eq!(result.flags, ReturnFlags::empty());

                    assert_ne!(&buffer[..nread], b"exit");
                    if &buffer[..nread] == b"sum" {
                        break;
                    }

                    sum += i32::from_str(&String::from_utf8_lossy(&buffer[..nread])).unwrap();
                }

                assert_eq!(sum, *expected_sum);
            }
            let result = recvmsg(
                &data_socket,
                &mut [IoSliceMut::new(&mut buffer)],
                &mut Default::default(),
                RecvFlags::empty(),
            )
            .unwrap();
            let nread = result.bytes;

            assert_eq!(&buffer[..nread], b"exit");
            assert_eq!(result.flags, ReturnFlags::empty());
        }
    };

    let client = move || {
        let runs: &[&[&str]] = &[&["1", "2"], &["4", "77", "103"], &["5", "78", "104"], &[]];

        for args in runs {
            let data_socket = socket(AddressFamily::UNIX, SocketType::DGRAM, None).unwrap();

            for arg in *args {
                sendmsg_addr(
                    &data_socket,
                    &addr,
                    &[IoSlice::new(arg.as_bytes())],
                    &mut Default::default(),
                    SendFlags::empty(),
                )
                .unwrap();
            }
            sendmsg_addr(
                &data_socket,
                &addr,
                &[IoSlice::new(b"sum")],
                &mut Default::default(),
                SendFlags::empty(),
            )
            .unwrap();
        }

        let data_socket = socket(AddressFamily::UNIX, SocketType::DGRAM, None).unwrap();
        sendmsg_addr(
            &data_socket,
            &addr,
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

#[cfg(not(any(target_os = "espidf", target_os = "redox", target_os = "wasi")))]
#[cfg(not(target_os = "freebsd"))] // TODO: Investigate why these tests fail on FreeBSD.
#[test]
fn test_unix_msg() {
    use rustix::ffi::CString;
    use std::os::unix::ffi::OsStrExt as _;

    crate::init();

    let tmpdir = tempfile::tempdir().unwrap();
    let path = tmpdir.path().join("scp_4804");

    let name = SocketAddrUnix::new(&path).unwrap();
    assert_eq!(
        name.path(),
        Some(CString::new(path.as_os_str().as_bytes()).unwrap().into())
    );
    assert_eq!(name.path_bytes(), Some(path.as_os_str().as_bytes()));
    #[cfg(linux_kernel)]
    assert!(!name.is_unnamed());
    do_test_unix_msg(name);

    unlinkat(CWD, path, AtFlags::empty()).unwrap();
}

/// Like `test_unix_msg` but tests `do_test_unix_msg_unconnected`.
#[cfg(not(any(target_os = "espidf", target_os = "redox", target_os = "wasi")))]
#[test]
fn test_unix_msg_unconnected() {
    use rustix::ffi::CString;
    use std::os::unix::ffi::OsStrExt as _;

    crate::init();

    let tmpdir = tempfile::tempdir().unwrap();
    let path = tmpdir.path().join("scp_4804");

    let name = SocketAddrUnix::new(&path).unwrap();
    assert_eq!(
        name.path(),
        Some(CString::new(path.as_os_str().as_bytes()).unwrap().into())
    );
    assert_eq!(name.path_bytes(), Some(path.as_os_str().as_bytes()));
    #[cfg(linux_kernel)]
    assert!(!name.is_unnamed());
    do_test_unix_msg_unconnected(name);

    unlinkat(CWD, path, AtFlags::empty()).unwrap();
}

#[cfg(linux_kernel)]
#[test]
fn test_abstract_unix_msg() {
    crate::init();

    use std::os::unix::ffi::OsStrExt as _;

    let tmpdir = tempfile::tempdir().unwrap();
    let path = tmpdir.path().join("scp_4804");

    let name = SocketAddrUnix::new_abstract_name(path.as_os_str().as_bytes()).unwrap();
    assert_eq!(name.abstract_name(), Some(path.as_os_str().as_bytes()));
    assert!(!name.is_unnamed());
    do_test_unix_msg(name);
}

/// Like `test_abstract_unix_msg` but tests `do_test_unix_msg_unconnected`.
#[cfg(linux_kernel)]
#[test]
fn test_abstract_unix_msg_unconnected() {
    crate::init();

    use std::os::unix::ffi::OsStrExt as _;

    let tmpdir = tempfile::tempdir().unwrap();
    let path = tmpdir.path().join("scp_4804");

    let name = SocketAddrUnix::new_abstract_name(path.as_os_str().as_bytes()).unwrap();
    assert_eq!(name.abstract_name(), Some(path.as_os_str().as_bytes()));
    assert!(!name.is_unnamed());
    do_test_unix_msg_unconnected(name);
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[cfg(feature = "pipe")]
#[cfg(not(target_os = "freebsd"))] // TODO: Investigate why these tests fail on FreeBSD.
#[test]
fn test_unix_msg_with_scm_rights() {
    crate::init();

    use rustix::fd::AsFd as _;
    use rustix::io::{IoSlice, IoSliceMut};
    use rustix::net::{
        recvmsg, sendmsg, RecvAncillaryBuffer, RecvAncillaryMessage, RecvFlags, ReturnFlags,
        SendAncillaryBuffer, SendAncillaryMessage, SendFlags,
    };
    use rustix::pipe::pipe;
    use std::string::ToString as _;

    let tmpdir = tempfile::tempdir().unwrap();
    let path = tmpdir.path().join("scp_4804");

    let server = {
        let path = path.clone();

        let connection_socket = socket(AddressFamily::UNIX, SocketType::SEQPACKET, None).unwrap();

        let name = SocketAddrUnix::new(&path).unwrap();
        bind(&connection_socket, &name).unwrap();
        listen(&connection_socket, 1).unwrap();

        move || {
            let mut pipe_end = None;

            let mut buffer = vec![0; BUFFER_SIZE];
            let mut cmsg_space = Vec::with_capacity(rustix::cmsg_space!(ScmRights(1)));

            'exit: loop {
                let data_socket = accept(&connection_socket).unwrap();
                let mut sum = 0;
                loop {
                    let mut cmsg_buffer = RecvAncillaryBuffer::new(cmsg_space.spare_capacity_mut());
                    let result = recvmsg(
                        &data_socket,
                        &mut [IoSliceMut::new(&mut buffer)],
                        &mut cmsg_buffer,
                        RecvFlags::empty(),
                    )
                    .unwrap();
                    let nread = result.bytes;

                    assert_eq!(result.flags, ReturnFlags::empty());

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
                sendmsg(
                    &data_socket,
                    &[IoSlice::new(data.as_bytes())],
                    &mut Default::default(),
                    SendFlags::empty(),
                )
                .unwrap();
            }

            unlinkat(CWD, path, AtFlags::empty()).unwrap();

            // Once we're done, send a message along the pipe.
            let pipe = pipe_end.unwrap();
            write(&pipe, b"pipe message!").unwrap();
        }
    };

    let client = move || {
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
            let data_socket = socket(AddressFamily::UNIX, SocketType::SEQPACKET, None).unwrap();
            connect(&data_socket, &addr).unwrap();

            for arg in *args {
                sendmsg(
                    &data_socket,
                    &[IoSlice::new(arg.as_bytes())],
                    &mut Default::default(),
                    SendFlags::empty(),
                )
                .unwrap();
            }
            sendmsg(
                &data_socket,
                &[IoSlice::new(b"sum")],
                &mut Default::default(),
                SendFlags::empty(),
            )
            .unwrap();

            let result = recvmsg(
                &data_socket,
                &mut [IoSliceMut::new(&mut buffer)],
                &mut Default::default(),
                RecvFlags::empty(),
            )
            .unwrap();
            let nread = result.bytes;
            assert_eq!(
                i32::from_str(&String::from_utf8_lossy(&buffer[..nread])).unwrap(),
                *sum
            );
            assert_eq!(result.flags, ReturnFlags::empty());
        }

        let data_socket = socket(AddressFamily::UNIX, SocketType::SEQPACKET, None).unwrap();

        // Format the CMSG.
        let we = [write_end.as_fd()];
        let msg = SendAncillaryMessage::ScmRights(&we);
        let mut space = Vec::with_capacity(msg.size());
        let mut cmsg_buffer = SendAncillaryBuffer::new(space.spare_capacity_mut());
        assert!(cmsg_buffer.push(msg));

        connect(&data_socket, &addr).unwrap();
        sendmsg(
            &data_socket,
            &[IoSlice::new(b"exit")],
            &mut cmsg_buffer,
            SendFlags::empty(),
        )
        .unwrap();

        // Read a value from the pipe.
        let mut buffer = [0_u8; 13];
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

#[cfg(all(feature = "process", linux_kernel))]
#[test]
fn test_unix_peercred() {
    crate::init();

    use rustix::io::{IoSlice, IoSliceMut};
    use rustix::net::{
        recvmsg, sendmsg, sockopt, RecvAncillaryBuffer, RecvAncillaryMessage, RecvFlags,
        SendAncillaryBuffer, SendAncillaryMessage, SendFlags, SocketFlags,
    };
    use rustix::process::{getgid, getpid, getuid};

    let (send_sock, recv_sock) = rustix::net::socketpair(
        AddressFamily::UNIX,
        SocketType::STREAM,
        SocketFlags::CLOEXEC,
        None,
    )
    .unwrap();

    sockopt::set_socket_passcred(&recv_sock, true).unwrap();

    let ucred = sockopt::socket_peercred(&send_sock).unwrap();
    assert_eq!(ucred.pid, getpid());
    assert_eq!(ucred.uid, getuid());
    assert_eq!(ucred.gid, getgid());

    let msg = SendAncillaryMessage::ScmCredentials(ucred);
    let mut space = Vec::with_capacity(msg.size());
    let mut cmsg_buffer = SendAncillaryBuffer::new(space.spare_capacity_mut());
    assert!(cmsg_buffer.push(msg));

    sendmsg(
        &send_sock,
        &[IoSlice::new(b"cred")],
        &mut cmsg_buffer,
        SendFlags::empty(),
    )
    .unwrap();

    let mut cmsg_space = Vec::with_capacity(rustix::cmsg_space!(ScmCredentials(1)));
    let mut cmsg_buffer = RecvAncillaryBuffer::new(cmsg_space.spare_capacity_mut());

    let mut buffer = vec![0; BUFFER_SIZE];
    recvmsg(
        &recv_sock,
        &mut [IoSliceMut::new(&mut buffer)],
        &mut cmsg_buffer,
        RecvFlags::empty(),
    )
    .unwrap();

    match cmsg_buffer.drain().next().unwrap() {
        RecvAncillaryMessage::ScmCredentials(ucred2) => assert_eq!(ucred2, ucred),
        _ => panic!("Unexpected ancillary message"),
    };
}

/// Like `test_unix_msg_with_scm_rights`, but with multiple file descriptors
/// over multiple control messages.
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[cfg(feature = "pipe")]
#[cfg(not(target_os = "freebsd"))] // TODO: Investigate why these tests fail on FreeBSD.
#[test]
fn test_unix_msg_with_combo() {
    crate::init();

    use rustix::fd::AsFd as _;
    use rustix::io::{IoSlice, IoSliceMut};
    use rustix::net::{
        recvmsg, sendmsg, RecvAncillaryBuffer, RecvAncillaryMessage, RecvFlags, ReturnFlags,
        SendAncillaryBuffer, SendAncillaryMessage, SendFlags,
    };
    use rustix::pipe::pipe;
    use std::string::ToString as _;

    let tmpdir = tempfile::tempdir().unwrap();
    let path = tmpdir.path().join("scp_4804");

    let server = {
        let path = path.clone();

        let connection_socket = socket(AddressFamily::UNIX, SocketType::SEQPACKET, None).unwrap();

        let name = SocketAddrUnix::new(&path).unwrap();
        bind(&connection_socket, &name).unwrap();
        listen(&connection_socket, 1).unwrap();

        move || {
            let mut pipe_end = None;
            let mut another_pipe_end = None;
            let mut yet_another_pipe_end = None;

            let mut buffer = vec![0; BUFFER_SIZE];
            let mut cmsg_space =
                Vec::with_capacity(rustix::cmsg_space!(ScmRights(1), ScmRights(2)));

            'exit: loop {
                let data_socket = accept(&connection_socket).unwrap();
                let mut sum = 0;
                loop {
                    let mut cmsg_buffer = RecvAncillaryBuffer::new(cmsg_space.spare_capacity_mut());
                    let result = recvmsg(
                        &data_socket,
                        &mut [IoSliceMut::new(&mut buffer)],
                        &mut cmsg_buffer,
                        RecvFlags::empty(),
                    )
                    .unwrap();
                    let nread = result.bytes;

                    assert_eq!(result.flags, ReturnFlags::empty());

                    // Read out the pipe if we got it.
                    for cmsg in cmsg_buffer.drain() {
                        match cmsg {
                            RecvAncillaryMessage::ScmRights(rights) => {
                                for right in rights {
                                    if pipe_end.is_none() {
                                        pipe_end = Some(right);
                                    } else if another_pipe_end.is_none() {
                                        another_pipe_end = Some(right);
                                    } else if yet_another_pipe_end.is_none() {
                                        yet_another_pipe_end = Some(right);
                                    } else {
                                        unreachable!();
                                    }
                                }
                            }
                            _ => {}
                        }
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
                sendmsg(
                    &data_socket,
                    &[IoSlice::new(data.as_bytes())],
                    &mut Default::default(),
                    SendFlags::empty(),
                )
                .unwrap();
            }

            unlinkat(CWD, path, AtFlags::empty()).unwrap();

            // Once we're done, send a message along the pipe.
            let pipe = pipe_end.unwrap();
            write(&pipe, b"pipe message!").unwrap();

            // Once we're done, send a message along the other pipe.
            let another_pipe = another_pipe_end.unwrap();
            write(&another_pipe, b"and another message!").unwrap();

            // Once we're done, send a message along the other pipe.
            let yet_another_pipe = yet_another_pipe_end.unwrap();
            write(&yet_another_pipe, b"yet another message!").unwrap();
        }
    };

    let client = move || {
        let addr = SocketAddrUnix::new(path).unwrap();
        let (read_end, write_end) = pipe().unwrap();
        let (another_read_end, another_write_end) = pipe().unwrap();
        let (yet_another_read_end, yet_another_write_end) = pipe().unwrap();
        let mut buffer = vec![0; BUFFER_SIZE];
        let runs: &[(&[&str], i32)] = &[
            (&["1", "2"], 3),
            (&["4", "77", "103"], 184),
            (&["5", "78", "104"], 187),
            (&[], 0),
        ];

        for (args, sum) in runs {
            let data_socket = socket(AddressFamily::UNIX, SocketType::SEQPACKET, None).unwrap();
            connect(&data_socket, &addr).unwrap();

            for arg in *args {
                sendmsg(
                    &data_socket,
                    &[IoSlice::new(arg.as_bytes())],
                    &mut Default::default(),
                    SendFlags::empty(),
                )
                .unwrap();
            }
            sendmsg(
                &data_socket,
                &[IoSlice::new(b"sum")],
                &mut Default::default(),
                SendFlags::empty(),
            )
            .unwrap();

            let result = recvmsg(
                &data_socket,
                &mut [IoSliceMut::new(&mut buffer)],
                &mut Default::default(),
                RecvFlags::empty(),
            )
            .unwrap();
            let nread = result.bytes;
            assert_eq!(
                i32::from_str(&String::from_utf8_lossy(&buffer[..nread])).unwrap(),
                *sum
            );
            assert_eq!(result.flags, ReturnFlags::empty());
        }

        let data_socket = socket(AddressFamily::UNIX, SocketType::SEQPACKET, None).unwrap();

        let mut space = Vec::with_capacity(rustix::cmsg_space!(ScmRights(1), ScmRights(2)));
        let mut cmsg_buffer = SendAncillaryBuffer::new(space.spare_capacity_mut());

        // Format a CMSG.
        let we = [write_end.as_fd(), another_write_end.as_fd()];
        let msg = SendAncillaryMessage::ScmRights(&we);
        assert!(cmsg_buffer.push(msg));

        // Format another CMSG.
        let we = [yet_another_write_end.as_fd()];
        let msg = SendAncillaryMessage::ScmRights(&we);
        assert!(cmsg_buffer.push(msg));

        connect(&data_socket, &addr).unwrap();
        sendmsg(
            &data_socket,
            &[IoSlice::new(b"exit")],
            &mut cmsg_buffer,
            SendFlags::empty(),
        )
        .unwrap();

        // Read a value from the pipe.
        let mut buffer = [0_u8; 13];
        read(&read_end, &mut buffer).unwrap();
        assert_eq!(&buffer, b"pipe message!".as_ref());

        // Read a value from the other pipe.
        let mut buffer = [0_u8; 20];
        read(&another_read_end, &mut buffer).unwrap();
        assert_eq!(&buffer, b"and another message!".as_ref());

        // Read a value from the other pipe.
        let mut buffer = [0_u8; 20];
        read(&yet_another_read_end, &mut buffer).unwrap();
        assert_eq!(&buffer, b"yet another message!".as_ref());
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
