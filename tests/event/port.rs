//! We assume that `port_get` etc. don't mutate the timestamp.
//!
//! illumos and Solaris document that the timestamp is `const`, but it's
//! `mut` in the Rust libc bindings. Test that it isn't actually mutated
//! in practice.

// Test that the timeout isn't mutated on a timeout.
#[test]
fn test_port_timeout_assumption() {
    unsafe {
        use rustix::fd::AsRawFd;
        use std::ffi::c_void;

        let port = libc::port_create();
        assert_ne!(port, -1);

        let file = std::fs::File::open("Cargo.toml").unwrap();
        let fd = file.as_raw_fd();

        let r = libc::port_associate(
            port,
            libc::PORT_SOURCE_FD,
            fd as _,
            libc::POLLERR.into(),
            7 as *mut c_void,
        );
        assert_ne!(r, -1);

        let mut event = std::mem::zeroed::<libc::port_event>();
        let orig_timeout = libc::timespec {
            tv_sec: 0,
            tv_nsec: 500_000_000,
        };
        let mut timeout = orig_timeout.clone();
        libc_errno::set_errno(libc_errno::Errno(0));
        let r = libc::port_get(port, &mut event, &mut timeout);
        assert_eq!(libc_errno::errno().0, libc::ETIME);
        assert_eq!(r, -1);

        assert_eq!(
            (timeout.tv_sec, timeout.tv_nsec),
            (orig_timeout.tv_sec, orig_timeout.tv_nsec)
        );
    }
}

// Test that the timeout isn't mutated on an immediate wake.
#[test]
fn test_port_event_assumption() {
    unsafe {
        use rustix::fd::AsRawFd;
        use std::ffi::c_void;

        let port = libc::port_create();
        assert_ne!(port, -1);

        let file = std::fs::File::open("Cargo.toml").unwrap();
        let fd = file.as_raw_fd();

        let r = libc::port_associate(
            port,
            libc::PORT_SOURCE_FD,
            fd as _,
            libc::POLLIN.into(),
            7 as *mut c_void,
        );
        assert_ne!(r, -1);

        let mut event = std::mem::zeroed::<libc::port_event>();
        let orig_timeout = libc::timespec {
            tv_sec: 1,
            tv_nsec: 5678,
        };
        let mut timeout = orig_timeout.clone();
        let r = libc::port_get(port, &mut event, &mut timeout);
        assert_ne!(r, -1);

        assert_eq!(
            (timeout.tv_sec, timeout.tv_nsec),
            (orig_timeout.tv_sec, orig_timeout.tv_nsec)
        );
        assert_eq!(event.portev_user, 7 as *mut c_void);
    }
}

// Test that the timeout isn't mutated when data arrives midway
// through a timeout.
#[cfg(feature = "fs")]
#[test]
fn test_port_delay_assumption() {
    use rustix::fs;

    let tmpdir = tempfile::tempdir().unwrap();
    let fifo_path = tmpdir.path().join("fifo");
    fs::mknodat(
        fs::CWD,
        &fifo_path,
        fs::FileType::Fifo,
        fs::Mode::RUSR
            | fs::Mode::WUSR
            | fs::Mode::RGRP
            | fs::Mode::WGRP
            | fs::Mode::ROTH
            | fs::Mode::WOTH,
        0,
    )
    .unwrap();

    let fifo_path_clone = fifo_path.clone();
    let _mutater = std::thread::Builder::new()
        .name("mutater".to_string())
        .spawn(|| {
            let fifo = fs::openat(
                fs::CWD,
                fifo_path_clone,
                fs::OFlags::WRONLY,
                fs::Mode::empty(),
            )
            .unwrap();

            for i in 0..10 {
                let buf = [b'A' + (i % 26)];
                match rustix::io::write(&fifo, &buf) {
                    Ok(1) => {}
                    Ok(n) => panic!("unexpected write of length {}", n),
                    Err(rustix::io::Errno::PIPE) => return,
                    Err(err) => Err(err).unwrap(),
                }
                std::thread::sleep(std::time::Duration::new(0, 4_000_000));
            }
            panic!("Loop iterated too many times without completing!");
        })
        .unwrap();

    let fifo = fs::openat(fs::CWD, &fifo_path, fs::OFlags::RDONLY, fs::Mode::empty()).unwrap();

    unsafe {
        use rustix::fd::AsRawFd;
        use std::ffi::c_void;

        let port = libc::port_create();
        assert_ne!(port, -1);

        for i in 0..5 {
            let r = libc::port_associate(
                port,
                libc::PORT_SOURCE_FD,
                fifo.as_raw_fd() as _,
                libc::POLLIN.into(),
                (9 + i) as *mut c_void,
            );
            assert_ne!(r, -1);

            let mut event = std::mem::zeroed::<libc::port_event>();
            let orig_timeout = libc::timespec {
                tv_sec: 5,
                tv_nsec: 5678,
            };
            let mut timeout = orig_timeout.clone();
            let r = libc::port_get(port, &mut event, &mut timeout);
            assert_ne!(r, -1, "port_get: {:?}", std::io::Error::last_os_error());

            assert_eq!(
                (timeout.tv_sec, timeout.tv_nsec),
                (orig_timeout.tv_sec, orig_timeout.tv_nsec)
            );
            assert_eq!(event.portev_user, (9 + i) as *mut c_void);

            let mut buf = [0_u8; 1];
            loop {
                match rustix::io::read(&fifo, &mut buf) {
                    Ok(1) => {
                        assert_eq!(buf[0], b'A' + i);
                        break;
                    }
                    Ok(n) => panic!("unexpected read of length {}", n),
                    Err(rustix::io::Errno::INTR) => continue,
                    Err(err) => Err(err).unwrap(),
                }
            }
        }
    }
}
