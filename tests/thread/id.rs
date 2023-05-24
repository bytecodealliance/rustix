#[cfg(feature = "process")]
use rustix::process;
use rustix::thread;

#[test]
fn test_gettid() {
    assert_eq!(thread::gettid(), thread::gettid());
}

#[cfg(feature = "process")]
#[test]
fn test_setuid() {
    thread::set_thread_uid(process::getuid()).unwrap();
}

#[cfg(feature = "process")]
#[test]
fn test_setresuid() {
    let uid = process::getuid();
    thread::set_thread_res_uid(uid, uid, uid).unwrap();
}

#[cfg(feature = "process")]
#[test]
fn test_setgid() {
    thread::set_thread_gid(process::getgid()).unwrap();
}

#[cfg(feature = "process")]
#[test]
fn test_setresgid() {
    let gid = process::getgid();
    thread::set_thread_res_gid(gid, gid, gid).unwrap();
}
