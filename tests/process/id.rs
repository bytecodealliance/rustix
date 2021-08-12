use rsix::process;
use std::env::var;
use std::str::FromStr;

#[test]
fn test_getuid() {
    assert_eq!(process::getuid(), process::getuid());
    if let Ok(uid) = var("UID") {
        assert_eq!(u32::from_str(&uid).unwrap(), process::getuid());
    }
}

#[test]
fn test_getgid() {
    assert_eq!(process::getgid(), process::getgid());
}

#[test]
fn test_geteuid() {
    assert_eq!(process::geteuid(), process::geteuid());
    if let Ok(euid) = var("EUID") {
        assert_eq!(u32::from_str(&euid).unwrap(), process::geteuid());
    }
}

#[test]
fn test_getegid() {
    assert_eq!(process::getegid(), process::getegid());
}

#[test]
fn test_getpid() {
    assert_ne!(process::getpid(), 0);
    assert_eq!(process::getpid(), process::getpid());
}

#[test]
fn test_getppid() {
    assert_eq!(process::getppid(), process::getppid());
}
