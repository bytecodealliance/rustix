#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[test]
fn test_mkdirat() {
    use rustix::fs::{
        accessat, cwd, mkdirat, openat, statat, unlinkat, Access, AtFlags, FileType, Mode, OFlags,
    };

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();

    mkdirat(&dir, "foo", Mode::RWXU).unwrap();
    let stat = statat(&dir, "foo", AtFlags::empty()).unwrap();
    assert_eq!(FileType::from_raw_mode(stat.st_mode), FileType::Directory);
    accessat(&dir, "foo", Access::READ_OK, AtFlags::empty()).unwrap();
    accessat(&dir, "foo", Access::WRITE_OK, AtFlags::empty()).unwrap();
    accessat(&dir, "foo", Access::EXEC_OK, AtFlags::empty()).unwrap();
    accessat(&dir, "foo", Access::EXISTS, AtFlags::empty()).unwrap();
    unlinkat(&dir, "foo", AtFlags::REMOVEDIR).unwrap();
}

#[cfg(linux_kernel)]
#[test]
fn test_mkdirat_with_o_path() {
    use rustix::fs::{
        accessat, cwd, mkdirat, openat, statat, unlinkat, Access, AtFlags, FileType, Mode, OFlags,
    };

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(
        cwd(),
        tmp.path(),
        OFlags::RDONLY | OFlags::PATH,
        Mode::empty(),
    )
    .unwrap();

    mkdirat(&dir, "foo", Mode::RWXU).unwrap();
    let stat = statat(&dir, "foo", AtFlags::empty()).unwrap();
    assert_eq!(FileType::from_raw_mode(stat.st_mode), FileType::Directory);
    accessat(&dir, "foo", Access::EXISTS, AtFlags::empty()).unwrap();
    unlinkat(&dir, "foo", AtFlags::REMOVEDIR).unwrap();
}
