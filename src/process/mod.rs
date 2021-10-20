//! Process-associated operations.

use crate::imp;
use crate::io;

mod auxv;
#[cfg(not(target_os = "wasi"))]
mod chdir;
mod exit;
#[cfg(not(target_os = "wasi"))] // WASI doesn't have get[gpu]id.
mod id;
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
mod membarrier;
#[cfg(not(any(target_os = "fuchsia", target_os = "wasi")))] // WASI doesn't have [gs]etpriority.
mod priority;
#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
mod rlimit;
#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "fuchsia",
    target_os = "dragonfly"
))]
mod sched;
#[cfg(not(target_os = "wasi"))] // WASI doesn't have uname.
mod uname;

#[cfg(target_vendor = "mustang")]
pub use auxv::init;
pub use auxv::page_size;
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use auxv::{linux_execfn, linux_hwcap};
#[cfg(not(target_os = "wasi"))]
pub use chdir::chdir;
#[cfg(not(any(target_os = "wasi", target_os = "fuchsia")))]
pub use chdir::fchdir;
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub use chdir::getcwd;
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use exit::exit_group;
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use id::Cpuid;
#[cfg(not(target_os = "wasi"))]
pub use id::{
    getegid, geteuid, getgid, getpid, getppid, getuid, Gid, Pid, RawGid, RawPid, RawUid, Uid,
};
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use membarrier::{
    membarrier, membarrier_cpu, membarrier_query, MembarrierCommand, MembarrierQuery,
};
#[cfg(not(any(target_os = "fuchsia", target_os = "wasi")))]
pub use priority::nice;
#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
pub use priority::{
    getpriority_pgrp, getpriority_process, getpriority_user, setpriority_pgrp, setpriority_process,
    setpriority_user,
};
#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
pub use rlimit::{getrlimit, Resource, Rlimit};
#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "fuchsia",
    target_os = "dragonfly"
))]
pub use sched::{sched_getaffinity, sched_setaffinity, CpuSet};
#[cfg(not(target_os = "wasi"))]
pub use uname::{uname, Uname};

/// `EXIT_SUCCESS` for use with [`exit`].
///
/// [`exit`]: std::process::exit
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/stdlib.h.html
/// [Linux]: https://man7.org/linux/man-pages/man3/exit.3.html
pub const EXIT_SUCCESS: i32 = imp::process::EXIT_SUCCESS;

/// `EXIT_FAILURE` for use with [`exit`].
///
/// [`exit`]: std::process::exit
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/stdlib.h.html
/// [Linux]: https://man7.org/linux/man-pages/man3/exit.3.html
pub const EXIT_FAILURE: i32 = imp::process::EXIT_FAILURE;

/// The exit status used by a process terminated with `SIGABRT` signal.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://tldp.org/LDP/abs/html/exitcodes.html
#[cfg(not(target_os = "wasi"))]
pub const EXIT_SIGNALED_SIGABRT: i32 = imp::process::EXIT_SIGNALED_SIGABRT;

/// `sched_yield()`—Hints to the OS that other processes should run.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/sched_yield.html
/// [Linux]: https://man7.org/linux/man-pages/man2/sched_yield.2.html
#[inline]
pub fn sched_yield() {
    imp::syscalls::sched_yield()
}

/// `fork()` — Creates a new process by duplicating the calling process.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/fork.html
/// [Linux]: https://man7.org/linux/man-pages/man2/fork.2.html
#[cfg(not(target_os = "wasi"))]
pub fn fork() -> io::Result<Pid> {
    imp::syscalls::fork()
}
