//! Terminal I/O stream operations.

#[cfg(not(target_os = "wasi"))]
mod cf;
#[cfg(not(target_os = "wasi"))]
mod constants;
#[cfg(not(target_os = "wasi"))]
mod tc;
#[cfg(not(windows))]
mod tty;

#[cfg(not(target_os = "wasi"))]
pub use cf::{cfgetispeed, cfgetospeed, cfmakeraw, cfsetispeed, cfsetospeed, cfsetspeed};

#[cfg(not(target_os = "wasi"))]
#[allow(unused_imports)]
pub use constants::*;

#[cfg(not(target_os = "wasi"))]
pub use tc::{
    tcdrain, tcflow, tcflush, tcgetattr, tcgetpgrp, tcgetsid, tcgetwinsize, tcsendbreak, tcsetattr,
    tcsetpgrp, tcsetwinsize, Action, OptionalActions, QueueSelector, Speed, Tcflag, Termios,
    Winsize,
};
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
pub use tc::{tcgetattr2, tcsetattr2, Termios2};
#[cfg(not(windows))]
pub use tty::isatty;
#[cfg(not(any(target_os = "fuchsia", target_os = "wasi")))]
#[cfg(feature = "procfs")]
pub use tty::ttyname;
