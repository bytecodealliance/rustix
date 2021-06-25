use crate::{imp, io};
use io_lifetimes::OwnedFd;

#[cfg(any(
    linux_raw,
    all(
        libc,
        not(any(target_os = "ios", target_os = "macos", target_os = "wasi"))
    )
))]
pub use imp::io::PipeFlags;

/// `pipe()`
#[cfg(any(target_os = "ios", target_os = "macos"))]
#[inline]
pub fn pipe() -> io::Result<(OwnedFd, OwnedFd)> {
    imp::syscalls::pipe()
}

/// `pipe2(flags)`
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "wasi")))]
#[inline]
pub fn pipe2(flags: PipeFlags) -> io::Result<(OwnedFd, OwnedFd)> {
    imp::syscalls::pipe2(flags)
}
