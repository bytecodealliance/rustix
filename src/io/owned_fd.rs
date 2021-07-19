use crate::io::{close, AsRawFd, FromRawFd};
use io_lifetimes::{AsFd, BorrowedFd, FromFd, IntoFd};
use std::{fmt, mem::ManuallyDrop};

/// A wrapper around `io_lifetimes::OwnedFd` which closes the file descriptor
/// using posish's own `close` rather than libc's `close`.
#[repr(transparent)]
pub struct OwnedFd {
    inner: ManuallyDrop<io_lifetimes::OwnedFd>,
}

impl AsFd for OwnedFd {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.inner.as_fd()
    }
}

impl IntoFd for OwnedFd {
    #[inline]
    fn into_fd(self) -> io_lifetimes::OwnedFd {
        unsafe { io_lifetimes::OwnedFd::from_raw_fd(self.inner.as_fd().as_raw_fd()) }
    }
}

impl FromFd for OwnedFd {
    #[inline]
    fn from_fd(owned_fd: io_lifetimes::OwnedFd) -> Self {
        Self {
            inner: ManuallyDrop::new(owned_fd),
        }
    }
}

impl From<io_lifetimes::OwnedFd> for OwnedFd {
    fn from(fd: io_lifetimes::OwnedFd) -> Self {
        Self::from_fd(fd)
    }
}

impl From<OwnedFd> for io_lifetimes::OwnedFd {
    fn from(fd: OwnedFd) -> Self {
        fd.into_fd()
    }
}

impl Drop for OwnedFd {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            close(self.as_fd().as_raw_fd());
        }
    }
}

impl fmt::Debug for OwnedFd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
