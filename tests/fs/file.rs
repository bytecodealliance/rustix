#[cfg(not(target_os = "redox"))]
#[test]
fn test_file() {
    rustix::fs::accessat(
        rustix::fs::CWD,
        "Cargo.toml",
        rustix::fs::Access::READ_OK,
        rustix::fs::AtFlags::empty(),
    )
    .unwrap();

    #[cfg(not(any(target_os = "emscripten", target_os = "android")))]
    #[allow(unreachable_patterns)]
    match rustix::fs::accessat(
        rustix::fs::CWD,
        "Cargo.toml",
        rustix::fs::Access::READ_OK,
        rustix::fs::AtFlags::EACCESS,
    ) {
        Ok(()) => (),
        Err(
            rustix::io::Errno::NOSYS | rustix::io::Errno::NOTSUP | rustix::io::Errno::OPNOTSUPP,
        ) => {
            #[cfg(feature = "process")]
            if rustix::process::getuid() == rustix::process::geteuid()
                && rustix::process::getgid() == rustix::process::getegid()
            {
                panic!(
                    "accessat with EACCESS should always work when the effective uid/gid match \
                     the real uid/gid"
                )
            }
        }
        Err(err) => Err(err).unwrap(),
    }

    // Check that `SYMLINK_FOLLOW` is rejected. Except on NetBSD which seems
    // to permit it.
    #[cfg(not(target_os = "netbsd"))]
    assert_eq!(
        rustix::fs::accessat(
            rustix::fs::CWD,
            "Cargo.toml",
            rustix::fs::Access::READ_OK,
            rustix::fs::AtFlags::SYMLINK_FOLLOW,
        ),
        Err(rustix::io::Errno::INVAL)
    );

    assert_eq!(
        rustix::fs::openat(
            rustix::fs::CWD,
            "Cagro.motl",
            rustix::fs::OFlags::RDONLY,
            rustix::fs::Mode::empty(),
        )
        .unwrap_err(),
        rustix::io::Errno::NOENT
    );

    let file = rustix::fs::openat(
        rustix::fs::CWD,
        "Cargo.toml",
        rustix::fs::OFlags::RDONLY,
        rustix::fs::Mode::empty(),
    )
    .unwrap();

    assert_eq!(
        rustix::fs::openat(
            &file,
            "Cargo.toml",
            rustix::fs::OFlags::RDONLY,
            rustix::fs::Mode::empty(),
        )
        .unwrap_err(),
        rustix::io::Errno::NOTDIR
    );

    #[cfg(not(any(
        apple,
        netbsdlike,
        solarish,
        target_os = "dragonfly",
        target_os = "espidf",
        target_os = "haiku",
        target_os = "redox",
    )))]
    rustix::fs::fadvise(&file, 0, 10, rustix::fs::Advice::Normal).unwrap();

    rustix::fs::fsync(&file).unwrap();

    #[cfg(not(any(
        apple,
        target_os = "dragonfly",
        target_os = "espidf",
        target_os = "haiku",
        target_os = "redox",
    )))]
    rustix::fs::fdatasync(&file).unwrap();

    // Test `fcntl_getfd`.
    assert_eq!(
        rustix::io::fcntl_getfd(&file).unwrap(),
        rustix::io::FdFlags::empty()
    );

    // Test `fcntl_getfl`.
    let fl = rustix::fs::fcntl_getfl(&file).unwrap();

    // On Linux, rustix automatically sets `O_LARGEFILE`, so clear it here so
    // that we can test that no other bits are present.
    #[cfg(linux_kernel)]
    let fl = fl - rustix::fs::OFlags::from_bits_retain(linux_raw_sys::general::O_LARGEFILE);

    // On illumos, the system automatically sets `O_LARGEFILE`, so clear it
    // here so that we can test that no other bits are present.
    #[cfg(target_os = "illumos")]
    let fl = fl - rustix::fs::OFlags::from_bits_retain(0x2000);

    assert_eq!(fl, rustix::fs::OFlags::empty());

    // Test `fcntl_setfd`.
    rustix::io::fcntl_setfd(&file, rustix::io::FdFlags::CLOEXEC).unwrap();
    assert_eq!(
        rustix::io::fcntl_getfd(&file).unwrap(),
        rustix::io::FdFlags::CLOEXEC
    );

    let stat = rustix::fs::fstat(&file).unwrap();
    assert!(stat.st_size > 0);
    assert!(stat.st_blocks > 0);

    #[cfg(not(any(
        solarish,
        target_os = "haiku",
        target_os = "netbsd",
        target_os = "redox",
        target_os = "wasi",
    )))]
    {
        let statfs = rustix::fs::fstatfs(&file).unwrap();
        assert!(statfs.f_blocks > 0);
    }

    #[cfg(not(any(target_os = "haiku", target_os = "redox", target_os = "wasi")))]
    {
        let statvfs = rustix::fs::fstatvfs(&file).unwrap();
        assert!(statvfs.f_frsize > 0);
    }

    #[cfg(all(feature = "fs", feature = "net"))]
    assert_eq!(rustix::io::is_read_write(&file).unwrap(), (true, false));

    assert_ne!(rustix::io::ioctl_fionread(&file).unwrap(), 0);
}

#[test]
fn test_setfl_append() {
    use rustix::fs::{Mode, OFlags};

    let tmp = tempfile::tempdir().unwrap();

    // Write some bytes to a file.
    let file = rustix::fs::open(
        tmp.path().join("test.file"),
        OFlags::WRONLY | OFlags::CREATE,
        Mode::RWXU,
    )
    .unwrap();
    assert_eq!(rustix::io::write(&file, b"abcdefghijklmnop"), Ok(16));

    // Overwite the first few bytes.
    let file =
        rustix::fs::open(tmp.path().join("test.file"), OFlags::WRONLY, Mode::empty()).unwrap();
    assert_eq!(rustix::io::write(&file, b"uvw"), Ok(3));

    // Append a few bytes.
    rustix::fs::fcntl_setfl(&file, OFlags::APPEND).unwrap();
    assert_eq!(rustix::io::write(&file, b"xyz"), Ok(3));

    // Check the final contents.
    let file =
        rustix::fs::open(tmp.path().join("test.file"), OFlags::RDONLY, Mode::empty()).unwrap();
    let mut buf = [0_u8; 32];
    assert_eq!(rustix::io::read(&file, &mut buf), Ok(19));
    assert_eq!(&buf, b"uvwdefghijklmnopxyz\0\0\0\0\0\0\0\0\0\0\0\0\0");
}
