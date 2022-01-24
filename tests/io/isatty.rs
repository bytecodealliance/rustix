use is_terminal::IsTerminal;
use rustix::io::isatty;
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
    // This test is flaky under qemu.
    if std::env::vars().any(|var| var.0.starts_with("CARGO_TARGET_") && var.0.ends_with("_RUNNER"))
    {
        return;
    }
    assert_eq!(isatty(&std::io::stdout()), std::io::stdout().is_terminal());
    assert_eq!(isatty(&std::io::stderr()), std::io::stderr().is_terminal());
}

#[test]
fn stdio_descriptors() {
    #[cfg(unix)]
    use std::os::unix::io::AsRawFd;
    #[cfg(target_os = "wasi")]
    use std::os::wasi::io::AsRawFd;

    unsafe {
        assert_eq!(
            rustix::io::stdin().as_raw_fd(),
            std::io::stdin().as_raw_fd()
        );
        assert_eq!(
            rustix::io::stdout().as_raw_fd(),
            std::io::stdout().as_raw_fd()
        );
        assert_eq!(
            rustix::io::stderr().as_raw_fd(),
            std::io::stderr().as_raw_fd()
        );
    }
}
