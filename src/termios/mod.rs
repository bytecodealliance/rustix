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
#[cfg(not(windows))]
pub use tty::isatty;
#[cfg(not(any(target_os = "fuchsia", target_os = "wasi")))]
#[cfg(feature = "procfs")]
pub use tty::ttyname;
