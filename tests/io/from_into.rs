#[cfg(feature = "fs")]
#[cfg(not(target_os = "redox"))]
#[test]
fn test_owned() {
    use rustix::fd::{AsFd as _, AsRawFd as _, FromRawFd as _, IntoRawFd as _};

    let file = rustix::fs::openat(
        rustix::fs::CWD,
        "Cargo.toml",
        rustix::fs::OFlags::RDONLY,
        rustix::fs::Mode::empty(),
    )
    .unwrap();

    let raw = file.as_raw_fd();
    assert_eq!(raw, file.as_fd().as_raw_fd());

    let inner = file.into_raw_fd();
    assert_eq!(raw, inner);

    let new = unsafe { rustix::fd::OwnedFd::from_raw_fd(inner) };
    let mut buf = [0_u8; 4];
    let _ = rustix::io::read(&new, &mut buf).unwrap();
}
