#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[test]
fn test_fcntl_dupfd_cloexec() {
    use rsix::io_lifetimes::AsFd;
    use std::os::unix::io::AsRawFd;

    let file = rsix::fs::openat(
        &rsix::fs::cwd(),
        "Cargo.toml",
        rsix::fs::OFlags::RDONLY,
        rsix::fs::Mode::empty(),
    )
    .unwrap();

    let new = rsix::fs::fcntl_dupfd_cloexec(&file, 700).unwrap();
    assert_eq!(new.as_fd().as_raw_fd(), 700);
}
