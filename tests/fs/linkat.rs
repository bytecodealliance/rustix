#[test]
fn test_link() {
    use rustix::fs::{link, open, readlink, stat, Mode, OFlags};

    let tmp = tempfile::tempdir().unwrap();

    let _ = open(
        tmp.path().join("file"),
        OFlags::CREATE | OFlags::WRONLY,
        Mode::RUSR,
    )
    .unwrap();

    link(tmp.path().join("file"), tmp.path().join("link")).unwrap();

    readlink(tmp.path().join("file"), Vec::new()).unwrap_err();
    readlink(tmp.path().join("link"), Vec::new()).unwrap_err();

    assert_eq!(
        stat(tmp.path().join("file")).unwrap().st_ino,
        stat(tmp.path().join("link")).unwrap().st_ino
    );

    link(tmp.path().join("link"), tmp.path().join("another")).unwrap();

    assert_eq!(
        stat(tmp.path().join("file")).unwrap().st_ino,
        stat(tmp.path().join("another")).unwrap().st_ino
    );
}

#[cfg(not(target_os = "redox"))]
#[test]
fn test_linkat() {
    use rustix::fs::{linkat, openat, readlinkat, statat, AtFlags, Mode, OFlags, CWD};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();

    let _ = openat(&dir, "file", OFlags::CREATE | OFlags::WRONLY, Mode::RUSR).unwrap();

    linkat(&dir, "file", &dir, "link", AtFlags::empty()).unwrap();

    readlinkat(&dir, "file", Vec::new()).unwrap_err();
    readlinkat(&dir, "link", Vec::new()).unwrap_err();

    assert_eq!(
        statat(&dir, "file", AtFlags::empty()).unwrap().st_ino,
        statat(&dir, "link", AtFlags::empty()).unwrap().st_ino
    );

    linkat(&dir, "link", &dir, "another", AtFlags::empty()).unwrap();

    assert_eq!(
        statat(&dir, "file", AtFlags::empty()).unwrap().st_ino,
        statat(&dir, "another", AtFlags::empty()).unwrap().st_ino
    );
}
