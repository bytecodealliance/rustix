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

use super::c;

pub(crate) const AT_FDCWD: c::c_int = linux_raw_sys::general::AT_FDCWD;
pub(crate) const STDIN_FILENO: c::c_uint = linux_raw_sys::general::STDIN_FILENO;
pub(crate) const STDOUT_FILENO: c::c_uint = linux_raw_sys::general::STDOUT_FILENO;
pub(crate) const STDERR_FILENO: c::c_uint = linux_raw_sys::general::STDERR_FILENO;
