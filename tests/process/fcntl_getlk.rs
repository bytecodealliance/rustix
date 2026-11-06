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

#[cfg(linux_kernel)]
#[test]
fn test_fcntl_ofd_getlk() {
    use rustix::fs::fcntl_ofd_lock;
    use rustix::process::{fcntl_ofd_getlk, getpid};

    let f = tempfile::NamedTempFile::new().unwrap();
    let g = f.reopen().unwrap();
    let read_lock = Flock::from(FlockType::ReadLock);
    let write_lock = Flock::from(FlockType::WriteLock);

    assert_eq!(fcntl_ofd_getlk(&g, &read_lock).unwrap(), None);
    assert_eq!(fcntl_ofd_getlk(&g, &write_lock).unwrap(), None);

    fcntl_ofd_lock(&f, FlockOperation::LockShared).unwrap();
    assert_eq!(fcntl_ofd_getlk(&g, &read_lock).unwrap(), None);
    assert_eq!(fcntl_ofd_getlk(&g, &write_lock).unwrap(), Some(read_lock));
    assert_eq!(fcntl_ofd_getlk(&f, &write_lock).unwrap(), None);

    fcntl_ofd_lock(&f, FlockOperation::LockExclusive).unwrap();
    assert_eq!(fcntl_ofd_getlk(&g, &read_lock).unwrap(), Some(write_lock));
    assert_eq!(fcntl_ofd_getlk(&g, &write_lock).unwrap(), Some(write_lock));
    assert_eq!(fcntl_getlk(&g, &write_lock).unwrap(), Some(write_lock));
    fcntl_ofd_lock(&f, FlockOperation::Unlock).unwrap();
    assert_eq!(fcntl_ofd_getlk(&g, &write_lock).unwrap(), None);

    fcntl_lock(&f, FlockOperation::LockExclusive).unwrap();
    let process_lock = Flock {
        pid: Some(getpid()),
        ..write_lock
    };
    assert_eq!(fcntl_ofd_getlk(&g, &read_lock).unwrap(), Some(process_lock));
    fcntl_lock(&f, FlockOperation::Unlock).unwrap();
}
