//! Tests for the `pidfd` type.

use libc::{kill, SIGCONT, SIGINT, SIGSTOP};
#[cfg(feature = "event")]
use rustix::event;
use rustix::fd::AsFd as _;
use rustix::{io, process};
use serial_test::serial;
use std::process::Command;

#[test]
#[serial]
fn test_pidfd_waitid() {
    // Create a new process.
    let mut child = Command::new("yes")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("failed to execute child");

    // Create a pidfd for the child process.
    let pid = process::Pid::from_child(&child);
    let pidfd = match process::pidfd_open(pid, process::PidfdFlags::empty()) {
        Ok(pidfd) => pidfd,
        Err(io::Errno::NOSYS) => {
            // The kernel does not support pidfds.
            unsafe { kill(child.id() as _, SIGINT) };
            child.wait().unwrap();
            return;
        }
        Err(e) => panic!("failed to open pidfd: {}", e),
    };

    // Wait for the child process to stop.
    unsafe { kill(child.id() as _, SIGSTOP) };

    let status = process::waitid(
        process::WaitId::PidFd(pidfd.as_fd()),
        process::WaitIdOptions::STOPPED,
    )
    .expect("failed to wait")
    .unwrap();

    assert!(status.stopped());
    assert!(!status.exited());
    assert!(!status.killed());
    assert!(!status.trapped());
    assert!(!status.dumped());
    assert!(!status.continued());

    assert_eq!(
        status.stopping_signal(),
        Some(process::Signal::STOP.as_raw())
    );
    assert_eq!(status.trapping_signal(), None);
    assert_eq!(status.exit_status(), None);
    assert_eq!(status.terminating_signal(), None);

    unsafe { kill(child.id() as _, SIGCONT) };

    let status = process::waitid(
        process::WaitId::PidFd(pidfd.as_fd()),
        process::WaitIdOptions::CONTINUED,
    )
    .expect("failed to wait")
    .unwrap();

    assert!(!status.stopped());
    assert!(!status.exited());
    assert!(!status.killed());
    assert!(!status.trapped());
    assert!(!status.dumped());
    assert!(status.continued());

    assert_eq!(status.stopping_signal(), None);
    assert_eq!(status.trapping_signal(), None);
    assert_eq!(status.exit_status(), None);
    assert_eq!(status.terminating_signal(), None);

    unsafe { kill(child.id() as _, SIGINT) };

    let status = process::waitid(
        process::WaitId::PidFd(pidfd.as_fd()),
        process::WaitIdOptions::EXITED,
    )
    .expect("failed to wait")
    .unwrap();

    assert!(!status.stopped());
    assert!(!status.exited());
    assert!(status.killed());
    assert!(!status.trapped());
    assert!(!status.dumped());
    assert!(!status.continued());

    assert_eq!(status.stopping_signal(), None);
    assert_eq!(status.trapping_signal(), None);
    assert_eq!(status.exit_status(), None);
    assert_eq!(status.terminating_signal(), Some(SIGINT));
}

// Similar to `test_pidfd_waitid`, but use `pidfd_send_signal` to send the
// signals.
#[test]
#[serial]
fn test_pidfd_send_signal() {
    // Create a new process.
    let mut child = Command::new("yes")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("failed to execute child");

    // Create a pidfd for the child process.
    let pid = process::Pid::from_child(&child);
    let pidfd = match process::pidfd_open(pid, process::PidfdFlags::empty()) {
        Ok(pidfd) => pidfd,
        Err(io::Errno::NOSYS) => {
            // The kernel does not support pidfds.
            process::kill_process(process::Pid::from_child(&child), process::Signal::INT).unwrap();
            child.wait().unwrap();
            return;
        }
        Err(e) => panic!("failed to open pidfd: {}", e),
    };

    // Wait for the child process to stop.
    process::pidfd_send_signal(&pidfd, process::Signal::STOP).unwrap();

    let status = process::waitid(
        process::WaitId::PidFd(pidfd.as_fd()),
        process::WaitIdOptions::STOPPED,
    )
    .expect("failed to wait")
    .unwrap();

    assert!(status.stopped());
    assert!(!status.exited());
    assert!(!status.killed());
    assert!(!status.trapped());
    assert!(!status.dumped());
    assert!(!status.continued());

    assert_eq!(
        status.stopping_signal(),
        Some(process::Signal::STOP.as_raw())
    );
    assert_eq!(status.trapping_signal(), None);
    assert_eq!(status.exit_status(), None);
    assert_eq!(status.terminating_signal(), None);

    process::pidfd_send_signal(&pidfd, process::Signal::CONT).unwrap();

    let status = process::waitid(
        process::WaitId::PidFd(pidfd.as_fd()),
        process::WaitIdOptions::CONTINUED,
    )
    .expect("failed to wait")
    .unwrap();

    assert!(!status.stopped());
    assert!(!status.exited());
    assert!(!status.killed());
    assert!(!status.trapped());
    assert!(!status.dumped());
    assert!(status.continued());

    assert_eq!(status.stopping_signal(), None);
    assert_eq!(status.trapping_signal(), None);
    assert_eq!(status.exit_status(), None);
    assert_eq!(status.terminating_signal(), None);

    process::pidfd_send_signal(&pidfd, process::Signal::INT).unwrap();

    let status = process::waitid(
        process::WaitId::PidFd(pidfd.as_fd()),
        process::WaitIdOptions::EXITED,
    )
    .expect("failed to wait")
    .unwrap();

    assert!(!status.stopped());
    assert!(!status.exited());
    assert!(status.killed());
    assert!(!status.trapped());
    assert!(!status.dumped());
    assert!(!status.continued());

    assert_eq!(status.stopping_signal(), None);
    assert_eq!(status.trapping_signal(), None);
    assert_eq!(status.exit_status(), None);
    assert_eq!(status.terminating_signal(), Some(SIGINT));
}

#[cfg(feature = "event")]
#[test]
#[serial]
fn test_pidfd_poll() {
    // Create a new process.
    let mut child = Command::new("sleep")
        .arg("1")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("failed to execute child");

    // Create a pidfd for the child process.
    let pid = process::Pid::from_child(&child);
    let pidfd = match process::pidfd_open(pid, process::PidfdFlags::NONBLOCK) {
        Ok(pidfd) => pidfd,
        Err(io::Errno::NOSYS) | Err(io::Errno::INVAL) => {
            // The kernel does not support non-blocking pidfds.
            process::kill_process(process::Pid::from_child(&child), process::Signal::INT).unwrap();
            child.wait().unwrap();
            return;
        }
        Err(e) => panic!("failed to open pidfd: {}", e),
    };

    // The child process should not have exited yet.
    match process::waitid(
        process::WaitId::PidFd(pidfd.as_fd()),
        process::WaitIdOptions::EXITED,
    ) {
        Err(io::Errno::AGAIN) => (),
        Err(e) => panic!("unexpected result: {:?}", e),
        Ok(_) => panic!("unexpected success"),
    }

    // Wait for the child process to exit.
    let pfd = event::PollFd::new(&pidfd, event::PollFlags::IN);
    event::poll(&mut [pfd], None).unwrap();

    // The child process should have exited.
    let status = process::waitid(
        process::WaitId::PidFd(pidfd.as_fd()),
        process::WaitIdOptions::EXITED,
    )
    .expect("failed to wait")
    .unwrap();

    assert!(!status.stopped());
    assert!(status.exited());
    assert!(!status.killed());
    assert!(!status.trapped());
    assert!(!status.dumped());
    assert!(!status.continued());

    assert_eq!(status.stopping_signal(), None);
    assert_eq!(status.trapping_signal(), None);
    assert_eq!(status.exit_status(), Some(0));
    assert_eq!(status.terminating_signal(), None);
}
