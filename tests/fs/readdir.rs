#![cfg(not(target_os = "redox"))]

use std::collections::HashMap;
use std::fs::File;
use std::io::{Seek, SeekFrom};

use rustix::fd::AsFd;
use rustix::fs::{Dir, DirEntry, RawDir, RawDirEntry};

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

#[test]
fn raw_dir_entries() {
    let tmpdir = tempfile::tempdir().expect("construct tempdir");
    let mut dirfd = File::open(tmpdir.path()).expect("open tempdir as file");
    let mut buf = Vec::with_capacity(8192);
    let mut dir = RawDir::new(dirfd.try_clone().unwrap(), buf.spare_capacity_mut());

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

fn read_raw_entries<'a, Fd: AsFd>(dir: &'a mut RawDir<'_, Fd>) -> HashMap<String, RawDirEntry<'a>> {
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
