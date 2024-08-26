use rustix::io::Errno;
use rustix::termios::tcgetsid;
use tempfile::tempdir;

#[cfg(feature = "fs")]
#[test]
fn sid_notty() {
    let tmpdir = tempdir().unwrap();
    let fd = rustix::fs::open(
        tmpdir.path(),
        rustix::fs::OFlags::RDONLY,
        rustix::fs::Mode::empty(),
    )
    .unwrap();

    // A file is not a tty.
    assert_eq!(tcgetsid(&fd), Err(Errno::NOTTY));
}

// Disable on Redox which lacks `getsid`.
#[cfg(not(target_os = "redox"))]
#[cfg(all(feature = "stdio", feature = "process"))]
#[test]
fn sid_match() {
    match tcgetsid(rustix::stdio::stdin()) {
        Ok(sid) => assert_eq!(sid, rustix::process::getsid(None).unwrap()),
        Err(_err) => {}
    }
}
