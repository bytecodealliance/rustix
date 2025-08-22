#[cfg(not(any(target_os = "redox", target_os = "solaris")))]
#[test]
fn test_flock() {
    use rustix::fs::{flock, openat, FlockOperation, Mode, OFlags, CWD};

    let tf = tempfile::NamedTempFile::new().unwrap();
    let f = openat(CWD, tf.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    flock(&f, FlockOperation::LockExclusive).unwrap();
    flock(&f, FlockOperation::Unlock).unwrap();
    let g = openat(CWD, tf.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    flock(&g, FlockOperation::LockExclusive).unwrap();
    flock(&g, FlockOperation::Unlock).unwrap();
    drop(f);
    drop(g);

    let f = openat(CWD, tf.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    flock(&f, FlockOperation::LockShared).unwrap();
    let g = openat(CWD, tf.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    flock(&g, FlockOperation::LockShared).unwrap();
    flock(&f, FlockOperation::Unlock).unwrap();
    flock(&g, FlockOperation::Unlock).unwrap();
    drop(f);
    drop(g);

    let f = openat(CWD, tf.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    flock(&f, FlockOperation::LockShared).unwrap();
    flock(&f, FlockOperation::LockExclusive).unwrap();
    flock(&f, FlockOperation::Unlock).unwrap();
    let g = openat(CWD, tf.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    flock(&g, FlockOperation::LockShared).unwrap();
    flock(&g, FlockOperation::LockExclusive).unwrap();
    flock(&g, FlockOperation::Unlock).unwrap();
    drop(f);
    drop(g);
}
