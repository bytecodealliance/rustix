#[cfg(not(any(
    target_os = "wasi",
    target_os = "redox",
    target_os = "macos",
    target_os = "ios"
)))]
#[test]
fn test_mknodat() {
    use posish::fs::{cwd, mknodat, openat, statat, unlinkat, AtFlags, FileType, Mode, OFlags};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(&cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();

    // Create a regular file. Not supported on FreeBSD.
    #[cfg(not(target_os = "freebsd"))]
    {
        mknodat(&dir, "foo", Mode::IFREG, 0).unwrap();
        let stat = statat(&dir, "foo", AtFlags::empty()).unwrap();
        assert_eq!(FileType::from_raw_mode(stat.st_mode), FileType::RegularFile);
        unlinkat(&dir, "foo", AtFlags::empty()).unwrap();
    }

    mknodat(&dir, "foo", Mode::IFIFO, 0).unwrap();
    let stat = statat(&dir, "foo", AtFlags::empty()).unwrap();
    assert_eq!(FileType::from_raw_mode(stat.st_mode), FileType::Fifo);
    unlinkat(&dir, "foo", AtFlags::empty()).unwrap();
}
