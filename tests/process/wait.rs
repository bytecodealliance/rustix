use libc::{kill, SIGSTOP};
use rsix::process;
use std::process::Command;
use std::process::Stdio;

#[test]
fn test_waitpid() {
    let child = Command::new("yes")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to execute child");
    unsafe { kill(child.id() as _, SIGSTOP) };

    let pid = unsafe { process::Pid::from_raw(child.id() as _) };
    let status = process::waitpid(pid, process::WaitOptions::UNTRACED)
        .expect("failed to wait")
        .unwrap();
    assert!(status.stopped());
}
