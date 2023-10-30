use rustix::io::Errno;
use rustix::termios::{tcgetpgrp, tcsetpgrp, Pid};
use tempfile::tempdir;

#[cfg(feature = "fs")]
#[test]
fn pgrp_notty() {
    let tmpdir = tempdir().unwrap();
    let fd = rustix::fs::open(
        tmpdir.path(),
        rustix::fs::OFlags::RDONLY,
        rustix::fs::Mode::empty(),
    )
    .unwrap();

    // A file is not a tty.
    assert_eq!(tcgetpgrp(&fd), Err(Errno::NOTTY));
    assert_eq!(tcsetpgrp(&fd, Pid::INIT), Err(Errno::NOTTY));
}

// Disable on illumos where `tcgetattr` doesn't appear to support
// pseudoterminals.
#[cfg(not(target_os = "illumos"))]
#[cfg(feature = "pty")]
#[test]
fn pgrp_pseudoterminal() {
    use rustix::pty::*;
    use rustix::termios::*;

    let pty = match openpt(OpenptFlags::NOCTTY) {
        Ok(pty) => pty,
        Err(rustix::io::Errno::NOSYS) => return,
        Err(e) => Err(e).unwrap(),
    };

    // Linux's `tcgetpgrp` returns 0 here, which is not documented, so rustix
    // translates it into `OPNOTSUPP`.
    #[cfg(linux_kernel)]
    assert_eq!(tcgetpgrp(&pty), Err(rustix::io::Errno::OPNOTSUPP));

    // FreeBSD's `tcgetpgrp` returns 100000 here, or presumably some other
    // number if that number is already taken, which is documented behavior,
    // but impossible to test for reliably.
    #[cfg(not(linux_kernel))]
    assert!(matches!(tcgetpgrp(&pty), Ok(_)));

    // We shouldn't be able to set the process group to pid 1.
    match tcsetpgrp(&pty, rustix::termios::Pid::INIT).unwrap_err() {
        #[cfg(freebsdlike)]
        rustix::io::Errno::PERM => {}
        #[cfg(any(apple, linux_kernel))]
        rustix::io::Errno::NOTTY => {}
        err => Err(err).unwrap(),
    }
}
