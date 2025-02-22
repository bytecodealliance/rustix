//! Tests that do what `openpty` would do.

#![allow(unused_mut, unused_imports)]

use rustix::fs::{openat, Mode, OFlags, CWD};
use rustix::pty::*;
use std::fs::File;
use std::io::{Read as _, Write as _};

#[test]
fn openpty_basic() {
    // Use `CLOEXEC` if we can.
    #[cfg(any(linux_kernel, target_os = "freebsd", target_os = "netbsd"))]
    let flags = OpenptFlags::RDWR | OpenptFlags::NOCTTY | OpenptFlags::CLOEXEC;
    #[cfg(not(any(linux_kernel, target_os = "freebsd", target_os = "netbsd")))]
    let flags = OpenptFlags::RDWR | OpenptFlags::NOCTTY;

    let controller = openpt(flags).unwrap();

    grantpt(&controller).unwrap();
    unlockpt(&controller).unwrap();

    let name = match ptsname(&controller, Vec::new()) {
        Ok(name) => name,
        #[cfg(target_os = "freebsd")]
        Err(rustix::io::Errno::NOSYS) => return, // FreeBSD 12 doesn't support this
        Err(err) => panic!("{:?}", err),
    };
    let user = openat(
        CWD,
        name,
        OFlags::RDWR | OFlags::NOCTTY | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .unwrap();

    let mut controller = File::from(controller);
    let mut user = File::from(user);

    // The `'\x04'` is Ctrl-D, the default EOF control code.
    controller.write_all(b"Hello, world!\n\x04").unwrap();

    let mut s = String::new();

    // Read the string back. Our `\x04` above ended the stream, so we can
    // read to the end of the stream.
    #[cfg(not(target_os = "illumos"))]
    {
        user.read_to_string(&mut s).unwrap();
    }

    // Except on illumos, where the `\0x04` doesn't seem to translate into an
    // EOF, so we didn't end the stream, so just the line.
    #[cfg(target_os = "illumos")]
    use std::io::{BufRead, BufReader};
    #[cfg(target_os = "illumos")]
    let mut user = BufReader::new(user);
    #[cfg(target_os = "illumos")]
    {
        user.read_line(&mut s).unwrap();
    }

    assert_eq!(s, "Hello, world!\n");
}

// Like `openpty_basic` but use `ioctl_tiocgptpeer` instead of `ptsname`.
#[cfg(target_os = "linux")]
#[test]
fn openpty_get_peer() {
    // Use `CLOEXEC` if we can.
    #[cfg(any(linux_kernel, target_os = "freebsd", target_os = "netbsd"))]
    let flags = OpenptFlags::RDWR | OpenptFlags::NOCTTY | OpenptFlags::CLOEXEC;
    #[cfg(not(any(linux_kernel, target_os = "freebsd", target_os = "netbsd")))]
    let flags = OpenptFlags::RDWR | OpenptFlags::NOCTTY;

    let controller = openpt(flags).unwrap();

    grantpt(&controller).unwrap();
    unlockpt(&controller).unwrap();

    let user = ioctl_tiocgptpeer(
        &controller,
        OpenptFlags::RDWR | OpenptFlags::NOCTTY | OpenptFlags::CLOEXEC,
    )
    .unwrap();

    let mut controller = File::from(controller);
    let mut user = File::from(user);

    // The `'\x04'` is Ctrl-D, the default EOF control code.
    controller.write_all(b"Hello, world!\n\x04").unwrap();

    let mut s = String::new();

    // Read the string back. Our `\x04` above ended the stream, so we can
    // read to the end of the stream.
    user.read_to_string(&mut s).unwrap();

    assert_eq!(s, "Hello, world!\n");
}
