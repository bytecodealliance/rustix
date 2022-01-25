//! Process-associated operations.

use crate::{imp, io};

mod auxv;
#[cfg(not(target_os = "wasi"))]
mod chdir;
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
#[cfg(any(
    linux_raw,
    all(
        libc,
        any(
            all(target_os = "android", target_pointer_width = "64"),
            target_os = "linux"
        )
    )
))]
pub use auxv::{linux_execfn, linux_hwcap};
#[cfg(not(target_os = "wasi"))]
pub use chdir::chdir;
#[cfg(not(any(target_os = "wasi", target_os = "fuchsia")))]
pub use chdir::fchdir;
#[cfg(not(target_os = "wasi"))]
pub use chdir::getcwd;
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use id::Cpuid;
#[cfg(not(target_os = "wasi"))]
pub use id::{
    getegid, geteuid, getgid, getpid, getppid, getuid, Gid, Pid, RawGid, RawNonZeroPid, RawPid,
    RawUid, Uid,
};
#[cfg(not(target_os = "wasi"))]
pub use imp::process::Signal;
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
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use rlimit::prlimit;
#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
pub use rlimit::{getrlimit, setrlimit, Resource, Rlimit};
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
/// This function always succeeds.
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

/// `waitpid(pid, waitopts)`—Wait for a specific process to change state.
///
/// If the pid is `None`, the call will wait for any child process whose
/// process group id matches that of the calling process.
///
/// If the pid is equal to `RawPid::MAX`, the call will wait for any child
/// process.
///
/// Otherwise if the `wrapping_neg` of pid is less than pid, the call will wait
/// for any child process with a group ID equal to the `wrapping_neg` of `pid`.
///
/// Otherwise, the call will wait for the child process with the given pid.
///
/// On Success, returns the status of the selected process.
///
/// If `NOHANG` was specified in the options, and the selected child process
/// didn't change state, returns `None`.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/wait.html
/// [Linux]: https://man7.org/linux/man-pages/man2/waitpid.2.html
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn waitpid(pid: Option<Pid>, waitopts: WaitOptions) -> io::Result<Option<WaitStatus>> {
    Ok(imp::syscalls::waitpid(pid, waitopts)?.map(|(_, status)| status))
}

/// `wait(waitopts)`—Wait for any of the children of calling process to
/// change state.
///
/// On success, returns the pid of the child process whose state changed, and
/// the status of said process.
///
/// If `NOHANG` was specified in the options, and the selected child process
/// didn't change state, returns `None`.
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
    imp::syscalls::wait(waitopts)
}

/// `setsid()`—Create a new session.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setsid.html
/// [Linux]: https://man7.org/linux/man-pages/man2/setsid.2.html
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn setsid() -> io::Result<Pid> {
    imp::syscalls::setsid()
}

/// `kill(pid, sig)`—Sends a signal to a process.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/kill.html
/// [Linux]: https://man7.org/linux/man-pages/man2/kill.2.html
#[cfg(not(target_os = "wasi"))]
#[inline]
#[doc(alias = "kill")]
pub fn kill_process(pid: Pid, sig: Signal) -> io::Result<()> {
    imp::syscalls::kill_process(pid, sig)
}

/// `kill(-pid, sig)`—Sends a signal to all processes in a process group.
///
/// If `pid` is 1, this sends a signal to all processes the current process
/// has permission to send signals to, except process process `1`, possibly
/// other system-specific processes, and on some systems, the current process.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/kill.html
/// [Linux]: https://man7.org/linux/man-pages/man2/kill.2.html
#[cfg(not(target_os = "wasi"))]
#[inline]
#[doc(alias = "kill")]
pub fn kill_process_group(pid: Pid, sig: Signal) -> io::Result<()> {
    imp::syscalls::kill_process_group(pid, sig)
}

/// `kill(0, sig)`—Sends a signal to all processes in the current process
/// group.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/kill.html
/// [Linux]: https://man7.org/linux/man-pages/man2/kill.2.html
#[cfg(not(target_os = "wasi"))]
#[inline]
#[doc(alias = "kill")]
pub fn kill_current_process_group(sig: Signal) -> io::Result<()> {
    imp::syscalls::kill_current_process_group(sig)
}
