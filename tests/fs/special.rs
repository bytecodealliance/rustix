#[cfg(feature = "process")]
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[test]
fn test_special_fds() {
    use rustix::fs::{fstat, open, openat, Mode, OFlags, Stat, ABS, CWD};
    use rustix::process::getcwd;
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt as _;
    use std::path::PathBuf;

    let cwd_path = getcwd(Vec::new()).unwrap().into_bytes();
    let cwd_path = OsStr::from_bytes(&cwd_path).to_owned();
    let cwd_path = PathBuf::from(cwd_path);

    // Open the same file several ways using special constants and make sure we
    // get the same file.

    // Use plain `open`.
    let a = open("Cargo.toml", OFlags::RDONLY, Mode::empty()).unwrap();

    // Use `CWD` with a relative path.
    let b = openat(CWD, "Cargo.toml", OFlags::RDONLY, Mode::empty()).unwrap();

    // Use `CWD` with an absolute path.
    let c = openat(
        CWD,
        cwd_path.join("Cargo.toml"),
        OFlags::RDONLY,
        Mode::empty(),
    )
    .unwrap();

    // Use `ABS` with an absolute path.
    let d = openat(
        ABS,
        cwd_path.join("Cargo.toml"),
        OFlags::RDONLY,
        Mode::empty(),
    )
    .unwrap();

    // Test that opening a relative path with `ABS` fails.
    let err = openat(ABS, "Cargo.toml", OFlags::RDONLY, Mode::empty()).unwrap_err();
    assert_eq!(err, rustix::io::Errno::BADF);

    let a_stat = fstat(a).unwrap();
    let b_stat = fstat(b).unwrap();
    let c_stat = fstat(c).unwrap();
    let d_stat = fstat(d).unwrap();

    assert!(same(&a_stat, &b_stat));
    assert!(same(&b_stat, &c_stat));
    assert!(same(&c_stat, &d_stat));

    fn same(a: &Stat, b: &Stat) -> bool {
        a.st_ino == b.st_ino && a.st_dev == b.st_dev
    }
}
