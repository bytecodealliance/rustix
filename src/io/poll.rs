use crate::negone_err;
use bitflags::bitflags;
use std::{convert::TryInto, io};
use unsafe_io::{os::posish::AsRawFd, AsUnsafeHandle, UnsafeHandle};

bitflags! {
    pub struct PollFlags: libc::c_short {
        const POLLIN = libc::POLLIN;
        #[cfg(not(target_os = "wasi"))]
        const POLLPRI = libc::POLLPRI;
        const POLLOUT = libc::POLLOUT;
        #[cfg(not(target_os = "redox"))]
        const POLLRDNORM = libc::POLLRDNORM;
        #[cfg(not(target_os = "redox"))]
        const POLLWRNORM = libc::POLLWRNORM;
        #[cfg(not(any(target_os = "wasi", target_os = "redox")))]
        const POLLRDBAND = libc::POLLRDBAND;
        #[cfg(not(any(target_os = "wasi", target_os = "redox")))]
        const POLLWRBAND = libc::POLLWRBAND;
        const POLLERR = libc::POLLERR;
        const POLLHUP = libc::POLLHUP;
        const POLLNVAL = libc::POLLNVAL;
    }
}

/// `pollfd`
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct PollFd(libc::pollfd);

impl PollFd {
    /// # Safety
    ///
    /// `PollFd` does not take ownership of the file descriptors passed in here,
    /// and they need to live at least through the `PollFdVec::poll` call.
    #[inline]
    pub unsafe fn new<Fd: AsUnsafeHandle>(fd: &Fd, events: PollFlags) -> Self {
        let fd = fd.as_unsafe_handle();
        Self::_new(fd, events)
    }

    #[inline]
    unsafe fn _new(fd: UnsafeHandle, events: PollFlags) -> Self {
        Self(libc::pollfd {
            fd: fd.as_raw_fd() as libc::c_int,
            events: events.bits(),
            revents: PollFlags::empty().bits(),
        })
    }

    /// Return the ready events.
    #[inline]
    pub fn revents(self) -> PollFlags {
        PollFlags::from_bits(self.0.revents).unwrap()
    }
}

/// A [`Vec`] of `libc::pollfd`.
///
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PollFdVec {
    fds: Vec<libc::pollfd>,
}

impl PollFdVec {
    /// `poll(self.fds.as_mut_ptr(), self.fds.len(), timeout)`
    pub fn poll(&mut self, timeout: libc::c_int) -> io::Result<usize> {
        let nfds = self
            .fds
            .len()
            .try_into()
            .map_err(|_convert_err| io::Error::from_raw_os_error(libc::EINVAL))?;

        let nready = negone_err(unsafe { libc::poll(self.fds.as_mut_ptr(), nfds, timeout) })?;

        Ok(nready.try_into().unwrap())
    }
}
