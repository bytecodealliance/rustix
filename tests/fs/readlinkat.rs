#[test]
fn test_readlink() {
    use rustix::fs::{open, readlink, symlink, Mode, OFlags};

    let tmp = tempfile::tempdir().unwrap();

    let _ = open(
        tmp.path().join("file"),
        OFlags::CREATE | OFlags::WRONLY,
        Mode::RUSR,
    )
    .unwrap();
    symlink("file", tmp.path().join("link")).unwrap();

    readlink(tmp.path().join("absent"), Vec::new()).unwrap_err();
    readlink(tmp.path().join("file"), Vec::new()).unwrap_err();

    let target = readlink(tmp.path().join("link"), Vec::new()).unwrap();
    assert_eq!(target.to_string_lossy(), "file");

    symlink("link", tmp.path().join("another")).unwrap();

    let target = readlink(tmp.path().join("link"), Vec::new()).unwrap();
    assert_eq!(target.to_string_lossy(), "file");
    let target = readlink(tmp.path().join("another"), Vec::new()).unwrap();
    assert_eq!(target.to_string_lossy(), "link");
}

#[cfg(not(target_os = "redox"))]
#[test]
fn test_readlinkat() {
    use rustix::fs::{openat, readlinkat, symlinkat, Mode, OFlags, CWD};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();

    let _ = openat(&dir, "file", OFlags::CREATE | OFlags::WRONLY, Mode::RUSR).unwrap();
    symlinkat("file", &dir, "link").unwrap();

    readlinkat(&dir, "absent", Vec::new()).unwrap_err();
    readlinkat(&dir, "file", Vec::new()).unwrap_err();

    let target = readlinkat(&dir, "link", Vec::new()).unwrap();
    assert_eq!(target.to_string_lossy(), "file");

    symlinkat("link", &dir, "another").unwrap();

    let target = readlinkat(&dir, "link", Vec::new()).unwrap();
    assert_eq!(target.to_string_lossy(), "file");
    let target = readlinkat(&dir, "another", Vec::new()).unwrap();
    assert_eq!(target.to_string_lossy(), "link");
}

#[cfg(not(target_os = "redox"))]
#[test]
fn test_readlinkat_raw() {
    use core::mem::MaybeUninit;
    use rustix::fs::{openat, readlinkat_raw, symlinkat, Mode, OFlags, CWD};
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt as _;

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();

    let _ = openat(&dir, "file", OFlags::CREATE | OFlags::WRONLY, Mode::RUSR).unwrap();
    symlinkat("file", &dir, "link").unwrap();

    let mut some = [MaybeUninit::<u8>::new(0); 32];
    let mut short = [MaybeUninit::<u8>::new(0); 2];
    readlinkat_raw(&dir, "absent", &mut some).unwrap_err();
    readlinkat_raw(&dir, "file", &mut some).unwrap_err();

    let (yes, no) = readlinkat_raw(&dir, "link", &mut some).unwrap();
    assert_eq!(OsStr::from_bytes(yes).to_string_lossy(), "file");
    assert!(!no.is_empty());

    let (yes, no) = readlinkat_raw(&dir, "link", &mut short).unwrap();
    assert_eq!(yes, b"fi");
    assert!(no.is_empty());

    symlinkat("link", &dir, "another").unwrap();

    let (yes, no) = readlinkat_raw(&dir, "link", &mut some).unwrap();
    assert_eq!(OsStr::from_bytes(yes).to_string_lossy(), "file");
    assert!(!no.is_empty());

    let (yes, no) = readlinkat_raw(&dir, "link", &mut short).unwrap();
    assert_eq!(yes, b"fi");
    assert!(no.is_empty());

    let (yes, no) = readlinkat_raw(&dir, "another", &mut some).unwrap();
    assert_eq!(OsStr::from_bytes(yes).to_string_lossy(), "link");
    assert!(!no.is_empty());

    let (yes, no) = readlinkat_raw(&dir, "another", &mut short).unwrap();
    assert_eq!(yes, b"li");
    assert!(no.is_empty());
}
