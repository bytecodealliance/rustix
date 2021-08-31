#[cfg(not(target_os = "redox"))]
#[test]
fn test_file() {
    rsix::fs::accessat(
        &rsix::fs::cwd(),
        "Cargo.toml",
        rsix::fs::Access::READ_OK,
        rsix::fs::AtFlags::empty(),
    )
    .unwrap();

    assert_eq!(
        rsix::fs::openat(
            &rsix::fs::cwd(),
            "Cagro.motl",
            rsix::fs::OFlags::RDONLY,
            rsix::fs::Mode::empty(),
        )
        .unwrap_err(),
        rsix::io::Error::NOENT
    );

    let file = rsix::fs::openat(
        &rsix::fs::cwd(),
        "Cargo.toml",
        rsix::fs::OFlags::RDONLY,
        rsix::fs::Mode::empty(),
    )
    .unwrap();

    assert_eq!(
        rsix::fs::openat(
            &file,
            "Cargo.toml",
            rsix::fs::OFlags::RDONLY,
            rsix::fs::Mode::empty(),
        )
        .unwrap_err(),
        rsix::io::Error::NOTDIR
    );

    #[cfg(not(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    )))]
    rsix::fs::fadvise(&file, 0, 10, rsix::fs::Advice::Normal).unwrap();

    assert_eq!(
        rsix::fs::fcntl_getfd(&file).unwrap(),
        rsix::fs::FdFlags::empty()
    );
    assert_eq!(
        rsix::fs::fcntl_getfl(&file).unwrap(),
        rsix::fs::OFlags::empty()
    );

    let stat = rsix::fs::fstat(&file).unwrap();
    assert!(stat.st_size > 0);
    assert!(stat.st_blocks > 0);

    #[cfg(not(any(target_os = "netbsd", target_os = "wasi")))]
    // not implemented in libc for netbsd yet
    {
        let statfs = rsix::fs::fstatfs(&file).unwrap();
        assert!(statfs.f_blocks > 0);
    }

    assert_eq!(rsix::io::is_read_write(&file).unwrap(), (true, false));

    assert_ne!(rsix::io::ioctl_fionread(&file).unwrap(), 0);
}
