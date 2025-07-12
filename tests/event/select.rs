use rustix::event::{
    fd_set_bound, fd_set_insert, fd_set_num_elements, fd_set_remove, select, FdSetElement,
    FdSetIter, Timespec,
};
use rustix::fd::{AsRawFd as _, RawFd};
#[cfg(feature = "pipe")]
#[cfg(not(windows))]
#[allow(unused_imports)]
use rustix::fd::{FromRawFd as _, OwnedFd};
use rustix::io::retry_on_intr;
use serial_test::serial;
use std::cmp::max;

#[cfg(feature = "pipe")]
#[cfg(not(windows))]
#[test]
fn test_select_with_pipes() {
    use rustix::io::{read, write};
    use rustix::pipe::pipe;

    // Create a pipe.
    let (reader, writer) = pipe().unwrap();
    let nfds = max(reader.as_raw_fd(), writer.as_raw_fd()) + 1;

    // `select` should say there's nothing ready to be read from the pipe.
    let mut readfds = vec![FdSetElement::default(); fd_set_num_elements(2, nfds)];
    fd_set_insert(&mut readfds, reader.as_raw_fd());
    let num = retry_on_intr(|| unsafe {
        select(
            nfds,
            Some(&mut readfds),
            None,
            None,
            Some(&Timespec {
                tv_sec: 0,
                tv_nsec: 0,
            }),
        )
    })
    .unwrap();
    assert_eq!(num, 0);
    assert!(!fd_set_contains(&readfds, reader.as_raw_fd()));
    assert_eq!(fd_set_bound(&readfds), 0);

    // Write a byte to the pipe.
    assert_eq!(retry_on_intr(|| write(&writer, b"a")).unwrap(), 1);

    // `select` should now say there's data to be read.
    let mut readfds = vec![FdSetElement::default(); fd_set_num_elements(2, nfds)];
    fd_set_insert(&mut readfds, reader.as_raw_fd());
    let num =
        retry_on_intr(|| unsafe { select(nfds, Some(&mut readfds), None, None, None) }).unwrap();
    assert_eq!(num, 1);
    assert!(fd_set_contains(&readfds, reader.as_raw_fd()));
    assert_eq!(fd_set_bound(&readfds), reader.as_raw_fd() + 1);
    fd_set_remove(&mut readfds, reader.as_raw_fd());
    assert!(!fd_set_contains(&readfds, reader.as_raw_fd()));
    assert_eq!(fd_set_bound(&readfds), 0);

    // Read the byte from the pipe.
    let mut buf = [b'\0'];
    assert_eq!(retry_on_intr(|| read(&reader, &mut buf)).unwrap(), 1);
    assert_eq!(buf[0], b'a');

    // Select should now say there's no more data to be read.
    fd_set_insert(&mut readfds, reader.as_raw_fd());
    let num = retry_on_intr(|| unsafe {
        select(
            nfds,
            Some(&mut readfds),
            None,
            None,
            Some(&Timespec {
                tv_sec: 0,
                tv_nsec: 0,
            }),
        )
    })
    .unwrap();
    assert_eq!(num, 0);
    assert!(!fd_set_contains(&readfds, reader.as_raw_fd()));
    assert_eq!(fd_set_bound(&readfds), 0);
}

#[cfg(feature = "pipe")]
#[cfg(feature = "process")]
#[cfg(not(windows))]
#[test]
#[serial] // for `setrlimit` usage
fn test_select_with_great_fds() {
    use core::cmp::max;
    use rustix::io::{read, write};
    use rustix::pipe::pipe;
    use rustix::process::{getrlimit, setrlimit, Resource};

    // Create a pipe.
    let (reader, writer) = pipe().unwrap();

    // Raise the file descriptor limit so that we can test fds above
    // `FD_SETSIZE`.
    let orig_rlimit = getrlimit(Resource::Nofile);
    let mut rlimit = orig_rlimit;
    if let Some(current) = rlimit.current {
        rlimit.current = Some(max(current, libc::FD_SETSIZE as u64 + 2));
    }
    setrlimit(Resource::Nofile, rlimit).unwrap();

    // Create an fd at `FD_SETSIZE + 1` out of thin air. Use `libc` instead of
    // `OwnedFd::from_raw_fd` because grabbing an fd out of thin air violates
    // Rust's concept of I/O safety (and wouldn't make sense to do in anything
    // other than a test like this).
    let great_fd = unsafe { libc::dup2(reader.as_raw_fd(), libc::FD_SETSIZE as RawFd + 1) };
    let reader = unsafe { OwnedFd::from_raw_fd(great_fd) };

    let nfds = max(reader.as_raw_fd(), writer.as_raw_fd()) + 1;

    // `select` should say there's nothing ready to be read from the pipe.
    let mut readfds = vec![FdSetElement::default(); fd_set_num_elements(2, nfds)];
    fd_set_insert(&mut readfds, reader.as_raw_fd());
    let num = retry_on_intr(|| unsafe {
        select(
            nfds,
            Some(&mut readfds),
            None,
            None,
            Some(&Timespec {
                tv_sec: 0,
                tv_nsec: 0,
            }),
        )
    })
    .unwrap();
    assert_eq!(num, 0);
    assert!(!fd_set_contains(&readfds, reader.as_raw_fd()));
    assert_eq!(fd_set_bound(&readfds), 0);

    // Write a byte to the pipe.
    assert_eq!(retry_on_intr(|| write(&writer, b"a")).unwrap(), 1);

    // `select` should now say there's data to be read.
    let mut readfds = vec![FdSetElement::default(); fd_set_num_elements(2, nfds)];
    fd_set_insert(&mut readfds, reader.as_raw_fd());
    let num =
        retry_on_intr(|| unsafe { select(nfds, Some(&mut readfds), None, None, None) }).unwrap();
    assert_eq!(num, 1);
    assert!(fd_set_contains(&readfds, reader.as_raw_fd()));
    assert_eq!(fd_set_bound(&readfds), reader.as_raw_fd() + 1);
    fd_set_remove(&mut readfds, reader.as_raw_fd());
    assert!(!fd_set_contains(&readfds, reader.as_raw_fd()));
    assert_eq!(fd_set_bound(&readfds), 0);

    // Read the byte from the pipe.
    let mut buf = [b'\0'];
    assert_eq!(retry_on_intr(|| read(&reader, &mut buf)).unwrap(), 1);
    assert_eq!(buf[0], b'a');

    // Select should now say there's no more data to be read.
    fd_set_insert(&mut readfds, reader.as_raw_fd());
    let num = retry_on_intr(|| unsafe {
        select(
            nfds,
            Some(&mut readfds),
            None,
            None,
            Some(&Timespec {
                tv_sec: 0,
                tv_nsec: 0,
            }),
        )
    })
    .unwrap();
    assert_eq!(num, 0);
    assert!(!fd_set_contains(&readfds, reader.as_raw_fd()));
    assert_eq!(fd_set_bound(&readfds), 0);

    // Reset the process limit.
    setrlimit(Resource::Nofile, orig_rlimit).unwrap();
}

#[cfg(feature = "net")]
#[test]
#[serial] // for `crate::init`
fn test_select_with_sockets() {
    use rustix::net::{recv, send, AddressFamily, RecvFlags, SendFlags, SocketType};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    crate::init();

    // Create a socket pair (but don't use `socketpair` because we want this to
    // work on Windows too).

    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");
    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let writer = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::connect(&writer, &local_addr).expect("connect");
    let reader = rustix::net::accept(&listener).expect("accept");

    let nfds = max(reader.as_raw_fd(), writer.as_raw_fd()) + 1;

    #[cfg(windows)]
    let nfds: i32 = nfds.try_into().unwrap();

    // `select` should say there's nothing ready to be read from the pipe.
    let mut readfds = vec![FdSetElement::default(); fd_set_num_elements(2, nfds as RawFd)];
    fd_set_insert(&mut readfds, reader.as_raw_fd());
    let num = retry_on_intr(|| unsafe {
        select(
            nfds,
            Some(&mut readfds),
            None,
            None,
            Some(&Timespec {
                tv_sec: 0,
                tv_nsec: 0,
            }),
        )
    })
    .unwrap();
    assert_eq!(num, 0);
    assert!(!fd_set_contains(&readfds, reader.as_raw_fd()));
    assert_eq!(fd_set_bound(&readfds), 0);

    // Write a byte to the pipe.
    assert_eq!(
        retry_on_intr(|| send(&writer, b"a", SendFlags::empty())).unwrap(),
        1
    );

    // `select` should now say there's data to be read.
    let mut readfds = vec![FdSetElement::default(); fd_set_num_elements(2, nfds as RawFd)];
    fd_set_insert(&mut readfds, reader.as_raw_fd());
    let num =
        retry_on_intr(|| unsafe { select(nfds, Some(&mut readfds), None, None, None) }).unwrap();
    assert_eq!(num, 1);
    assert!(fd_set_contains(&readfds, reader.as_raw_fd()));
    assert_eq!(fd_set_bound(&readfds), reader.as_raw_fd() + 1);
    fd_set_remove(&mut readfds, reader.as_raw_fd());
    assert!(!fd_set_contains(&readfds, reader.as_raw_fd()));
    assert_eq!(fd_set_bound(&readfds), 0);

    // Read the byte from the pipe.
    let mut buf = [b'\0'];
    assert_eq!(
        retry_on_intr(|| recv(&reader, &mut buf, RecvFlags::empty())).unwrap(),
        (1, 1)
    );
    assert_eq!(buf[0], b'a');

    // Select should now say there's no more data to be read.
    fd_set_insert(&mut readfds, reader.as_raw_fd());
    let num = retry_on_intr(|| unsafe {
        select(
            nfds,
            Some(&mut readfds),
            None,
            None,
            Some(&Timespec {
                tv_sec: 0,
                tv_nsec: 0,
            }),
        )
    })
    .unwrap();
    assert_eq!(num, 0);
    assert!(!fd_set_contains(&readfds, reader.as_raw_fd()));
    assert_eq!(fd_set_bound(&readfds), 0);
}

// Like `test_select_with_sockets` but test with the maximum permitted fd
// value.
#[cfg(feature = "net")]
#[cfg(feature = "process")]
#[cfg(not(windows))] // for `dup2` usage
#[test]
#[serial] // for `setrlimit` usage, and `crate::init`
fn test_select_with_maxfd_sockets() {
    use rustix::net::{recv, send, AddressFamily, RecvFlags, SendFlags, SocketType};
    use rustix::process::{getrlimit, setrlimit, Resource};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    crate::init();

    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let addr = SocketAddr::new(localhost, 0);
    let listener = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::bind(&listener, &addr).expect("bind");
    rustix::net::listen(&listener, 1).expect("listen");
    let local_addr = rustix::net::getsockname(&listener).unwrap();
    let writer = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    rustix::net::connect(&writer, &local_addr).expect("connect");
    let reader = rustix::net::accept(&listener).expect("accept");

    // Raise the fd limit to the maximum.
    let orig_rlimit = getrlimit(Resource::Nofile);
    let mut rlimit = orig_rlimit;
    let mut fd_limit = libc::FD_SETSIZE as RawFd;
    if let Some(maximum) = rlimit.maximum {
        rlimit.current = Some(maximum);
        fd_limit = maximum as RawFd;
    }
    setrlimit(Resource::Nofile, rlimit).unwrap();

    // Renumber the fds to the maximum possible values.
    let great_fd = unsafe { libc::dup2(reader.as_raw_fd(), fd_limit as RawFd - 1) };

    // On old versions of macOS, the above `dup2` call fails with `EBADF`. Just
    // skip the rest of this test in that case.
    #[cfg(apple)]
    if great_fd == -1 && libc_errno::errno().0 == libc::EBADF {
        return;
    }

    let reader = unsafe { OwnedFd::from_raw_fd(great_fd) };
    let great_fd = unsafe { libc::dup2(writer.as_raw_fd(), fd_limit as RawFd - 2) };
    let writer = unsafe { OwnedFd::from_raw_fd(great_fd) };

    let nfds = max(reader.as_raw_fd(), writer.as_raw_fd()) + 1;

    #[cfg(windows)]
    let nfds: i32 = nfds.try_into().unwrap();

    // `select` should say there's nothing ready to be read from the pipe.
    let mut readfds = vec![FdSetElement::default(); fd_set_num_elements(2, nfds as RawFd)];
    fd_set_insert(&mut readfds, reader.as_raw_fd());
    let num = retry_on_intr(|| unsafe {
        select(
            nfds,
            Some(&mut readfds),
            None,
            None,
            Some(&Timespec {
                tv_sec: 0,
                tv_nsec: 0,
            }),
        )
    })
    .unwrap();
    assert_eq!(num, 0);
    assert!(!fd_set_contains(&readfds, reader.as_raw_fd()));
    assert_eq!(fd_set_bound(&readfds), 0);

    // Write a byte to the pipe.
    assert_eq!(
        retry_on_intr(|| send(&writer, b"a", SendFlags::empty())).unwrap(),
        1
    );

    // `select` should now say there's data to be read.
    let mut readfds = vec![FdSetElement::default(); fd_set_num_elements(2, nfds as RawFd)];
    fd_set_insert(&mut readfds, reader.as_raw_fd());
    let num =
        retry_on_intr(|| unsafe { select(nfds, Some(&mut readfds), None, None, None) }).unwrap();
    assert_eq!(num, 1);
    assert!(fd_set_contains(&readfds, reader.as_raw_fd()));
    assert_eq!(fd_set_bound(&readfds), reader.as_raw_fd() + 1);
    fd_set_remove(&mut readfds, reader.as_raw_fd());
    assert!(!fd_set_contains(&readfds, reader.as_raw_fd()));
    assert_eq!(fd_set_bound(&readfds), 0);

    // Read the byte from the pipe.
    let mut buf = [b'\0'];
    assert_eq!(
        retry_on_intr(|| recv(&reader, &mut buf, RecvFlags::empty())).unwrap(),
        (1, 1)
    );
    assert_eq!(buf[0], b'a');

    // Select should now say there's no more data to be read.
    fd_set_insert(&mut readfds, reader.as_raw_fd());
    let num = retry_on_intr(|| unsafe {
        select(
            nfds,
            Some(&mut readfds),
            None,
            None,
            Some(&Timespec {
                tv_sec: 0,
                tv_nsec: 0,
            }),
        )
    })
    .unwrap();
    assert_eq!(num, 0);
    assert!(!fd_set_contains(&readfds, reader.as_raw_fd()));
    assert_eq!(fd_set_bound(&readfds), 0);

    setrlimit(Resource::Nofile, orig_rlimit).unwrap();
}

#[test]
fn test_select_iter() {
    for stuff in [
        &[1, 3, 31, 64, 128, 1024, 1025, 1030][..],
        &[100, 101, 102, 103, 104, 105, 106, 107, 2999][..],
        &[0, 8, 32, 64, 128][..],
        &[0, 1, 2, 3, 31, 32, 33, 34, 35][..],
        &[500][..],
        &[128][..],
        &[127][..],
        &[0][..],
        &[][..],
    ] {
        let nfds = if stuff.is_empty() {
            0
        } else {
            *stuff.last().unwrap() + 1
        };
        let mut fds = vec![FdSetElement::default(); fd_set_num_elements(stuff.len(), nfds)];
        for fd in stuff {
            assert!(!fd_set_contains(&fds, *fd));
            fd_set_insert(&mut fds, *fd);
            assert!(fd_set_contains(&fds, *fd));
            fd_set_remove(&mut fds, *fd);
            assert!(!fd_set_contains(&fds, *fd));
            fd_set_insert(&mut fds, *fd);
            assert!(fd_set_contains(&fds, *fd));
        }
        assert_eq!(fd_set_bound(&fds), nfds);
        assert_eq!(FdSetIter::new(&fds).collect::<Vec<RawFd>>(), stuff);
    }
}

// This isn't in rustix's public API because it isn't constant time. On
// bitvector platforms it could be, but on fd array platforms it can't be.
fn fd_set_contains(fds: &[FdSetElement], fd: RawFd) -> bool {
    FdSetIter::new(fds).any(|x| x == fd)
}
