/// Test seek positions related to file "holes".
#[cfg(any(apple, freebsdlike, linux_kernel, solarish))]
#[test]
fn test_seek_holes() {
    use rustix::fs::{fstat, openat, seek, Mode, OFlags, SeekFrom, CWD};
    use std::io::Write;

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let foo = openat(
        &dir,
        "foo",
        OFlags::RDWR | OFlags::CREATE | OFlags::TRUNC,
        Mode::RUSR | Mode::WUSR,
    )
    .unwrap();
    let mut foo = std::fs::File::from(foo);

    let stat = fstat(&foo).unwrap();
    let hole_size = stat.st_blksize as u64;

    #[cfg(any(solarish, freebsdlike, netbsdlike))]
    let hole_size = unsafe {
        use std::os::unix::io::AsRawFd;

        let r = libc::fpathconf(foo.as_raw_fd(), libc::_PC_MIN_HOLE_SIZE);

        if r < 0 {
            // Holes not supported.
            return;
        }

        // Holes are supported.
        core::cmp::max(hole_size, r as u64)
    };

    foo.write_all(b"prefix").unwrap();
    assert_eq!(
        seek(&foo, SeekFrom::Start(hole_size * 2)),
        Ok(hole_size * 2)
    );
    foo.write_all(b"suffix").unwrap();
    assert_eq!(seek(&foo, SeekFrom::Start(0)), Ok(0));
    assert_eq!(seek(&foo, SeekFrom::Current(0)), Ok(0));
    assert_eq!(seek(&foo, SeekFrom::Hole(0)), Ok(hole_size));
    assert_eq!(seek(&foo, SeekFrom::Hole(hole_size as i64)), Ok(hole_size));
    assert_eq!(
        seek(&foo, SeekFrom::Hole(hole_size as i64 * 2)),
        Ok(hole_size * 2 + 6)
    );
    assert_eq!(seek(&foo, SeekFrom::Data(0)), Ok(0));
    assert_eq!(
        seek(&foo, SeekFrom::Data(hole_size as i64)),
        Ok(hole_size * 2)
    );
    assert_eq!(
        seek(&foo, SeekFrom::Data(hole_size as i64 * 2)),
        Ok(hole_size * 2)
    );
}
