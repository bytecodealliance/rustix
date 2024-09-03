//! Tests for the `pidfd` type.

use libc::{kill, SIGSTOP};
#[cfg(feature = "event")]
use rustix::event;
use rustix::fd::AsFd;
use rustix::{io, process};
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
    let pid = process::Pid::from_child(&child);
    let pidfd = match process::pidfd_open(pid, process::PidfdFlags::empty()) {
        Ok(pidfd) => pidfd,
        Err(e) if e == io::Errno::NOSYS => {
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

#[cfg(feature = "event")]
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
    let pid = process::Pid::from_child(&child);
    let pidfd = match process::pidfd_open(pid, process::PidfdFlags::NONBLOCK) {
        Ok(pidfd) => pidfd,
        Err(e) if e == io::Errno::NOSYS || e == io::Errno::INVAL => {
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
        Err(io::Errno::AGAIN) => (),
        Err(e) => panic!("unexpected result: {:?}", e),
        Ok(_) => panic!("unexpected success"),
    }

    // Wait for the child process to exit.
    let pfd = event::PollFd::new(&pidfd, event::PollFlags::IN);
    event::poll(&mut [pfd], -1).unwrap();

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
