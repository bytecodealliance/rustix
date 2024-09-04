#[test]
fn test_termios_flush() {
    use rustix::pty::*;
    use rustix::termios::*;

    let pty = match openpt(OpenptFlags::empty()) {
        Ok(pty) => pty,
        Err(rustix::io::Errno::NOSYS) => return,
        Err(err) => panic!("{:?}", err),
    };
    let tio = match tcgetattr(&pty) {
        Ok(tio) => tio,
        Err(rustix::io::Errno::NOSYS) => return,
        #[cfg(apple)]
        Err(rustix::io::Errno::NOTTY) => return,
        Err(err) => panic!("{:?}", err),
    };
    tcsetattr(&pty, OptionalActions::Now, &tio).unwrap();

    tcflush(&pty, QueueSelector::IOFlush).unwrap();
}

#[test]
fn test_termios_drain() {
    use rustix::pty::*;
    use rustix::termios::*;

    let pty = match openpt(OpenptFlags::empty()) {
        Ok(pty) => pty,
        Err(rustix::io::Errno::NOSYS) => return,
        Err(err) => panic!("{:?}", err),
    };
    let tio = match tcgetattr(&pty) {
        Ok(tio) => tio,
        Err(rustix::io::Errno::NOSYS) => return,
        #[cfg(apple)]
        Err(rustix::io::Errno::NOTTY) => return,
        Err(err) => panic!("{:?}", err),
    };
    tcsetattr(&pty, OptionalActions::Now, &tio).unwrap();

    tcdrain(&pty).unwrap();
}

#[test]
fn test_termios_winsize() {
    use rustix::pty::*;
    use rustix::termios::*;

    let pty = match openpt(OpenptFlags::empty()) {
        Ok(pty) => pty,
        Err(rustix::io::Errno::NOSYS) => return,
        Err(err) => panic!("{:?}", err),
    };

    // Sizes for a pseudoterminal start out 0.
    let mut sizes = match tcgetwinsize(&pty) {
        Ok(sizes) => sizes,
        // Apple doesn't appear to support `tcgetwinsize` on a pty.
        #[cfg(apple)]
        Err(rustix::io::Errno::NOTTY) => return,
        Err(err) => panic!("{:?}", err),
    };
    assert_eq!(sizes.ws_row, 0);
    assert_eq!(sizes.ws_col, 0);
    assert_eq!(sizes.ws_xpixel, 0);
    assert_eq!(sizes.ws_ypixel, 0);

    // Set some arbitrary sizes.
    sizes.ws_row = 28;
    sizes.ws_col = 82;
    sizes.ws_xpixel = 365;
    sizes.ws_ypixel = 794;
    tcsetwinsize(&pty, sizes).unwrap();

    // Check that the sizes roundtripped.
    let check_sizes = tcgetwinsize(&pty).unwrap();
    assert_eq!(check_sizes.ws_row, sizes.ws_row);
    assert_eq!(check_sizes.ws_col, sizes.ws_col);
    assert_eq!(check_sizes.ws_xpixel, sizes.ws_xpixel);
    assert_eq!(check_sizes.ws_ypixel, sizes.ws_ypixel);
}

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
        Err(err) => panic!("{:?}", err),
    };
    let mut tio = match tcgetattr(&pty) {
        Ok(tio) => tio,
        Err(rustix::io::Errno::NOSYS) => return,
        #[cfg(apple)]
        Err(rustix::io::Errno::NOTTY) => return,
        Err(err) => panic!("{:?}", err),
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

    let new_tio = tcgetattr(&pty).unwrap();
    assert_eq!(new_tio.input_speed(), speed::B50);
    assert_eq!(new_tio.output_speed(), speed::B50);

    // Set it to 134 with `set_speed`.
    tio.set_speed(speed::B134).unwrap();
    assert_eq!(tio.input_speed(), speed::B134);
    assert_eq!(tio.output_speed(), speed::B134);
    tcsetattr(&pty, OptionalActions::Now, &tio).unwrap();

    let new_tio = tcgetattr(&pty).unwrap();
    assert_eq!(new_tio.input_speed(), speed::B134);
    assert_eq!(new_tio.output_speed(), speed::B134);

    // Check various speeds.
    for custom_speed in [speed::B50, speed::B19200, speed::B38400] {
        tio.set_speed(custom_speed).unwrap();
        assert_eq!(tio.input_speed(), custom_speed);
        assert_eq!(tio.output_speed(), custom_speed);
        tcsetattr(&pty, OptionalActions::Now, &tio).unwrap();

        let new_tio = tcgetattr(&pty).unwrap();
        assert_eq!(new_tio.input_speed(), custom_speed);
        assert_eq!(new_tio.output_speed(), custom_speed);
    }

    // Similar, but using `set_input_speed` and `set_output_speed` instead
    // of `set_speed`.
    for custom_speed in [speed::B50, speed::B19200, speed::B38400] {
        tio.set_input_speed(custom_speed).unwrap();
        tio.set_output_speed(custom_speed).unwrap();
        assert_eq!(tio.input_speed(), custom_speed);
        assert_eq!(tio.output_speed(), custom_speed);
        tcsetattr(&pty, OptionalActions::Now, &tio).unwrap();

        let new_tio = tcgetattr(&pty).unwrap();
        assert_eq!(new_tio.input_speed(), custom_speed);
        assert_eq!(new_tio.output_speed(), custom_speed);
    }

    // These platforms are known to support arbitrary not-pre-defined-by-POSIX
    // speeds.
    #[cfg(any(bsd, linux_kernel))]
    {
        for custom_speed in [speed::B50, 51, 31250, 74880, 256000] {
            tio.set_speed(custom_speed).unwrap();
            assert_eq!(tio.input_speed(), custom_speed);
            assert_eq!(tio.output_speed(), custom_speed);
            tcsetattr(&pty, OptionalActions::Now, &tio).unwrap();

            let new_tio = tcgetattr(&pty).unwrap();
            assert_eq!(new_tio.input_speed(), custom_speed);
            assert_eq!(new_tio.output_speed(), custom_speed);
        }

        // Similar, but using `set_input_speed` and `set_output_speed` instead
        // of `set_speed`.
        for custom_speed in [speed::B50, 51, 31250, 74880, 256000] {
            tio.set_input_speed(custom_speed).unwrap();
            tio.set_output_speed(custom_speed).unwrap();
            assert_eq!(tio.input_speed(), custom_speed);
            assert_eq!(tio.output_speed(), custom_speed);
            tcsetattr(&pty, OptionalActions::Now, &tio).unwrap();

            let new_tio = tcgetattr(&pty).unwrap();
            assert_eq!(new_tio.input_speed(), custom_speed);
            assert_eq!(new_tio.output_speed(), custom_speed);
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

        let new_tio = tcgetattr(&pty).unwrap();
        assert_eq!(new_tio.input_speed(), speed::B75);
        assert_eq!(new_tio.output_speed(), speed::B110);
    }

    // These platforms are known to support arbitrary not-pre-defined-by-POSIX
    // speeds that also differ between input and output.
    #[cfg(any(bsd, linux_kernel))]
    {
        tio.set_output_speed(speed::B134).unwrap();
        for custom_speed in [speed::B50, 51, 31250, 74880, 256000] {
            tio.set_input_speed(custom_speed).unwrap();
            assert_eq!(tio.input_speed(), custom_speed);
            assert_eq!(tio.output_speed(), speed::B134);
            tcsetattr(&pty, OptionalActions::Now, &tio).unwrap();

            let new_tio = tcgetattr(&pty).unwrap();
            assert_eq!(new_tio.input_speed(), custom_speed);
            assert_eq!(new_tio.output_speed(), speed::B134);
        }

        tio.set_input_speed(speed::B150).unwrap();
        for custom_speed in [speed::B50, 51, 31250, 74880, 256000] {
            tio.set_output_speed(custom_speed).unwrap();
            assert_eq!(tio.input_speed(), speed::B150);
            assert_eq!(tio.output_speed(), custom_speed);
            tcsetattr(&pty, OptionalActions::Now, &tio).unwrap();

            let new_tio = tcgetattr(&pty).unwrap();
            assert_eq!(new_tio.input_speed(), speed::B150);
            assert_eq!(new_tio.output_speed(), custom_speed);
        }
    }
}

#[test]
fn test_termios_tcgetattr_not_tty() {
    let file = tempfile::tempfile().unwrap();
    assert_eq!(
        rustix::termios::tcgetattr(&file).unwrap_err(),
        rustix::io::Errno::NOTTY
    );
}
