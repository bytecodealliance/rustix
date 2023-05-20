use rustix::fs::{cwd, openat, Mode, OFlags};
use rustix::pty::*;
use std::fs::File;
use std::io;
use std::io::{Read, Write};

#[test]
fn openpty_basic() -> io::Result<()> {
    // Use `CLOEXEC` if we can.
    #[cfg(any(linux_kernel, target_os = "freebsd", target_os = "netbsd"))]
    let flags = OpenptFlags::RDWR | OpenptFlags::NOCTTY | OpenptFlags::CLOEXEC;
    #[cfg(not(any(linux_kernel, target_os = "freebsd", target_os = "netbsd")))]
    let flags = OpenptFlags::RDWR | OpenptFlags::NOCTTY;

    let controller = openpt(flags)?;

    grantpt(&controller)?;
    unlockpt(&controller)?;

    let name = ptsname(&controller, Vec::new())?;
    let user = openat(
        cwd(),
        name,
        OFlags::RDWR | OFlags::NOCTTY | OFlags::CLOEXEC,
        Mode::empty(),
    )?;

    let mut controller = File::from(controller);
    let mut user = File::from(user);

    // The '\x04' is Ctrl-D, the default EOF control code.
    controller.write_all(b"Hello, world!\n\x04")?;

    let mut s = String::new();
    user.read_to_string(&mut s)?;

    assert_eq!(s, "Hello, world!\n");
    Ok(())
}

// Like `openpty_basic` but use `ioctl_tiocgptpeer` instead of `ptsname`.
#[cfg(target_os = "linux")]
#[test]
fn openpty_get_peer() -> io::Result<()> {
    // Use `CLOEXEC` if we can.
    #[cfg(any(linux_kernel, target_os = "freebsd", target_os = "netbsd"))]
    let flags = OpenptFlags::RDWR | OpenptFlags::NOCTTY | OpenptFlags::CLOEXEC;
    #[cfg(not(any(linux_kernel, target_os = "freebsd", target_os = "netbsd")))]
    let flags = OpenptFlags::RDWR | OpenptFlags::NOCTTY;

    let controller = openpt(flags)?;

    grantpt(&controller)?;
    unlockpt(&controller)?;

    let user = ioctl_tiocgptpeer(
        &controller,
        OpenptFlags::RDWR | OpenptFlags::NOCTTY | OpenptFlags::CLOEXEC,
    )?;

    let mut controller = File::from(controller);
    let mut user = File::from(user);

    // The '\x04' is Ctrl-D, the default EOF control code.
    controller.write_all(b"Hello, world!\n\x04")?;

    let mut s = String::new();
    user.read_to_string(&mut s)?;

    assert_eq!(s, "Hello, world!\n");
    Ok(())
}
