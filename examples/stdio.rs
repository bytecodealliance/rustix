//! A command which prints out information about the standard input,
//! output, and error streams provided to it.

#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]

#[cfg(feature = "termios")]
#[cfg(all(not(windows), feature = "stdio"))]
use rustix::termios::isatty;
#[cfg(all(
    not(any(windows, target_os = "fuchsia")),
    feature = "termios",
    feature = "procfs"
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
    println!(" - ready: {:?}", rustix::io::ioctl_fionread(fd)?);

    #[cfg(feature = "termios")]
    if isatty(fd) {
        #[cfg(feature = "procfs")]
        #[cfg(not(target_os = "fuchsia"))]
        println!(" - ttyname: {}", ttyname(fd, Vec::new())?.to_string_lossy());

        #[cfg(target_os = "wasi")]
        println!(" - is a tty");

        #[cfg(not(target_os = "wasi"))]
        println!(" - process group: {:?}", rustix::termios::tcgetpgrp(fd)?);

        #[cfg(not(target_os = "wasi"))]
        println!(" - winsize: {:?}", rustix::termios::tcgetwinsize(fd)?);

        #[cfg(not(target_os = "wasi"))]
        {
            use rustix::termios::*;
            let term = tcgetattr(fd)?;

            if let Some(speed) = speed_value(cfgetispeed(&term)) {
                println!(" - ispeed: {}", speed);
            }
            if let Some(speed) = speed_value(cfgetospeed(&term)) {
                println!(" - ospeed: {}", speed);
            }

            print!(" - in flags:");
            if (term.c_iflag & IGNBRK) != 0 {
                print!(" IGNBRK");
            }
            if (term.c_iflag & BRKINT) != 0 {
                print!(" BRKINT");
            }
            if (term.c_iflag & IGNPAR) != 0 {
                print!(" IGNPAR");
            }
            if (term.c_iflag & PARMRK) != 0 {
                print!(" PARMRK");
            }
            if (term.c_iflag & INPCK) != 0 {
                print!(" INPCK");
            }
            if (term.c_iflag & ISTRIP) != 0 {
                print!(" ISTRIP");
            }
            if (term.c_iflag & INLCR) != 0 {
                print!(" INLCR");
            }
            if (term.c_iflag & IGNCR) != 0 {
                print!(" IGNCR");
            }
            if (term.c_iflag & ICRNL) != 0 {
                print!(" ICRNL");
            }
            #[cfg(any(linux_raw, all(libc, any(solarish, target_os = "haiku"))))]
            if (term.c_iflag & IUCLC) != 0 {
                print!(" IUCLC");
            }
            if (term.c_iflag & IXON) != 0 {
                print!(" IXON");
            }
            #[cfg(not(target_os = "redox"))]
            if (term.c_iflag & IXANY) != 0 {
                print!(" IXANY");
            }
            if (term.c_iflag & IXOFF) != 0 {
                print!(" IXOFF");
            }
            #[cfg(not(any(target_os = "haiku", target_os = "redox")))]
            if (term.c_iflag & IMAXBEL) != 0 {
                print!(" IMAXBEL");
            }
            #[cfg(not(any(
                freebsdlike,
                netbsdlike,
                solarish,
                target_os = "aix",
                target_os = "emscripten",
                target_os = "haiku",
                target_os = "redox",
            )))]
            if (term.c_iflag & IUTF8) != 0 {
                print!(" IUTF8");
            }
            println!();

            print!(" - out flags:");
            if (term.c_oflag & OPOST) != 0 {
                print!(" OPOST");
            }
            #[cfg(not(any(
                apple,
                freebsdlike,
                target_os = "aix",
                target_os = "netbsd",
                target_os = "redox"
            )))]
            if (term.c_oflag & OLCUC) != 0 {
                print!(" OLCUC");
            }
            if (term.c_oflag & ONLCR) != 0 {
                print!(" ONLCR");
            }
            if (term.c_oflag & OCRNL) != 0 {
                print!(" OCRNL");
            }
            if (term.c_oflag & ONOCR) != 0 {
                print!(" ONOCR");
            }
            if (term.c_oflag & ONLRET) != 0 {
                print!(" ONLRET");
            }
            #[cfg(not(bsd))]
            if (term.c_oflag & OFILL) != 0 {
                print!(" OFILL");
            }
            #[cfg(not(bsd))]
            if (term.c_oflag & OFDEL) != 0 {
                print!(" OFDEL");
            }
            #[cfg(not(any(bsd, solarish, target_os = "redox")))]
            if (term.c_oflag & NLDLY) != 0 {
                print!(" NLDLY");
            }
            #[cfg(not(any(bsd, solarish, target_os = "aix", target_os = "redox")))]
            if (term.c_oflag & CRDLY) != 0 {
                print!(" CRDLY");
            }
            #[cfg(not(any(netbsdlike, solarish, target_os = "dragonfly", target_os = "redox")))]
            if (term.c_oflag & TABDLY) != 0 {
                print!(" TABDLY");
            }
            #[cfg(not(any(bsd, solarish, target_os = "redox")))]
            if (term.c_oflag & BSDLY) != 0 {
                print!(" BSDLY");
            }
            #[cfg(not(any(all(libc, target_env = "musl"), bsd, solarish, target_os = "redox")))]
            if (term.c_oflag & VTDLY) != 0 {
                print!(" VTDLY");
            }
            #[cfg(not(any(all(libc, target_env = "musl"), bsd, solarish, target_os = "redox")))]
            if (term.c_oflag & FFDLY) != 0 {
                print!(" FFDLY");
            }
            println!();

            print!(" - control flags:");
            #[cfg(not(any(bsd, target_os = "haiku", target_os = "redox")))]
            if (term.c_cflag & CBAUD) != 0 {
                print!(" CBAUD");
            }
            #[cfg(not(any(
                bsd,
                solarish,
                target_os = "aix",
                target_os = "haiku",
                target_os = "redox"
            )))]
            if (term.c_cflag & CBAUDEX) != 0 {
                print!(" CBAUDEX");
            }
            if (term.c_cflag & CSIZE) != 0 {
                print!(" CSIZE");
            }
            if (term.c_cflag & CSTOPB) != 0 {
                print!(" CSTOPB");
            }
            if (term.c_cflag & CREAD) != 0 {
                print!(" CREAD");
            }
            if (term.c_cflag & PARENB) != 0 {
                print!(" PARENB");
            }
            if (term.c_cflag & PARODD) != 0 {
                print!(" PARODD");
            }
            if (term.c_cflag & HUPCL) != 0 {
                print!(" HUPCL");
            }
            if (term.c_cflag & CLOCAL) != 0 {
                print!(" CLOCAL");
            }
            #[cfg(not(any(
                bsd,
                target_os = "emscripten",
                target_os = "haiku",
                target_os = "redox",
            )))]
            if (term.c_cflag & CIBAUD) != 0 {
                print!(" CIBAUD");
            }
            #[cfg(not(any(
                bsd,
                solarish,
                target_os = "emscripten",
                target_os = "haiku",
                target_os = "redox",
            )))]
            if (term.c_cflag & CMSPAR) != 0 {
                print!(" CMSPAR");
            }
            #[cfg(not(any(target_os = "aix", target_os = "redox")))]
            if (term.c_cflag & CRTSCTS) != 0 {
                print!(" CRTSCTS");
            }
            println!();

            print!(" - local flags:");
            if (term.c_lflag & ISIG) != 0 {
                print!(" ISIG");
            }
            if (term.c_lflag & ICANON) != 0 {
                print!(" ICANON");
            }
            #[cfg(any(linux_raw, all(libc, any(target_arch = "s390x", target_os = "haiku"))))]
            if (term.c_lflag & XCASE) != 0 {
                print!(" XCASE");
            }
            if (term.c_lflag & ECHO) != 0 {
                print!(" ECHO");
            }
            if (term.c_lflag & ECHOE) != 0 {
                print!(" ECHOE");
            }
            if (term.c_lflag & ECHOK) != 0 {
                print!(" ECHOK");
            }
            if (term.c_lflag & ECHONL) != 0 {
                print!(" ECHONL");
            }
            #[cfg(not(any(target_os = "redox")))]
            if (term.c_lflag & ECHOCTL) != 0 {
                print!(" ECHOCTL");
            }
            #[cfg(not(any(target_os = "redox")))]
            if (term.c_lflag & ECHOPRT) != 0 {
                print!(" ECHOPRT");
            }
            #[cfg(not(any(target_os = "redox")))]
            if (term.c_lflag & ECHOKE) != 0 {
                print!(" ECHOKE");
            }
            #[cfg(not(any(target_os = "redox")))]
            if (term.c_lflag & FLUSHO) != 0 {
                print!(" FLUSHO");
            }
            if (term.c_lflag & NOFLSH) != 0 {
                print!(" NOFLSH");
            }
            if (term.c_lflag & TOSTOP) != 0 {
                print!(" TOSTOP");
            }
            #[cfg(not(any(target_os = "redox")))]
            if (term.c_lflag & PENDIN) != 0 {
                print!(" PENDIN");
            }
            if (term.c_lflag & IEXTEN) != 0 {
                print!(" IEXTEN");
            }
            println!();

            println!(
                " - keys: INTR={} QUIT={} ERASE={} KILL={} EOF={} TIME={} MIN={} ",
                key(term.c_cc[VINTR]),
                key(term.c_cc[VQUIT]),
                key(term.c_cc[VERASE]),
                key(term.c_cc[VKILL]),
                key(term.c_cc[VEOF]),
                term.c_cc[VTIME],
                term.c_cc[VMIN]
            );
            println!(
                "         START={} STOP={} SUSP={} EOL={}",
                key(term.c_cc[VSTART]),
                key(term.c_cc[VSTOP]),
                key(term.c_cc[VSUSP]),
                key(term.c_cc[VEOL]),
            );
            #[cfg(not(target_os = "haiku"))]
            println!(
                "         REPRINT={} DISCARD={}",
                key(term.c_cc[VREPRINT]),
                key(term.c_cc[VDISCARD])
            );
            #[cfg(not(target_os = "haiku"))]
            println!(
                "         WERASE={} VLNEXT={}",
                key(term.c_cc[VWERASE]),
                key(term.c_cc[VLNEXT]),
            );
            println!("         EOL2={}", key(term.c_cc[VEOL2]));
        }
    } else {
        println!(" - is not a tty");
    }

    println!();
    Ok(())
}

#[cfg(feature = "termios")]
#[cfg(all(not(windows), feature = "stdio"))]
fn key(b: u8) -> String {
    if b == 0 {
        "<undef>".to_string()
    } else if b < 0x20 {
        format!("^{}", (b + 0x40) as char)
    } else if b == 0x7f {
        "^?".to_string()
    } else {
        format!("{}", b as char)
    }
}

#[cfg(any(windows, not(feature = "stdio")))]
fn main() {
    unimplemented!()
}
