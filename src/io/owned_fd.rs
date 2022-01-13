//! A wrapper around `io_lifetimes::OwnedFd`.
//!
//! rustix needs to wrap `OwnedFd` so that it can call its own [`close`]
//! function when the `OwnedFd` is dropped.
//!
//! [`close`]: crate::io::close
//!
//! # Safety
//!
//! We wrap an `OwnedFd` in a `ManuallyDrop` so that we can extract the
//! file descriptor and close it ourselves.
#![allow(unsafe_code)]

#[cfg(windows)]
use crate::imp::fd::AsSocketAsFd;
use crate::imp::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(all(not(io_lifetimes_use_std), feature = "std"))]
use crate::imp::fd::{FromFd, IntoFd};
use crate::io::close;
use core::fmt;
use core::mem::{forget, ManuallyDrop};

/// A wrapper around `io_lifetimes::OwnedFd` which closes the file descriptor
/// using `rustix`'s own [`close`] rather than libc's `close`.
///
/// [`close`]: crate::io::close
#[repr(transparent)]
pub struct OwnedFd {
    inner: ManuallyDrop<crate::imp::fd::OwnedFd>,
}

impl AsFd for OwnedFd {
    #[cfg(not(windows))]
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.inner.as_fd()
    }

    #[cfg(windows)]
    #[inline]
    fn as_socket(&self) -> BorrowedFd<'_> {
        self.inner.as_socket()
    }
}

#[cfg(io_lifetimes_use_std)]
impl From<OwnedFd> for crate::imp::fd::OwnedFd {
    #[inline]
    fn from(owned_fd: OwnedFd) -> Self {
        // Safety: We use `as_fd().as_raw_fd()` to extract the raw file
        // descriptor from `self.inner`, and then `forget` `self` so
        // that they remain valid until the new `OwnedFd` acquires them.
        let raw_fd = owned_fd.inner.as_fd().as_raw_fd();
        forget(owned_fd);
        unsafe { crate::imp::fd::OwnedFd::from_raw_fd(raw_fd) }
    }
}

#[cfg(not(any(io_lifetimes_use_std, not(feature = "std"))))]
impl IntoFd for OwnedFd {
    #[inline]
    fn into_fd(self) -> crate::imp::fd::OwnedFd {
        // Safety: We use `as_fd().as_raw_fd()` to extract the raw file
        // descriptor from `self.inner`, and then `forget` `self` so
        // that they remain valid until the new `OwnedFd` acquires them.
        let raw_fd = self.inner.as_fd().as_raw_fd();
        forget(self);
        unsafe { crate::imp::fd::OwnedFd::from_raw_fd(raw_fd) }
    }
}

#[cfg(any(io_lifetimes_use_std, not(feature = "std")))]
impl From<crate::imp::fd::OwnedFd> for OwnedFd {
    #[inline]
    fn from(owned_fd: crate::imp::fd::OwnedFd) -> Self {
        Self {
            inner: ManuallyDrop::new(owned_fd),
        }
    }
}

#[cfg(all(not(io_lifetimes_use_std), feature = "std"))]
impl FromFd for OwnedFd {
    #[inline]
    fn from_fd(owned_fd: crate::imp::fd::OwnedFd) -> Self {
        Self {
            inner: ManuallyDrop::new(owned_fd),
        }
    }
}

#[cfg(not(any(io_lifetimes_use_std, not(feature = "std"))))]
impl From<crate::imp::fd::OwnedFd> for OwnedFd {
    #[inline]
    fn from(fd: crate::imp::fd::OwnedFd) -> Self {
        Self {
            inner: ManuallyDrop::new(fd),
        }
    }
}

#[cfg(not(any(io_lifetimes_use_std, not(feature = "std"))))]
impl From<OwnedFd> for crate::imp::fd::OwnedFd {
    #[inline]
    fn from(fd: OwnedFd) -> Self {
        // Safety: We use `as_fd().as_raw_fd()` to extract the raw file
        // descriptor from `self.inner`, and then `forget` `self` so
        // that they remain valid until the new `OwnedFd` acquires them.
        let raw_fd = fd.inner.as_fd().as_raw_fd();
        forget(fd);
        unsafe { crate::imp::fd::OwnedFd::from_raw_fd(raw_fd) }
    }
}

impl AsRawFd for OwnedFd {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.inner.as_raw_fd()
    }
}

impl IntoRawFd for OwnedFd {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        let raw_fd = self.inner.as_fd().as_raw_fd();
        forget(self);
        raw_fd
    }
}

impl FromRawFd for OwnedFd {
    #[inline]
    unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
        Self {
            inner: ManuallyDrop::new(crate::imp::fd::OwnedFd::from_raw_fd(raw_fd)),
        }
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
