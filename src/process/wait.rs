//! Wait for processes to change state.
//!
//! # Safety
//!
//! This code needs to implement `Send` and `Sync` for `WaitIdStatus` because
//! the linux-raw-sys bindings generate a type that doesn't do so
//! automatically.
#![allow(unsafe_code)]
use crate::process::Pid;
use crate::{backend, io};
use bitflags::bitflags;

#[cfg(target_os = "linux")]
use crate::fd::BorrowedFd;

#[cfg(linux_raw)]
use crate::backend::process::wait::SiginfoExt as _;

bitflags! {
    /// Options for modifying the behavior of [`wait`]/[`waitpid`].
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct WaitOptions: u32 {
        /// Return immediately if no child has exited.
        const NOHANG = bitcast!(backend::process::wait::WNOHANG);
        /// Return if a child has stopped (but not traced via [`ptrace`]).
        ///
        /// [`ptrace`]: https://man7.org/linux/man-pages/man2/ptrace.2.html
        const UNTRACED = bitcast!(backend::process::wait::WUNTRACED);
        /// Return if a stopped child has been resumed by delivery of
        /// [`Signal::Cont`].
        ///
        /// [`Signal::Cont`]: crate::process::Signal::Cont
        const CONTINUED = bitcast!(backend::process::wait::WCONTINUED);

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

#[cfg(not(any(target_os = "openbsd", target_os = "redox", target_os = "wasi")))]
bitflags! {
    /// Options for modifying the behavior of [`waitid`].
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct WaitIdOptions: u32 {
        /// Return immediately if no child has exited.
        const NOHANG = bitcast!(backend::process::wait::WNOHANG);
        /// Return if a stopped child has been resumed by delivery of
        /// [`Signal::Cont`].
        ///
        /// [`Signal::Cont`]: crate::process::Signal::Cont
        const CONTINUED = bitcast!(backend::process::wait::WCONTINUED);
        /// Wait for processed that have exited.
        const EXITED = bitcast!(backend::process::wait::WEXITED);
        /// Keep processed in a waitable state.
        const NOWAIT = bitcast!(backend::process::wait::WNOWAIT);
        /// Wait for processes that have been stopped.
        const STOPPED = bitcast!(backend::process::wait::WSTOPPED);

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

/// The status of a child process after calling [`wait`]/[`waitpid`].
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct WaitStatus(u32);

impl WaitStatus {
    /// Creates a `WaitStatus` out of an integer.
    #[inline]
    pub(crate) fn new(status: u32) -> Self {
        Self(status)
    }

    /// Converts a `WaitStatus` into its raw representation as an integer.
    #[inline]
    pub const fn as_raw(self) -> u32 {
        self.0
    }

    /// Returns whether the process is currently stopped.
    #[inline]
    pub fn stopped(self) -> bool {
        backend::process::wait::WIFSTOPPED(self.0 as _)
    }

    /// Returns whether the process has exited normally.
    #[inline]
    pub fn exited(self) -> bool {
        backend::process::wait::WIFEXITED(self.0 as _)
    }

    /// Returns whether the process was terminated by a signal.
    #[inline]
    pub fn signaled(self) -> bool {
        backend::process::wait::WIFSIGNALED(self.0 as _)
    }

    /// Returns whether the process has continued from a job control stop.
    #[inline]
    pub fn continued(self) -> bool {
        backend::process::wait::WIFCONTINUED(self.0 as _)
    }

    /// Returns the number of the signal that stopped the process, if the
    /// process was stopped by a signal.
    #[inline]
    pub fn stopping_signal(self) -> Option<u32> {
        if self.stopped() {
            Some(backend::process::wait::WSTOPSIG(self.0 as _) as _)
        } else {
            None
        }
    }

    /// Returns the exit status number returned by the process, if it exited
    /// normally.
    #[inline]
    pub fn exit_status(self) -> Option<u32> {
        if self.exited() {
            Some(backend::process::wait::WEXITSTATUS(self.0 as _) as _)
        } else {
            None
        }
    }

    /// Returns the number of the signal that terminated the process, if the
    /// process was terminated by a signal.
    #[inline]
    pub fn terminating_signal(self) -> Option<u32> {
        if self.signaled() {
            Some(backend::process::wait::WTERMSIG(self.0 as _) as _)
        } else {
            None
        }
    }
}

/// The status of a process after calling [`waitid`].
#[derive(Clone, Copy)]
#[repr(transparent)]
#[cfg(not(any(target_os = "openbsd", target_os = "redox", target_os = "wasi")))]
pub struct WaitIdStatus(pub(crate) backend::c::siginfo_t);

#[cfg(linux_raw)]
// SAFETY: `siginfo_t` does contain some raw pointers, such as the `si_ptr`
// and the `si_addr` fields, however it's up to users to use those correctly.
unsafe impl Send for WaitIdStatus {}

#[cfg(linux_raw)]
// SAFETY: Same as with `Send`.
unsafe impl Sync for WaitIdStatus {}

#[cfg(not(any(target_os = "openbsd", target_os = "redox", target_os = "wasi")))]
impl WaitIdStatus {
    /// Returns whether the process is currently stopped.
    #[inline]
    pub fn stopped(&self) -> bool {
        self.raw_code() == bitcast!(backend::c::CLD_STOPPED)
    }

    /// Returns whether the process is currently trapped.
    #[inline]
    pub fn trapped(&self) -> bool {
        self.raw_code() == bitcast!(backend::c::CLD_TRAPPED)
    }

    /// Returns whether the process has exited normally.
    #[inline]
    pub fn exited(&self) -> bool {
        self.raw_code() == bitcast!(backend::c::CLD_EXITED)
    }

    /// Returns whether the process was terminated by a signal and did not
    /// create a core file.
    #[inline]
    pub fn killed(&self) -> bool {
        self.raw_code() == bitcast!(backend::c::CLD_KILLED)
    }

    /// Returns whether the process was terminated by a signal and did create a
    /// core file.
    #[inline]
    pub fn dumped(&self) -> bool {
        self.raw_code() == bitcast!(backend::c::CLD_DUMPED)
    }

    /// Returns whether the process has continued from a job control stop.
    #[inline]
    pub fn continued(&self) -> bool {
        self.raw_code() == bitcast!(backend::c::CLD_CONTINUED)
    }

    /// Returns the number of the signal that stopped the process, if the
    /// process was stopped by a signal.
    #[inline]
    #[cfg(not(any(target_os = "emscripten", target_os = "fuchsia", target_os = "netbsd")))]
    pub fn stopping_signal(&self) -> Option<u32> {
        if self.stopped() {
            Some(self.si_status() as _)
        } else {
            None
        }
    }

    /// Returns the number of the signal that trapped the process, if the
    /// process was trapped by a signal.
    #[inline]
    #[cfg(not(any(target_os = "emscripten", target_os = "fuchsia", target_os = "netbsd")))]
    pub fn trapping_signal(&self) -> Option<u32> {
        if self.trapped() {
            Some(self.si_status() as _)
        } else {
            None
        }
    }

    /// Returns the exit status number returned by the process, if it exited
    /// normally.
    #[inline]
    #[cfg(not(any(target_os = "emscripten", target_os = "fuchsia", target_os = "netbsd")))]
    pub fn exit_status(&self) -> Option<u32> {
        if self.exited() {
            Some(self.si_status() as _)
        } else {
            None
        }
    }

    /// Returns the number of the signal that terminated the process, if the
    /// process was terminated by a signal.
    #[inline]
    #[cfg(not(any(target_os = "emscripten", target_os = "fuchsia", target_os = "netbsd")))]
    pub fn terminating_signal(&self) -> Option<u32> {
        if self.killed() || self.dumped() {
            Some(self.si_status() as _)
        } else {
            None
        }
    }

    /// Return the raw `si_signo` value returned from `waitid`.
    #[cfg(linux_raw)]
    pub fn raw_signo(&self) -> crate::ffi::c_int {
        self.0.si_signo()
    }

    /// Return the raw `si_signo` value returned from `waitid`.
    #[cfg(not(linux_raw))]
    pub fn raw_signo(&self) -> crate::ffi::c_int {
        self.0.si_signo
    }

    /// Return the raw `si_errno` value returned from `waitid`.
    #[cfg(linux_raw)]
    pub fn raw_errno(&self) -> crate::ffi::c_int {
        self.0.si_errno()
    }

    /// Return the raw `si_errno` value returned from `waitid`.
    #[cfg(not(linux_raw))]
    pub fn raw_errno(&self) -> crate::ffi::c_int {
        self.0.si_errno
    }

    /// Return the raw `si_code` value returned from `waitid`.
    #[cfg(linux_raw)]
    pub fn raw_code(&self) -> crate::ffi::c_int {
        self.0.si_code()
    }

    /// Return the raw `si_code` value returned from `waitid`.
    #[cfg(not(linux_raw))]
    pub fn raw_code(&self) -> crate::ffi::c_int {
        self.0.si_code
    }

    #[cfg(not(any(target_os = "emscripten", target_os = "fuchsia", target_os = "netbsd")))]
    #[allow(unsafe_code)]
    fn si_status(&self) -> crate::ffi::c_int {
        // SAFETY: POSIX [specifies] that the `siginfo_t` returned by a
        // `waitid` call always has a valid `si_status` value.
        //
        // [specifies]: https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/signal.h.html
        unsafe { self.0.si_status() }
    }
}

/// The identifier to wait on in a call to [`waitid`].
#[cfg(not(any(target_os = "openbsd", target_os = "redox", target_os = "wasi")))]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum WaitId<'a> {
    /// Wait on all processes.
    #[doc(alias = "P_ALL")]
    All,

    /// Wait for a specific process ID.
    #[doc(alias = "P_PID")]
    Pid(Pid),

    /// Wait for a specific process group ID, or the calling process' group ID.
    #[doc(alias = "P_PGID")]
    Pgid(Option<Pid>),

    /// Wait for a specific process file descriptor.
    #[cfg(target_os = "linux")]
    #[doc(alias = "P_PIDFD")]
    PidFd(BorrowedFd<'a>),

    /// Eat the lifetime for non-Linux platforms.
    #[doc(hidden)]
    #[cfg(not(target_os = "linux"))]
    __EatLifetime(core::marker::PhantomData<&'a ()>),
}

/// `waitpid(pid, waitopts)`—Wait for a specific process to change state.
///
/// If the pid is `None`, the call will wait for any child process whose
/// process group id matches that of the calling process.
///
/// Otherwise, the call will wait for the child process with the given pid.
///
/// On Success, returns the status of the selected process.
///
/// If `NOHANG` was specified in the options, and the selected child process
/// didn't change state, returns `None`.
///
/// # Bugs
///
/// This function does not currently support waiting for given process group
/// (the `< -1` case of `waitpid`); to do that, currently the [`waitpgid`] or
/// [`waitid`] function must be used.
///
/// This function does not currently support waiting for any process (the
/// `-1` case of `waitpid`); to do that, currently the [`wait`] function must
/// be used.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/wait.html
/// [Linux]: https://man7.org/linux/man-pages/man2/waitpid.2.html
#[doc(alias = "wait4")]
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn waitpid(pid: Option<Pid>, waitopts: WaitOptions) -> io::Result<Option<(Pid, WaitStatus)>> {
    backend::process::syscalls::waitpid(pid, waitopts)
}

/// `waitpid(-pgid, waitopts)`—Wait for a process in a specific process group
/// to change state.
///
/// The call will wait for any child process with the given pgid.
///
/// On Success, returns the status of the selected process.
///
/// If `NOHANG` was specified in the options, and no selected child process
/// changed state, returns `None`.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/wait.html
/// [Linux]: https://man7.org/linux/man-pages/man2/waitpid.2.html
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn waitpgid(pgid: Pid, waitopts: WaitOptions) -> io::Result<Option<(Pid, WaitStatus)>> {
    backend::process::syscalls::waitpgid(pgid, waitopts)
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
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/wait.html
/// [Linux]: https://man7.org/linux/man-pages/man2/waitpid.2.html
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn wait(waitopts: WaitOptions) -> io::Result<Option<(Pid, WaitStatus)>> {
    backend::process::syscalls::wait(waitopts)
}

/// `waitid(_, _, _, opts)`—Wait for the specified child process to change
/// state.
#[cfg(not(any(target_os = "openbsd", target_os = "redox", target_os = "wasi")))]
#[inline]
pub fn waitid<'a, Id: Into<WaitId<'a>>>(
    id: Id,
    options: WaitIdOptions,
) -> io::Result<Option<WaitIdStatus>> {
    backend::process::syscalls::waitid(id.into(), options)
}
