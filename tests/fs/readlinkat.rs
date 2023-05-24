#[test]
fn test_readlink() {
    use rustix::fs::{open, readlink, symlink, Mode, OFlags};

    let tmp = tempfile::tempdir().unwrap();

    let _ = open(
        tmp.path().join("foo"),
        OFlags::CREATE | OFlags::WRONLY,
        Mode::RUSR,
    )
    .unwrap();
    symlink("foo", tmp.path().join("link")).unwrap();

    readlink(tmp.path().join("absent"), Vec::new()).unwrap_err();
    readlink(tmp.path().join("foo"), Vec::new()).unwrap_err();

    let target = readlink(tmp.path().join("link"), Vec::new()).unwrap();
    assert_eq!(target.to_string_lossy(), "foo");

    symlink("link", tmp.path().join("another")).unwrap();

    let target = readlink(tmp.path().join("link"), Vec::new()).unwrap();
    assert_eq!(target.to_string_lossy(), "foo");
    let target = readlink(tmp.path().join("another"), Vec::new()).unwrap();
    assert_eq!(target.to_string_lossy(), "link");
}

#[test]
fn test_readlinkat() {
    use rustix::fs::{cwd, openat, readlinkat, symlinkat, Mode, OFlags};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();

    let _ = openat(&dir, "foo", OFlags::CREATE | OFlags::WRONLY, Mode::RUSR).unwrap();
    symlinkat("foo", &dir, "link").unwrap();

    readlinkat(&dir, "absent", Vec::new()).unwrap_err();
    readlinkat(&dir, "foo", Vec::new()).unwrap_err();

    let target = readlinkat(&dir, "link", Vec::new()).unwrap();
    assert_eq!(target.to_string_lossy(), "foo");

    symlinkat("link", &dir, "another").unwrap();

    let target = readlinkat(&dir, "link", Vec::new()).unwrap();
    assert_eq!(target.to_string_lossy(), "foo");
    let target = readlinkat(&dir, "another", Vec::new()).unwrap();
    assert_eq!(target.to_string_lossy(), "link");
}
