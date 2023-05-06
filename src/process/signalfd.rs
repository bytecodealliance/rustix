//! A file descriptor that waits on a signal.

use super::kill::Signal;
use crate::backend::{
    c,
    process::{sigset, syscalls},
};
use crate::fd::{AsFd, OwnedFd};
use crate::io;

/// A set of signals.
#[derive(Debug, Copy, Clone)]
pub struct SigSet(pub(crate) c::sigset_t);

impl SigSet {
    /// Create a new, empty signal set.
    #[doc(alias = "sigemptyset")]
    pub fn new() -> Self {
        Self(sigset::new_sigset())
    }

    /// Add a signal to the set.
    #[doc(alias = "sigaddset")]
    pub fn add(&mut self, sig: Signal) {
        sigset::add_sig(&mut self.0, sig);
    }

    /// Remove a signal from the set.
    #[doc(alias = "sigdelset")]
    pub fn remove(&mut self, sig: Signal) {
        sigset::del_sig(&mut self.0, sig);
    }

    /// Check if a signal is in the set.
    #[doc(alias = "sigismember")]
    pub fn contains(&self, sig: Signal) -> bool {
        sigset::has_sig(&self.0, sig)
    }
}

impl Default for SigSet {
    fn default() -> Self {
        Self::new()
    }
}

bitflags::bitflags! {
    /// The flags that can be passed to `signalfd`.
    pub struct SignalfdFlags : c::c_int {
        /// Set the `O_NONBLOCK` flag on the file descriptor.
        const NONBLOCK = c::SFD_NONBLOCK;

        /// Set the `O_CLOEXEC` flag on the file descriptor.
        const CLOEXEC = c::SFD_CLOEXEC;
    }
}

/// `signalfd(-1, set, flags)` - Create a new file descriptor for handling signals.
///
/// # References
///
/// - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/signalfd.2.html
#[doc(alias = "signalfd")]
pub fn signalfd_create(set: &SigSet, flags: SignalfdFlags) -> io::Result<OwnedFd> {
    syscalls::signalfd_create(set, flags)
}

/// `signalfd(fd, set, flags)` - Change the signal set or flags of an existing file descriptor.
///
/// # References
///
/// - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/signalfd.2.html
#[doc(alias = "signalfd")]
pub fn signalfd_modify<F: AsFd>(fd: F, set: &SigSet, flags: SignalfdFlags) -> io::Result<()> {
    syscalls::signalfd_modify(fd.as_fd(), set, flags)
}
