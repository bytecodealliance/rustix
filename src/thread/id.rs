use crate::process::{Gid, Pid, Uid};
use crate::{backend, io};

/// `gettid()`—Returns the thread ID.
///
/// This returns the OS thread ID, which is not necessarily the same as the
/// `rust::thread::Thread::id` or the pthread ID.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/gettid.2.html
#[inline]
#[must_use]
pub fn gettid() -> Pid {
    backend::thread::syscalls::gettid()
}

/// `setuid(uid)`
///
/// # Warning
///
/// This is not the setxid you are looking for... POSIX requires xids to be
/// process granular, but on Linux they are per-thread. Thus, this call only
/// changes the xid for the current *thread*, not the entire process even
/// though that is in violation of the POSIX standard.
///
/// For details on this distinction, see the C library vs. kernel differences
/// in the [man page][linux_notes]. This call implements the kernel behavior.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setuid.html
/// [Linux]: https://man7.org/linux/man-pages/man2/setuid.2.html
/// [linux_notes]: https://man7.org/linux/man-pages/man2/setuid.2.html#NOTES
#[inline]
pub fn set_thread_uid(uid: Uid) -> io::Result<()> {
    backend::thread::syscalls::setuid_thread(uid)
}

/// `setgid(gid)`
///
/// # Warning
///
/// This is not the setxid you are looking for... POSIX requires xids to be
/// process granular, but on Linux they are per-thread. Thus, this call only
/// changes the xid for the current *thread*, not the entire process even
/// though that is in violation of the POSIX standard.
///
/// For details on this distinction, see the C library vs. kernel differences
/// in the [man page][linux_notes]. This call implements the kernel behavior.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setgid.html
/// [Linux]: https://man7.org/linux/man-pages/man2/setgid.2.html
/// [linux_notes]: https://man7.org/linux/man-pages/man2/setgid.2.html#NOTES
#[inline]
pub fn set_thread_gid(gid: Gid) -> io::Result<()> {
    backend::thread::syscalls::setgid_thread(gid)
}
