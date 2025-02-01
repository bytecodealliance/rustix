#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[test]
fn test_futimens() {
    use rustix::fs::{fstat, futimens, openat, Mode, OFlags, Timespec, Timestamps, CWD};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();

    let file = openat(
        &dir,
        "file",
        OFlags::CREATE | OFlags::WRONLY | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .unwrap();

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
    futimens(&file, &times).unwrap();

    let after = fstat(&file).unwrap();

    assert_eq!(times.last_modification.tv_sec as u64, after.st_mtime as u64);
    assert_eq!(
        times.last_modification.tv_nsec as u64,
        after.st_mtime_nsec as u64
    );
}
