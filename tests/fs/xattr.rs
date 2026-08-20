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

#[cfg(linux_kernel)]
#[test]
fn xattrat_missing_path_and_unknown_flags() {
    use rustix::fs::{getxattrat, listxattrat, AtFlags};
    use rustix::io::Errno;

    let dir = std::fs::File::open(".").unwrap();
    let mut empty: [u8; 0] = [];

    let get_error = getxattrat(
        &dir,
        "rustix-xattrat-missing-path",
        "user.rustix_test",
        &mut empty,
        AtFlags::empty(),
    )
    .unwrap_err();
    if get_error == Errno::NOSYS {
        return;
    }
    assert_eq!(get_error, Errno::NOENT);

    let list_error = listxattrat(
        &dir,
        "rustix-xattrat-missing-path",
        &mut empty,
        AtFlags::empty(),
    )
    .unwrap_err();
    if list_error == Errno::NOSYS {
        return;
    }
    assert_eq!(list_error, Errno::NOENT);

    let unknown = AtFlags::from_bits_retain(0x8000_0000);
    assert_eq!(
        getxattrat(&dir, "Cargo.toml", "user.rustix_test", &mut empty, unknown,).unwrap_err(),
        Errno::INVAL
    );
    assert_eq!(
        listxattrat(&dir, "Cargo.toml", &mut empty, unknown).unwrap_err(),
        Errno::INVAL
    );
}

#[cfg(linux_kernel)]
#[test]
fn xattrat_dirfd_symlink_and_empty_path() {
    use rustix::fs::{
        getxattrat, listxattrat, openat, setxattr, AtFlags, Mode, OFlags, XattrFlags,
    };
    use rustix::io::Errno;
    use std::os::unix::fs::symlink;

    const NAME: &str = "user.rustix_xattrat_test";
    const VALUE: &[u8] = b"target-value";

    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(tmp.path().join("target"), b"").unwrap();
    symlink("target", tmp.path().join("link")).unwrap();
    match setxattr(tmp.path().join("target"), NAME, VALUE, XattrFlags::empty()) {
        Ok(()) => {}
        Err(Errno::NOTSUP | Errno::PERM | Errno::ACCESS) => return,
        Err(error) => panic!("failed to prepare xattr fixture: {error}"),
    }

    let dir = std::fs::File::open(tmp.path()).unwrap();
    let mut value = [0_u8; 64];
    let followed = match getxattrat(&dir, "link", NAME, &mut value, AtFlags::empty()) {
        Err(Errno::NOSYS) => return,
        result => result.unwrap(),
    };
    assert_eq!(&value[..followed], VALUE);

    assert_eq!(
        getxattrat(&dir, "link", NAME, &mut value, AtFlags::SYMLINK_NOFOLLOW).unwrap_err(),
        Errno::NODATA
    );

    let mut followed_list_storage = [0_u8; 4096];
    let followed_list =
        match listxattrat(&dir, "link", &mut followed_list_storage, AtFlags::empty()) {
            Err(Errno::NOSYS) => return,
            result => result.unwrap(),
        };
    assert!(xattr_list_contains(
        &followed_list_storage[..followed_list],
        NAME.as_bytes()
    ));

    let mut link_list_storage = [0_u8; 4096];
    let link_list = listxattrat(
        &dir,
        "link",
        &mut link_list_storage,
        AtFlags::SYMLINK_NOFOLLOW,
    )
    .unwrap();
    assert!(!xattr_list_contains(
        &link_list_storage[..link_list],
        NAME.as_bytes()
    ));

    let target = std::fs::File::open(tmp.path().join("target")).unwrap();
    let target_value_len = getxattrat(&target, "", NAME, &mut value, AtFlags::EMPTY_PATH).unwrap();
    assert_eq!(&value[..target_value_len], VALUE);
    let mut target_list_storage = [0_u8; 4096];
    let target_list =
        listxattrat(&target, "", &mut target_list_storage, AtFlags::EMPTY_PATH).unwrap();
    assert!(xattr_list_contains(
        &target_list_storage[..target_list],
        NAME.as_bytes()
    ));

    let path_symlink =
        openat(&dir, "link", OFlags::PATH | OFlags::NOFOLLOW, Mode::empty()).unwrap();
    assert_eq!(
        getxattrat(&path_symlink, "", NAME, &mut value, AtFlags::EMPTY_PATH).unwrap_err(),
        Errno::BADF
    );
    let mut path_list_storage = [0_u8; 4096];
    assert_eq!(
        listxattrat(
            &path_symlink,
            "",
            &mut path_list_storage,
            AtFlags::EMPTY_PATH,
        )
        .unwrap_err(),
        Errno::BADF
    );
}

#[cfg(linux_kernel)]
#[test]
fn xattrat_buffer_bounds() {
    use rustix::buffer::spare_capacity;
    use rustix::fs::{getxattrat, listxattrat, setxattr, AtFlags, XattrFlags};
    use rustix::io::Errno;
    use std::mem::MaybeUninit;

    const NAME: &str = "user.rustix_xattrat_bounds";
    const VALUE: &[u8] = b"more-than-one-byte";

    let tmp = tempfile::tempdir().unwrap();
    let target_path = tmp.path().join("target");
    std::fs::write(&target_path, b"").unwrap();
    match setxattr(&target_path, NAME, VALUE, XattrFlags::empty()) {
        Ok(()) => {}
        Err(Errno::NOTSUP | Errno::PERM | Errno::ACCESS) => return,
        Err(error) => panic!("failed to prepare xattr bounds fixture: {error}"),
    }

    let dir = std::fs::File::open(tmp.path()).unwrap();

    let mut successful_uninit = [MaybeUninit::<u8>::uninit(); 32];
    let (initialized, remaining) = match getxattrat(
        &dir,
        "target",
        NAME,
        &mut successful_uninit,
        AtFlags::empty(),
    ) {
        Err(Errno::NOSYS) => return,
        result => result.unwrap(),
    };
    assert_eq!(initialized, VALUE);
    assert_eq!(remaining.len(), 32 - VALUE.len());

    let prefix = [0xde, 0xad];
    let mut appended = prefix.to_vec();
    appended.reserve(VALUE.len());
    let appended_len = getxattrat(
        &dir,
        "target",
        NAME,
        spare_capacity(&mut appended),
        AtFlags::empty(),
    )
    .unwrap();
    assert_eq!(appended_len, VALUE.len());
    assert_eq!(&appended[..prefix.len()], &prefix);
    assert_eq!(&appended[prefix.len()..], VALUE);

    let expected_list_len = libc_listxattr(target_path.to_str().unwrap());
    assert!(expected_list_len > NAME.len());
    let mut exact_list = vec![MaybeUninit::<u8>::uninit(); expected_list_len];
    let (initialized_list, remaining_list) =
        listxattrat(&dir, "target", exact_list.as_mut_slice(), AtFlags::empty()).unwrap();
    assert_eq!(initialized_list.len(), expected_list_len);
    assert!(remaining_list.is_empty());
    assert!(xattr_list_contains(initialized_list, NAME.as_bytes()));

    let mut list_with_slack = vec![MaybeUninit::<u8>::uninit(); expected_list_len + 1];
    let (initialized_list, remaining_list) = listxattrat(
        &dir,
        "target",
        list_with_slack.as_mut_slice(),
        AtFlags::empty(),
    )
    .unwrap();
    assert_eq!(initialized_list.len(), expected_list_len);
    assert_eq!(remaining_list.len(), 1);
    assert!(xattr_list_contains(initialized_list, NAME.as_bytes()));

    let mut zero_uninit: [MaybeUninit<u8>; 0] = [];
    let get_error = match getxattrat(&dir, "target", NAME, &mut zero_uninit, AtFlags::empty()) {
        Err(Errno::NOSYS) => return,
        result => result.unwrap_err(),
    };
    assert_eq!(get_error, Errno::RANGE);

    let mut zero_initialized: [u8; 0] = [];
    assert_eq!(
        getxattrat(
            &dir,
            "target",
            NAME,
            &mut zero_initialized,
            AtFlags::empty(),
        )
        .unwrap_err(),
        Errno::RANGE
    );
    assert_eq!(
        listxattrat(&dir, "target", &mut zero_uninit, AtFlags::empty(),).unwrap_err(),
        Errno::RANGE
    );

    let mut zero_spare = Vec::with_capacity(1);
    zero_spare.resize(zero_spare.capacity(), 0xa5);
    let original = zero_spare.clone();
    assert_eq!(
        getxattrat(
            &dir,
            "target",
            NAME,
            spare_capacity(&mut zero_spare),
            AtFlags::empty(),
        )
        .unwrap_err(),
        Errno::RANGE
    );
    assert_eq!(zero_spare, original);
    assert_eq!(
        listxattrat(
            &dir,
            "target",
            spare_capacity(&mut zero_spare),
            AtFlags::empty(),
        )
        .unwrap_err(),
        Errno::RANGE
    );
    assert_eq!(zero_spare, original);

    let mut short_get = [MaybeUninit::<u8>::uninit(); 1];
    assert_eq!(
        getxattrat(&dir, "target", NAME, &mut short_get, AtFlags::empty(),).unwrap_err(),
        Errno::RANGE
    );
    let mut short_list = [MaybeUninit::<u8>::uninit(); 1];
    assert_eq!(
        listxattrat(&dir, "target", &mut short_list, AtFlags::empty(),).unwrap_err(),
        Errno::RANGE
    );
}

#[cfg(linux_kernel)]
fn xattr_list_contains(list: &[u8], name: &[u8]) -> bool {
    list.split(|byte| *byte == 0).any(|entry| entry == name)
}
