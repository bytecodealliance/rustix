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
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    {
        pwritev(&foo, &[IoSlice::new(b"hello")], 200).unwrap();
    }
    // macOS only has pwritev in newer versions; allow it to fail with `ENOSYS`.
    #[cfg(any(target_os = "ios", target_os = "macos"))]
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
    use rustix::io::{read, write};
    use std::io::SeekFrom;

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

#[cfg(any(target_os = "android", target_os = "linux"))]
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
        // Skip the rest of the test if we don't have `pwritev2` and `RWF_APPEND`.
        Err(rustix::io::Errno::NOSYS) | Err(rustix::io::Errno::NOTSUP) => return,
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

#[cfg(linux_kernel)]
#[cfg(all(feature = "net", feature = "pipe"))]
#[test]
fn test_preadv2_nowait() {
    use rustix::io::{preadv2, ReadWriteFlags};
    use rustix::net::{socketpair, AddressFamily, SocketFlags, SocketType};
    use rustix::pipe::pipe;

    let mut buf = [0_u8; 5];

    let (reader, _writer) = socketpair(
        AddressFamily::UNIX,
        SocketType::STREAM,
        SocketFlags::CLOEXEC,
        None,
    )
    .unwrap();
    match preadv2(
        &reader,
        &mut [IoSliceMut::new(&mut buf)],
        u64::MAX,
        ReadWriteFlags::NOWAIT,
    ) {
        Err(rustix::io::Errno::OPNOTSUPP) | Err(rustix::io::Errno::NOSYS) => {}
        Err(rustix::io::Errno::AGAIN) => {}
        Ok(_) => panic!("preadv2 unexpectedly succeeded"),
        Err(e) => panic!("preadv2 failed with an unexpected error: {:?}", e),
    }

    let (reader, _writer) = pipe().unwrap();
    match preadv2(
        &reader,
        &mut [IoSliceMut::new(&mut buf)],
        u64::MAX,
        ReadWriteFlags::NOWAIT,
    ) {
        Err(rustix::io::Errno::OPNOTSUPP) | Err(rustix::io::Errno::NOSYS) => {}
        Err(rustix::io::Errno::AGAIN) => {}
        Ok(_) => panic!("preadv2 unexpectedly succeeded"),
        Err(e) => panic!("preadv2 failed with an unexpected error: {:?}", e),
    }
}

#[cfg(feature = "net")]
#[cfg(not(target_os = "espidf"))] // no preadv/pwritev
#[cfg(not(target_os = "solaris"))] // no preadv/pwritev
#[cfg(not(target_os = "haiku"))] // no preadv/pwritev
#[test]
fn test_p_offsets() {
    use rustix::fs::{cwd, openat, Mode, OFlags};
    use rustix::io::{pread, preadv, pwrite, pwritev};
    #[cfg(linux_kernel)]
    use rustix::io::{preadv2, pwritev2, ReadWriteFlags};

    let mut buf = [0_u8; 5];

    let tmp = tempfile::tempdir().unwrap();
    let f = openat(
        cwd(),
        tmp.path().join("file"),
        OFlags::RDWR | OFlags::CREATE | OFlags::TRUNC,
        Mode::RUSR | Mode::WUSR,
    )
    .unwrap();

    // Test that offset 0 works.
    match pread(&f, &mut buf, 0_u64) {
        Err(rustix::io::Errno::OPNOTSUPP) | Err(rustix::io::Errno::NOSYS) => {}
        Ok(_) => {}
        Err(e) => panic!("pread failed with an unexpected error: {:?}", e),
    }
    match pwrite(&f, &buf, 0_u64) {
        Err(rustix::io::Errno::OPNOTSUPP) | Err(rustix::io::Errno::NOSYS) => {}
        Ok(_) => {}
        Err(e) => panic!("pwrite failed with an unexpected error: {:?}", e),
    }
    match preadv(&f, &mut [IoSliceMut::new(&mut buf)], 0_u64) {
        Err(rustix::io::Errno::OPNOTSUPP) | Err(rustix::io::Errno::NOSYS) => {}
        Ok(_) => {}
        Err(e) => panic!("preadv failed with an unexpected error: {:?}", e),
    }
    match pwritev(&f, &[IoSlice::new(&buf)], 0_u64) {
        Err(rustix::io::Errno::OPNOTSUPP) | Err(rustix::io::Errno::NOSYS) => {}
        Ok(_) => {}
        Err(e) => panic!("pwritev failed with an unexpected error: {:?}", e),
    }
    #[cfg(linux_kernel)]
    {
        match preadv2(
            &f,
            &mut [IoSliceMut::new(&mut buf)],
            0_u64,
            ReadWriteFlags::empty(),
        ) {
            Err(rustix::io::Errno::OPNOTSUPP) | Err(rustix::io::Errno::NOSYS) => {}
            Ok(_) => {}
            Err(e) => panic!("preadv2 failed with an unexpected error: {:?}", e),
        }
        match pwritev2(&f, &[IoSlice::new(&buf)], 0_u64, ReadWriteFlags::empty()) {
            Err(rustix::io::Errno::OPNOTSUPP) | Err(rustix::io::Errno::NOSYS) => {}
            Ok(_) => {}
            Err(e) => panic!("pwritev2 failed with an unexpected error: {:?}", e),
        }
    }

    // Test that negative offsets fail with `INVAL`.
    for invalid_offset in &[i32::MIN as u64, !1 as u64, i64::MIN as u64] {
        let invalid_offset = *invalid_offset;
        match pread(&f, &mut buf, invalid_offset) {
            Err(rustix::io::Errno::OPNOTSUPP) | Err(rustix::io::Errno::NOSYS) => {}
            Err(rustix::io::Errno::INVAL) => {}
            Ok(_) => panic!("pread unexpectedly succeeded"),
            Err(e) => panic!("pread failed with an unexpected error: {:?}", e),
        }
        match pwrite(&f, &buf, invalid_offset) {
            Err(rustix::io::Errno::OPNOTSUPP) | Err(rustix::io::Errno::NOSYS) => {}
            Err(rustix::io::Errno::INVAL) => {}
            Ok(_) => panic!("pwrite unexpectedly succeeded"),
            Err(e) => panic!("pwrite failed with an unexpected error: {:?}", e),
        }
        match preadv(&f, &mut [IoSliceMut::new(&mut buf)], invalid_offset) {
            Err(rustix::io::Errno::OPNOTSUPP) | Err(rustix::io::Errno::NOSYS) => {}
            Err(rustix::io::Errno::INVAL) => {}
            Ok(_) => panic!("preadv unexpectedly succeeded"),
            Err(e) => panic!("preadv failed with an unexpected error: {:?}", e),
        }
        match pwritev(&f, &[IoSlice::new(&buf)], invalid_offset) {
            Err(rustix::io::Errno::OPNOTSUPP) | Err(rustix::io::Errno::NOSYS) => {}
            Err(rustix::io::Errno::INVAL) => {}
            Ok(_) => panic!("pwritev unexpectedly succeeded"),
            Err(e) => panic!("pwritev failed with an unexpected error: {:?}", e),
        }
        #[cfg(linux_kernel)]
        {
            match preadv2(
                &f,
                &mut [IoSliceMut::new(&mut buf)],
                invalid_offset,
                ReadWriteFlags::empty(),
            ) {
                Err(rustix::io::Errno::OPNOTSUPP) | Err(rustix::io::Errno::NOSYS) => {}
                Err(rustix::io::Errno::INVAL) => {}
                Ok(_) => panic!("preadv2 unexpectedly succeeded"),
                Err(e) => panic!("preadv2 failed with an unexpected error: {:?}", e),
            }
            match pwritev2(
                &f,
                &[IoSlice::new(&buf)],
                invalid_offset,
                ReadWriteFlags::empty(),
            ) {
                Err(rustix::io::Errno::OPNOTSUPP) | Err(rustix::io::Errno::NOSYS) => {}
                Err(rustix::io::Errno::INVAL) => {}
                Ok(_) => panic!("pwritev2 unexpectedly succeeded"),
                Err(e) => panic!("pwritev2 failed with an unexpected error: {:?}", e),
            }
        }
    }
}
