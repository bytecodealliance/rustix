#[cfg(not(target_os = "wasi"))]
#[test]
fn test_chmodat() {
    use rustix::fs::{chmodat, cwd, openat, statat, symlinkat, AtFlags, Mode, OFlags};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(cwd(), tmp.path(), OFlags::RDONLY, Mode::RWXU).unwrap();

    let _ = openat(&dir, "foo", OFlags::CREATE | OFlags::WRONLY, Mode::RWXU).unwrap();
    symlinkat("foo", &dir, "link").unwrap();

    match chmodat(&dir, "link", Mode::empty(), AtFlags::SYMLINK_NOFOLLOW) {
        Ok(()) => (),
        Err(rustix::io::Errno::OPNOTSUPP) => return,
        Err(e) => Err(e).unwrap(),
    }

    let before = statat(&dir, "foo", AtFlags::empty()).unwrap();
    assert_ne!(before.st_mode as u64 & libc::S_IRWXU as u64, 0);

    chmodat(&dir, "foo", Mode::empty(), AtFlags::empty()).unwrap();

    let after = statat(&dir, "foo", AtFlags::empty()).unwrap();
    assert_eq!(after.st_mode as u64 & libc::S_IRWXU as u64, 0);

    chmodat(&dir, "foo", Mode::RWXU, AtFlags::empty()).unwrap();

    let reverted = statat(&dir, "foo", AtFlags::empty()).unwrap();
    assert_ne!(reverted.st_mode as u64 & libc::S_IRWXU as u64, 0);
}
