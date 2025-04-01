//! A command which prints out information about the standard input, output,
//! and error streams provided to it.

#[cfg(feature = "termios")]
#[cfg(all(not(windows), feature = "stdio"))]
use rustix::termios::isatty;
#[cfg(all(
    not(any(windows, target_os = "fuchsia")),
    feature = "fs",
    feature = "termios",
    feature = "alloc"
))]
use rustix::termios::ttyname;
#[cfg(all(not(windows), feature = "stdio"))]
use {
    rustix::fd::AsFd,
    rustix::io,
    rustix::stdio::{stderr, stdin, stdout},
};

#[cfg(all(not(windows), feature = "stdio"))]
fn main() -> io::Result<()> {
    let (stdin, stdout, stderr) = (stdin(), stdout(), stderr());

    println!("Stdin:");
    show(stdin)?;

    println!("Stdout:");
    show(stdout)?;

    println!("Stderr:");
    show(stderr)?;

    Ok(())
}

#[cfg(all(not(windows), feature = "stdio"))]
fn show<Fd: AsFd>(fd: Fd) -> io::Result<()> {
    let fd = fd.as_fd();

    #[cfg(not(target_os = "espidf"))]
    println!(" - ready bytes: {:?}", rustix::io::ioctl_fionread(fd)?);

    #[cfg(feature = "termios")]
    if isatty(fd) {
        #[cfg(feature = "alloc")]
        #[cfg(feature = "fs")]
        #[cfg(not(target_os = "fuchsia"))]
        println!(" - ttyname: {}", ttyname(fd, Vec::new())?.to_string_lossy());

        #[cfg(target_os = "wasi")]
        println!(" - is a tty");

        #[cfg(not(target_os = "wasi"))]
        println!(" - process group: {:?}", rustix::termios::tcgetpgrp(fd)?);

        #[cfg(not(any(target_os = "espidf", target_os = "wasi")))]
        println!(" - winsize: {:?}", rustix::termios::tcgetwinsize(fd)?);

        #[cfg(not(any(target_os = "espidf", target_os = "wasi")))]
        {
            use rustix::termios::*;
            let term = tcgetattr(fd)?;

            println!(" - input_speed: {}", term.input_speed());
            println!(" - output_speed: {}", term.output_speed());

            print!(" - in flags:");
            if term.input_modes.contains(InputModes::IGNBRK) {
                print!(" IGNBRK");
            }
            if term.input_modes.contains(InputModes::BRKINT) {
                print!(" BRKINT");
            }
            if term.input_modes.contains(InputModes::IGNPAR) {
                print!(" IGNPAR");
            }
            if term.input_modes.contains(InputModes::PARMRK) {
                print!(" PARMRK");
            }
            if term.input_modes.contains(InputModes::INPCK) {
                print!(" INPCK");
            }
            if term.input_modes.contains(InputModes::ISTRIP) {
                print!(" ISTRIP");
            }
            if term.input_modes.contains(InputModes::INLCR) {
                print!(" INLCR");
            }
            if term.input_modes.contains(InputModes::IGNCR) {
                print!(" IGNCR");
            }
            if term.input_modes.contains(InputModes::ICRNL) {
                print!(" ICRNL");
            }
            #[cfg(any(
                linux_kernel,
                solarish,
                target_os = "aix",
                target_os = "haiku",
                target_os = "nto"
            ))]
            if term.input_modes.contains(InputModes::IUCLC) {
                print!(" IUCLC");
            }
            if term.input_modes.contains(InputModes::IXON) {
                print!(" IXON");
            }
            #[cfg(not(target_os = "redox"))]
            if term.input_modes.contains(InputModes::IXANY) {
                print!(" IXANY");
            }
            if term.input_modes.contains(InputModes::IXOFF) {
                print!(" IXOFF");
            }
            #[cfg(not(any(target_os = "haiku", target_os = "redox")))]
            if term.input_modes.contains(InputModes::IMAXBEL) {
                print!(" IMAXBEL");
            }
            #[cfg(not(any(
                freebsdlike,
                netbsdlike,
                solarish,
                target_os = "aix",
                target_os = "emscripten",
                target_os = "haiku",
                target_os = "hurd",
                target_os = "redox",
            )))]
            if term.input_modes.contains(InputModes::IUTF8) {
                print!(" IUTF8");
            }
            println!();

            print!(" - out flags:");
            if term.output_modes.contains(OutputModes::OPOST) {
                print!(" OPOST");
            }
            #[cfg(not(any(
                apple,
                freebsdlike,
                target_os = "aix",
                target_os = "netbsd",
                target_os = "redox"
            )))]
            if term.output_modes.contains(OutputModes::OLCUC) {
                print!(" OLCUC");
            }
            if term.output_modes.contains(OutputModes::ONLCR) {
                print!(" ONLCR");
            }
            if term.output_modes.contains(OutputModes::OCRNL) {
                print!(" OCRNL");
            }
            if term.output_modes.contains(OutputModes::ONOCR) {
                print!(" ONOCR");
            }
            if term.output_modes.contains(OutputModes::ONLRET) {
                print!(" ONLRET");
            }
            #[cfg(not(bsd))]
            if term.output_modes.contains(OutputModes::OFILL) {
                print!(" OFILL");
            }
            #[cfg(not(bsd))]
            if term.output_modes.contains(OutputModes::OFDEL) {
                print!(" OFDEL");
            }
            #[cfg(not(any(bsd, solarish, target_os = "redox")))]
            if term.output_modes.contains(OutputModes::NLDLY) {
                print!(" NLDLY");
            }
            #[cfg(not(any(bsd, solarish, target_os = "aix", target_os = "redox")))]
            if term.output_modes.contains(OutputModes::CRDLY) {
                print!(" CRDLY");
            }
            #[cfg(not(any(netbsdlike, solarish, target_os = "dragonfly", target_os = "redox")))]
            if term.output_modes.contains(OutputModes::TABDLY) {
                print!(" TABDLY");
            }
            #[cfg(not(any(bsd, solarish, target_os = "redox")))]
            if term.output_modes.contains(OutputModes::BSDLY) {
                print!(" BSDLY");
            }
            #[cfg(not(any(bsd, solarish, target_os = "redox")))]
            if term.output_modes.contains(OutputModes::VTDLY) {
                print!(" VTDLY");
            }
            #[cfg(not(any(bsd, solarish, target_os = "redox")))]
            if term.output_modes.contains(OutputModes::FFDLY) {
                print!(" FFDLY");
            }
            println!();

            print!(" - control flags:");
            if term.control_modes.contains(ControlModes::CSIZE) {
                print!(" CSIZE");
            }
            if term.control_modes.contains(ControlModes::CSTOPB) {
                print!(" CSTOPB");
            }
            if term.control_modes.contains(ControlModes::CREAD) {
                print!(" CREAD");
            }
            if term.control_modes.contains(ControlModes::PARENB) {
                print!(" PARENB");
            }
            if term.control_modes.contains(ControlModes::PARODD) {
                print!(" PARODD");
            }
            if term.control_modes.contains(ControlModes::HUPCL) {
                print!(" HUPCL");
            }
            if term.control_modes.contains(ControlModes::CLOCAL) {
                print!(" CLOCAL");
            }
            #[cfg(not(any(
                bsd,
                solarish,
                target_os = "emscripten",
                target_os = "haiku",
                target_os = "hurd",
                target_os = "redox",
            )))]
            if term.control_modes.contains(ControlModes::CMSPAR) {
                print!(" CMSPAR");
            }
            #[cfg(not(any(target_os = "aix", target_os = "redox")))]
            if term.control_modes.contains(ControlModes::CRTSCTS) {
                print!(" CRTSCTS");
            }
            println!();

            print!(" - local flags:");
            if term.local_modes.contains(LocalModes::ISIG) {
                print!(" ISIG");
            }
            if term.local_modes.contains(LocalModes::ICANON) {
                print!(" ICANON");
            }
            #[cfg(any(linux_kernel, target_arch = "s390x", target_os = "haiku"))]
            if term.local_modes.contains(LocalModes::XCASE) {
                print!(" XCASE");
            }
            if term.local_modes.contains(LocalModes::ECHO) {
                print!(" ECHO");
            }
            if term.local_modes.contains(LocalModes::ECHOE) {
                print!(" ECHOE");
            }
            if term.local_modes.contains(LocalModes::ECHOK) {
                print!(" ECHOK");
            }
            if term.local_modes.contains(LocalModes::ECHONL) {
                print!(" ECHONL");
            }
            #[cfg(not(any(target_os = "redox")))]
            if term.local_modes.contains(LocalModes::ECHOCTL) {
                print!(" ECHOCTL");
            }
            #[cfg(not(any(target_os = "cygwin", target_os = "redox")))]
            if term.local_modes.contains(LocalModes::ECHOPRT) {
                print!(" ECHOPRT");
            }
            #[cfg(not(any(target_os = "redox")))]
            if term.local_modes.contains(LocalModes::ECHOKE) {
                print!(" ECHOKE");
            }
            #[cfg(not(any(target_os = "redox")))]
            if term.local_modes.contains(LocalModes::FLUSHO) {
                print!(" FLUSHO");
            }
            if term.local_modes.contains(LocalModes::NOFLSH) {
                print!(" NOFLSH");
            }
            if term.local_modes.contains(LocalModes::TOSTOP) {
                print!(" TOSTOP");
            }
            #[cfg(not(any(target_os = "cygwin", target_os = "redox")))]
            if term.local_modes.contains(LocalModes::PENDIN) {
                print!(" PENDIN");
            }
            if term.local_modes.contains(LocalModes::IEXTEN) {
                print!(" IEXTEN");
            }
            println!();

            println!(
                " - keys: INTR={} QUIT={} ERASE={} KILL={} EOF={} TIME={} MIN={} ",
                key(term.special_codes[SpecialCodeIndex::VINTR]),
                key(term.special_codes[SpecialCodeIndex::VQUIT]),
                key(term.special_codes[SpecialCodeIndex::VERASE]),
                key(term.special_codes[SpecialCodeIndex::VKILL]),
                key(term.special_codes[SpecialCodeIndex::VEOF]),
                term.special_codes[SpecialCodeIndex::VTIME],
                term.special_codes[SpecialCodeIndex::VMIN]
            );
            #[cfg(not(any(
                bsd,
                solarish,
                target_os = "aix",
                target_os = "haiku",
                target_os = "hurd",
                target_os = "nto",
            )))]
            println!(
                "         VSWTC={}",
                term.special_codes[SpecialCodeIndex::VSWTC]
            );
            println!(
                "         START={} STOP={} SUSP={} EOL={}",
                key(term.special_codes[SpecialCodeIndex::VSTART]),
                key(term.special_codes[SpecialCodeIndex::VSTOP]),
                key(term.special_codes[SpecialCodeIndex::VSUSP]),
                key(term.special_codes[SpecialCodeIndex::VEOL]),
            );
            #[cfg(not(target_os = "haiku"))]
            println!(
                "         REPRINT={} DISCARD={}",
                key(term.special_codes[SpecialCodeIndex::VREPRINT]),
                key(term.special_codes[SpecialCodeIndex::VDISCARD])
            );
            #[cfg(not(target_os = "haiku"))]
            println!(
                "         WERASE={} VLNEXT={}",
                key(term.special_codes[SpecialCodeIndex::VWERASE]),
                key(term.special_codes[SpecialCodeIndex::VLNEXT]),
            );
            println!(
                "         EOL2={}",
                key(term.special_codes[SpecialCodeIndex::VEOL2])
            );
        }
    } else {
        println!(" - is not a tty");
    }

    println!();
    Ok(())
}

#[cfg(feature = "termios")]
#[cfg(all(not(target_os = "espidf"), not(windows), feature = "stdio"))]
fn key(b: u8) -> String {
    if b == 0 {
        "<undef>".to_string()
    } else if b < 0x20 {
        format!("^{}", (b + 0x40) as char)
    } else if b == 0x7f {
        "^?".to_string()
    } else if b >= 0x80 {
        format!("M-{}", key(b - 0x80))
    } else {
        format!("{}", b as char)
    }
}

#[cfg(any(windows, not(feature = "stdio")))]
fn main() -> Result<(), &'static str> {
    Err("This example requires --features=stdio and is not supported on Windows.")
}
