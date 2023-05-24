#[test]
fn test_symlink() {
    use rustix::fs::{lstat, open, readlink, symlink, Mode, OFlags};

    let tmp = tempfile::tempdir().unwrap();

    let _ = open(
        tmp.path().join("foo"),
        OFlags::CREATE | OFlags::WRONLY,
        Mode::RUSR,
    )
    .unwrap();
    symlink("foo", tmp.path().join("link")).unwrap();

    let target = readlink(tmp.path().join("link"), Vec::new()).unwrap();
    assert_eq!(target.to_string_lossy(), "foo");

    assert_eq!(
        lstat(tmp.path().join("link")).unwrap().st_mode as u64 & libc::S_IFMT as u64,
        libc::S_IFLNK as u64
    );
}

#[test]
fn test_symlinkat() {
    use rustix::fs::{cwd, openat, readlinkat, statat, symlinkat, AtFlags, Mode, OFlags};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();

    let _ = openat(&dir, "foo", OFlags::CREATE | OFlags::WRONLY, Mode::RUSR).unwrap();
    symlinkat("foo", &dir, "link").unwrap();

    let target = readlinkat(&dir, "link", Vec::new()).unwrap();
    assert_eq!(target.to_string_lossy(), "foo");

    assert_eq!(
        statat(&dir, "link", AtFlags::SYMLINK_NOFOLLOW)
            .unwrap()
            .st_mode as u64
            & libc::S_IFMT as u64,
        libc::S_IFLNK as u64
    );
}
