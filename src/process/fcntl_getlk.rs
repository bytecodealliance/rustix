use super::Flock;
use crate::fd::AsFd;
use crate::{backend, io};

/// `fcntl(fd, F_GETLK)`—Get the first lock that blocks the lock description
/// pointed to by the argument `lock`. If no such lock is found, then `None` is
/// returned.
///
/// If `lock.typ` is set to `FlockType::Unlocked`, the returned value/error is
/// not explicitly defined, as per POSIX, and will depend on the underlying
/// platform implementation.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/fcntl.html
/// [Linux]: https://man7.org/linux/man-pages/man2/fcntl.2.html
#[inline]
#[doc(alias = "F_GETLK")]
pub fn fcntl_getlk<Fd: AsFd>(fd: Fd, lock: &Flock) -> io::Result<Option<Flock>> {
    backend::process::syscalls::fcntl_getlk(fd.as_fd(), lock)
}

/// `fcntl(fd, F_OFD_GETLK)`—Get the first lock that blocks an open file
/// description lock described by `lock`. If no such lock is found, then `None`
/// is returned.
///
/// The blocking lock may be an open file description lock or a
/// process-associated lock. For an open file description lock, the returned
/// [`Flock::pid`] is `None`.
///
/// `lock.pid` must be `None`, and `lock.typ` must be `FlockType::ReadLock` or
/// `FlockType::WriteLock`.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/F_OFD_GETLK.2const.html
#[cfg(linux_kernel)]
#[inline]
#[doc(alias = "F_OFD_GETLK")]
pub fn fcntl_ofd_getlk<Fd: AsFd>(fd: Fd, lock: &Flock) -> io::Result<Option<Flock>> {
    backend::process::syscalls::fcntl_ofd_getlk(fd.as_fd(), lock)
}
