#[cfg(not(any(target_os = "espidf", target_os = "horizon", target_os = "vita")))]
#[test]
fn test_mknodat() {
    use rustix::fs::{
        accessat, mknodat, openat, statat, unlinkat, Access, AtFlags, FileType, Mode, OFlags, CWD,
    };

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();

    // Create a regular file. Not supported on Apple, FreeBSD, OpenBSD,
    // illumos, or NetBSD.
    #[cfg(not(any(apple, solarish, netbsdlike, target_os = "freebsd")))]
    {
        mknodat(&dir, "file", FileType::RegularFile, Mode::empty(), 0).unwrap();
        let stat = statat(&dir, "file", AtFlags::empty()).unwrap();
        assert_eq!(FileType::from_raw_mode(stat.st_mode), FileType::RegularFile);
        unlinkat(&dir, "file", AtFlags::empty()).unwrap();
    }

    match mknodat(&dir, "file", FileType::Fifo, Mode::empty(), 0) {
        Ok(()) => (),
        #[cfg(apple)]
        // Apple platforms before macOS 13.0, iOS 16.0, tvOS 16.0, and
        // watchOS 9.0 lack `mknodat`.
        Err(rustix::io::Errno::NOSYS) => return,
        Err(err) => panic!("{:?}", err),
    }
    let stat = statat(&dir, "file", AtFlags::empty()).unwrap();
    assert_eq!(FileType::from_raw_mode(stat.st_mode), FileType::Fifo);
    accessat(&dir, "file", Access::EXISTS, AtFlags::empty()).unwrap();
    unlinkat(&dir, "file", AtFlags::empty()).unwrap();
}
