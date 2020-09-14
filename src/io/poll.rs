use crate::negone_err;
use bitflags::bitflags;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, RawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, RawFd};
use std::{convert::TryInto, io};

bitflags! {
    pub struct PollFlags: libc::c_short {
        const POLLIN = libc::POLLIN;
        const POLLPRI = libc::POLLPRI;
        const POLLOUT = libc::POLLOUT;
        const POLLRDNORM = libc::POLLRDNORM;
        const POLLWRNORM = libc::POLLWRNORM;
        const POLLRDBAND = libc::POLLRDBAND;
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
    pub unsafe fn new<Fd: AsRawFd>(fd: &Fd, events: PollFlags) -> Self {
        let fd = fd.as_raw_fd();
        Self::_new(fd, events)
    }

    unsafe fn _new(fd: RawFd, events: PollFlags) -> Self {
        Self(libc::pollfd {
            fd: fd as libc::c_int,
            events: events.bits(),
            revents: PollFlags::empty().bits(),
        })
    }

    /// Return the ready events.
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
            .map_err(|_| io::Error::from_raw_os_error(libc::EINVAL))?;

        let nready = negone_err(unsafe { libc::poll(self.fds.as_mut_ptr(), nfds, timeout) })?;

        Ok(nready.try_into().unwrap())
    }
}
