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
