#[test]
fn test_error() {
    assert_eq!(
        rsix::io::Error::INVAL,
        rsix::io::Error::from_raw_os_error(rsix::io::Error::INVAL.raw_os_error())
    );
    #[cfg(not(windows))]
    assert_eq!(rsix::io::Error::INVAL.raw_os_error(), libc::EINVAL);
    #[cfg(windows)]
    assert_eq!(
        rsix::io::Error::INVAL.raw_os_error(),
        winapi::um::winsock2::WSAEINVAL
    );
}
