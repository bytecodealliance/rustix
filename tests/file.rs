#[cfg(not(target_os = "redox"))]
#[test]
fn test_file() {
    posish::fs::accessat(
        &posish::fs::cwd(),
        "Cargo.toml",
        posish::fs::Access::READ_OK,
        posish::fs::AtFlags::empty(),
    )
    .unwrap();

    assert_eq!(
        posish::fs::openat(
            &posish::fs::cwd(),
            "Cagro.motl",
            posish::fs::OFlags::RDONLY,
            posish::fs::Mode::empty(),
        )
        .unwrap_err(),
        posish::io::Error::NOENT
    );

    let file = posish::fs::openat(
        &posish::fs::cwd(),
        "Cargo.toml",
        posish::fs::OFlags::RDONLY,
        posish::fs::Mode::empty(),
    )
    .unwrap();

    assert_eq!(
        posish::fs::openat(
            &file,
            "Cargo.toml",
            posish::fs::OFlags::RDONLY,
            posish::fs::Mode::empty(),
        )
        .unwrap_err(),
        posish::io::Error::NOTDIR
    );

    #[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "netbsd")))]
    posish::fs::fadvise(&file, 0, 10, posish::fs::Advice::Normal).unwrap();

    assert_eq!(
        posish::fs::fcntl_getfd(&file).unwrap(),
        posish::fs::FdFlags::empty()
    );
    assert_eq!(
        posish::fs::fcntl_getfl(&file).unwrap(),
        posish::fs::OFlags::empty()
    );

    let stat = posish::fs::fstat(&file).unwrap();
    assert!(stat.st_size > 0);
    assert!(stat.st_blocks > 0);

    #[cfg(not(any(target_os = "netbsd", target_os = "wasi")))]
    // not implemented in libc for netbsd yet
    {
        let statfs = posish::fs::fstatfs(&file).unwrap();
        assert!(statfs.f_blocks > 0);
    }

    assert_eq!(posish::io::is_read_write(&file).unwrap(), (true, false));

    assert_ne!(posish::io::ioctl_fionread(&file).unwrap(), 0);
}
