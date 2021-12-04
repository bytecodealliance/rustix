use rustix::process;

#[test]
fn test_getuid() {
    assert_eq!(process::getuid(), process::getuid());
}

#[test]
fn test_getgid() {
    assert_eq!(process::getgid(), process::getgid());
}

#[test]
fn test_geteuid() {
    assert_eq!(process::geteuid(), process::geteuid());
}

#[test]
fn test_getegid() {
    assert_eq!(process::getegid(), process::getegid());
}

#[test]
fn test_getpid() {
    assert_eq!(process::getpid(), process::getpid());
}

#[test]
fn test_getppid() {
    assert_eq!(process::getppid(), process::getppid());
}
