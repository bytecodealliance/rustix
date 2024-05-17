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

#[test]
fn parallel_self_proc_status() {
    const THREADS: usize = 3;

    fn self_proc_status() {
        rustix::procfs::proc_self_status().expect("error getting proc/self/status pid");
    }

    let mut handles = Vec::with_capacity(THREADS);
    for _ in 0..THREADS {
        handles.push(std::thread::spawn(self_proc_status));
    }
    for handle in handles.drain(..) {
        handle.join().expect("thread crashed");
    }
}
