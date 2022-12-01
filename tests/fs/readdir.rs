#![cfg(not(target_os = "redox"))]

use std::collections::HashMap;
use std::fs::File;
#[cfg(any(target_os = "android", target_os = "linux"))]
use std::mem::MaybeUninit;

use rustix::fs::{Dir, DirEntry};

#[test]
fn dir_entries() {
    let tmpdir = tempfile::tempdir().expect("construct tempdir");
    let dirfd = File::open(tmpdir.path()).expect("open tempdir as file");
    let mut dir = Dir::read_from(dirfd).expect("construct Dir from dirfd");

    let entries = read_entries(&mut dir);
    assert_eq!(entries.len(), 0, "no files in directory");

    let _f1 = File::create(tmpdir.path().join("file1")).expect("create file1");

    let entries = read_entries(&mut dir);
    assert!(
        entries.get("file1").is_some(),
        "directory contains `file1`: {:?}",
        entries
    );
    assert_eq!(entries.len(), 1);

    let _f2 = File::create(tmpdir.path().join("file2")).expect("create file1");
    let entries = read_entries(&mut dir);
    assert!(
        entries.get("file1").is_some(),
        "directory contains `file1`: {:?}",
        entries
    );
    assert!(
        entries.get("file2").is_some(),
        "directory contains `file2`: {:?}",
        entries
    );
    assert_eq!(entries.len(), 2);
}

fn read_entries(dir: &mut Dir) -> HashMap<String, DirEntry> {
    dir.rewind();
    let mut out = HashMap::new();
    while let Some(err) = dir.read() {
        let err = err.expect("non-error entry");
        let name = err.file_name().to_str().expect("utf8 filename").to_owned();
        if name != "." && name != ".." {
            out.insert(name, err);
        }
    }
    out
}

#[cfg(any(target_os = "android", target_os = "linux"))]
fn test_raw_dir(buf: &mut [MaybeUninit<u8>]) {
    use std::io::{Seek, SeekFrom};

    use rustix::fd::AsFd;
    use rustix::fs::{RawDir, RawDirEntry};

    fn read_raw_entries<'a, Fd: AsFd>(
        dir: &'a mut RawDir<'_, Fd>,
    ) -> HashMap<String, RawDirEntry<'a>> {
        let mut out = HashMap::new();
        for entry in dir {
            let entry = entry.expect("non-error entry");
            let name = entry
                .file_name()
                .to_str()
                .expect("utf8 filename")
                .to_owned();
            if name != "." && name != ".." {
                out.insert(name, entry);
            }
        }
        out
    }

    let tmpdir = tempfile::tempdir().expect("construct tempdir");
    let mut dirfd = File::open(tmpdir.path()).expect("open tempdir as file");
    let mut dir = RawDir::new(dirfd.try_clone().unwrap(), buf);

    let entries = read_raw_entries(&mut dir);
    assert_eq!(entries.len(), 0, "no files in directory");

    let _f1 = File::create(tmpdir.path().join("file1")).expect("create file1");

    dirfd.seek(SeekFrom::Start(0)).unwrap();
    let entries = read_raw_entries(&mut dir);
    assert!(
        entries.get("file1").is_some(),
        "directory contains `file1`: {:?}",
        entries
    );
    assert_eq!(entries.len(), 1);

    let _f2 = File::create(tmpdir.path().join("file2")).expect("create file1");
    dirfd.seek(SeekFrom::Start(0)).unwrap();
    let entries = read_raw_entries(&mut dir);
    assert!(
        entries.get("file1").is_some(),
        "directory contains `file1`: {:?}",
        entries
    );
    assert!(
        entries.get("file2").is_some(),
        "directory contains `file2`: {:?}",
        entries
    );
    assert_eq!(entries.len(), 2);
}

#[test]
#[cfg(any(target_os = "android", target_os = "linux"))]
fn raw_dir_entries_heap() {
    // When we can depend on Rust 1.60, we can use the spare_capacity_mut version instead.
    /*
    let mut buf = Vec::with_capacity(8192);
    test_raw_dir(buf.spare_capacity_mut());
    */
    let mut buf = [MaybeUninit::new(0); 8192];
    test_raw_dir(&mut buf);
}

#[test]
#[cfg(any(target_os = "android", target_os = "linux"))]
fn raw_dir_entries_stack() {
    let mut buf = [MaybeUninit::uninit(); 2048];
    test_raw_dir(&mut buf);
}

#[test]
#[cfg(any(target_os = "android", target_os = "linux"))]
fn raw_dir_entries_unaligned() {
    let mut buf = [MaybeUninit::uninit(); 2048];
    let buf = &mut buf[1..];

    assert!(!(buf.as_ptr() as usize).is_power_of_two());

    test_raw_dir(buf);
}

#[test]
fn dir_from_openat() {
    let dirfd = rustix::fs::openat(
        rustix::fs::cwd(),
        ".",
        rustix::fs::OFlags::RDONLY,
        rustix::fs::Mode::empty(),
    )
    .expect("open cwd as file");
    let _dir = Dir::read_from(dirfd).expect("construct Dir from dirfd");
}
