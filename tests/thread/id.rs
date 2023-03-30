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
fn test_setresuid() {
    let uid = process::getuid();
    thread::set_thread_res_uid(uid, uid, uid).unwrap();
}

#[test]
fn test_setgid() {
    thread::set_thread_gid(process::getgid()).unwrap();
}

#[test]
fn test_setresgid() {
    let gid = process::getgid();
    thread::set_thread_res_gid(gid, gid, gid).unwrap();
}
