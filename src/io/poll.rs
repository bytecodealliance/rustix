use crate::{imp, io};
use std::{os::raw::c_int, vec::IntoIter};

pub use imp::io::{PollFd, PollFlags};

/// A [`Vec`] of `pollfd`.
///
/// [`Vec`]: std::vec::Vec
#[derive(Clone, Debug)]
pub struct PollFdVec<'fd> {
    fds: Vec<PollFd<'fd>>,
}

impl<'fd> PollFdVec<'fd> {
    /// Construct a new empty `PollFdVec`.
    #[inline]
    pub const fn new() -> Self {
        Self { fds: Vec::new() }
    }

    /// Append a fd.
    #[inline]
    pub fn push(&mut self, fd: PollFd<'fd>) {
        self.fds.push(fd)
    }
}

impl<'fd> IntoIterator for PollFdVec<'fd> {
    type IntoIter = IntoIter<PollFd<'fd>>;
    type Item = PollFd<'fd>;

    #[inline]
    fn into_iter(self) -> IntoIter<PollFd<'fd>> {
        self.fds.into_iter()
    }
}

impl<'fd> PollFdVec<'fd> {
    /// `poll(self.fds.as_mut_ptr(), self.fds.len(), timeout)`
    pub fn poll(&mut self, timeout: c_int) -> io::Result<usize> {
        imp::syscalls::poll(&mut self.fds, timeout)
    }
}
