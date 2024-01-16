use rustix::fd::AsRawFd;

#[test]
fn test_proc_self() {
    // Verify that this API works at all
    let fd = rustix::procfs::proc_self_fd().unwrap();
    assert_ne!(fd.as_raw_fd(), 0);
}

#[test]
fn test_status_twice() {
    let fd = rustix::procfs::proc_self_status().unwrap();
    drop(fd);
    let fd = rustix::procfs::proc_self_status().unwrap();
    drop(fd);
}
