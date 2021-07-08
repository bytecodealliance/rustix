use super::super::conv::borrowed_fd;
use bitflags::bitflags;
use io_lifetimes::{AsFd, BorrowedFd};
use std::marker::PhantomData;

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
        /// `POLLRDHUP`
        // TODO: Submitted to upstream libc:
        // <https://github.com/rust-lang/libc/pull/2247>
        #[cfg(not(any(target_arch = "sparc", target_arch = "sparc64")))]
        const RDHUP = 0x2000;
    }
}

/// `struct pollfd`
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct PollFd<'fd> {
    pollfd: libc::pollfd,
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
        Self {
            pollfd: libc::pollfd {
                fd: borrowed_fd(fd),
                events: events.bits(),
                revents: 0,
            },
            _phantom: PhantomData,
        }
    }

    /// Return the ready events.
    #[inline]
    pub fn revents(self) -> PollFlags {
        // Use `unwrap()` here because in theory we know we know all the bits
        // the OS might set here, but OS's have added extensions in the past.
        PollFlags::from_bits(self.pollfd.revents).unwrap()
    }
}
