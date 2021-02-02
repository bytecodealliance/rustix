use posish::fs::{Dir, Entry};
use std::collections::HashMap;

#[test]
fn dir_entries() {
    let tmpdir = tempfile::tempdir().expect("construct tempdir");
    let dirfd = std::fs::File::open(tmpdir.path()).expect("open tempdir as file");
    let dir = Dir::from(dirfd).expect("construct Dir from dirfd");

    let entries = read_entries(&dir);
    assert_eq!(entries.len(), 0, "no files in directory");

    let _f1 = std::fs::File::create(tmpdir.path().join("file1")).expect("create file1");

    let entries = read_entries(&dir);
    assert!(
        entries.get("file1").is_some(),
        "directory contains `file1`: {:?}",
        entries
    );
    assert_eq!(entries.len(), 1);

    let _f2 = std::fs::File::create(tmpdir.path().join("file2")).expect("create file1");
    let entries = read_entries(&dir);
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

fn read_entries(dir: &Dir) -> HashMap<String, Entry> {
    dir.rewind();
    let mut out = HashMap::new();
    loop {
        match dir.read() {
            Some(e) => {
                let e = e.expect("non-error entry");
                let name = e.file_name().to_str().expect("utf8 filename").to_owned();
                if name != "." && name != ".." {
                    out.insert(name, e);
                }
            }
            None => break,
        }
    }
    out
}
