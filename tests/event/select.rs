#[cfg(feature = "pipe")]
use {
    rustix::event::{fd_bitvector_len, fd_clr, fd_isset, fd_set, select, FdSetElement, Timespec},
    rustix::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd},
    rustix::io::retry_on_intr,
    std::cmp::max,
};

#[cfg(feature = "pipe")]
#[test]
fn test_select() {
    use core::ptr::null_mut;
    use rustix::io::{read, write};
    use rustix::pipe::pipe;

    // Create a pipe.
    let (reader, writer) = pipe().unwrap();
    let nfds = max(reader.as_raw_fd(), writer.as_raw_fd()) + 1;

    // `select` should say there's nothing ready to be read from the pipe.
    let mut readfds = vec![0 as FdSetElement; fd_bitvector_len(nfds)];
    fd_set(reader.as_raw_fd(), &mut readfds);
    let num = retry_on_intr(|| unsafe {
        select(
            nfds,
            readfds.as_mut_ptr(),
            null_mut(),
            null_mut(),
            Some(&Timespec {
                tv_sec: 0,
                tv_nsec: 0,
            }),
        )
    })
    .unwrap();
    assert_eq!(num, 0);
    assert!(!fd_isset(reader.as_raw_fd(), &readfds));

    // Write a byte to the pipe.
    assert_eq!(retry_on_intr(|| write(&writer, b"a")).unwrap(), 1);

    // `select` should now say there's data to be read.
    let mut readfds = vec![0 as FdSetElement; fd_bitvector_len(nfds)];
    fd_set(reader.as_raw_fd(), &mut readfds);
    let num = retry_on_intr(|| unsafe {
        select(nfds, readfds.as_mut_ptr(), null_mut(), null_mut(), None)
    })
    .unwrap();
    assert_eq!(num, 1);
    assert!(fd_isset(reader.as_raw_fd(), &readfds));
    fd_clr(reader.as_raw_fd(), &mut readfds);
    assert!(!fd_isset(reader.as_raw_fd(), &readfds));

    // Read the byte from the pipe.
    let mut buf = [b'\0'];
    assert_eq!(retry_on_intr(|| read(&reader, &mut buf)).unwrap(), 1);
    assert_eq!(buf[0], b'a');

    // Select should now say there's no more data to be read.
    fd_set(reader.as_raw_fd(), &mut readfds);
    let num = retry_on_intr(|| unsafe {
        select(
            nfds,
            readfds.as_mut_ptr(),
            null_mut(),
            null_mut(),
            Some(&Timespec {
                tv_sec: 0,
                tv_nsec: 0,
            }),
        )
    })
    .unwrap();
    assert_eq!(num, 0);
    assert!(!fd_isset(reader.as_raw_fd(), &readfds));
}

#[cfg(feature = "pipe")]
#[test]
fn test_select_with_great_fds() {
    use core::cmp::max;
    use core::ptr::null_mut;
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
    let mut readfds = vec![0 as FdSetElement; fd_bitvector_len(nfds)];
    fd_set(reader.as_raw_fd(), &mut readfds);
    let num = retry_on_intr(|| unsafe {
        select(
            nfds,
            readfds.as_mut_ptr(),
            null_mut(),
            null_mut(),
            Some(&Timespec {
                tv_sec: 0,
                tv_nsec: 0,
            }),
        )
    })
    .unwrap();
    assert_eq!(num, 0);
    assert!(!fd_isset(reader.as_raw_fd(), &readfds));

    // Write a byte to the pipe.
    assert_eq!(retry_on_intr(|| write(&writer, b"a")).unwrap(), 1);

    // `select` should now say there's data to be read.
    let mut readfds = vec![0 as FdSetElement; fd_bitvector_len(nfds)];
    fd_set(reader.as_raw_fd(), &mut readfds);
    let num = retry_on_intr(|| unsafe {
        select(nfds, readfds.as_mut_ptr(), null_mut(), null_mut(), None)
    })
    .unwrap();
    assert_eq!(num, 1);
    assert!(fd_isset(reader.as_raw_fd(), &readfds));
    fd_clr(reader.as_raw_fd(), &mut readfds);
    assert!(!fd_isset(reader.as_raw_fd(), &readfds));

    // Read the byte from the pipe.
    let mut buf = [b'\0'];
    assert_eq!(retry_on_intr(|| read(&reader, &mut buf)).unwrap(), 1);
    assert_eq!(buf[0], b'a');

    // Select should now say there's no more data to be read.
    fd_set(reader.as_raw_fd(), &mut readfds);
    let num = retry_on_intr(|| unsafe {
        select(
            nfds,
            readfds.as_mut_ptr(),
            null_mut(),
            null_mut(),
            Some(&Timespec {
                tv_sec: 0,
                tv_nsec: 0,
            }),
        )
    })
    .unwrap();
    assert_eq!(num, 0);
    assert!(!fd_isset(reader.as_raw_fd(), &readfds));

    // Reset the process limit.
    setrlimit(Resource::Nofile, orig_rlimit).unwrap();
}
