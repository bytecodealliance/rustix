//! Tests for the `pidfd` type.

use libc::{kill, SIGSTOP};
use rustix::{fd::AsFd, io, process};
use serial_test::serial;
use std::process::Command;

#[test]
#[serial]
fn test_pidfd_waitid() {
    // Create a new process.
    let child = Command::new("yes")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("failed to execute child");

    // Create a pidfd for the child process.
    let pid = unsafe { process::Pid::from_raw(child.id() as _) }.unwrap();
    let pidfd = match process::pidfd_open(pid, process::PidfdFlags::empty()) {
        Ok(pidfd) => pidfd,
        Err(e) if e == rustix::io::Errno::NOSYS => {
            // The kernel does not support pidfds.
            return;
        }
        Err(e) => panic!("failed to open pidfd: {}", e),
    };

    // Wait for the child process to stop.
    unsafe { kill(child.id() as _, SIGSTOP) };

    let status = process::waitid(
        process::WaitId::PidFd(pidfd.as_fd()),
        process::WaitidOptions::STOPPED,
    )
    .expect("failed to wait")
    .unwrap();

    // TODO
    let _ = status;
}

#[test]
#[serial]
fn test_pidfd_poll() {
    // Create a new process.
    let child = Command::new("sleep")
        .arg("1")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("failed to execute child");

    // Create a pidfd for the child process.
    let pid = unsafe { process::Pid::from_raw(child.id() as _) }.unwrap();
    let pidfd = match process::pidfd_open(pid, process::PidfdFlags::NONBLOCK) {
        Ok(pidfd) => pidfd,
        Err(e) if e == rustix::io::Errno::NOSYS || e == rustix::io::Errno::INVAL => {
            // The kernel does not support non-blocking pidfds.
            return;
        }
        Err(e) => panic!("failed to open pidfd: {}", e),
    };

    // The child process should not have exited yet.
    match process::waitid(
        process::WaitId::PidFd(pidfd.as_fd()),
        process::WaitidOptions::EXITED,
    ) {
        Err(e) if e == rustix::io::Errno::AGAIN => (),
        _ => panic!("unexpected result"),
    }

    // Wait for the child process to exit.
    let pfd = io::PollFd::new(&pidfd, io::PollFlags::IN);
    io::poll(&mut [pfd], -1).unwrap();

    // The child process should have exited.
    let status = process::waitid(
        process::WaitId::PidFd(pidfd.as_fd()),
        process::WaitidOptions::EXITED,
    )
    .expect("failed to wait")
    .unwrap();

    // TODO
    let _ = status;
}
