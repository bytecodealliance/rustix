#[cfg(not(any(target_os = "cygwin", target_os = "wasi")))]
#[test]
fn test_chmod() {
    use rustix::fs::{chmod, open, stat, symlink, Mode, OFlags};

    let tmp = tempfile::tempdir().unwrap();

    let _ = open(
        tmp.path().join("file"),
        OFlags::CREATE | OFlags::WRONLY,
        Mode::RWXU,
    )
    .unwrap();
    symlink(tmp.path().join("file"), tmp.path().join("link")).unwrap();

    let before = stat(tmp.path().join("file")).unwrap();
    assert_ne!(before.st_mode as u64 & libc::S_IRWXU as u64, 0);

    chmod(tmp.path().join("file"), Mode::empty()).unwrap();

    let after = stat(tmp.path().join("file")).unwrap();
    assert_eq!(after.st_mode as u64 & libc::S_IRWXU as u64, 0);

    chmod(tmp.path().join("file"), Mode::RWXU).unwrap();

    let reverted = stat(tmp.path().join("file")).unwrap();
    assert_ne!(reverted.st_mode as u64 & libc::S_IRWXU as u64, 0);
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[test]
fn test_chmodat() {
    use rustix::fs::{chmodat, openat, statat, symlinkat, AtFlags, Mode, OFlags, CWD};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::RWXU).unwrap();

    let _ = openat(&dir, "file", OFlags::CREATE | OFlags::WRONLY, Mode::RWXU).unwrap();
    symlinkat("file", &dir, "link").unwrap();

    match chmodat(&dir, "link", Mode::empty(), AtFlags::SYMLINK_NOFOLLOW) {
        Ok(()) => (),
        Err(rustix::io::Errno::OPNOTSUPP) => return,
        Err(err) => panic!("{:?}", err),
    }

    let before = statat(&dir, "file", AtFlags::empty()).unwrap();
    assert_ne!(before.st_mode as u64 & libc::S_IRWXU as u64, 0);

    chmodat(&dir, "file", Mode::empty(), AtFlags::empty()).unwrap();

    let after = statat(&dir, "file", AtFlags::empty()).unwrap();
    assert_eq!(after.st_mode as u64 & libc::S_IRWXU as u64, 0);

    chmodat(&dir, "file", Mode::RWXU, AtFlags::empty()).unwrap();

    let reverted = statat(&dir, "file", AtFlags::empty()).unwrap();
    assert_ne!(reverted.st_mode as u64 & libc::S_IRWXU as u64, 0);
}
