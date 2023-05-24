use rustix::fs::Stat;

fn same(a: &Stat, b: &Stat) -> bool {
    a.st_ino == b.st_ino && a.st_dev == b.st_dev
}

#[test]
fn test_rename() {
    use rustix::fs::{access, open, rename, stat, Access, Mode, OFlags};

    let tmp = tempfile::tempdir().unwrap();

    let _ = open(
        tmp.path().join("foo"),
        OFlags::CREATE | OFlags::WRONLY,
        Mode::empty(),
    )
    .unwrap();
    let before = stat(tmp.path().join("foo")).unwrap();

    access(tmp.path().join("foo"), Access::EXISTS).unwrap();
    access(tmp.path().join("bar"), Access::EXISTS).unwrap_err();

    rename(tmp.path().join("foo"), tmp.path().join("bar")).unwrap();
    let renamed = stat(tmp.path().join("bar")).unwrap();
    assert!(same(&before, &renamed));

    access(tmp.path().join("foo"), Access::EXISTS).unwrap_err();
    access(tmp.path().join("bar"), Access::EXISTS).unwrap();
}

#[cfg(linux_kernel)]
#[test]
fn test_renameat() {
    use rustix::fs::{accessat, openat, renameat, statat, Access, AtFlags, Mode, OFlags, CWD};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(
        CWD,
        tmp.path(),
        OFlags::RDONLY | OFlags::PATH,
        Mode::empty(),
    )
    .unwrap();

    let _ = openat(&dir, "foo", OFlags::CREATE | OFlags::WRONLY, Mode::empty()).unwrap();
    let before = statat(&dir, "foo", AtFlags::empty()).unwrap();

    accessat(&dir, "foo", Access::EXISTS, AtFlags::empty()).unwrap();
    accessat(&dir, "bar", Access::EXISTS, AtFlags::empty()).unwrap_err();

    renameat(&dir, "foo", &dir, "bar").unwrap();
    let renamed = statat(&dir, "bar", AtFlags::empty()).unwrap();
    assert!(same(&before, &renamed));

    accessat(&dir, "foo", Access::EXISTS, AtFlags::empty()).unwrap_err();
    accessat(&dir, "bar", Access::EXISTS, AtFlags::empty()).unwrap();
}

/// Like `test_renameat` but the file already exists, so `renameat`
/// overwrites it.
#[cfg(linux_kernel)]
#[test]
fn test_renameat_overwrite() {
    use rustix::fs::{openat, renameat, statat, AtFlags, Mode, OFlags, CWD};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(
        CWD,
        tmp.path(),
        OFlags::RDONLY | OFlags::PATH,
        Mode::empty(),
    )
    .unwrap();

    let _ = openat(&dir, "foo", OFlags::CREATE | OFlags::WRONLY, Mode::empty()).unwrap();
    let _ = openat(&dir, "bar", OFlags::CREATE | OFlags::WRONLY, Mode::empty()).unwrap();
    let before = statat(&dir, "foo", AtFlags::empty()).unwrap();
    renameat(&dir, "foo", &dir, "bar").unwrap();
    let renamed = statat(&dir, "bar", AtFlags::empty()).unwrap();
    assert!(same(&before, &renamed));
}

#[cfg(linux_kernel)]
#[test]
fn test_renameat_with() {
    use rustix::fs::{openat, renameat_with, statat, AtFlags, Mode, OFlags, RenameFlags, CWD};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(
        CWD,
        tmp.path(),
        OFlags::RDONLY | OFlags::PATH,
        Mode::empty(),
    )
    .unwrap();

    let _ = openat(&dir, "foo", OFlags::CREATE | OFlags::WRONLY, Mode::empty()).unwrap();
    let before = statat(&dir, "foo", AtFlags::empty()).unwrap();

    match renameat_with(&dir, "foo", &dir, "red", RenameFlags::empty()) {
        Ok(()) => (),
        Err(err) if err == rustix::io::Errno::NOSYS => return,
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

    #[cfg(all(target_os = "linux", target_env = "gnu"))]
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
