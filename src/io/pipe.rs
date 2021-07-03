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
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/pipe.html
/// [Linux]: https://man7.org/linux/man-pages/man2/pipe.2.html
#[cfg(any(target_os = "ios", target_os = "macos"))]
#[inline]
pub fn pipe() -> io::Result<(OwnedFd, OwnedFd)> {
    imp::syscalls::pipe()
}

/// `pipe2(flags)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/pipe2.2.html
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "wasi")))]
#[inline]
pub fn pipe2(flags: PipeFlags) -> io::Result<(OwnedFd, OwnedFd)> {
    imp::syscalls::pipe2(flags)
}
