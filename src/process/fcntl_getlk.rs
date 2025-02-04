use super::Flock;
use crate::fd::AsFd;
use crate::{backend, io};

/// `fcntl(fd, F_GETLK)`—Get the first lock that blocks the lock description pointed to by the argument `lock`.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/fcntl.html
/// [Linux]: https://man7.org/linux/man-pages/man2/fcntl.2.html
#[cfg(not(any(
    target_os = "emscripten",
    target_os = "espidf",
    target_os = "fuchsia",
    target_os = "redox",
    target_os = "vita",
    target_os = "wasi"
)))]
#[inline]
#[doc(alias = "F_GETLK")]
pub fn fcntl_getlk<Fd: AsFd>(fd: Fd, lock: &Flock) -> io::Result<Option<Flock>> {
    backend::process::syscalls::fcntl_getlk(fd.as_fd(), lock)
}
