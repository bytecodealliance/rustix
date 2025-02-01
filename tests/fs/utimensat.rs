#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[test]
fn test_utimensat() {
    use rustix::fs::{openat, statat, utimensat, AtFlags, Mode, OFlags, Timespec, Timestamps, CWD};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(
        CWD,
        tmp.path(),
        OFlags::RDONLY | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .unwrap();

    let _ = openat(
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
    utimensat(&dir, "file", &times, AtFlags::empty()).unwrap();

    let after = statat(&dir, "file", AtFlags::empty()).unwrap();

    assert_eq!(times.last_modification.tv_sec as u64, after.st_mtime as u64);
    assert_eq!(
        times.last_modification.tv_nsec as u64,
        after.st_mtime_nsec as u64
    );
    assert!(times.last_access.tv_sec as u64 >= after.st_atime as u64);
    assert!(
        times.last_access.tv_sec as u64 > after.st_atime as u64
            || times.last_access.tv_nsec as u64 >= after.st_atime_nsec as u64
    );
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[test]
fn test_utimensat_noent() {
    use rustix::fs::{openat, utimensat, AtFlags, Mode, OFlags, Timespec, Timestamps, CWD};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(
        CWD,
        tmp.path(),
        OFlags::RDONLY | OFlags::CLOEXEC,
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
    assert_eq!(
        utimensat(&dir, "file", &times, AtFlags::empty()).unwrap_err(),
        rustix::io::Errno::NOENT
    );
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[test]
fn test_utimensat_notdir() {
    use rustix::fs::{openat, utimensat, AtFlags, Mode, OFlags, Timespec, Timestamps, CWD};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(
        CWD,
        tmp.path(),
        OFlags::RDONLY | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .unwrap();

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
    assert_eq!(
        utimensat(&file, "bar", &times, AtFlags::empty()).unwrap_err(),
        rustix::io::Errno::NOTDIR
    );
}
