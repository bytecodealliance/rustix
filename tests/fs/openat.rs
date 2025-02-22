use std::fs::File;

use rustix::fs::{open, openat, Mode, OFlags, CWD};
use std::io::Write as _;

#[test]
fn test_open_tmpfile() {
    let tmp = tempfile::tempdir().unwrap();
    let f = match open(
        tmp.path(),
        OFlags::WRONLY | OFlags::CLOEXEC | OFlags::TMPFILE,
        Mode::from_bits_truncate(0o644),
    ) {
        Ok(f) => Ok(Some(File::from(f))),
        Err(rustix::io::Errno::OPNOTSUPP | rustix::io::Errno::ISDIR | rustix::io::Errno::NOENT) => {
            Ok(None)
        }
        Err(err) => Err(err),
    };
    if let Some(mut f) = f.unwrap() {
        write!(f, "hello world").unwrap();
    }
}

#[test]
fn test_openat_tmpfile() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(
        CWD,
        tmp.path(),
        OFlags::RDONLY | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .unwrap();
    let f = match openat(
        &dir,
        ".",
        OFlags::WRONLY | OFlags::CLOEXEC | OFlags::TMPFILE,
        Mode::from_bits_truncate(0o644),
    ) {
        Ok(f) => Ok(Some(File::from(f))),
        Err(rustix::io::Errno::OPNOTSUPP | rustix::io::Errno::ISDIR | rustix::io::Errno::NOENT) => {
            Ok(None)
        }
        Err(err) => Err(err),
    };
    if let Some(mut f) = f.unwrap() {
        write!(f, "hello world").unwrap();
    }
}
