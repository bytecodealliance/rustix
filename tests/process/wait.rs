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

#[test]
fn test_wait() {
    let child = Command::new("yes")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to execute child");
    unsafe { kill(child.id() as _, SIGSTOP) };

    let pid = unsafe { process::Pid::from_raw(child.id() as _) };
    let (child_pid, status) = process::wait(process::WaitOptions::UNTRACED)
        .expect("failed to wait")
        .unwrap();
    assert!(status.stopped());
    assert_eq!(child_pid, pid);
}
