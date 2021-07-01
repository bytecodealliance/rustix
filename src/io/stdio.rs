//! Functions returning the stdio file descriptors.
//!
//! # Safety
//!
//! These access the file descriptors by absolute index value, and nothing
//! prevents them from being closed and reused. They should only be used in
//! `main` or other situations where one is in control of the process'
//! stdio streams.
#![allow(unsafe_code)]

use crate::{imp, io::RawFd};
use io_lifetimes::BorrowedFd;

/// `STDIN_FILENO`
///
/// # Safety
///
/// The stdin file descriptor can be closed in which case the file descriptor
/// index value could be dynamically reused, potentially on a different thread.
/// Typically, it is only safe to call this from within `main` or in the
/// vicinity, where one knows there aren't any other threads yet and nothing
/// else has closed stdin.
pub unsafe fn stdin() -> BorrowedFd<'static> {
    BorrowedFd::borrow_raw_fd(imp::io::STDIN_FILENO as RawFd)
}

/// `STDOUT_FILENO`
///
/// # Safety
///
/// The stdout file descriptor can be closed in which case the file descriptor
/// index value could be dynamically reused, potentially on a different thread.
/// Typically, it is only safe to call this from within `main` or in the
/// vicinity, where one knows there aren't any other threads yet and nothing
/// else has closed stdout.
pub unsafe fn stdout() -> BorrowedFd<'static> {
    BorrowedFd::borrow_raw_fd(imp::io::STDOUT_FILENO as RawFd)
}

/// `STDERR_FILENO`
///
/// # Safety
///
/// The stderr file descriptor can be closed in which case the file descriptor
/// index value could be dynamically reused, potentially on a different thread.
/// Typically, it is only safe to call this from within `main` or in the
/// vicinity, where one knows there aren't any other threads yet and nothing
/// else has closed stderr.
pub unsafe fn stderr() -> BorrowedFd<'static> {
    BorrowedFd::borrow_raw_fd(imp::io::STDERR_FILENO as RawFd)
}
