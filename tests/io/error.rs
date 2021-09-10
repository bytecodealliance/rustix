#[test]
fn test_error() {
    assert_eq!(
        rsix::io::Error::INVAL,
        rsix::io::Error::from_raw_os_error(rsix::io::Error::INVAL.raw_os_error())
    );
    assert_eq!(rsix::io::Error::INVAL.raw_os_error(), libc::EINVAL);
}
