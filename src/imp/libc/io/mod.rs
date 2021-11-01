mod error;
#[cfg(not(windows))]
mod poll_fd;
#[cfg(not(windows))]
mod types;

#[cfg(not(windows))]
pub(crate) mod syscalls;

#[cfg(any(target_os = "android", target_os = "linux"))]
pub mod epoll;
pub use error::Error;
#[cfg(not(windows))]
pub use poll_fd::{PollFd, PollFlags};
#[cfg(not(any(windows, target_os = "redox", target_os = "wasi")))]
pub use types::Advice;
#[cfg(target_os = "linux")]
pub use types::MremapFlags;
#[cfg(all(
    libc,
    not(any(windows, target_os = "ios", target_os = "macos", target_os = "wasi"))
))]
pub use types::PipeFlags;
#[cfg(not(any(windows, target_os = "redox", target_os = "wasi")))]
pub use types::PIPE_BUF;
#[cfg(not(any(windows, target_os = "wasi")))]
pub use types::{DupFlags, MapFlags, MprotectFlags, ProtFlags, Tcflag, Termios, Winsize, ICANON};
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use types::{EventfdFlags, MlockFlags, ReadWriteFlags, UserfaultfdFlags};

#[cfg(not(any(windows, target_os = "redox")))]
pub(crate) const AT_FDCWD: libc::c_int = libc::AT_FDCWD;
#[cfg(not(windows))]
pub(crate) const STDIN_FILENO: libc::c_int = libc::STDIN_FILENO;
#[cfg(not(windows))]
pub(crate) const STDOUT_FILENO: libc::c_int = libc::STDOUT_FILENO;
#[cfg(not(windows))]
pub(crate) const STDERR_FILENO: libc::c_int = libc::STDERR_FILENO;
