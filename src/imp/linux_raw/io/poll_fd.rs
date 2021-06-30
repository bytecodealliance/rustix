use bitflags::bitflags;
use io_lifetimes::{AsFd, BorrowedFd};

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
        /// `POLLRDHUP`
        const RDHUP = linux_raw_sys::general::POLLRDHUP as u16;
    }
}

/// `struct pollfd`
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PollFd<'fd> {
    pub(crate) fd: BorrowedFd<'fd>,
    pub(crate) events: u16,
    pub(crate) revents: u16,
}

impl<'fd> PollFd<'fd> {
    /// Construct a new `PollFd` holding `fd` and `events`.
    #[inline]
    pub fn new<Fd: AsFd>(fd: &'fd Fd, events: PollFlags) -> Self {
        Self::from_borrowed_fd(fd.as_fd(), events)
    }

    /// Construct a new `PollFd` holding `fd` and `events`.
    ///
    /// This is the same as `new`, but can be used to avoid borrowing the
    /// `BorrowedFd`, which can be tricky in situations where the `BorrowedFd`
    /// is a temporary.
    #[inline]
    pub fn from_borrowed_fd(fd: BorrowedFd<'fd>, events: PollFlags) -> Self {
        Self {
            fd,
            events: events.bits(),
            revents: 0,
        }
    }

    /// Return the ready events.
    #[inline]
    pub fn revents(self) -> PollFlags {
        // Use `unwrap()` here because in theory we know we know all the bits
        // the OS might set here, but OS's have added extensions in the past.
        PollFlags::from_bits(self.revents).unwrap()
    }
}
