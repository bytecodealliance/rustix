#[test]
fn test_linkat() {
    use rustix::fs::{cwd, linkat, openat, readlinkat, statat, AtFlags, Mode, OFlags};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();

    let _ = openat(&dir, "foo", OFlags::CREATE | OFlags::WRONLY, Mode::RUSR).unwrap();

    linkat(&dir, "foo", &dir, "link", AtFlags::empty()).unwrap();

    readlinkat(&dir, "foo", Vec::new()).unwrap_err();
    readlinkat(&dir, "link", Vec::new()).unwrap_err();

    assert_eq!(
        statat(&dir, "foo", AtFlags::empty()).unwrap().st_ino,
        statat(&dir, "link", AtFlags::empty()).unwrap().st_ino
    );

    linkat(&dir, "link", &dir, "another", AtFlags::empty()).unwrap();

    assert_eq!(
        statat(&dir, "foo", AtFlags::empty()).unwrap().st_ino,
        statat(&dir, "another", AtFlags::empty()).unwrap().st_ino
    );
}
