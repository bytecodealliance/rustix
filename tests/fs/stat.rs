#[test]
fn test_stat() {
    use rustix::fs::{fstat, lstat, stat, symlink};
    use std::io::Write as _;

    let tmp = tempfile::tempdir().unwrap();

    let mut w = std::fs::File::create(tmp.path().join("file")).unwrap();
    writeln!(&mut w, "Hello, File!").unwrap();

    assert_eq!(fstat(&w).unwrap().st_size, 13);
    assert_eq!(stat(tmp.path().join("file")).unwrap().st_size, 13);
    assert_eq!(lstat(tmp.path().join("file")).unwrap().st_size, 13);

    symlink("file", tmp.path().join("link")).unwrap();

    assert_eq!(stat(tmp.path().join("link")).unwrap().st_size, 13);
    assert_eq!(lstat(tmp.path().join("link")).unwrap().st_size, 4);
}
