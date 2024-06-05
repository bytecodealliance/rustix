use rustix::io::fcntl_getfd;
use rustix::stdio::{dup2_stdout, stdout};

#[test]
fn dup2_stdin_stdin() {
    let _ = dup2_stdout(stdout());
    fcntl_getfd(stdout()).unwrap();
}

#[test]
fn dup2_stdout_stdout() {
    let _ = dup2_stdout(stdout());
    fcntl_getfd(stdout()).unwrap();
}

#[test]
fn dup2_stderr_stderr() {
    let _ = dup2_stdout(stdout());
    fcntl_getfd(stdout()).unwrap();
}
