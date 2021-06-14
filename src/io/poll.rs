use bitflags::bitflags;
use io_lifetimes::{AsFd, BorrowedFd};
use std::{io, marker::PhantomData, vec::IntoIter};
#[cfg(libc)]
use {crate::negone_err, std::convert::TryInto, unsafe_io::os::posish::AsRawFd};

#[cfg(libc)]
bitflags! {
    /// `POLL*`
    pub struct PollFlags: libc::c_short {
        /// `POLLIN`
        const IN = libc::POLLIN;
        /// `POLLPRI`
        #[cfg(not(target_os = "wasi"))]
        const PRI = libc::POLLPRI;
        /// `POLLOUT`
        const OUT = libc::POLLOUT;
        /// `POLLRDNORM`
        #[cfg(not(target_os = "redox"))]
        const RDNORM = libc::POLLRDNORM;
        /// `POLLWRNORM`
        #[cfg(not(target_os = "redox"))]
        const WRNORM = libc::POLLWRNORM;
        /// `POLLRDBAND`
        #[cfg(not(any(target_os = "wasi", target_os = "redox")))]
        const RDBAND = libc::POLLRDBAND;
        /// `POLLWRBAND`
        #[cfg(not(any(target_os = "wasi", target_os = "redox")))]
        const WRBAND = libc::POLLWRBAND;
        /// `POLLERR`
        const ERR = libc::POLLERR;
        /// `POLLHUP`
        const HUP = libc::POLLHUP;
        /// `POLLNVAL`
        const NVAL = libc::POLLNVAL;
    }
}

#[cfg(linux_raw)]
bitflags! {
    /// `POLL*`
    pub struct PollFlags: u16 {
        /// `POLLIN`
        const IN = linux_raw_sys::general::POLLIN as u16;
        /// `POLLPRI`
        const PRI = linux_raw_sys::general::POLLPRI as u16;
        /// `POLLOUT`
        const OUT = linux_raw_sys::general::POLLOUT as u16;
        /// `POLLNORM`
        const RDNORM = linux_raw_sys::general::POLLRDNORM as u16;
        /// `POLLNORM`
        const WRNORM = linux_raw_sys::general::POLLWRNORM as u16;
        /// `POLLRDBAND`
        const RDBAND = linux_raw_sys::general::POLLRDBAND as u16;
        /// `POLLWRBAND`
        const WRBAND = linux_raw_sys::general::POLLWRBAND as u16;
        /// `POLLERR`
        const ERR = linux_raw_sys::general::POLLERR as u16;
        /// `POLLHUP`
        const HUP = linux_raw_sys::general::POLLHUP as u16;
        /// `POLLNVAL`
        const NVAL = linux_raw_sys::general::POLLNVAL as u16;
    }
}

/// `pollfd`
#[cfg(libc)]
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct PollFd<'fd> {
    pollfd: libc::pollfd,
    _phantom: PhantomData<BorrowedFd<'fd>>,
}

/// `pollfd`
#[cfg(linux_raw)]
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct PollFd<'fd> {
    pollfd: crate::linux_raw::PollFd<'fd>,
    _phantom: PhantomData<BorrowedFd<'fd>>,
}

impl<'fd> PollFd<'fd> {
    /// Construct a new `PollFd` holding `fd` and `events`.
    #[inline]
    pub fn new<Fd: AsFd<'fd>>(fd: Fd, events: PollFlags) -> Self {
        Self::_new(fd.as_fd(), events)
    }

    #[cfg(libc)]
    #[inline]
    fn _new(fd: BorrowedFd<'fd>, events: PollFlags) -> Self {
        Self {
            pollfd: libc::pollfd {
                fd: fd.as_raw_fd() as libc::c_int,
                events: events.bits(),
                revents: PollFlags::empty().bits(),
            },
            _phantom: PhantomData,
        }
    }

    #[cfg(linux_raw)]
    #[inline]
    fn _new(fd: BorrowedFd<'fd>, events: PollFlags) -> Self {
        Self {
            pollfd: crate::linux_raw::PollFd {
                fd,
                events: events.bits(),
                revents: 0,
            },
            _phantom: PhantomData,
        }
    }

    /// Return the ready events.
    #[inline]
    pub fn revents(self) -> PollFlags {
        PollFlags::from_bits(self.pollfd.revents).unwrap()
    }
}

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
    pub fn new() -> Self {
        Self { fds: Vec::new() }
    }

    /// Append a fd.
    #[inline]
    pub fn push(&mut self, fd: PollFd<'fd>) {
        self.fds.push(fd)
    }

    /// Consume self and return an iterator over the fds.
    #[inline]
    pub fn into_iter(self) -> IntoIter<PollFd<'fd>> {
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
            .map_err(|_convert_err| io::Error::from_raw_os_error(libc::EINVAL))?;

        let nready =
            negone_err(unsafe { libc::poll(self.fds.as_mut_ptr() as *mut _, nfds, timeout) })?;

        Ok(nready.try_into().unwrap())
    }
}

#[cfg(linux_raw)]
impl<'fd> PollFdVec<'fd> {
    /// `poll(self.fds.as_mut_ptr(), self.fds.len(), timeout)`
    pub fn poll(&mut self, timeout: std::os::raw::c_int) -> io::Result<usize> {
        // `Pollfd` is `repr(transparent)` so we can transmute slices of it.
        crate::linux_raw::poll(
            unsafe { std::mem::transmute(self.fds.as_mut_slice()) },
            timeout,
        )
    }
}
