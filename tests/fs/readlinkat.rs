#[test]
fn test_readlink() {
    use rustix::fs::{open, readlink, symlink, Mode, OFlags};

    let tmp = tempfile::tempdir().unwrap();

    let _ = open(
        tmp.path().join("foo"),
        OFlags::CREATE | OFlags::WRONLY,
        Mode::RUSR,
    )
    .unwrap();
    symlink("foo", tmp.path().join("link")).unwrap();

    readlink(tmp.path().join("absent"), Vec::new()).unwrap_err();
    readlink(tmp.path().join("foo"), Vec::new()).unwrap_err();

    let target = readlink(tmp.path().join("link"), Vec::new()).unwrap();
    assert_eq!(target.to_string_lossy(), "foo");

    symlink("link", tmp.path().join("another")).unwrap();

    let target = readlink(tmp.path().join("link"), Vec::new()).unwrap();
    assert_eq!(target.to_string_lossy(), "foo");
    let target = readlink(tmp.path().join("another"), Vec::new()).unwrap();
    assert_eq!(target.to_string_lossy(), "link");
}

#[cfg(not(target_os = "redox"))]
#[test]
fn test_readlinkat() {
    use rustix::fs::{openat, readlinkat, symlinkat, Mode, OFlags, CWD};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();

    let _ = openat(&dir, "foo", OFlags::CREATE | OFlags::WRONLY, Mode::RUSR).unwrap();
    symlinkat("foo", &dir, "link").unwrap();

    readlinkat(&dir, "absent", Vec::new()).unwrap_err();
    readlinkat(&dir, "foo", Vec::new()).unwrap_err();

    let target = readlinkat(&dir, "link", Vec::new()).unwrap();
    assert_eq!(target.to_string_lossy(), "foo");

    symlinkat("link", &dir, "another").unwrap();

    let target = readlinkat(&dir, "link", Vec::new()).unwrap();
    assert_eq!(target.to_string_lossy(), "foo");
    let target = readlinkat(&dir, "another", Vec::new()).unwrap();
    assert_eq!(target.to_string_lossy(), "link");
}

#[cfg(not(target_os = "redox"))]
#[test]
fn test_readlinkat_raw() {
    use core::mem::MaybeUninit;
    use rustix::fs::{openat, readlinkat_raw, symlinkat, Mode, OFlags, CWD};
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();

    let _ = openat(&dir, "foo", OFlags::CREATE | OFlags::WRONLY, Mode::RUSR).unwrap();
    symlinkat("foo", &dir, "link").unwrap();

    let mut some = [MaybeUninit::<u8>::new(0); 32];
    let mut short = [MaybeUninit::<u8>::new(0); 2];
    readlinkat_raw(&dir, "absent", &mut some).unwrap_err();
    readlinkat_raw(&dir, "foo", &mut some).unwrap_err();

    let (yes, no) = readlinkat_raw(&dir, "link", &mut some).unwrap();
    assert_eq!(OsStr::from_bytes(yes).to_string_lossy(), "foo");
    assert!(!no.is_empty());

    let (yes, no) = readlinkat_raw(&dir, "link", &mut short).unwrap();
    assert_eq!(yes, &[b'f', b'o']);
    assert!(no.is_empty());

    symlinkat("link", &dir, "another").unwrap();

    let (yes, no) = readlinkat_raw(&dir, "link", &mut some).unwrap();
    assert_eq!(OsStr::from_bytes(yes).to_string_lossy(), "foo");
    assert!(!no.is_empty());

    let (yes, no) = readlinkat_raw(&dir, "link", &mut short).unwrap();
    assert_eq!(yes, &[b'f', b'o']);
    assert!(no.is_empty());

    let (yes, no) = readlinkat_raw(&dir, "another", &mut some).unwrap();
    assert_eq!(OsStr::from_bytes(yes).to_string_lossy(), "link");
    assert!(!no.is_empty());

    let (yes, no) = readlinkat_raw(&dir, "another", &mut short).unwrap();
    assert_eq!(yes, &[b'l', b'i']);
    assert!(no.is_empty());
}
