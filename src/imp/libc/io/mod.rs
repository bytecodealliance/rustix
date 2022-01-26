mod error;
#[cfg(not(windows))]
#[cfg(not(feature = "std"))]
mod io_slice;
mod poll_fd;
#[cfg(not(windows))]
mod types;

#[cfg(not(windows))]
pub(crate) mod syscalls;

#[cfg(windows)]
pub(crate) mod syscalls {
    use super::super::conv::{borrowed_fd, ret, ret_c_int};
    use super::super::fd::LibcFd;
    use super::c;
    use crate::fd::{BorrowedFd, RawFd};
    use crate::io;
    use crate::io::PollFd;
    use core::convert::TryInto;

    pub(crate) unsafe fn close(raw_fd: RawFd) {
        let _ = c::close(raw_fd as LibcFd);
    }

    pub(crate) fn ioctl_fionbio(fd: BorrowedFd<'_>, value: bool) -> io::Result<()> {
        unsafe {
            let mut data = value as c::c_uint;
            ret(c::ioctl(borrowed_fd(fd), c::FIONBIO, &mut data))
        }
    }

    pub(crate) fn poll(fds: &mut [PollFd<'_>], timeout: c::c_int) -> io::Result<usize> {
        let nfds = fds
            .len()
            .try_into()
            .map_err(|_convert_err| io::Error::INVAL)?;

        ret_c_int(unsafe { c::poll(fds.as_mut_ptr().cast(), nfds, timeout) })
            .map(|nready| nready as usize)
    }
}

#[cfg(not(windows))]
#[cfg(not(feature = "std"))]
pub use io_slice::{IoSlice, IoSliceMut};
#[cfg(any(target_os = "android", target_os = "linux"))]
pub mod epoll;
pub use error::Error;
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
#[cfg(not(any(
    windows,
    target_os = "illumos",
    target_os = "redox",
    target_os = "wasi"
)))]
pub use types::PIPE_BUF;
#[cfg(not(any(windows, target_os = "wasi")))]
pub use types::{
    DupFlags, MapFlags, MprotectFlags, MsyncFlags, ProtFlags, Tcflag, Termios, Winsize, ICANON,
};
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use types::{
    EventfdFlags, MlockFlags, ReadWriteFlags, UffdEvent, UffdFeatureFlags, UffdMsg,
    UffdPagefaultFlags, UffdioApi, UffdioCopy, UffdioCopyModeFlags, UffdioIoctlFlags, UffdioRange,
    UffdioRegister, UffdioRegisterModeFlags, UffdioWriteprotect, UffdioZeropage,
    UffdioZeropageModeFlags, UserfaultfdFlags, UFFD_API,
};

use super::c;

#[cfg(not(any(windows, target_os = "redox")))]
pub(crate) const AT_FDCWD: c::c_int = c::AT_FDCWD;
#[cfg(not(windows))]
pub(crate) const STDIN_FILENO: c::c_int = c::STDIN_FILENO;
#[cfg(not(windows))]
pub(crate) const STDOUT_FILENO: c::c_int = c::STDOUT_FILENO;
#[cfg(not(windows))]
pub(crate) const STDERR_FILENO: c::c_int = c::STDERR_FILENO;
