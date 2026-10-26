use std::io;

#[test]
fn xattr_basic() {
    use rustix::fs::XattrFlags;

    // The error code when an attribute doesn't exist.
    #[cfg(not(apple))]
    let enodata = libc::ENODATA;
    #[cfg(apple)]
    let enodata = libc::ENOATTR;

    let mut empty: [u8; 0] = [];

    assert_eq!(
        rustix::fs::getxattr("/no/such/path", "user.test", &mut empty)
            .unwrap_err()
            .kind(),
        io::ErrorKind::NotFound
    );
    assert_eq!(
        rustix::fs::lgetxattr("/no/such/path", "user.test", &mut empty)
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
        rustix::fs::listxattr("/no/such/path", &mut empty)
            .unwrap_err()
            .kind(),
        io::ErrorKind::NotFound
    );
    assert_eq!(
        rustix::fs::llistxattr("/no/such/path", &mut empty)
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
        rustix::fs::getxattr("Cargo.toml", "user.test", &mut empty)
            .unwrap_err()
            .raw_os_error(),
        enodata
    );
    assert_eq!(
        rustix::fs::lgetxattr("Cargo.toml", "user.test", &mut empty)
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
    assert_eq!(
        rustix::fs::listxattr("Cargo.toml", &mut empty).unwrap(),
        libc_listxattr("Cargo.toml")
    );
    assert_eq!(
        rustix::fs::llistxattr("Cargo.toml", &mut empty).unwrap(),
        libc_listxattr("Cargo.toml")
    );
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
        rustix::fs::fgetxattr(&file, "user.test", &mut empty)
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
    assert_eq!(
        rustix::fs::flistxattr(&file, &mut empty).unwrap(),
        libc_listxattr("Cargo.toml")
    );
    assert_eq!(
        rustix::fs::fremovexattr(&file, "user.test")
            .unwrap_err()
            .raw_os_error(),
        enodata
    );
}

#[test]
fn xattr_size_query_uninit() {
    use rustix::fs::XattrFlags;
    use std::mem::MaybeUninit;

    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("file");
    std::fs::write(&path, b"").unwrap();
    rustix::fs::setxattr(&path, "user.test", b"hello", XattrFlags::empty()).unwrap();

    let empty: &mut [MaybeUninit<u8>] = &mut [];
    let (init, uninit) = rustix::fs::getxattr(&path, "user.test", empty).unwrap();
    assert!(init.is_empty());
    assert!(uninit.is_empty());

    let empty: &mut [MaybeUninit<u8>] = &mut [];
    let (init, uninit) = rustix::fs::listxattr(&path, empty).unwrap();
    assert!(init.is_empty());
    assert!(uninit.is_empty());
}

/// To check the correctness of the tested implementations of *listxattr(), their output can be
/// compared to an external implementation, in this case listxattr() from the libc crate.
fn libc_listxattr(path: &str) -> usize {
    let path = std::ffi::CString::new(path).unwrap();
    let path: *const _ = path.as_ptr();

    let list = std::ffi::CString::new("").unwrap();
    let list = list.as_ptr() as *mut _;

    ({
        #[cfg(not(apple))]
        unsafe {
            libc::listxattr(path, list, 0)
        }

        #[cfg(apple)]
        unsafe {
            libc::listxattr(path, list, 0, 0)
        }
    }) as usize
}
