use super::super::fd::{AsFd, BorrowedFd};
use super::super::wasi_filesystem;
use bitflags::bitflags;
use core::marker::PhantomData;

bitflags! {
    /// `POLL*` flags for use with [`poll`].
    ///
    /// [`poll`]: rustix::io::poll
    pub struct PollFlags: u16 {
        /* FIXME: poll flags
        /// `POLLIN`
        const IN = wasi_poll::POLLIN;
        /// `POLLPRI`
        /// `POLLOUT`
        const OUT = wasi_poll::POLLOUT;
        /// `POLLRDNORM`
        const RDNORM = wasi_poll::POLLRDNORM;
        /// `POLLWRNORM`
        const WRNORM = wasi_poll::POLLWRNORM;
        /// `POLLERR`
        const ERR = wasi_poll::POLLERR;
        /// `POLLHUP`
        const HUP = wasi_poll::POLLHUP;
        /// `POLLNVAL`
        const NVAL = wasi_poll::POLLNVAL;
        /// `POLLRDHUP`
        const RDHUP = wasi_poll::POLLRDHUP;
        */
    }
}

/// `struct pollfd`—File descriptor and flags for use with [`poll`].
///
/// [`poll`]: rustix::io::poll
#[doc(alias = "pollfd")]
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct PollFd<'fd> {
    /*
    pollfd: wasi_poll::pollfd,
    */
    _phantom: PhantomData<BorrowedFd<'fd>>,
}

impl<'fd> PollFd<'fd> {
    /// Constructs a new `PollFd` holding `fd` and `events`.
    #[inline]
    pub fn new<Fd: AsFd>(fd: &'fd Fd, events: PollFlags) -> Self {
        Self::from_borrowed_fd(fd.as_fd(), events)
    }

    /// Constructs a new `PollFd` holding `fd` and `events`.
    ///
    /// This is the same as `new`, but can be used to avoid borrowing the
    /// `BorrowedFd`, which can be tricky in situations where the `BorrowedFd`
    /// is a temporary.
    #[inline]
    pub fn from_borrowed_fd(fd: BorrowedFd<'fd>, events: PollFlags) -> Self {
        todo!("from_borrowed_fd")
        /*
        Self {
            pollfd: wasi_poll::pollfd {
                fd: borrowed_fd(fd),
                events: events.bits(),
                revents: 0,
            },
            _phantom: PhantomData,
        }
        */
    }

    /// Return the ready events.
    #[inline]
    pub fn revents(&self) -> PollFlags {
        todo!("revents")
        /*
        // Use `unwrap()` here because in theory we know we know all the bits
        // the OS might set here, but OS's have added extensions in the past.
        PollFlags::from_bits(self.pollfd.revents).unwrap()
        */
    }
}
