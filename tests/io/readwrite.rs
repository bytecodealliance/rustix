use std::io::{IoSlice, IoSliceMut};

#[test]
fn test_readwrite_pv() {
    use posish::{
        fs::{cwd, openat, Mode, OFlags},
        io::{preadv, pwritev},
    };

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(&cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let foo = openat(
        &dir,
        "foo",
        OFlags::RDWR | OFlags::CREATE | OFlags::TRUNC,
        Mode::IRUSR | Mode::IWUSR,
    )
    .unwrap();

    // For most targets, just call pwritev.
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    {
        pwritev(&foo, &[IoSlice::new(b"hello")], 200).unwrap();
    }
    // macOS only has pwritev in newer versions; allow it to fail with `ENOSYS`.
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    {
        match pwritev(&foo, &[IoSlice::new(b"hello")], 200) {
            Ok(_) => (),
            Err(posish::io::Error::NOSYS) => return,
            Err(err) => Err(err).unwrap(),
        }
    }
    pwritev(&foo, &[IoSlice::new(b"world")], 300).unwrap();
    let mut buf = [0_u8; 5];
    preadv(&foo, &[IoSliceMut::new(&mut buf)], 200).unwrap();
    assert_eq!(&buf, b"hello");
    preadv(&foo, &[IoSliceMut::new(&mut buf)], 300).unwrap();
    assert_eq!(&buf, b"world");
}

#[test]
fn test_readwrite_p() {
    use posish::{
        fs::{cwd, openat, Mode, OFlags},
        io::{pread, pwrite},
    };

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(&cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let foo = openat(
        &dir,
        "foo",
        OFlags::RDWR | OFlags::CREATE | OFlags::TRUNC,
        Mode::IRUSR | Mode::IWUSR,
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

#[test]
fn test_readwrite_v() {
    use posish::{
        fs::{cwd, openat, seek, Mode, OFlags},
        io::{readv, writev},
    };
    use std::io::SeekFrom;

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(&cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let foo = openat(
        &dir,
        "foo",
        OFlags::RDWR | OFlags::CREATE | OFlags::TRUNC,
        Mode::IRUSR | Mode::IWUSR,
    )
    .unwrap();

    writev(&foo, &[IoSlice::new(b"hello")]).unwrap();
    writev(&foo, &[IoSlice::new(b"world")]).unwrap();
    seek(&foo, SeekFrom::Start(0)).unwrap();
    let mut buf = [0_u8; 5];
    readv(&foo, &[IoSliceMut::new(&mut buf)]).unwrap();
    assert_eq!(&buf, b"hello");
    readv(&foo, &[IoSliceMut::new(&mut buf)]).unwrap();
    assert_eq!(&buf, b"world");
}

#[test]
fn test_readwrite() {
    use posish::{
        fs::{cwd, openat, seek, Mode, OFlags},
        io::{read, write},
    };
    use std::io::SeekFrom;

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(&cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let foo = openat(
        &dir,
        "foo",
        OFlags::RDWR | OFlags::CREATE | OFlags::TRUNC,
        Mode::IRUSR | Mode::IWUSR,
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
