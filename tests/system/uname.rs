#[test]
fn test_uname() {
    let name: rustix::system::Uname = rustix::system::uname();

    assert!(!name.sysname().to_bytes().is_empty());
    assert!(!name.nodename().to_bytes().is_empty());
    assert!(!name.release().to_bytes().is_empty());
    assert!(!name.version().to_bytes().is_empty());
    assert!(!name.machine().to_bytes().is_empty());

    #[cfg(linux_kernel)]
    assert!(!name.domainname().to_bytes().is_empty());

    #[cfg(linux_kernel)]
    assert_eq!(name.sysname(), rustix::cstr!("Linux"));
}
