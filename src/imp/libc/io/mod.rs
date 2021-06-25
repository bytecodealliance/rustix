mod poll_fd;
mod types;

pub use poll_fd::{PollFd, PollFlags};
#[cfg(any(
    linux_raw,
    all(
        libc,
        not(any(target_os = "ios", target_os = "macos", target_os = "wasi"))
    )
))]
pub use types::PipeFlags;
#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub use types::ReadWriteFlags;
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use types::UserFaultFdFlags;
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub use types::PIPE_BUF;
#[cfg(not(target_os = "wasi"))]
pub use types::{DupFlags, MapFlags, ProtFlags, Tcflag, Termios, Winsize, ICANON};

use std::os::raw::c_int;

#[cfg(not(target_os = "redox"))]
pub(crate) const AT_FDCWD: c_int = libc::AT_FDCWD;
pub(crate) const STDIN_FILENO: c_int = libc::STDIN_FILENO;
pub(crate) const STDOUT_FILENO: c_int = libc::STDOUT_FILENO;
pub(crate) const STDERR_FILENO: c_int = libc::STDERR_FILENO;
