use crate::io;
use std::vec::IntoIter;
#[cfg(libc)]
use {crate::negone_err, std::convert::TryInto};

#[cfg(libc)]
pub use super::poll_fd::PollFlags;

#[cfg(linux_raw)]
pub use crate::linux_raw::PollFlags;

#[cfg(libc)]
pub use super::poll_fd::PollFd;

#[cfg(linux_raw)]
pub use crate::linux_raw::PollFd;

/// A [`Vec`] of `pollfd`.
///
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
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

#[cfg(libc)]
impl<'fd> PollFdVec<'fd> {
    /// `poll(self.fds.as_mut_ptr(), self.fds.len(), timeout)`
    pub fn poll(&mut self, timeout: libc::c_int) -> io::Result<usize> {
        let nfds = self
            .fds
            .len()
            .try_into()
            .map_err(|_convert_err| io::Error::INVAL)?;

        let nready =
            negone_err(unsafe { libc::poll(self.fds.as_mut_ptr().cast::<_>(), nfds, timeout) })?;

        Ok(nready as usize)
    }
}

#[cfg(linux_raw)]
impl<'fd> PollFdVec<'fd> {
    /// `poll(self.fds.as_mut_ptr(), self.fds.len(), timeout)`
    pub fn poll(&mut self, timeout: std::os::raw::c_int) -> io::Result<usize> {
        // `PollFd` is `repr(transparent)` so we can transmute slices of it.
        crate::linux_raw::poll(&mut self.fds, timeout)
    }
}
