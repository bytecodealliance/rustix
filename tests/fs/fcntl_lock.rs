#[test]
fn test_fcntl_lock() {
    use rustix::fs::{fcntl_lock, FlockOperation};

    let f = tempfile::tempfile().unwrap();
    fcntl_lock(&f, FlockOperation::LockExclusive).unwrap();
    fcntl_lock(&f, FlockOperation::Unlock).unwrap();
    let g = tempfile::tempfile().unwrap();
    fcntl_lock(&g, FlockOperation::LockExclusive).unwrap();
    fcntl_lock(&g, FlockOperation::Unlock).unwrap();
    drop(f);
    drop(g);

    let f = tempfile::tempfile().unwrap();
    fcntl_lock(&f, FlockOperation::LockShared).unwrap();
    let g = tempfile::tempfile().unwrap();
    fcntl_lock(&g, FlockOperation::LockShared).unwrap();
    fcntl_lock(&f, FlockOperation::Unlock).unwrap();
    fcntl_lock(&g, FlockOperation::Unlock).unwrap();
    drop(f);
    drop(g);

    let f = tempfile::tempfile().unwrap();
    fcntl_lock(&f, FlockOperation::LockShared).unwrap();
    fcntl_lock(&f, FlockOperation::LockExclusive).unwrap();
    fcntl_lock(&f, FlockOperation::Unlock).unwrap();
    let g = tempfile::tempfile().unwrap();
    fcntl_lock(&g, FlockOperation::LockShared).unwrap();
    fcntl_lock(&g, FlockOperation::LockExclusive).unwrap();
    fcntl_lock(&g, FlockOperation::Unlock).unwrap();
    drop(f);
    drop(g);
}
