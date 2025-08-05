// `ioctl_fionread` on Windows doesn't work on files.
#[cfg(not(any(windows, target_os = "cygwin", target_os = "haiku")))]
#[test]
fn test_ioctls() {
    let file = std::fs::File::open("Cargo.toml").unwrap();

    assert_eq!(
        rustix::io::ioctl_fionread(&file).unwrap(),
        file.metadata().unwrap().len()
    );
}

#[cfg(all(target_os = "linux", feature = "fs"))]
#[test]
fn test_int_setter() {
    use rustix::fs::{open, Mode, OFlags};
    use rustix::ioctl::{ioctl, IntegerSetter, Opcode};

    const TUNSETOFFLOAD: Opcode = 0x4004_54D0;

    let tun = open("/dev/net/tun", OFlags::RDWR, Mode::empty()).unwrap();

    // SAFETY: `TUNSETOFFLOAD` is defined for TUN.
    unsafe {
        let code = IntegerSetter::<TUNSETOFFLOAD>::new_usize(0);
        assert!(ioctl(&tun, code).is_err());
    }
}
