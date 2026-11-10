use rustix::fs::{fcntl_lock, fcntl_ofd_lock, FlockOperation};
use rustix::io::Errno;

#[test]
fn test_fcntl_ofd_lock() {
    let f = tempfile::tempfile().unwrap();
    fcntl_ofd_lock(&f, FlockOperation::LockExclusive).unwrap();
    fcntl_ofd_lock(&f, FlockOperation::Unlock).unwrap();
    let g = tempfile::tempfile().unwrap();
    fcntl_ofd_lock(&g, FlockOperation::LockExclusive).unwrap();
    fcntl_ofd_lock(&g, FlockOperation::Unlock).unwrap();

    fcntl_ofd_lock(&f, FlockOperation::LockShared).unwrap();
    fcntl_ofd_lock(&g, FlockOperation::LockShared).unwrap();
    fcntl_ofd_lock(&f, FlockOperation::LockExclusive).unwrap();
    fcntl_ofd_lock(&g, FlockOperation::LockExclusive).unwrap();
    fcntl_ofd_lock(&f, FlockOperation::Unlock).unwrap();
    fcntl_ofd_lock(&g, FlockOperation::Unlock).unwrap();
}

#[test]
fn test_fcntl_ofd_lock_nonblocking() {
    let f = tempfile::NamedTempFile::new().unwrap();
    let g = f.reopen().unwrap();

    fcntl_ofd_lock(&f, FlockOperation::NonBlockingLockShared).unwrap();
    fcntl_ofd_lock(&g, FlockOperation::NonBlockingLockShared).unwrap();
    assert_eq!(
        fcntl_ofd_lock(&f, FlockOperation::NonBlockingLockExclusive),
        Err(Errno::AGAIN)
    );
    fcntl_ofd_lock(&g, FlockOperation::NonBlockingUnlock).unwrap();

    fcntl_ofd_lock(&f, FlockOperation::NonBlockingLockExclusive).unwrap();
    assert_eq!(
        fcntl_ofd_lock(&g, FlockOperation::NonBlockingLockShared),
        Err(Errno::AGAIN)
    );
    assert_eq!(
        fcntl_ofd_lock(&g, FlockOperation::NonBlockingLockExclusive),
        Err(Errno::AGAIN)
    );
    fcntl_ofd_lock(&f, FlockOperation::NonBlockingUnlock).unwrap();

    fcntl_ofd_lock(&g, FlockOperation::NonBlockingLockExclusive).unwrap();
    fcntl_ofd_lock(&g, FlockOperation::NonBlockingLockShared).unwrap();
    fcntl_ofd_lock(&g, FlockOperation::NonBlockingUnlock).unwrap();
}

#[test]
fn test_fcntl_ofd_lock_conflicts_with_fcntl_lock() {
    let f = tempfile::tempfile().unwrap();

    fcntl_lock(&f, FlockOperation::LockExclusive).unwrap();
    assert_eq!(
        fcntl_ofd_lock(&f, FlockOperation::NonBlockingLockExclusive),
        Err(Errno::AGAIN)
    );
    fcntl_lock(&f, FlockOperation::Unlock).unwrap();

    fcntl_ofd_lock(&f, FlockOperation::LockExclusive).unwrap();
    assert_eq!(
        fcntl_lock(&f, FlockOperation::NonBlockingLockExclusive),
        Err(Errno::AGAIN)
    );
    fcntl_ofd_lock(&f, FlockOperation::Unlock).unwrap();
}
