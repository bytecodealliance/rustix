use crate::imp;
use bitflags::bitflags;

bitflags! {
    /// Options for modifying the behavior of wait/waitpid
    pub struct WaitOptions: u32 {
        /// Return immediately if no child has exited.
        const NOHANG = imp::process::WNOHANG as _;
        /// Return if a child has stopped (but not traced via `ptrace(2)`)
        const UNTRACED = imp::process::WUNTRACED as _;
        /// Return if a stopped child has been resumed by delivery of `SIGCONT`
        const CONTINUED = imp::process::WCONTINUED as _;
    }
}

/// the status of the child processes the caller waited on
#[derive(Debug, Clone, Copy)]
pub struct WaitStatus(u32);

impl WaitStatus {
    /// create a `WaitStatus` out of an integer.
    #[inline]
    pub(crate) fn new(status: u32) -> Self {
        WaitStatus(status)
    }

    /// Converts a `WaitStatus` into its raw representation as an integer.
    #[inline]
    pub const fn as_raw(self) -> u32 {
        self.0
    }

    /// Returns whether the process is currently stopped.
    #[inline]
    pub fn stopped(self) -> bool {
        imp::process::WIFSTOPPED(self.0 as _)
    }

    /// Returns whether the process has continued from a job control stop.
    #[inline]
    pub fn continued(self) -> bool {
        imp::process::WIFCONTINUED(self.0 as _)
    }

    /// Returns the number of the signal that stopped the process,
    /// if the process was stopped by a signal.
    #[inline]
    pub fn stopping_signal(self) -> Option<u32> {
        if self.stopped() {
            Some(imp::process::WSTOPSIG(self.0 as _) as _)
        } else {
            None
        }
    }

    /// Returns the exit status number returned by the process,
    /// if it exited normally.
    #[inline]
    pub fn exit_status(self) -> Option<u32> {
        if imp::process::WIFEXITED(self.0 as _) {
            Some(imp::process::WEXITSTATUS(self.0 as _) as _)
        } else {
            None
        }
    }

    /// Returns the number of the signal that terminated the process,
    /// if the process was terminated by a signal.
    #[inline]
    pub fn terminating_signal(self) -> Option<u32> {
        if imp::process::WIFSIGNALED(self.0 as _) {
            Some(imp::process::WTERMSIG(self.0 as _) as _)
        } else {
            None
        }
    }
}
