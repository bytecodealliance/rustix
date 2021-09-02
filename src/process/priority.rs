use crate::process::{Pid, Uid};
use crate::{imp, io};

/// `nice()`—Adjust the scheduling priority of the current process.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/nice.html
/// [Linux]: https://man7.org/linux/man-pages/man2/nice.2.html
#[inline]
pub fn nice(inc: i32) -> io::Result<i32> {
    imp::syscalls::nice(inc)
}

/// `getpriority(PRIO_USER, uid)`—Get the scheduling priority of the given
/// user.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpriority.html
/// [Linux]: https://man7.org/linux/man-pages/man2/getpriority.2.html
#[cfg(not(target_os = "redox"))]
#[inline]
#[doc(alias = "getpriority")]
pub fn getpriority_user(uid: Uid) -> io::Result<i32> {
    imp::syscalls::getpriority_user(uid)
}

/// `getpriority(PRIO_PGRP, gid)`—Get the scheduling priority of the given
/// process group.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpriority.html
/// [Linux]: https://man7.org/linux/man-pages/man2/getpriority.2.html
#[cfg(not(target_os = "redox"))]
#[inline]
#[doc(alias = "getpriority")]
pub fn getpriority_pgrp(pgid: Pid) -> io::Result<i32> {
    imp::syscalls::getpriority_pgrp(pgid)
}

/// `getpriority(PRIO_PROCESS, pid)`—Get the scheduling priority of the given
/// process.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpriority.html
/// [Linux]: https://man7.org/linux/man-pages/man2/getpriority.2.html
#[cfg(not(target_os = "redox"))]
#[inline]
#[doc(alias = "getpriority")]
pub fn getpriority_process(pid: Pid) -> io::Result<i32> {
    imp::syscalls::getpriority_process(pid)
}

/// `setpriority(PRIO_USER, uid)`—Get the scheduling priority of the given
/// user.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setpriority.html
/// [Linux]: https://man7.org/linux/man-pages/man2/setpriority.2.html
#[cfg(not(target_os = "redox"))]
#[inline]
#[doc(alias = "setpriority")]
pub fn setpriority_user(uid: Uid, priority: i32) -> io::Result<()> {
    imp::syscalls::setpriority_user(uid, priority)
}

/// `setpriority(PRIO_PGRP, pgid)`—Get the scheduling priority of the given
/// process group.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setpriority.html
/// [Linux]: https://man7.org/linux/man-pages/man2/setpriority.2.html
#[cfg(not(target_os = "redox"))]
#[inline]
#[doc(alias = "setpriority")]
pub fn setpriority_pgrp(pgid: Pid, priority: i32) -> io::Result<()> {
    imp::syscalls::setpriority_pgrp(pgid, priority)
}

/// `setpriority(PRIO_PROCESS, pid)`—Get the scheduling priority of the given
/// process.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setpriority.html
/// [Linux]: https://man7.org/linux/man-pages/man2/setpriority.2.html
#[cfg(not(target_os = "redox"))]
#[inline]
#[doc(alias = "setpriority")]
pub fn setpriority_process(pid: Pid, priority: i32) -> io::Result<()> {
    imp::syscalls::setpriority_process(pid, priority)
}
