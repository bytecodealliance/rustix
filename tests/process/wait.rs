use libc::{kill, SIGSTOP};
use rustix::{io, process};
use serial_test::serial;
use std::process::{Command, Stdio};

// these tests must execute serially to prevent race condition,
// where `test_wait` waits for the child process spawned in `test_waitpid`,
// causing the tests to get stuck.

#[test]
#[serial]
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
#[serial]
fn test_wait() {
    let child = Command::new("yes")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to execute child");
    unsafe { kill(child.id() as _, SIGSTOP) };

    let pid = unsafe { process::Pid::from_raw(child.id() as _) }.unwrap();
    let (child_pid, status) = process::wait(process::WaitOptions::UNTRACED)
        .expect("failed to wait")
        .unwrap();
    assert!(status.stopped());
    assert_eq!(child_pid, pid);
}

#[test]
#[serial]
fn test_posix_spawn() {
    let (read_pipe, write_pipe) = io::pipe().unwrap();
    let message = "posix_spawn works";
    let env_vars: &[&str] = &[];
    let mut config = process::SpawnConfig::default();
    let stdout = unsafe { io::stdout() };

    config.add_dup2_action(&write_pipe, &stdout);

    let pid =
        process::posix_spawn("/usr/bin/echo", &["echo", "-n", message], env_vars, &config).unwrap();

    // ensure reading from the pipe ends when the child finishes writing
    core::mem::drop(write_pipe);
    let mut buf = [0; 32];

    let len = io::read(&read_pipe, &mut buf).unwrap();
    let output = std::str::from_utf8(&buf[..len]).unwrap();
    assert_eq!(output, message);
    process::waitpid(Some(pid), process::WaitOptions::empty()).unwrap();
}
