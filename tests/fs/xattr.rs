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
fn set() {
    use rustix::fs::XattrFlags;

    let mut buf = vec![0_u8; 128];
    let value = b"blue green";

    let tempdir = tempfile::tempdir().unwrap();

    let file_path = tempdir.path().join("file.file");
    let file = std::fs::File::create(&file_path).unwrap();

    assert_eq!(rustix::fs::listxattr(&file_path, &mut buf), Ok(0));
    assert_eq!(rustix::fs::listxattr_query(&file_path), Ok(0));
    assert_eq!(rustix::fs::llistxattr(&file_path, &mut buf), Ok(0));
    assert_eq!(rustix::fs::llistxattr_query(&file_path), Ok(0));
    assert_eq!(rustix::fs::flistxattr(&file, &mut buf), Ok(0));
    assert_eq!(rustix::fs::flistxattr_query(&file), Ok(0));
    match rustix::fs::setxattr(
        &file_path,
        "user.my fun attribute",
        value,
        XattrFlags::empty(),
    ) {
        Ok(()) => {}
        Err(rustix::io::Errno::NOTSUP) => return,
        Err(err) => Err(err).unwrap(),
    }
    assert!(rustix::fs::listxattr(&file_path, &mut buf).unwrap() > value.len());
    assert!(rustix::fs::listxattr_query(&file_path).unwrap() > value.len());
    assert!(rustix::fs::llistxattr(&file_path, &mut buf).unwrap() > value.len());
    assert!(rustix::fs::llistxattr_query(&file_path).unwrap() > value.len());
    assert!(rustix::fs::flistxattr(&file, &mut buf).unwrap() > value.len());
    assert!(rustix::fs::flistxattr_query(&file).unwrap() > value.len());
    buf.fill(0);
    assert_eq!(
        rustix::fs::getxattr_query(&file_path, "user.my fun attribute"),
        Ok(value.len())
    );
    assert_eq!(
        rustix::fs::getxattr(&file_path, "user.my fun attribute", &mut buf),
        Ok(value.len())
    );
    assert_eq!(&buf[0..value.len()], value);
    buf.fill(0);
    assert_eq!(
        rustix::fs::lgetxattr_query(&file_path, "user.my fun attribute"),
        Ok(value.len())
    );
    assert_eq!(
        rustix::fs::lgetxattr(&file_path, "user.my fun attribute", &mut buf),
        Ok(value.len())
    );
    assert_eq!(&buf[0..value.len()], value);
    buf.fill(0);
    assert_eq!(
        rustix::fs::fgetxattr_query(&file, "user.my fun attribute"),
        Ok(value.len())
    );
    assert_eq!(
        rustix::fs::fgetxattr(&file, "user.my fun attribute", &mut buf),
        Ok(value.len())
    );
    assert_eq!(&buf[0..value.len()], value);
}

/// Test that the underlying special case for empty buffers is handled properly.
#[test]
fn empty_buffers() {
    use rustix::fs::XattrFlags;

    let mut empty: [u8; 0] = [];
    let value = b"purple orange";

    let tempdir = tempfile::tempdir().unwrap();

    let file_path = tempdir.path().join("file.file");
    let file = std::fs::File::create(&file_path).unwrap();

    assert_eq!(rustix::fs::listxattr(&file_path, &mut empty), Ok(0));
    assert_eq!(rustix::fs::llistxattr(&file_path, &mut empty), Ok(0));
    assert_eq!(rustix::fs::flistxattr(&file, &mut empty), Ok(0));
    match rustix::fs::setxattr(
        &file_path,
        "user.my fun attribute",
        value,
        XattrFlags::empty(),
    ) {
        Ok(()) => {}
        Err(rustix::io::Errno::NOTSUP) => return,
        Err(err) => Err(err).unwrap(),
    }
    assert_eq!(rustix::fs::listxattr(&file_path, &mut empty), Ok(0));
    assert_eq!(rustix::fs::llistxattr(&file_path, &mut empty), Ok(0));
    assert_eq!(rustix::fs::flistxattr(&file, &mut empty), Ok(0));
    assert_eq!(
        rustix::fs::getxattr(&file_path, "user.my fun attribute", &mut empty),
        Ok(0)
    );
    assert_eq!(
        rustix::fs::lgetxattr(&file_path, "user.my fun attribute", &mut empty),
        Ok(0)
    );
    assert_eq!(
        rustix::fs::fgetxattr(&file, "user.my fun attribute", &mut empty),
        Ok(0)
    );
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
