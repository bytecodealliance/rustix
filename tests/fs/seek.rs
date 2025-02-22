/// Test seek positions related to file “holes”.
#[cfg(any(apple, freebsdlike, linux_kernel, solarish))]
#[test]
fn test_seek_holes() {
    use rustix::fs::{fstat, openat, seek, Mode, OFlags, SeekFrom, CWD};
    use std::io::Write as _;

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let file = openat(
        &dir,
        "file",
        OFlags::RDWR | OFlags::CREATE | OFlags::TRUNC,
        Mode::RUSR | Mode::WUSR,
    )
    .unwrap();
    let mut file = std::fs::File::from(file);

    let stat = fstat(&file).unwrap();
    let hole_size = stat.st_blksize as u64;

    #[cfg(any(solarish, freebsdlike, netbsdlike))]
    let hole_size = unsafe {
        use std::os::unix::io::AsRawFd;

        let r = libc::fpathconf(file.as_raw_fd(), libc::_PC_MIN_HOLE_SIZE);

        if r < 0 {
            // Holes not supported.
            return;
        }

        // Holes are supported.
        core::cmp::max(hole_size, r as u64)
    };

    file.write_all(b"prefix").unwrap();
    assert_eq!(
        seek(&file, SeekFrom::Start(hole_size * 2)),
        Ok(hole_size * 2)
    );
    file.write_all(b"suffix").unwrap();
    assert_eq!(seek(&file, SeekFrom::Start(0)), Ok(0));
    assert_eq!(seek(&file, SeekFrom::Current(0)), Ok(0));
    assert_eq!(seek(&file, SeekFrom::Hole(0)), Ok(hole_size));
    assert_eq!(seek(&file, SeekFrom::Hole(hole_size)), Ok(hole_size));
    assert_eq!(
        seek(&file, SeekFrom::Hole(hole_size * 2)),
        Ok(hole_size * 2 + 6)
    );
    assert_eq!(seek(&file, SeekFrom::Data(0)), Ok(0));
    assert_eq!(seek(&file, SeekFrom::Data(hole_size)), Ok(hole_size * 2));
    assert_eq!(
        seek(&file, SeekFrom::Data(hole_size * 2)),
        Ok(hole_size * 2)
    );
}

#[test]
fn test_seek_offsets() {
    use rustix::fs::{open, seek, Mode, OFlags, SeekFrom};

    let f = open("Cargo.toml", OFlags::RDONLY, Mode::empty()).unwrap();

    match seek(&f, SeekFrom::Start(0)) {
        Ok(_) => {}
        Err(e) => panic!("seek failed with an unexpected error: {:?}", e),
    }
    for invalid_offset in [i32::MIN as u64, !1, i64::MIN as u64] {
        match seek(&f, SeekFrom::Start(invalid_offset)) {
            Err(rustix::io::Errno::INVAL) => {}
            Ok(_) => panic!("seek unexpectedly succeeded"),
            Err(e) => panic!("seek failed with an unexpected error: {:?}", e),
        }
    }
}
