// `is_read_write` is not yet implemented on Windows. And `ioctl_fionread`
// on Windows doesn't work on files.
#[cfg(not(windows))]
#[test]
fn test_ioctls() {
    let file = std::fs::File::open("Cargo.toml").unwrap();

    #[cfg(all(feature = "fs", feature = "net"))]
    assert_eq!(rustix::io::is_read_write(&file).unwrap(), (true, false));

    assert_eq!(
        rustix::io::ioctl_fionread(&file).unwrap(),
        file.metadata().unwrap().len()
    );
}

#[cfg(all(target_os = "linux", feature = "fs"))]
#[test]
fn test_int_setter() {
    use rustix::fs::{open, Mode, OFlags};
    use rustix::ioctl::{ioctl, BadOpcode, IntegerSetter, RawOpcode};

    const TUNSETOFFLOAD: RawOpcode = 0x400454D0;

    let tun = open("/dev/net/tun", OFlags::RDWR, Mode::empty()).unwrap();

    // SAFETY: TUNSETOFFLOAD is defined for TUN.
    unsafe {
        let code = IntegerSetter::<BadOpcode<{ TUNSETOFFLOAD }>>::new(0);
        assert!(ioctl(&tun, code).is_err());
    }
}
