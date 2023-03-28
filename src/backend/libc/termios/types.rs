use super::super::c;

/// `TCSA*` values for use with [`tcsetattr`].
///
/// [`tcsetattr`]: crate::termios::tcsetattr
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum OptionalActions {
    /// `TCSANOW`—Make the change immediately.
    #[doc(alias = "TCSANOW")]
    Now = c::TCSANOW,

    /// `TCSADRAIN`—Make the change after all output has been transmitted.
    #[doc(alias = "TCSADRAIN")]
    Drain = c::TCSADRAIN,

    /// `TCSAFLUSH`—Discard any pending input and then make the change
    /// after all output has been transmitted.
    #[doc(alias = "TCSAFLUSH")]
    Flush = c::TCSAFLUSH,
}

/// `TC*` values for use with [`tcflush`].
///
/// [`tcflush`]: crate::termios::tcflush
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum QueueSelector {
    /// `TCIFLUSH`—Flush data received but not read.
    #[doc(alias = "TCIFLUSH")]
    IFlush = c::TCIFLUSH,

    /// `TCOFLUSH`—Flush data written but not transmitted.
    #[doc(alias = "TCOFLUSH")]
    OFlush = c::TCOFLUSH,

    /// `TCIOFLUSH`—`IFlush` and `OFlush` combined.
    #[doc(alias = "TCIOFLUSH")]
    IOFlush = c::TCIOFLUSH,
}

/// `TC*` values for use with [`tcflow`].
///
/// [`tcflow`]: crate::termios::tcflow
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Action {
    /// `TCOOFF`—Suspend output.
    #[doc(alias = "TCOOFF")]
    OOff = c::TCOOFF,

    /// `TCOON`—Restart suspended output.
    #[doc(alias = "TCOON")]
    OOn = c::TCOON,

    /// `TCIOFF`—Transmits a STOP byte.
    #[doc(alias = "TCIOFF")]
    IOff = c::TCIOFF,

    /// `TCION`—Transmits a START byte.
    #[doc(alias = "TCION")]
    IOn = c::TCION,
}

/// `struct termios` for use with [`tcgetattr`] and [`tcsetattr`].
///
/// [`tcgetattr`]: crate::termios::tcgetattr
/// [`tcsetattr`]: crate::termios::tcsetattr
#[doc(alias = "termios")]
pub type Termios = c::termios;

/// `struct termios2` for use with [`tcgetattr2`] and [`tcsetattr2`].
///
/// [`tcgetattr2`]: crate::termios::tcgetattr2
/// [`tcsetattr2`]: crate::termios::tcsetattr2
#[cfg(all(
    any(target_os = "android", target_os = "linux"),
    any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "x32",
        target_arch = "riscv64",
        target_arch = "aarch64",
        target_arch = "arm",
        target_arch = "mips",
        target_arch = "mips64",
    )
))]
#[doc(alias = "termios2")]
pub type Termios2 = c::termios2;

/// `struct winsize` for use with [`tcgetwinsize`].
///
/// [`tcgetwinsize`]: crate::termios::tcgetwinsize
#[doc(alias = "winsize")]
pub type Winsize = c::winsize;

/// `tcflag_t`—A type for the flags fields of [`Termios`].
#[doc(alias = "tcflag_t")]
pub type Tcflag = c::tcflag_t;

/// `speed_t`—A return type for [`cfsetspeed`] and similar.
///
/// [`cfsetspeed`]: crate::termios::cfsetspeed
#[doc(alias = "speed_t")]
pub type Speed = c::speed_t;

/// `VINTR`
pub const VINTR: usize = c::VINTR as usize;

/// `VQUIT`
pub const VQUIT: usize = c::VQUIT as usize;

/// `VERASE`
pub const VERASE: usize = c::VERASE as usize;

/// `VKILL`
pub const VKILL: usize = c::VKILL as usize;

/// `VEOF`
pub const VEOF: usize = c::VEOF as usize;

/// `VTIME`
pub const VTIME: usize = c::VTIME as usize;

/// `VMIN`
pub const VMIN: usize = c::VMIN as usize;

/// `VSWTC`
#[cfg(not(any(
    apple,
    solarish,
    target_os = "aix",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "haiku",
    target_os = "netbsd",
    target_os = "openbsd",
)))]
pub const VSWTC: usize = c::VSWTC as usize;

/// `VSTART`
pub const VSTART: usize = c::VSTART as usize;

/// `VSTOP`
pub const VSTOP: usize = c::VSTOP as usize;

/// `VSUSP`
pub const VSUSP: usize = c::VSUSP as usize;

/// `VEOL`
pub const VEOL: usize = c::VEOL as usize;

/// `VREPRINT`
#[cfg(not(target_os = "haiku"))]
pub const VREPRINT: usize = c::VREPRINT as usize;

/// `VDISCARD`
#[cfg(not(any(target_os = "aix", target_os = "haiku")))]
pub const VDISCARD: usize = c::VDISCARD as usize;

/// `VWERASE`
#[cfg(not(any(target_os = "aix", target_os = "haiku")))]
pub const VWERASE: usize = c::VWERASE as usize;

/// `VLNEXT`
#[cfg(not(target_os = "haiku"))]
pub const VLNEXT: usize = c::VLNEXT as usize;

/// `VEOL2`
pub const VEOL2: usize = c::VEOL2 as usize;

/// `IGNBRK`
#[cfg(not(apple))]
pub const IGNBRK: c::c_uint = c::IGNBRK;

/// `BRKINT`
#[cfg(not(apple))]
pub const BRKINT: c::c_uint = c::BRKINT;

/// `IGNPAR`
#[cfg(not(apple))]
pub const IGNPAR: c::c_uint = c::IGNPAR;

/// `PARMRK`
#[cfg(not(apple))]
pub const PARMRK: c::c_uint = c::PARMRK;

/// `INPCK`
#[cfg(not(apple))]
pub const INPCK: c::c_uint = c::INPCK;

/// `ISTRIP`
#[cfg(not(apple))]
pub const ISTRIP: c::c_uint = c::ISTRIP;

/// `INLCR`
#[cfg(not(apple))]
pub const INLCR: c::c_uint = c::INLCR;

/// `IGNCR`
#[cfg(not(apple))]
pub const IGNCR: c::c_uint = c::IGNCR;

/// `ICRNL`
#[cfg(not(apple))]
pub const ICRNL: c::c_uint = c::ICRNL;

/// `IUCLC`
#[cfg(any(solarish, target_os = "haiku"))]
pub const IUCLC: c::c_uint = c::IUCLC;

/// `IXON`
#[cfg(not(apple))]
pub const IXON: c::c_uint = c::IXON;

/// `IXANY`
#[cfg(not(any(apple, target_os = "redox")))]
pub const IXANY: c::c_uint = c::IXANY;

/// `IXOFF`
#[cfg(not(apple))]
pub const IXOFF: c::c_uint = c::IXOFF;

/// `IMAXBEL`
#[cfg(not(any(apple, target_os = "haiku", target_os = "redox")))]
pub const IMAXBEL: c::c_uint = c::IMAXBEL;

/// `IUTF8`
#[cfg(not(any(
    apple,
    solarish,
    target_os = "aix",
    target_os = "dragonfly",
    target_os = "emscripten",
    target_os = "freebsd",
    target_os = "haiku",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox",
)))]
pub const IUTF8: c::c_uint = c::IUTF8;

/// `OPOST`
#[cfg(not(apple))]
pub const OPOST: c::c_uint = c::OPOST;

/// `OLCUC`
#[cfg(not(any(
    apple,
    target_os = "aix",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "redox",
)))]
pub const OLCUC: c::c_uint = c::OLCUC;

/// `ONLCR`
#[cfg(not(apple))]
pub const ONLCR: c::c_uint = c::ONLCR;

/// `OCRNL`
#[cfg(not(apple))]
pub const OCRNL: c::c_uint = c::OCRNL;

/// `ONOCR`
#[cfg(not(apple))]
pub const ONOCR: c::c_uint = c::ONOCR;

/// `ONLRET`
#[cfg(not(apple))]
pub const ONLRET: c::c_uint = c::ONLRET;

/// `OFILL`
#[cfg(not(bsd))]
pub const OFILL: c::c_uint = c::OFILL;

/// `OFDEL`
#[cfg(not(bsd))]
pub const OFDEL: c::c_uint = c::OFDEL;

/// `NLDLY`
#[cfg(not(any(bsd, solarish, target_os = "redox")))]
pub const NLDLY: c::c_uint = c::NLDLY;

/// `NL0`
#[cfg(not(any(bsd, solarish, target_os = "fuchsia", target_os = "redox")))]
pub const NL0: c::c_uint = c::NL0;

/// `NL1`
#[cfg(not(any(bsd, solarish, target_os = "fuchsia", target_os = "redox")))]
pub const NL1: c::c_uint = c::NL1;

/// `CRDLY`
#[cfg(not(any(bsd, solarish, target_os = "redox")))]
pub const CRDLY: c::c_uint = c::CRDLY;

/// `CR0`
#[cfg(not(any(bsd, solarish, target_os = "fuchsia", target_os = "redox")))]
pub const CR0: c::c_uint = c::CR0;

/// `CR1`
#[cfg(not(any(
    target_env = "musl",
    bsd,
    solarish,
    target_os = "emscripten",
    target_os = "fuchsia",
    target_os = "redox",
)))]
pub const CR1: c::c_uint = c::CR1;

/// `CR2`
#[cfg(not(any(
    target_env = "musl",
    bsd,
    solarish,
    target_os = "emscripten",
    target_os = "fuchsia",
    target_os = "redox",
)))]
pub const CR2: c::c_uint = c::CR2;

/// `CR3`
#[cfg(not(any(
    target_env = "musl",
    bsd,
    solarish,
    target_os = "emscripten",
    target_os = "fuchsia",
    target_os = "redox",
)))]
pub const CR3: c::c_uint = c::CR3;

/// `TABDLY`
#[cfg(not(any(
    apple,
    netbsdlike,
    solarish,
    target_os = "dragonfly",
    target_os = "redox",
)))]
pub const TABDLY: c::c_uint = c::TABDLY;

/// `TAB0`
#[cfg(not(any(
    apple,
    netbsdlike,
    solarish,
    target_os = "dragonfly",
    target_os = "fuchsia",
    target_os = "redox",
)))]
pub const TAB0: c::c_uint = c::TAB0;

/// `TAB1`
#[cfg(not(any(
    target_env = "musl",
    bsd,
    solarish,
    target_os = "emscripten",
    target_os = "fuchsia",
    target_os = "redox",
)))]
pub const TAB1: c::c_uint = c::TAB1;

/// `TAB2`
#[cfg(not(any(
    target_env = "musl",
    bsd,
    solarish,
    target_os = "emscripten",
    target_os = "fuchsia",
    target_os = "redox",
)))]
pub const TAB2: c::c_uint = c::TAB2;

/// `TAB3`
#[cfg(not(any(
    target_env = "musl",
    bsd,
    solarish,
    target_os = "emscripten",
    target_os = "fuchsia",
    target_os = "redox",
)))]
pub const TAB3: c::c_uint = c::TAB3;

/// `BSDLY`
#[cfg(not(any(bsd, solarish, target_os = "redox")))]
pub const BSDLY: c::c_uint = c::BSDLY;

/// `BS0`
#[cfg(not(any(bsd, solarish, target_os = "fuchsia", target_os = "redox")))]
pub const BS0: c::c_uint = c::BS0;

/// `BS1`
#[cfg(not(any(
    target_env = "musl",
    bsd,
    solarish,
    target_os = "emscripten",
    target_os = "fuchsia",
    target_os = "redox",
)))]
pub const BS1: c::c_uint = c::BS1;

/// `FFDLY`
#[cfg(not(any(target_env = "musl", bsd, solarish, target_os = "redox")))]
pub const FFDLY: c::c_uint = c::FFDLY;

/// `FF0`
#[cfg(not(any(bsd, solarish, target_os = "fuchsia", target_os = "redox")))]
pub const FF0: c::c_uint = c::FF0;

/// `FF1`
#[cfg(not(any(
    target_env = "musl",
    bsd,
    solarish,
    target_os = "emscripten",
    target_os = "fuchsia",
    target_os = "redox",
)))]
pub const FF1: c::c_uint = c::FF1;

/// `VTDLY`
#[cfg(not(any(target_env = "musl", bsd, solarish, target_os = "redox")))]
pub const VTDLY: c::c_uint = c::VTDLY;

/// `VT0`
#[cfg(not(any(bsd, solarish, target_os = "fuchsia", target_os = "redox")))]
pub const VT0: c::c_uint = c::VT0;

/// `VT1`
#[cfg(not(any(
    target_env = "musl",
    bsd,
    solarish,
    target_os = "emscripten",
    target_os = "fuchsia",
    target_os = "redox",
)))]
pub const VT1: c::c_uint = c::VT1;

/// `B0`
pub const B0: Speed = c::B0;

/// `B50`
pub const B50: Speed = c::B50;

/// `B75`
pub const B75: Speed = c::B75;

/// `B110`
pub const B110: Speed = c::B110;

/// `B134`
pub const B134: Speed = c::B134;

/// `B150`
pub const B150: Speed = c::B150;

/// `B200`
pub const B200: Speed = c::B200;

/// `B300`
pub const B300: Speed = c::B300;

/// `B600`
pub const B600: Speed = c::B600;

/// `B1200`
pub const B1200: Speed = c::B1200;

/// `B1800`
pub const B1800: Speed = c::B1800;

/// `B2400`
pub const B2400: Speed = c::B2400;

/// `B4800`
pub const B4800: Speed = c::B4800;

/// `B9600`
pub const B9600: Speed = c::B9600;

/// `B19200`
pub const B19200: Speed = c::B19200;

/// `B38400`
pub const B38400: Speed = c::B38400;

/// `B57600`
#[cfg(not(target_os = "aix"))]
pub const B57600: Speed = c::B57600;

/// `B115200`
#[cfg(not(target_os = "aix"))]
pub const B115200: Speed = c::B115200;

/// `B230400`
#[cfg(not(target_os = "aix"))]
pub const B230400: Speed = c::B230400;

/// `B460800`
#[cfg(not(any(
    apple,
    target_os = "aix",
    target_os = "dragonfly",
    target_os = "haiku",
    target_os = "openbsd"
)))]
pub const B460800: Speed = c::B460800;

/// `B500000`
#[cfg(not(any(bsd, solarish, target_os = "aix", target_os = "haiku")))]
pub const B500000: Speed = c::B500000;

/// `B576000`
#[cfg(not(any(bsd, solarish, target_os = "aix", target_os = "haiku")))]
pub const B576000: Speed = c::B576000;

/// `B921600`
#[cfg(not(any(
    apple,
    target_os = "aix",
    target_os = "dragonfly",
    target_os = "haiku",
    target_os = "openbsd"
)))]
pub const B921600: Speed = c::B921600;

/// `B1000000`
#[cfg(not(any(bsd, target_os = "aix", target_os = "haiku", target_os = "solaris")))]
pub const B1000000: Speed = c::B1000000;

/// `B1152000`
#[cfg(not(any(bsd, target_os = "aix", target_os = "haiku", target_os = "solaris")))]
pub const B1152000: Speed = c::B1152000;

/// `B1500000`
#[cfg(not(any(bsd, target_os = "aix", target_os = "haiku", target_os = "solaris")))]
pub const B1500000: Speed = c::B1500000;

/// `B2000000`
#[cfg(not(any(bsd, target_os = "aix", target_os = "haiku", target_os = "solaris")))]
pub const B2000000: Speed = c::B2000000;

/// `B2500000`
#[cfg(not(any(
    target_arch = "sparc",
    target_arch = "sparc64",
    bsd,
    target_os = "aix",
    target_os = "haiku",
    target_os = "solaris",
)))]
pub const B2500000: Speed = c::B2500000;

/// `B3000000`
#[cfg(not(any(
    target_arch = "sparc",
    target_arch = "sparc64",
    bsd,
    target_os = "aix",
    target_os = "haiku",
    target_os = "solaris",
)))]
pub const B3000000: Speed = c::B3000000;

/// `B3500000`
#[cfg(not(any(
    target_arch = "sparc",
    target_arch = "sparc64",
    bsd,
    target_os = "aix",
    target_os = "haiku",
    target_os = "solaris",
)))]
pub const B3500000: Speed = c::B3500000;

/// `B4000000`
#[cfg(not(any(
    target_arch = "sparc",
    target_arch = "sparc64",
    bsd,
    target_os = "aix",
    target_os = "haiku",
    target_os = "solaris",
)))]
pub const B4000000: Speed = c::B4000000;

/// `BOTHER`
#[cfg(any(target_os = "android", target_os = "linux"))]
pub const BOTHER: c::c_uint = c::BOTHER;

/// `CSIZE`
#[cfg(not(apple))]
pub const CSIZE: c::c_uint = c::CSIZE;

/// `CS5`
#[cfg(not(apple))]
pub const CS5: c::c_uint = c::CS5;

/// `CS6`
#[cfg(not(apple))]
pub const CS6: c::c_uint = c::CS6;

/// `CS7`
#[cfg(not(apple))]
pub const CS7: c::c_uint = c::CS7;

/// `CS8`
#[cfg(not(apple))]
pub const CS8: c::c_uint = c::CS8;

/// `CSTOPB`
#[cfg(not(apple))]
pub const CSTOPB: c::c_uint = c::CSTOPB;

/// `CREAD`
#[cfg(not(apple))]
pub const CREAD: c::c_uint = c::CREAD;

/// `PARENB`
#[cfg(not(apple))]
pub const PARENB: c::c_uint = c::PARENB;

/// `PARODD`
#[cfg(not(apple))]
pub const PARODD: c::c_uint = c::PARODD;

/// `HUPCL`
#[cfg(not(apple))]
pub const HUPCL: c::c_uint = c::HUPCL;

/// `CLOCAL`
#[cfg(not(apple))]
pub const CLOCAL: c::c_uint = c::CLOCAL;

/// `ISIG`
#[cfg(not(apple))]
pub const ISIG: c::c_uint = c::ISIG;

/// `ICANON`—A flag for the `c_lflag` field of [`Termios`] indicating
/// canonical mode.
pub const ICANON: Tcflag = c::ICANON;

/// `ECHO`
#[cfg(not(apple))]
pub const ECHO: c::c_uint = c::ECHO;

/// `ECHOE`
#[cfg(not(apple))]
pub const ECHOE: c::c_uint = c::ECHOE;

/// `ECHOK`
#[cfg(not(apple))]
pub const ECHOK: c::c_uint = c::ECHOK;

/// `ECHONL`
#[cfg(not(apple))]
pub const ECHONL: c::c_uint = c::ECHONL;

/// `NOFLSH`
#[cfg(not(apple))]
pub const NOFLSH: c::c_uint = c::NOFLSH;

/// `TOSTOP`
#[cfg(not(apple))]
pub const TOSTOP: c::c_uint = c::TOSTOP;

/// `IEXTEN`
#[cfg(not(apple))]
pub const IEXTEN: c::c_uint = c::IEXTEN;

/// `EXTA`
#[cfg(not(any(
    apple,
    solarish,
    target_os = "emscripten",
    target_os = "haiku",
    target_os = "redox",
)))]
pub const EXTA: c::c_uint = c::EXTA;

/// `EXTB`
#[cfg(not(any(
    apple,
    solarish,
    target_os = "emscripten",
    target_os = "haiku",
    target_os = "redox",
)))]
pub const EXTB: c::c_uint = c::EXTB;

/// `CBAUD`
#[cfg(not(any(bsd, target_os = "haiku", target_os = "redox")))]
pub const CBAUD: c::c_uint = c::CBAUD;

/// `CBAUDEX`
#[cfg(not(any(
    bsd,
    solarish,
    target_os = "aix",
    target_os = "haiku",
    target_os = "redox",
)))]
pub const CBAUDEX: c::c_uint = c::CBAUDEX;

/// `CIBAUD`
#[cfg(not(any(
    target_arch = "powerpc",
    target_arch = "powerpc64",
    bsd,
    target_os = "emscripten",
    target_os = "haiku",
    target_os = "redox",
)))]
pub const CIBAUD: c::tcflag_t = c::CIBAUD;

/// `CIBAUD`
// TODO: Upstream this.
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
pub const CIBAUD: c::tcflag_t = 0o77600000;

/// `CMSPAR`
#[cfg(not(any(
    bsd,
    solarish,
    target_os = "aix",
    target_os = "emscripten",
    target_os = "haiku",
    target_os = "redox",
)))]
pub const CMSPAR: c::c_uint = c::CMSPAR;

/// `CRTSCTS`
#[cfg(not(any(apple, target_os = "aix", target_os = "redox")))]
pub const CRTSCTS: c::c_uint = c::CRTSCTS;

/// `XCASE`
#[cfg(any(target_arch = "s390x", target_os = "haiku"))]
pub const XCASE: c::c_uint = c::XCASE;

/// `ECHOCTL`
#[cfg(not(any(apple, target_os = "redox")))]
pub const ECHOCTL: c::c_uint = c::ECHOCTL;

/// `ECHOPRT`
#[cfg(not(any(apple, target_os = "redox")))]
pub const ECHOPRT: c::c_uint = c::ECHOPRT;

/// `ECHOKE`
#[cfg(not(any(apple, target_os = "redox")))]
pub const ECHOKE: c::c_uint = c::ECHOKE;

/// `FLUSHO`
#[cfg(not(any(apple, target_os = "redox")))]
pub const FLUSHO: c::c_uint = c::FLUSHO;

/// `PENDIN`
#[cfg(not(any(apple, target_os = "redox")))]
pub const PENDIN: c::c_uint = c::PENDIN;

/// `EXTPROC`
#[cfg(not(any(apple, target_os = "aix", target_os = "haiku", target_os = "redox")))]
pub const EXTPROC: c::c_uint = c::EXTPROC;

/// `XTABS`
#[cfg(not(any(
    bsd,
    solarish,
    target_os = "aix",
    target_os = "haiku",
    target_os = "redox",
)))]
pub const XTABS: c::c_uint = c::XTABS;
