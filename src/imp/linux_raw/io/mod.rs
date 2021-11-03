pub mod epoll;
pub(super) mod error;
mod poll_fd;
mod types;

pub use error::Error;
pub use poll_fd::{PollFd, PollFlags};
pub use types::{
    Advice, DupFlags, EventfdFlags, MapFlags, MlockFlags, MprotectFlags, MremapFlags, PipeFlags,
    ProtFlags, ReadWriteFlags, Tcflag, Termios, UserfaultfdFlags, Winsize, ICANON, PIPE_BUF,
};

use super::libc;

pub(crate) const AT_FDCWD: libc::c_int = linux_raw_sys::general::AT_FDCWD;
pub(crate) const STDIN_FILENO: libc::c_uint = linux_raw_sys::general::STDIN_FILENO;
pub(crate) const STDOUT_FILENO: libc::c_uint = linux_raw_sys::general::STDOUT_FILENO;
pub(crate) const STDERR_FILENO: libc::c_uint = linux_raw_sys::general::STDERR_FILENO;
