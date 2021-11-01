mod error;
mod poll_fd;
mod types;

pub(crate) mod syscalls;

#[cfg(any(target_os = "android", target_os = "linux"))]
pub mod epoll;
pub use error::Error;
pub use poll_fd::{PollFd, PollFlags};
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub use types::Advice;
#[cfg(target_os = "linux")]
pub use types::MremapFlags;
#[cfg(all(
    libc,
    not(any(target_os = "ios", target_os = "macos", target_os = "wasi"))
))]
pub use types::PipeFlags;
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub use types::PIPE_BUF;
#[cfg(not(target_os = "wasi"))]
pub use types::{DupFlags, MapFlags, MprotectFlags, ProtFlags, Tcflag, Termios, Winsize, ICANON};
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use types::{EventfdFlags, MlockFlags, ReadWriteFlags, UserfaultfdFlags};

use libc::c_int;

#[cfg(not(target_os = "redox"))]
pub(crate) const AT_FDCWD: c_int = libc::AT_FDCWD;
pub(crate) const STDIN_FILENO: c_int = libc::STDIN_FILENO;
pub(crate) const STDOUT_FILENO: c_int = libc::STDOUT_FILENO;
pub(crate) const STDERR_FILENO: c_int = libc::STDERR_FILENO;
