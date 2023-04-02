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

#[cfg(not(any(
    target_os = "wasi",
    target_os = "emscripten",
    target_os = "redox",
    target_os = "openbsd"
)))]
#[test]
#[serial]
fn test_waitid() {
    let child = Command::new("yes")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to execute child");
    unsafe { kill(child.id() as _, SIGSTOP) };

    let pid = unsafe { process::Pid::from_raw(child.id() as _) };
    let status = process::waitid(
        process::WaitId::Pid(pid.unwrap()),
        process::WaitidOptions::STOPPED,
    )
    .expect("failed to wait")
    .unwrap();

    assert!(status.stopped());
    #[cfg(not(any(target_os = "netbsd", target_os = "fuchsia")))]
    assert_eq!(status.stopping_signal(), Some(SIGSTOP as _));

    unsafe { kill(child.id() as _, SIGCONT) };

    let status = process::waitid(
        process::WaitId::Pid(pid.unwrap()),
        process::WaitidOptions::CONTINUED,
    )
    .expect("failed to wait")
    .unwrap();

    assert!(status.continued());

    let status = process::waitid(
        process::WaitId::All,
        process::WaitidOptions::EXITED | process::WaitidOptions::NOHANG,
    )
    .expect("failed to wait");

    assert!(status.is_none());

    unsafe { kill(child.id() as _, SIGKILL) };

    let status = process::waitid(
        process::WaitId::Pid(pid.unwrap()),
        process::WaitidOptions::EXITED | process::WaitidOptions::NOWAIT,
    )
    .expect("failed to wait")
    .unwrap();

    assert!(status.killed());
    #[cfg(not(any(target_os = "netbsd", target_os = "fuchsia")))]
    assert_eq!(status.terminating_signal(), Some(SIGKILL as _));

    let status = process::waitid(
        process::WaitId::Pid(pid.unwrap()),
        process::WaitidOptions::EXITED,
    )
    .expect("failed to wait")
    .unwrap();

    assert!(status.killed());
    #[cfg(not(any(target_os = "netbsd", target_os = "fuchsia")))]
    assert_eq!(status.terminating_signal(), Some(SIGKILL as _));
}
