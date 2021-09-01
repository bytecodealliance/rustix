#[test]
fn test_uname() {
    let name: rsix::process::Uname = rsix::process::uname();

    assert!(!name.sysname().is_empty());
    assert!(!name.nodename().is_empty());
    assert!(!name.release().is_empty());
    assert!(!name.version().is_empty());
    assert!(!name.machine().is_empty());

    #[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
    assert!(!name.domainname().is_empty());
}
