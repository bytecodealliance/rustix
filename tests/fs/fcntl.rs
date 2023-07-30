#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[test]
fn test_fcntl_dupfd_cloexec() {
    use rustix::fd::AsFd;
    use std::os::unix::io::AsRawFd;

    let file = rustix::fs::openat(
        rustix::fs::CWD,
        "Cargo.toml",
        rustix::fs::OFlags::RDONLY,
        rustix::fs::Mode::empty(),
    )
    .unwrap();

    let new = rustix::fs::fcntl_dupfd_cloexec(&file, 700).unwrap();
    assert_eq!(new.as_fd().as_raw_fd(), 700);
}

#[cfg(apple)]
#[test]
fn test_fcntl_apple() {
    use rustix::fs::{openat, Mode, OFlags, CWD};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let foo = openat(
        &dir,
        "foo",
        OFlags::RDWR | OFlags::CREATE | OFlags::TRUNC,
        Mode::RUSR | Mode::WUSR,
    )
    .unwrap();

    rustix::fs::fcntl_rdadvise(&foo, 0, 0).unwrap();
    rustix::fs::fcntl_fullfsync(&foo).unwrap();
    rustix::fs::fcntl_nocache(&foo, true).unwrap();
    rustix::fs::fcntl_global_nocache(&foo, true).unwrap();
}
