use super::super::c;
use super::super::conv::borrowed_fd;
use crate::fd::{AsFd, AsRawFd, BorrowedFd};
use bitflags::bitflags;
use core::marker::PhantomData;

bitflags! {
    /// `POLL*` flags for use with [`poll`].
    ///
    /// [`poll`]: rustix::io::poll
    pub struct PollFlags: c::c_short {
        /// `POLLIN`
        const IN = c::POLLIN;
        /// `POLLPRI`
        #[cfg(not(target_os = "wasi"))]
        const PRI = c::POLLPRI;
        /// `POLLOUT`
        const OUT = c::POLLOUT;
        /// `POLLRDNORM`
        #[cfg(not(target_os = "redox"))]
        const RDNORM = c::POLLRDNORM;
        /// `POLLWRNORM`
        #[cfg(not(target_os = "redox"))]
        const WRNORM = c::POLLWRNORM;
        /// `POLLRDBAND`
        #[cfg(not(any(target_os = "redox", target_os = "wasi")))]
        const RDBAND = c::POLLRDBAND;
        /// `POLLWRBAND`
        #[cfg(not(any(target_os = "redox", target_os = "wasi")))]
        const WRBAND = c::POLLWRBAND;
        /// `POLLERR`
        const ERR = c::POLLERR;
        /// `POLLHUP`
        const HUP = c::POLLHUP;
        /// `POLLNVAL`
        const NVAL = c::POLLNVAL;
        /// `POLLRDHUP`
        #[cfg(all(
            any(target_os = "android", target_os = "linux"),
            not(any(target_arch = "sparc", target_arch = "sparc64")))
        )]
        const RDHUP = c::POLLRDHUP;
    }
}

/// `struct pollfd`—File descriptor and flags for use with [`poll`].
///
/// [`poll`]: rustix::io::poll
#[doc(alias = "pollfd")]
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct PollFd<'fd> {
    pollfd: c::pollfd,
    _phantom: PhantomData<BorrowedFd<'fd>>,
}

impl<'fd> PollFd<'fd> {
    /// Constructs a new `PollFd` holding `fd` and `events`.
    #[inline]
    pub fn new<Fd: AsFd>(fd: &'fd Fd, events: PollFlags) -> Self {
        Self::from_borrowed_fd(fd.as_fd(), events)
    }

    /// Sets the contained file descriptor to `fd`.
    #[inline]
    pub fn set_fd<Fd: AsFd>(&mut self, fd: &'fd Fd) {
        self.pollfd.fd = fd.as_fd().as_raw_fd();
    }

    /// Constructs a new `PollFd` holding `fd` and `events`.
    ///
    /// This is the same as `new`, but can be used to avoid borrowing the
    /// `BorrowedFd`, which can be tricky in situations where the `BorrowedFd`
    /// is a temporary.
    #[inline]
    pub fn from_borrowed_fd(fd: BorrowedFd<'fd>, events: PollFlags) -> Self {
        Self {
            pollfd: c::pollfd {
                fd: borrowed_fd(fd),
                events: events.bits(),
                revents: 0,
            },
            _phantom: PhantomData,
        }
    }

    /// Returns the ready events.
    #[inline]
    pub fn revents(&self) -> PollFlags {
        // Use `unwrap()` here because in theory we know we know all the bits
        // the OS might set here, but OS's have added extensions in the past.
        PollFlags::from_bits(self.pollfd.revents).unwrap()
    }
}

impl<'fd> AsFd for PollFd<'fd> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        // Safety:
        //
        // Our constructors and `set_fd` require `pollfd.fd` to be valid
        // for the `fd lifetime.
        unsafe { BorrowedFd::borrow_raw_fd(self.pollfd.fd) }
    }
}
