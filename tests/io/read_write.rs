#[cfg(feature = "fs")]
use std::io::{IoSlice, IoSliceMut};

#[cfg(feature = "fs")]
#[cfg(not(target_os = "solaris"))] // no preadv/pwritev
#[cfg(not(target_os = "haiku"))] // no preadv/pwritev
#[test]
fn test_readwrite_pv() {
    use rustix::fs::{cwd, openat, Mode, OFlags};
    use rustix::io::{preadv, pwritev};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let foo = openat(
        &dir,
        "foo",
        OFlags::RDWR | OFlags::CREATE | OFlags::TRUNC,
        Mode::RUSR | Mode::WUSR,
    )
    .unwrap();

    // For most targets, just call `pwritev`.
    #[cfg(not(apple))]
    {
        pwritev(&foo, &[IoSlice::new(b"hello")], 200).unwrap();
    }
    // macOS only has pwritev in newer versions; allow it to fail with `ENOSYS`.
    #[cfg(apple)]
    {
        match pwritev(&foo, &[IoSlice::new(b"hello")], 200) {
            Ok(_) => (),
            Err(rustix::io::Errno::NOSYS) => return,
            Err(err) => Err(err).unwrap(),
        }
    }
    pwritev(&foo, &[IoSlice::new(b"world")], 300).unwrap();
    let mut buf = [0_u8; 5];
    preadv(&foo, &mut [IoSliceMut::new(&mut buf)], 200).unwrap();
    assert_eq!(&buf, b"hello");
    preadv(&foo, &mut [IoSliceMut::new(&mut buf)], 300).unwrap();
    assert_eq!(&buf, b"world");
}

#[cfg(feature = "fs")]
#[test]
fn test_readwrite_p() {
    use rustix::fs::{cwd, openat, Mode, OFlags};
    use rustix::io::{pread, pwrite};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let foo = openat(
        &dir,
        "foo",
        OFlags::RDWR | OFlags::CREATE | OFlags::TRUNC,
        Mode::RUSR | Mode::WUSR,
    )
    .unwrap();

    pwrite(&foo, b"hello", 200).unwrap();
    pwrite(&foo, b"world", 300).unwrap();
    let mut buf = [0_u8; 5];
    pread(&foo, &mut buf, 200).unwrap();
    assert_eq!(&buf, b"hello");
    pread(&foo, &mut buf, 300).unwrap();
    assert_eq!(&buf, b"world");
}

#[cfg(feature = "fs")]
#[test]
fn test_readwrite_v() {
    use rustix::fs::{cwd, openat, seek, Mode, OFlags};
    use rustix::io::{readv, writev, SeekFrom};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let foo = openat(
        &dir,
        "foo",
        OFlags::RDWR | OFlags::CREATE | OFlags::TRUNC,
        Mode::RUSR | Mode::WUSR,
    )
    .unwrap();

    writev(&foo, &[IoSlice::new(b"hello")]).unwrap();
    writev(&foo, &[IoSlice::new(b"world")]).unwrap();
    seek(&foo, SeekFrom::Start(0)).unwrap();
    let mut buf = [0_u8; 5];
    readv(&foo, &mut [IoSliceMut::new(&mut buf)]).unwrap();
    assert_eq!(&buf, b"hello");
    readv(&foo, &mut [IoSliceMut::new(&mut buf)]).unwrap();
    assert_eq!(&buf, b"world");
}

#[cfg(feature = "fs")]
#[test]
fn test_readwrite() {
    use rustix::fs::{cwd, openat, seek, Mode, OFlags};
    use rustix::io::{read, write, SeekFrom};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let foo = openat(
        &dir,
        "foo",
        OFlags::RDWR | OFlags::CREATE | OFlags::TRUNC,
        Mode::RUSR | Mode::WUSR,
    )
    .unwrap();

    write(&foo, b"hello").unwrap();
    write(&foo, b"world").unwrap();
    seek(&foo, SeekFrom::Start(0)).unwrap();
    let mut buf = [0_u8; 5];
    read(&foo, &mut buf).unwrap();
    assert_eq!(&buf, b"hello");
    read(&foo, &mut buf).unwrap();
    assert_eq!(&buf, b"world");
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
#[test]
fn test_rwf_values() {
    // We use the kernel's values for these flags; check that libc doesn't
    // have different values.
    assert_eq!(
        rustix::io::ReadWriteFlags::APPEND.bits() as i32,
        libc::RWF_APPEND
    );
    assert_eq!(
        rustix::io::ReadWriteFlags::DSYNC.bits() as i32,
        libc::RWF_DSYNC
    );
    assert_eq!(
        rustix::io::ReadWriteFlags::HIPRI.bits() as i32,
        libc::RWF_HIPRI
    );
    assert_eq!(
        rustix::io::ReadWriteFlags::NOWAIT.bits() as i32,
        libc::RWF_NOWAIT
    );
    assert_eq!(
        rustix::io::ReadWriteFlags::SYNC.bits() as i32,
        libc::RWF_SYNC
    );
}

#[cfg(linux_kernel)]
#[cfg(feature = "fs")]
#[test]
fn test_pwritev2() {
    use rustix::fs::{cwd, openat, seek, Mode, OFlags};
    use rustix::io::{preadv2, pwritev2, writev, ReadWriteFlags, SeekFrom};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let foo = openat(
        &dir,
        "foo",
        OFlags::RDWR | OFlags::CREATE | OFlags::TRUNC,
        Mode::RUSR | Mode::WUSR,
    )
    .unwrap();

    writev(&foo, &[IoSlice::new(b"hello")]).unwrap();
    seek(&foo, SeekFrom::Start(0)).unwrap();

    // pwritev2 to append with a 0 offset: don't update the current position.
    match pwritev2(&foo, &[IoSlice::new(b"world")], 0, ReadWriteFlags::APPEND) {
        Ok(_) => {}
        // Skip the rest of the test if we don't have `pwritev2` and
        // `RWF_APPEND`.
        Err(rustix::io::Errno::NOSYS | rustix::io::Errno::NOTSUP) => return,
        Err(err) => Err(err).unwrap(),
    }
    assert_eq!(seek(&foo, SeekFrom::Current(0)).unwrap(), 0);

    // pwritev2 to append with a !0 offset: do update the current position.
    pwritev2(&foo, &[IoSlice::new(b"world")], !0, ReadWriteFlags::APPEND).unwrap();
    assert_eq!(seek(&foo, SeekFrom::Current(0)).unwrap(), 15);

    seek(&foo, SeekFrom::Start(0)).unwrap();
    let mut buf = [0_u8; 5];
    preadv2(
        &foo,
        &mut [IoSliceMut::new(&mut buf)],
        0,
        ReadWriteFlags::empty(),
    )
    .unwrap();
    assert_eq!(&buf, b"hello");
    preadv2(
        &foo,
        &mut [IoSliceMut::new(&mut buf)],
        5,
        ReadWriteFlags::empty(),
    )
    .unwrap();
    assert_eq!(&buf, b"world");
}
