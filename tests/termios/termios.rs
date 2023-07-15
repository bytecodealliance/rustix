// Disable on illumos where `tcgetattr` doesn't appear to support
// pseudoterminals.
#[cfg(not(target_os = "illumos"))]
#[test]
fn test_termios_speeds() {
    use rustix::pty::*;
    use rustix::termios::*;

    let pty = match openpt(OpenptFlags::empty()) {
        Ok(pty) => pty,
        Err(rustix::io::Errno::NOSYS) => return,
        Err(e) => Err(e).unwrap(),
    };
    let mut tio = match tcgetattr(&pty) {
        Ok(tio) => tio,
        Err(rustix::io::Errno::NOSYS) => return,
        #[cfg(apple)]
        Err(rustix::io::Errno::NOTTY) => return,
        Err(e) => Err(e).unwrap(),
    };

    // Assume it doesn't default to 50, and then set it to 50.
    assert_eq!(speed::B50, 50);
    assert_ne!(tio.input_speed(), speed::B50);
    assert_ne!(tio.output_speed(), speed::B50);
    tio.set_input_speed(speed::B50).unwrap();
    tio.set_output_speed(speed::B50).unwrap();
    assert_eq!(tio.input_speed(), speed::B50);
    assert_eq!(tio.output_speed(), speed::B50);
    tcsetattr(&pty, OptionalActions::Now, &tio).unwrap();

    #[allow(unused_variables)]
    let new_tio = tcgetattr(&pty).unwrap();

    // QEMU appears to have a bug on PowerPC. On PowerPC, there is no
    // `TCSETS2` ioctl, and the `TCSETS` ioctl has the behavior of
    // `TCSETS2`. QEMU doesn't appear to know this, and it gives `TCSETS`
    // the old `TCSETS` behavior.
    #[cfg(not(all(linux_kernel, any(target_arch = "powerpc", target_arch = "powerpc64"))))]
    {
        assert_eq!(new_tio.input_speed(), speed::B50);
        assert_eq!(new_tio.output_speed(), speed::B50);
    }

    // Set it to 134 with `set_speed`.
    tio.set_speed(speed::B134).unwrap();
    assert_eq!(tio.input_speed(), speed::B134);
    assert_eq!(tio.output_speed(), speed::B134);
    tcsetattr(&pty, OptionalActions::Now, &tio).unwrap();

    #[allow(unused_variables)]
    let new_tio = tcgetattr(&pty).unwrap();

    #[cfg(not(all(linux_kernel, any(target_arch = "powerpc", target_arch = "powerpc64"))))]
    {
        assert_eq!(new_tio.input_speed(), speed::B134);
        assert_eq!(new_tio.output_speed(), speed::B134);
    }

    // These platforms are known to support arbitrary not-pre-defined-by-POSIX
    // speeds.
    #[cfg(any(bsd, linux_kernel))]
    {
        tio.set_input_speed(51).unwrap();
        tio.set_output_speed(51).unwrap();
        assert_eq!(tio.input_speed(), 51);
        assert_eq!(tio.output_speed(), 51);
        tcsetattr(&pty, OptionalActions::Now, &tio).unwrap();

        #[allow(unused_variables)]
        let new_tio = tcgetattr(&pty).unwrap();

        #[cfg(not(all(linux_kernel, any(target_arch = "powerpc", target_arch = "powerpc64"))))]
        {
            assert_eq!(new_tio.input_speed(), 51);
            assert_eq!(new_tio.output_speed(), 51);
        }
    }

    // These platforms are known to support differing input and output speeds.
    #[cfg(any(bsd, linux_kernel))]
    {
        tio.set_input_speed(speed::B75).unwrap();
        tio.set_output_speed(speed::B110).unwrap();
        assert_eq!(tio.input_speed(), speed::B75);
        assert_eq!(tio.output_speed(), speed::B110);
        tcsetattr(&pty, OptionalActions::Now, &tio).unwrap();

        #[allow(unused_variables)]
        let new_tio = tcgetattr(&pty).unwrap();

        #[cfg(not(all(linux_kernel, any(target_arch = "powerpc", target_arch = "powerpc64"))))]
        {
            assert_eq!(new_tio.input_speed(), speed::B75);
            assert_eq!(new_tio.output_speed(), speed::B110);
        }
    }
}
