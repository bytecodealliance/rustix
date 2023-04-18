use std::io;

#[test]
fn xattr_basic() {
    use rustix::fs::XattrFlags;

    // The error code when an attribute doesn't exist.
    #[cfg(not(apple))]
    let enodata = libc::ENODATA;
    #[cfg(apple)]
    let enodata = libc::ENOATTR;

    assert_eq!(
        rustix::fs::getxattr("/no/such/path", "user.test", &mut [])
            .unwrap_err()
            .kind(),
        io::ErrorKind::NotFound
    );
    assert_eq!(
        rustix::fs::lgetxattr("/no/such/path", "user.test", &mut [])
            .unwrap_err()
            .kind(),
        io::ErrorKind::NotFound
    );
    assert_eq!(
        rustix::fs::setxattr("/no/such/path", "user.test", &[], XattrFlags::REPLACE)
            .unwrap_err()
            .kind(),
        io::ErrorKind::NotFound
    );
    assert_eq!(
        rustix::fs::lsetxattr("/no/such/path", "user.test", &[], XattrFlags::REPLACE)
            .unwrap_err()
            .kind(),
        io::ErrorKind::NotFound
    );
    assert_eq!(
        rustix::fs::listxattr("/no/such/path", &mut [])
            .unwrap_err()
            .kind(),
        io::ErrorKind::NotFound
    );
    assert_eq!(
        rustix::fs::llistxattr("/no/such/path", &mut [])
            .unwrap_err()
            .kind(),
        io::ErrorKind::NotFound
    );
    assert_eq!(
        rustix::fs::removexattr("/no/such/path", "user.test")
            .unwrap_err()
            .kind(),
        io::ErrorKind::NotFound
    );
    assert_eq!(
        rustix::fs::lremovexattr("/no/such/path", "user.test")
            .unwrap_err()
            .kind(),
        io::ErrorKind::NotFound
    );

    assert_eq!(
        rustix::fs::getxattr("Cargo.toml", "user.test", &mut [])
            .unwrap_err()
            .raw_os_error(),
        enodata
    );
    assert_eq!(
        rustix::fs::lgetxattr("Cargo.toml", "user.test", &mut [])
            .unwrap_err()
            .raw_os_error(),
        enodata
    );
    assert_eq!(
        rustix::fs::setxattr("Cargo.toml", "user.test", &[], XattrFlags::REPLACE)
            .unwrap_err()
            .raw_os_error(),
        enodata
    );
    assert_eq!(
        rustix::fs::lsetxattr("Cargo.toml", "user.test", &[], XattrFlags::REPLACE)
            .unwrap_err()
            .raw_os_error(),
        enodata
    );
    assert_eq!(rustix::fs::listxattr("Cargo.toml", &mut []).unwrap(), 0);
    assert_eq!(rustix::fs::llistxattr("Cargo.toml", &mut []).unwrap(), 0);
    assert_eq!(
        rustix::fs::removexattr("Cargo.toml", "user.test")
            .unwrap_err()
            .raw_os_error(),
        enodata
    );
    assert_eq!(
        rustix::fs::lremovexattr("Cargo.toml", "user.test")
            .unwrap_err()
            .raw_os_error(),
        enodata
    );

    let file = std::fs::File::open("Cargo.toml").unwrap();
    assert_eq!(
        rustix::fs::fgetxattr(&file, "user.test", &mut [])
            .unwrap_err()
            .raw_os_error(),
        enodata
    );
    assert_eq!(
        rustix::fs::fsetxattr(&file, "user.test", &[], XattrFlags::REPLACE)
            .unwrap_err()
            .raw_os_error(),
        enodata
    );
    assert_eq!(rustix::fs::flistxattr(&file, &mut []).unwrap(), 0);
    assert_eq!(
        rustix::fs::fremovexattr(&file, "user.test")
            .unwrap_err()
            .raw_os_error(),
        enodata
    );
}
