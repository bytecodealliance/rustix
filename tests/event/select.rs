use rustix::event::{
    fd_set_bound, fd_set_contains, fd_set_insert, fd_set_num_elements, fd_set_remove, FdSetElement,
    FdSetIter,
};
use rustix::fd::RawFd;
#[cfg(feature = "pipe")]
#[cfg(not(windows))]
use {
    rustix::event::{select, Timespec},
    rustix::fd::{AsRawFd, FromRawFd, OwnedFd},
    rustix::io::retry_on_intr,
    std::cmp::max,
};

#[cfg(feature = "pipe")]
#[cfg(not(windows))]
#[test]
fn test_select() {
    use rustix::io::{read, write};
    use rustix::pipe::pipe;

    // Create a pipe.
    let (reader, writer) = pipe().unwrap();
    let nfds = max(reader.as_raw_fd(), writer.as_raw_fd()) + 1;

    // `select` should say there's nothing ready to be read from the pipe.
    let mut readfds = vec![0 as FdSetElement; fd_set_num_elements(nfds)];
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
    let mut readfds = vec![0 as FdSetElement; fd_set_num_elements(nfds)];
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
#[cfg(not(windows))]
#[test]
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

    // Create a fd at `FD_SETSIZE + 1` out of thin air. Use `libc` instead
    // of `OwnedFd::from_raw_fd` because grabbing a fd out of thin air
    // violates Rust's concept of I/O safety (and wouldn't make sense to do
    // in anything other than a test like this).
    let great_fd = unsafe { libc::dup2(reader.as_raw_fd(), libc::FD_SETSIZE as RawFd + 1) };
    let reader = unsafe { OwnedFd::from_raw_fd(great_fd) };

    let nfds = max(reader.as_raw_fd(), writer.as_raw_fd()) + 1;

    // `select` should say there's nothing ready to be read from the pipe.
    let mut readfds = vec![0 as FdSetElement; fd_set_num_elements(nfds)];
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
    let mut readfds = vec![0 as FdSetElement; fd_set_num_elements(nfds)];
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
        let mut fds = vec![0 as FdSetElement; fd_set_num_elements(nfds)];
        for fd in stuff {
            assert!(!fd_set_contains(&mut fds, *fd));
            fd_set_insert(&mut fds, *fd);
            assert!(fd_set_contains(&mut fds, *fd));
            fd_set_remove(&mut fds, *fd);
            assert!(!fd_set_contains(&mut fds, *fd));
            fd_set_insert(&mut fds, *fd);
            assert!(fd_set_contains(&mut fds, *fd));
        }
        assert_eq!(fd_set_bound(&fds), nfds);
        assert_eq!(FdSetIter::new(&fds).collect::<Vec<RawFd>>(), stuff);
    }
}
