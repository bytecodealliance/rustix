#[test]
fn test_fcntl_getlk() {
    use rustix::fs::{fcntl_getlk, flock, FlockOperation};

    let f = tempfile::tempfile().unwrap();
    flock(&f, FlockOperation::LockExclusive).unwrap();
    assert!(fcntl_getlk(&f).unwrap());
    flock(&f, FlockOperation::Unlock).unwrap();
    assert!(!fcntl_getlk(&f).unwrap());
    let g = tempfile::tempfile().unwrap();
    flock(&g, FlockOperation::LockExclusive).unwrap();
    assert!(fcntl_getlk(&g).unwrap());
    flock(&g, FlockOperation::Unlock).unwrap();
    assert!(!fcntl_getlk(&g).unwrap());

    // when a file is locked shared, the getlk function wiil return l_type = F_UNLCK
    let f = tempfile::tempfile().unwrap();
    flock(&f, FlockOperation::LockShared).unwrap();
    assert!(!fcntl_getlk(&f).unwrap());
    flock(&f, FlockOperation::Unlock).unwrap();
    assert!(!fcntl_getlk(&f).unwrap());
    let g = tempfile::tempfile().unwrap();
    flock(&g, FlockOperation::LockShared).unwrap();
    assert!(!fcntl_getlk(&g).unwrap());
    flock(&g, FlockOperation::Unlock).unwrap();
    assert!(!fcntl_getlk(&g).unwrap());
}
