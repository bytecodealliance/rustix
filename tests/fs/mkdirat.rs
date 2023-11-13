#[test]
fn test_mkdir() {
    use rustix::fs::{access, mkdir, rmdir, stat, unlink, Access, FileType, Mode};

    let tmp = tempfile::tempdir().unwrap();

    mkdir(tmp.path().join("file"), Mode::RWXU).unwrap();
    let stat = stat(tmp.path().join("file")).unwrap();
    assert_eq!(FileType::from_raw_mode(stat.st_mode), FileType::Directory);
    access(tmp.path().join("file"), Access::READ_OK).unwrap();
    access(tmp.path().join("file"), Access::WRITE_OK).unwrap();
    access(tmp.path().join("file"), Access::EXEC_OK).unwrap();
    access(tmp.path().join("file"), Access::EXISTS).unwrap();
    unlink(tmp.path().join("file")).unwrap_err();
    rmdir(tmp.path().join("file")).unwrap();
}

#[cfg(not(target_os = "redox"))]
#[test]
fn test_mkdirat() {
    use rustix::fs::{
        accessat, mkdirat, openat, statat, unlinkat, Access, AtFlags, FileType, Mode, OFlags, CWD,
    };

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();

    mkdirat(&dir, "file", Mode::RWXU).unwrap();
    let stat = statat(&dir, "file", AtFlags::empty()).unwrap();
    assert_eq!(FileType::from_raw_mode(stat.st_mode), FileType::Directory);
    accessat(&dir, "file", Access::READ_OK, AtFlags::empty()).unwrap();
    accessat(&dir, "file", Access::WRITE_OK, AtFlags::empty()).unwrap();
    accessat(&dir, "file", Access::EXEC_OK, AtFlags::empty()).unwrap();
    accessat(&dir, "file", Access::EXISTS, AtFlags::empty()).unwrap();
    unlinkat(&dir, "file", AtFlags::empty()).unwrap_err();
    unlinkat(&dir, "file", AtFlags::REMOVEDIR).unwrap();
}

#[cfg(linux_kernel)]
#[test]
fn test_mkdirat_with_o_path() {
    use rustix::fs::{
        accessat, mkdirat, openat, statat, unlinkat, Access, AtFlags, FileType, Mode, OFlags, CWD,
    };

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(
        CWD,
        tmp.path(),
        OFlags::RDONLY | OFlags::PATH,
        Mode::empty(),
    )
    .unwrap();

    mkdirat(&dir, "file", Mode::RWXU).unwrap();
    let stat = statat(&dir, "file", AtFlags::empty()).unwrap();
    assert_eq!(FileType::from_raw_mode(stat.st_mode), FileType::Directory);
    accessat(&dir, "file", Access::EXISTS, AtFlags::empty()).unwrap();
    unlinkat(&dir, "file", AtFlags::empty()).unwrap_err();
    unlinkat(&dir, "file", AtFlags::REMOVEDIR).unwrap();
}
