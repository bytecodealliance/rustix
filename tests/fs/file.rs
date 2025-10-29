#[cfg(not(any(
    apple,
    netbsdlike,
    target_os = "dragonfly",
    target_os = "espidf",
    target_os = "haiku",
    target_os = "redox",
    target_os = "solaris",
)))]
use core::num::NonZeroU64;

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

    rustix::fs::chown("Cargo.toml", None, None).unwrap();

    #[cfg(not(any(target_os = "android", target_os = "emscripten")))]
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
        Err(err) => panic!("{:?}", err),
    }

    // Check that `SYMLINK_FOLLOW` is rejected. Except on NetBSD which seems to
    // permit it.
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

    rustix::fs::fchown(&file, None, None).unwrap();

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
        target_os = "dragonfly",
        target_os = "espidf",
        target_os = "haiku",
        target_os = "solaris",
    )))]
    rustix::fs::fadvise(&file, 0, NonZeroU64::new(10), rustix::fs::Advice::Normal).unwrap();

    #[cfg(not(target_os = "cygwin"))]
    rustix::fs::fsync(&file).unwrap();

    #[cfg(not(any(
        apple,
        target_os = "cygwin",
        target_os = "dragonfly",
        target_os = "espidf",
        target_os = "haiku",
    )))]
    rustix::fs::fdatasync(&file).unwrap();

    // Test `fcntl_getfd`.
    assert_eq!(
        rustix::io::fcntl_getfd(&file).unwrap(),
        rustix::io::FdFlags::empty()
    );

    // Test `fcntl_getfl`.
    let fl = rustix::fs::fcntl_getfl(&file).unwrap();

    // Clear `O_LARGEFILE`, which may be set by rustix on 32-bit Linux or
    // automatically by some kernel on 64-bit (Linux and illumos).
    #[cfg(any(linux_kernel, solarish))]
    let fl = fl - rustix::fs::OFlags::LARGEFILE;

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
        target_os = "wasi",
    )))]
    {
        let statfs = rustix::fs::fstatfs(&file).unwrap();
        assert!(statfs.f_blocks > 0);
    }

    #[cfg(not(any(target_os = "redox", target_os = "wasi")))]
    {
        let statvfs = rustix::fs::fstatvfs(&file).unwrap();
        assert!(statvfs.f_frsize > 0);
    }

    #[cfg(not(target_os = "cygwin"))]
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

    // Overwrite the first few bytes.
    let file =
        rustix::fs::open(tmp.path().join("test.file"), OFlags::WRONLY, Mode::empty()).unwrap();
    assert_eq!(rustix::io::write(&file, b"uvw"), Ok(3));

    // Append a few bytes.
    rustix::fs::fcntl_setfl(&file, OFlags::APPEND).unwrap();
    assert_eq!(rustix::io::write(&file, b"xyz"), Ok(3));

    // Check the resulting contents.
    let file =
        rustix::fs::open(tmp.path().join("test.file"), OFlags::RDONLY, Mode::empty()).unwrap();
    let mut buf = [0_u8; 32];
    assert_eq!(rustix::io::read(&file, &mut buf), Ok(19));
    assert_eq!(&buf, b"uvwdefghijklmnopxyz\0\0\0\0\0\0\0\0\0\0\0\0\0");
}

#[test]
fn test_mode() {
    use rustix::fs::{Mode, RawMode};

    let mode = Mode::from_raw_mode((libc::S_IFSOCK | libc::S_IRUSR) as RawMode);
    assert_eq!(mode, Mode::RUSR);
    assert_eq!(mode.bits(), libc::S_IRUSR as RawMode);

    let mode = Mode::from_raw_mode((libc::S_IFSOCK | libc::S_IRWXU) as RawMode);
    assert_eq!(mode, Mode::RWXU);
    assert_eq!(mode.bits(), libc::S_IRWXU as RawMode);
}
