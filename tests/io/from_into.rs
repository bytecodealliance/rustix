#[cfg(not(target_os = "redox"))]
#[test]
fn test_owned() {
    use io_lifetimes::AsFd;
    #[cfg(unix)]
    use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};
    #[cfg(target_os = "wasi")]
    use std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd};

    let file = rsix::fs::openat(
        &rsix::fs::cwd(),
        "Cargo.toml",
        rsix::fs::OFlags::RDONLY,
        rsix::fs::Mode::empty(),
    )
    .unwrap();

    let raw = file.as_raw_fd();
    assert_eq!(raw, file.as_fd().as_raw_fd());

    let owned: io_lifetimes::OwnedFd = file.into();
    let inner = owned.into_raw_fd();
    assert_eq!(raw, inner);

    let new = unsafe { rsix::io::OwnedFd::from_raw_fd(inner) };
    let mut buf = [0_u8; 4];
    let _ = rsix::io::read(&new, &mut buf).unwrap();
}
