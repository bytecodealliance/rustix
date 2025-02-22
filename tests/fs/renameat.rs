use rustix::fs::{OFlags, Stat};

fn same(a: &Stat, b: &Stat) -> bool {
    a.st_ino == b.st_ino && a.st_dev == b.st_dev
}

#[cfg(linux_kernel)]
const DIR_OPEN_FLAGS: OFlags = OFlags::RDONLY.union(OFlags::PATH);
#[cfg(apple)]
const DIR_OPEN_FLAGS: OFlags = OFlags::RDONLY;

#[test]
fn test_rename() {
    use rustix::fs::{access, open, rename, stat, Access, Mode};

    let tmp = tempfile::tempdir().unwrap();

    let _ = open(
        tmp.path().join("file"),
        OFlags::CREATE | OFlags::WRONLY,
        Mode::empty(),
    )
    .unwrap();
    let before = stat(tmp.path().join("file")).unwrap();

    access(tmp.path().join("file"), Access::EXISTS).unwrap();
    access(tmp.path().join("bar"), Access::EXISTS).unwrap_err();

    rename(tmp.path().join("file"), tmp.path().join("bar")).unwrap();
    let renamed = stat(tmp.path().join("bar")).unwrap();
    assert!(same(&before, &renamed));

    access(tmp.path().join("file"), Access::EXISTS).unwrap_err();
    access(tmp.path().join("bar"), Access::EXISTS).unwrap();
}

#[cfg(any(linux_kernel, apple))]
#[test]
fn test_renameat() {
    use rustix::fs::{accessat, openat, renameat, statat, Access, AtFlags, Mode, CWD};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), DIR_OPEN_FLAGS, Mode::empty()).unwrap();

    let _ = openat(&dir, "file", OFlags::CREATE | OFlags::WRONLY, Mode::empty()).unwrap();
    let before = statat(&dir, "file", AtFlags::empty()).unwrap();

    accessat(&dir, "file", Access::EXISTS, AtFlags::empty()).unwrap();
    accessat(&dir, "bar", Access::EXISTS, AtFlags::empty()).unwrap_err();

    renameat(&dir, "file", &dir, "bar").unwrap();
    let renamed = statat(&dir, "bar", AtFlags::empty()).unwrap();
    assert!(same(&before, &renamed));

    accessat(&dir, "file", Access::EXISTS, AtFlags::empty()).unwrap_err();
    accessat(&dir, "bar", Access::EXISTS, AtFlags::empty()).unwrap();
}

/// Like `test_renameat` but the file already exists, so `renameat`
/// overwrites it.
#[cfg(any(linux_kernel, apple))]
#[test]
fn test_renameat_overwrite() {
    use rustix::fs::{openat, renameat, statat, AtFlags, Mode, CWD};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), DIR_OPEN_FLAGS, Mode::empty()).unwrap();

    let _ = openat(&dir, "file", OFlags::CREATE | OFlags::WRONLY, Mode::empty()).unwrap();
    let _ = openat(&dir, "bar", OFlags::CREATE | OFlags::WRONLY, Mode::empty()).unwrap();
    let before = statat(&dir, "file", AtFlags::empty()).unwrap();
    renameat(&dir, "file", &dir, "bar").unwrap();
    let renamed = statat(&dir, "bar", AtFlags::empty()).unwrap();
    assert!(same(&before, &renamed));
}

#[cfg(any(linux_kernel, apple))]
#[test]
fn test_renameat_with() {
    use rustix::fs::{openat, renameat_with, statat, AtFlags, Mode, RenameFlags, CWD};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), DIR_OPEN_FLAGS, Mode::empty()).unwrap();

    let _ = openat(&dir, "file", OFlags::CREATE | OFlags::WRONLY, Mode::empty()).unwrap();
    let before = statat(&dir, "file", AtFlags::empty()).unwrap();

    match renameat_with(&dir, "file", &dir, "red", RenameFlags::empty()) {
        Ok(()) => (),
        Err(rustix::io::Errno::NOSYS) => return,
        Err(err) => unreachable!("unexpected error from renameat_with: {:?}", err),
    }

    let renamed = statat(&dir, "red", AtFlags::empty()).unwrap();
    assert!(same(&before, &renamed));

    let _ = openat(
        &dir,
        "green",
        OFlags::CREATE | OFlags::WRONLY,
        Mode::empty(),
    )
    .unwrap();

    #[cfg(any(apple, all(target_os = "linux", target_env = "gnu")))]
    {
        let green = statat(&dir, "green", AtFlags::empty()).unwrap();

        renameat_with(&dir, "red", &dir, "green", RenameFlags::NOREPLACE).unwrap_err();
        let renamed = statat(&dir, "red", AtFlags::empty()).unwrap();
        assert!(same(&before, &renamed));
        let orig = statat(&dir, "green", AtFlags::empty()).unwrap();
        assert!(same(&green, &orig));

        renameat_with(&dir, "red", &dir, "green", RenameFlags::EXCHANGE).unwrap();
        let renamed = statat(&dir, "red", AtFlags::empty()).unwrap();
        assert!(same(&green, &renamed));
        let orig = statat(&dir, "green", AtFlags::empty()).unwrap();
        assert!(same(&before, &orig));
    }
}
