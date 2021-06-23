#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[test]
fn test_mkdirat() {
    use posish::fs::{cwd, mkdirat, openat, statat, unlinkat, AtFlags, FileType, Mode, OFlags};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();

    mkdirat(&dir, "foo", Mode::IRWXU).unwrap();
    let stat = statat(&dir, "foo", AtFlags::empty()).unwrap();
    assert_eq!(FileType::from_raw_mode(stat.st_mode), FileType::Directory);
    unlinkat(&dir, "foo", AtFlags::REMOVEDIR).unwrap();
}
