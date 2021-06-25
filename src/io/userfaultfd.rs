//! The Linux `userfaultfd` API.
//!
//! # Safety
//!
//! Calling `userfaultfd` is safe, but the returned file descriptor lets users
//! observe and manipulate process memory in magical ways.
#![allow(unsafe_code)]

use crate::{imp, io};
use io_lifetimes::OwnedFd;

pub use imp::io::UserFaultFdFlags;

/// `userfaultfd(flags)`
///
/// # Safety
///
/// The call itself is safe, but the returned file descriptor lets users
/// observe and manipuate process memory in magical ways.
#[inline]
pub unsafe fn userfaultfd(flags: UserFaultFdFlags) -> io::Result<OwnedFd> {
    imp::syscalls::userfaultfd(flags)
}
