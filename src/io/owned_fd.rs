//! A wrapper around `io_lifetimes::OwnedFd`.
//!
//! # Safety
//!
//! We wrap an `OwnedFd` in a `ManuallyDrop` so that we can extract the
//! file descriptor and close it ourselves.
#![allow(unsafe_code)]

use crate::io::{close, AsRawFd, FromRawFd};
use io_lifetimes::{AsFd, BorrowedFd};
#[cfg(not(io_lifetimes_use_std))]
use io_lifetimes::{FromFd, IntoFd};
use std::fmt;
use std::mem::{forget, ManuallyDrop};

/// A wrapper around `io_lifetimes::OwnedFd` which closes the file descriptor
/// using rsix's own `close` rather than libc's `close`.
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

#[cfg(io_lifetimes_use_std)]
impl From<OwnedFd> for io_lifetimes::OwnedFd {
    #[inline]
    fn from(owned_fd: OwnedFd) -> Self {
        // Safety: We use `as_fd().as_raw_fd()` to extract the raw file
        // descriptor from `self.inner`, and then `forget` `self` so
        // that they remain valid until the new `OwnedFd` acquires them.
        let raw_fd = owned_fd.inner.as_fd().as_raw_fd();
        forget(owned_fd);
        unsafe { io_lifetimes::OwnedFd::from_raw_fd(raw_fd) }
    }
}

#[cfg(not(io_lifetimes_use_std))]
impl IntoFd for OwnedFd {
    #[inline]
    fn into_fd(self) -> io_lifetimes::OwnedFd {
        // Safety: We use `as_fd().as_raw_fd()` to extract the raw file
        // descriptor from `self.inner`, and then `forget` `self` so
        // that they remain valid until the new `OwnedFd` acquires them.
        let raw_fd = self.inner.as_fd().as_raw_fd();
        forget(self);
        unsafe { io_lifetimes::OwnedFd::from_raw_fd(raw_fd) }
    }
}

#[cfg(io_lifetimes_use_std)]
impl From<io_lifetimes::OwnedFd> for OwnedFd {
    #[inline]
    fn from(owned_fd: io_lifetimes::OwnedFd) -> Self {
        Self {
            inner: ManuallyDrop::new(owned_fd),
        }
    }
}

#[cfg(not(io_lifetimes_use_std))]
impl FromFd for OwnedFd {
    #[inline]
    fn from_fd(owned_fd: io_lifetimes::OwnedFd) -> Self {
        Self {
            inner: ManuallyDrop::new(owned_fd),
        }
    }
}

#[cfg(not(io_lifetimes_use_std))]
impl From<io_lifetimes::OwnedFd> for OwnedFd {
    #[inline]
    fn from(fd: io_lifetimes::OwnedFd) -> Self {
        Self::from_fd(fd)
    }
}

#[cfg(not(io_lifetimes_use_std))]
impl From<OwnedFd> for io_lifetimes::OwnedFd {
    #[inline]
    fn from(fd: OwnedFd) -> Self {
        fd.into_fd()
    }
}

impl Drop for OwnedFd {
    #[inline]
    fn drop(&mut self) {
        // Safety: We use `as_fd().as_raw_fd()` to extract the raw file
        // descriptor from `self.inner`. `self.inner` is wrapped with
        // `ManuallyDrop` so dropping it doesn't invalid them.
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
