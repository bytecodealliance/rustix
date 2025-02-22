#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[test]
fn test_fcntl_dupfd_cloexec() {
    use rustix::fd::AsFd as _;
    use std::os::unix::io::AsRawFd as _;

    let file = rustix::fs::openat(
        rustix::fs::CWD,
        "Cargo.toml",
        rustix::fs::OFlags::RDONLY,
        rustix::fs::Mode::empty(),
    )
    .unwrap();

    let new = rustix::io::fcntl_dupfd_cloexec(&file, 700).unwrap();
    assert_eq!(new.as_fd().as_raw_fd(), 700);
}

#[cfg(apple)]
#[test]
fn test_fcntl_apple() {
    use rustix::fs::{openat, Mode, OFlags, CWD};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let file = openat(
        &dir,
        "file",
        OFlags::RDWR | OFlags::CREATE | OFlags::TRUNC,
        Mode::RUSR | Mode::WUSR,
    )
    .unwrap();

    // It appears `fsync_rdadvise` at offset 0 length 0 doesn't work if the
    // file has size zero, so write in some bytes.
    assert_eq!(
        rustix::io::write(&file, b"data").expect("write"),
        4,
        "write failed"
    );

    rustix::fs::fcntl_rdadvise(&file, 0, 0).unwrap();
    rustix::fs::fcntl_fullfsync(&file).unwrap();
    rustix::fs::fcntl_nocache(&file, true).unwrap();
    rustix::fs::fcntl_global_nocache(&file, true).unwrap();
}
