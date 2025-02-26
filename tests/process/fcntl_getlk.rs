use rustix::fd::{AsRawFd as _, BorrowedFd};
use rustix::fs::{fcntl_lock, FlockOperation};
use rustix::process::{fcntl_getlk, getppid, Flock, FlockType};
use serial_test::serial;
use std::fs::File;
use std::os::unix::process::CommandExt as _;
use std::process::Command;

#[test]
#[serial]
fn test_fcntl_getlk() {
    let f = tempfile::tempfile().unwrap();

    fcntl_lock(&f, FlockOperation::Unlock).unwrap();
    unsafe {
        child_process(&f, |fd| {
            let lock = fcntl_getlk(&fd, &Flock::from(FlockType::ReadLock)).unwrap();
            assert_eq!(lock, None);

            let lock = fcntl_getlk(&fd, &Flock::from(FlockType::WriteLock)).unwrap();
            assert_eq!(lock, None);
        })
    };

    fcntl_lock(&f, FlockOperation::LockShared).unwrap();
    unsafe {
        child_process(&f, |fd| {
            let lock = fcntl_getlk(&fd, &Flock::from(FlockType::ReadLock)).unwrap();
            assert_eq!(lock, None);

            let lock = fcntl_getlk(&fd, &Flock::from(FlockType::WriteLock)).unwrap();
            assert_eq!(lock.and_then(|l| l.pid), getppid());
        })
    };

    fcntl_lock(&f, FlockOperation::LockExclusive).unwrap();
    unsafe {
        child_process(&f, |fd| {
            let lock = fcntl_getlk(&fd, &Flock::from(FlockType::ReadLock)).unwrap();
            assert_eq!(lock.and_then(|l| l.pid), getppid());

            let lock = fcntl_getlk(&fd, &Flock::from(FlockType::WriteLock)).unwrap();
            assert_eq!(lock.and_then(|l| l.pid), getppid());
        })
    };
}

unsafe fn child_process<F>(file: &File, f: F)
where
    F: Fn(BorrowedFd<'static>) + Send + Sync + 'static,
{
    let fd = BorrowedFd::borrow_raw(file.as_raw_fd());
    let output = Command::new("true")
        .pre_exec(move || {
            f(fd);
            Ok(())
        })
        .output()
        .unwrap();
    if !output.status.success() {
        panic!("{}", std::str::from_utf8(&output.stderr).unwrap());
    }
}
