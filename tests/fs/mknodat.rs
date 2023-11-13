#[cfg(not(any(apple, target_os = "redox", target_os = "wasi")))]
#[test]
fn test_mknodat() {
    use rustix::fs::{
        accessat, mknodat, openat, statat, unlinkat, Access, AtFlags, FileType, Mode, OFlags, CWD,
    };

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();

    // Create a regular file. Not supported on FreeBSD, OpenBSD, illumos,
    // or NetBSD.
    #[cfg(not(any(solarish, netbsdlike, target_os = "freebsd")))]
    {
        mknodat(&dir, "file", FileType::RegularFile, Mode::empty(), 0).unwrap();
        let stat = statat(&dir, "file", AtFlags::empty()).unwrap();
        assert_eq!(FileType::from_raw_mode(stat.st_mode), FileType::RegularFile);
        unlinkat(&dir, "file", AtFlags::empty()).unwrap();
    }

    mknodat(&dir, "file", FileType::Fifo, Mode::empty(), 0).unwrap();
    let stat = statat(&dir, "file", AtFlags::empty()).unwrap();
    assert_eq!(FileType::from_raw_mode(stat.st_mode), FileType::Fifo);
    accessat(&dir, "file", Access::EXISTS, AtFlags::empty()).unwrap();
    unlinkat(&dir, "file", AtFlags::empty()).unwrap();
}
