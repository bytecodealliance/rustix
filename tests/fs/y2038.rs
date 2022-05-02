/// Test that we can set a file timestamp to a date past the year 2038 with
/// `utimensat` and read it back again.
///
/// See tests/time/y2038.rs for more information about y2038 testing.
#[test]
#[cfg(not(all(target_env = "musl", target_pointer_width = "32")))]
#[cfg(not(all(target_os = "android", target_pointer_width = "32")))]
#[cfg(not(all(target_os = "emscripten", target_pointer_width = "32")))]
fn test_y2038_with_utimensat() {
    use rustix::fs::{cwd, openat, statat, utimensat, AtFlags, Mode, OFlags, Timespec, Timestamps};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(&cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();

    let m_sec = 1_u64 << 32;
    let m_nsec = 17_u32;
    let a_sec = m_sec + 1;
    let a_nsec = m_nsec + 1;

    let timestamps = Timestamps {
        last_modification: Timespec {
            tv_sec: m_sec as _,
            tv_nsec: m_nsec as _,
        },
        last_access: Timespec {
            tv_sec: a_sec as _,
            tv_nsec: a_nsec as _,
        },
    };
    let _ = openat(&dir, "foo", OFlags::CREATE | OFlags::WRONLY, Mode::empty()).unwrap();
    let _ = utimensat(&dir, "foo", &timestamps, AtFlags::empty()).unwrap();
    let stat = statat(&dir, "foo", AtFlags::empty()).unwrap();

    assert_eq!(stat.st_mtime.try_into().unwrap() as u64, m_sec);
    assert_eq!(stat.st_mtime_nsec as u32, m_nsec);
    assert!(stat.st_atime.try_into().unwrap() as u64 >= a_sec);
    assert!(stat.st_atime_nsec as u32 >= a_nsec);
}

/// Test that we can set a file timestamp to a date past the year 2038 with
/// `futimens` and read it back again.
///
/// See tests/time/y2038.rs for more information about y2038 testing.
#[test]
#[cfg(not(all(target_env = "musl", target_pointer_width = "32")))]
#[cfg(not(all(target_os = "android", target_pointer_width = "32")))]
#[cfg(not(all(target_os = "emscripten", target_pointer_width = "32")))]
fn test_y2038_with_futimens() {
    use rustix::fs::{cwd, futimens, openat, statat, AtFlags, Mode, OFlags, Timespec, Timestamps};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(&cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();

    let m_sec = 1_u64 << 32;
    let m_nsec = 17_u32;
    let a_sec = m_sec + 1;
    let a_nsec = m_nsec + 1;

    let timestamps = Timestamps {
        last_modification: Timespec {
            tv_sec: m_sec as _,
            tv_nsec: m_nsec as _,
        },
        last_access: Timespec {
            tv_sec: a_sec as _,
            tv_nsec: a_nsec as _,
        },
    };
    let file = openat(&dir, "foo", OFlags::CREATE | OFlags::WRONLY, Mode::empty()).unwrap();
    let _ = futimens(&file, &timestamps).unwrap();
    let stat = statat(&dir, "foo", AtFlags::empty()).unwrap();

    assert_eq!(stat.st_mtime.try_into().unwrap(), m_sec);
    assert_eq!(stat.st_mtime_nsec as u32, m_nsec);
    assert!(stat.st_atime.try_into().unwrap() >= a_sec);
    assert!(stat.st_atime_nsec as u32 >= a_nsec);
}
