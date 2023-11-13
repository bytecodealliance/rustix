#[test]
fn test_symlink() {
    use rustix::fs::{lstat, open, readlink, symlink, Mode, OFlags};

    let tmp = tempfile::tempdir().unwrap();

    let _ = open(
        tmp.path().join("file"),
        OFlags::CREATE | OFlags::WRONLY,
        Mode::RUSR,
    )
    .unwrap();
    symlink("file", tmp.path().join("link")).unwrap();

    let target = readlink(tmp.path().join("link"), Vec::new()).unwrap();
    assert_eq!(target.to_string_lossy(), "file");

    assert_eq!(
        lstat(tmp.path().join("link")).unwrap().st_mode as u64 & libc::S_IFMT as u64,
        libc::S_IFLNK as u64
    );
}

#[cfg(not(target_os = "redox"))]
#[test]
fn test_symlinkat() {
    use rustix::fs::{openat, readlinkat, statat, symlinkat, AtFlags, Mode, OFlags, CWD};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();

    let _ = openat(&dir, "file", OFlags::CREATE | OFlags::WRONLY, Mode::RUSR).unwrap();
    symlinkat("file", &dir, "link").unwrap();

    let target = readlinkat(&dir, "link", Vec::new()).unwrap();
    assert_eq!(target.to_string_lossy(), "file");

    assert_eq!(
        statat(&dir, "link", AtFlags::SYMLINK_NOFOLLOW)
            .unwrap()
            .st_mode as u64
            & libc::S_IFMT as u64,
        libc::S_IFLNK as u64
    );
}
