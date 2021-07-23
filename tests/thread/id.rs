use rsix::thread;

#[cfg(any(target_os = "android", target_os = "linux"))]
#[test]
fn test_gettid() {
    assert_ne!(thread::gettid(), 0);
    assert_eq!(thread::gettid(), thread::gettid());
}
