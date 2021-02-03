use posish::fs::{Dir, Entry};
use std::collections::HashMap;
use std::path::Path;

#[test]
fn test_dir_entries() {
    let tmpdir = tempfile::tempdir().expect("construct tempdir");

    let entries = dir_entries(&tmpdir.path());
    assert!(
        entries.get("..").is_some(),
        "directory contains `..`: {:?}",
        entries
    );
    assert!(
        entries.get(".").is_some(),
        "directory contains `.`: {:?}",
        entries
    );
    assert_eq!(entries.len(), 2, "empty dir is just . and ..");

    let _f1 = std::fs::File::create(tmpdir.path().join("file1")).expect("create file1");

    let entries = dir_entries(&tmpdir.path());
    assert!(
        entries.get("file1").is_some(),
        "directory contains `file1`: {:?}",
        entries
    );
    assert!(
        entries.get(".").is_some(),
        "directory contains `.`: {:?}",
        entries
    );
    assert_eq!(entries.len(), 3);

    let _f2 = std::fs::File::create(tmpdir.path().join("file2")).expect("create file1");
    let entries = dir_entries(&tmpdir.path());
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
    assert_eq!(entries.len(), 4);
}

fn dir_entries(path: &Path) -> HashMap<String, Entry> {
    let dirfd = std::fs::File::open(path).unwrap();
    let dir = Dir::from(dirfd).expect("construct Dir from dirfd");
    read_entries(dir)
}

fn read_entries(dir: Dir) -> HashMap<String, Entry> {
    let mut out = HashMap::new();
    loop {
        match dir.read() {
            Some(e) => {
                let e = e.expect("non-error entry");
                let name = e.file_name().to_str().expect("utf8 filename").to_owned();
                assert!(out.get(&name).is_none(), "name already read: {}", name);
                out.insert(name, e);
            }
            None => break,
        }
    }
    out
}
