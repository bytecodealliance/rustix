#[allow(unused_imports)]
use libc::{kill, SIGCONT, SIGKILL, SIGSTOP};
use rustix::process;
use serial_test::serial;
use std::process::{Command, Stdio};

// These tests must execute serially to prevent race condition, where
// `test_wait` waits for the child process spawned in `test_waitpid`, causing
// the tests to get stuck.

#[test]
#[serial]
fn test_waitpid_none() {
    let child = Command::new("yes")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to execute child");
    unsafe { kill(child.id() as _, SIGSTOP) };

    let (pid, status) = process::waitpid(None, process::WaitOptions::UNTRACED)
        .expect("failed to wait")
        .unwrap();
    assert_eq!(pid, process::Pid::from_child(&child));
    assert!(status.stopped());
    assert_ne!(status.as_raw(), 0);

    // Clean up the child process.
    unsafe { kill(child.id() as _, SIGKILL) };

    let (pid, status) = process::waitpid(None, process::WaitOptions::UNTRACED)
        .expect("failed to wait")
        .unwrap();
    assert_eq!(pid, process::Pid::from_child(&child));
    assert!(status.signaled());
}

#[test]
#[serial]
fn test_waitpid_some() {
    let child = Command::new("yes")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to execute child");
    unsafe { kill(child.id() as _, SIGSTOP) };

    let pid = process::Pid::from_child(&child);
    let (rpid, status) = process::waitpid(Some(pid), process::WaitOptions::UNTRACED)
        .expect("failed to wait")
        .unwrap();
    assert_eq!(rpid, pid);
    assert!(status.stopped());
    assert_ne!(status.as_raw(), 0);

    // Clean up the child process.
    unsafe { kill(child.id() as _, SIGKILL) };

    let (rpid, status) = process::waitpid(Some(pid), process::WaitOptions::UNTRACED)
        .expect("failed to wait")
        .unwrap();
    assert_eq!(rpid, pid);
    assert!(status.signaled());
}

#[test]
#[serial]
fn test_waitpgid() {
    let child = Command::new("yes")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to execute child");
    unsafe { kill(child.id() as _, SIGSTOP) };

    let pgid = process::getpgrp();
    let (pid, status) = process::waitpgid(pgid, process::WaitOptions::UNTRACED)
        .expect("failed to wait")
        .unwrap();
    assert_eq!(pid, process::Pid::from_child(&child));
    assert!(status.stopped());
    assert_ne!(status.as_raw(), 0);

    // Clean up the child process.
    unsafe { kill(child.id() as _, SIGKILL) };

    let (pid, status) = process::waitpgid(pgid, process::WaitOptions::UNTRACED)
        .expect("failed to wait")
        .unwrap();
    assert_eq!(pid, process::Pid::from_child(&child));
    assert!(status.signaled());
}

#[cfg(not(any(
    target_os = "cygwin",
    target_os = "emscripten",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
#[test]
#[serial]
fn test_waitid() {
    let child = Command::new("yes")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to execute child");
    let pid = process::Pid::from_child(&child);
    let pgid = process::getpgid(Some(pid)).unwrap();

    // Test waiting for the process by pid.

    unsafe { kill(child.id() as _, SIGSTOP) };

    let status = process::waitid(process::WaitId::Pid(pid), process::WaitIdOptions::STOPPED)
        .expect("failed to wait")
        .unwrap();

    assert!(status.stopped());
    #[cfg(not(any(target_os = "fuchsia", target_os = "netbsd")))]
    assert_eq!(status.stopping_signal(), Some(SIGSTOP as _));
    assert_eq!(status.raw_signo(), libc::SIGCHLD);
    assert_eq!(status.raw_errno(), 0);
    assert_eq!(status.raw_code(), libc::CLD_STOPPED);

    unsafe { kill(child.id() as _, SIGCONT) };

    let status = process::waitid(process::WaitId::Pid(pid), process::WaitIdOptions::CONTINUED)
        .expect("failed to wait")
        .unwrap();

    assert!(status.continued());

    // Now do the same thing with the pgid.

    unsafe { kill(child.id() as _, SIGSTOP) };

    let status = process::waitid(
        process::WaitId::Pgid(Some(pgid)),
        process::WaitIdOptions::STOPPED,
    )
    .expect("failed to wait")
    .unwrap();

    assert!(status.stopped());
    #[cfg(not(any(target_os = "fuchsia", target_os = "netbsd")))]
    assert_eq!(status.stopping_signal(), Some(SIGSTOP as _));
    assert_eq!(status.raw_signo(), libc::SIGCHLD);
    assert_eq!(status.raw_errno(), 0);
    assert_eq!(status.raw_code(), libc::CLD_STOPPED);

    unsafe { kill(child.id() as _, SIGCONT) };

    let status = process::waitid(
        process::WaitId::Pgid(Some(pgid)),
        process::WaitIdOptions::CONTINUED,
    )
    .expect("failed to wait")
    .unwrap();

    assert!(status.continued());

    let status = process::waitid(
        process::WaitId::Pid(pid),
        process::WaitIdOptions::EXITED | process::WaitIdOptions::NOHANG,
    )
    .expect("failed to wait");

    assert!(status.is_none());

    unsafe { kill(child.id() as _, SIGKILL) };

    let status = process::waitid(
        process::WaitId::Pid(pid),
        process::WaitIdOptions::EXITED | process::WaitIdOptions::NOWAIT,
    )
    .expect("failed to wait")
    .unwrap();

    assert!(status.killed());
    #[cfg(not(any(target_os = "fuchsia", target_os = "netbsd")))]
    assert_eq!(status.terminating_signal(), Some(SIGKILL as _));

    let status = process::waitid(process::WaitId::Pid(pid), process::WaitIdOptions::EXITED)
        .expect("failed to wait")
        .unwrap();

    assert!(status.killed());
    #[cfg(not(any(target_os = "fuchsia", target_os = "netbsd")))]
    assert_eq!(status.terminating_signal(), Some(SIGKILL as _));
}
