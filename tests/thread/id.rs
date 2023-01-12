use rustix::{process, thread};

#[test]
fn test_gettid() {
    assert_eq!(thread::gettid(), thread::gettid());
}

#[test]
fn test_setuid() {
    thread::set_thread_uid(process::getuid()).unwrap();
}

#[test]
fn test_setgid() {
    thread::set_thread_gid(process::getgid()).unwrap();
}
