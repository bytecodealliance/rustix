use posish::io::isatty;
use tempfile::{tempdir, TempDir};

#[allow(unused)]
fn tmpdir() -> TempDir {
    tempdir().expect("expected to be able to create a temporary directory")
}

#[test]
fn std_file_is_not_terminal() {
    let tmpdir = tempfile::tempdir().unwrap();
    assert!(!isatty(
        &std::fs::File::create(tmpdir.path().join("file")).unwrap()
    ));
    assert!(!isatty(
        &std::fs::File::open(tmpdir.path().join("file")).unwrap()
    ));
}

#[test]
fn stdout_stderr_terminals() {
    assert_eq!(isatty(&std::io::stdout()), atty::is(atty::Stream::Stdout));
    assert_eq!(isatty(&std::io::stderr()), atty::is(atty::Stream::Stderr));
}
