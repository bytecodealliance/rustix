use bitflags::bitflags;
use io_lifetimes::{AsFd, BorrowedFd};
use std::{io, marker::PhantomData};
#[cfg(libc)]
use {crate::negone_err, std::convert::TryInto, unsafe_io::os::posish::AsRawFd};

#[cfg(libc)]
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

#[cfg(linux_raw)]
bitflags! {
    pub struct PollFlags: u16 {
        const POLLIN = linux_raw_sys::general::POLLIN as u16;
        const POLLPRI = linux_raw_sys::general::POLLPRI as u16;
        const POLLOUT = linux_raw_sys::general::POLLOUT as u16;
        const POLLRDNORM = linux_raw_sys::general::POLLRDNORM as u16;
        const POLLWRNORM = linux_raw_sys::general::POLLWRNORM as u16;
        const POLLRDBAND = linux_raw_sys::general::POLLRDBAND as u16;
        const POLLWRBAND = linux_raw_sys::general::POLLWRBAND as u16;
        const POLLERR = linux_raw_sys::general::POLLERR as u16;
        const POLLHUP = linux_raw_sys::general::POLLHUP as u16;
        const POLLNVAL = linux_raw_sys::general::POLLNVAL as u16;
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
    /// # Safety
    ///
    /// `PollFd` does not take ownership of the file descriptors passed in
    /// here, and they need to live at least through the `PollFdVec::poll`
    /// call.
    #[inline]
    pub unsafe fn new<Fd: AsFd<'fd>>(fd: Fd, events: PollFlags) -> Self {
        Self::_new(fd.as_fd(), events)
    }

    #[cfg(libc)]
    #[inline]
    unsafe fn _new(fd: BorrowedFd<'fd>, events: PollFlags) -> Self {
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
    unsafe fn _new(fd: BorrowedFd<'fd>, events: PollFlags) -> Self {
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
