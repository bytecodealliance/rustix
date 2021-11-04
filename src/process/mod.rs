//! Process-associated operations.

use crate::{imp, io};

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
#[cfg(not(target_os = "wasi"))]
mod wait;

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
#[cfg(not(target_os = "wasi"))]
pub use wait::{WaitOptions, WaitStatus};

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

/// `waitpid`—Wait for a specific process to change state.
///
/// If the selected PID is `Pid::NONE`, the call will wait for the child
/// process, whose PID matches that of the calling process.
///
/// On Success, returns the status of the selected process.
///
/// If `NOHANG` was specified in the options,
/// and the selected child process didn't change state, returns `None`.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/wait.html
/// [Linux]: https://man7.org/linux/man-pages/man2/waitpid.2.html
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn waitpid(pid: Pid, waitopts: WaitOptions) -> io::Result<Option<WaitStatus>> {
    Ok(imp::syscalls::waitpid(pid.as_raw(), waitopts)?.map(|(_, status)| status))
}

/// `wait`—Wait for any of the childern of calling process to change state.
///
/// On success, returns the pid of the child process whose state changed,
/// and the status of said process.
///
/// If `NOHANG` was specified in the options,
/// and no child process changed state, returns `None`.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/wait.html
/// [Linux]: https://man7.org/linux/man-pages/man2/waitpid.2.html
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn wait(waitopts: WaitOptions) -> io::Result<Option<(Pid, WaitStatus)>> {
    imp::syscalls::waitpid(!0, waitopts)
}
