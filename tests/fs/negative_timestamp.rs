#[cfg(not(target_os = "redox"))]
#[cfg(not(target_os = "cygwin"))]
#[test]
fn negative_file_timestamp() {
    use rustix::fs::{
        fstat, futimens, lstat, open, stat, statat, AtFlags, Mode, OFlags, Timespec, Timestamps,
        CWD,
    };

    let tmp = tempfile::tempdir().unwrap();

    let file = open(
        tmp.path().join("foo"),
        OFlags::CREATE | OFlags::WRONLY,
        Mode::RWXU,
    )
    .unwrap();

    let stamps = Timestamps {
        last_modification: Timespec {
            tv_sec: -20,
            tv_nsec: 12,
        },
        last_access: Timespec {
            tv_sec: -23,
            tv_nsec: 14,
        },
    };
    futimens(&file, &stamps).unwrap();

    let st = fstat(file).unwrap();
    assert_eq!(st.st_mtime, -20);
    assert_eq!(st.st_mtime_nsec, 12);
    assert_eq!(st.st_atime, -23);
    assert_eq!(st.st_atime_nsec, 14);

    let st = stat(tmp.path().join("foo")).unwrap();
    assert_eq!(st.st_mtime, -20);
    assert_eq!(st.st_mtime_nsec, 12);
    assert_eq!(st.st_atime, -23);
    assert_eq!(st.st_atime_nsec, 14);

    let st = lstat(tmp.path().join("foo")).unwrap();
    assert_eq!(st.st_mtime, -20);
    assert_eq!(st.st_mtime_nsec, 12);
    assert_eq!(st.st_atime, -23);
    assert_eq!(st.st_atime_nsec, 14);

    let st = statat(CWD, tmp.path().join("foo"), AtFlags::empty()).unwrap();
    assert_eq!(st.st_mtime, -20);
    assert_eq!(st.st_mtime_nsec, 12);
    assert_eq!(st.st_atime, -23);
    assert_eq!(st.st_atime_nsec, 14);
}
