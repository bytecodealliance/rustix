#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[test]
fn test_futimens() {
    use rustix::fs::{cwd, fstat, futimens, openat, Mode, OFlags, Timestamps};
    use rustix::time::Timespec;
    use rustix::fd::{AsRawFd, FromRawFd};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(&cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();

    let foo = openat(
        &dir,
        "foo",
        OFlags::CREATE | OFlags::WRONLY | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .unwrap();

    let before = fstat(&foo).unwrap();
    dbg!(&before);

    let bf = unsafe { std::fs::File::from_raw_fd(foo.as_raw_fd()) };
    let bm = bf.metadata().unwrap();
    dbg!(&bm.modified());
    std::mem::forget(bf);

    let times = Timestamps {
        last_access: Timespec {
            tv_sec: 44000,
            tv_nsec: 45000,
        },
        last_modification: Timespec {
            tv_sec: 46000,
            tv_nsec: 47000,
        },
    };
    futimens(&foo, &times).unwrap();

    let after = fstat(&foo).unwrap();

    dbg!(&after);

    let af = unsafe { std::fs::File::from_raw_fd(foo.as_raw_fd()) };
    let am = af.metadata().unwrap();
    dbg!(&am.modified());
    std::mem::forget(af);

    dbg!(&times);

    assert_eq!(times.last_modification.tv_sec as u64, after.st_mtime as u64);
    assert_eq!(
        times.last_modification.tv_nsec as u64,
        after.st_mtime_nsec as u64
    );
}
