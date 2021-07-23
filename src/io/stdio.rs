//! Functions returning the stdio file descriptors.
//!
//! # Safety
//!
//! These access the file descriptors by absolute index value, and nothing
//! prevents them from being closed and reused. They should only be used in
//! `main` or other situations where one is in control of the process'
//! stdio streams.
#![allow(unsafe_code)]

use crate::imp;
use crate::io::{FromRawFd, RawFd};
use io_lifetimes::{BorrowedFd, OwnedFd};

/// `STDIN_FILENO`—Standard input, borrowed.
///
/// # Safety
///
/// The stdin file descriptor can be closed in which case the file descriptor
/// index value could be dynamically reused, potentially on a different thread.
/// Typically, it is only safe to call this from within `main` or in the
/// vicinity, where one knows there aren't any other threads yet and nothing
/// else has closed stdin.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stdin.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stdin.3.html
#[inline]
pub unsafe fn stdin() -> BorrowedFd<'static> {
    BorrowedFd::borrow_raw_fd(imp::io::STDIN_FILENO as RawFd)
}

/// `STDIN_FILENO`—Standard input, owned.
///
/// # Safety
///
/// This acquires ownership of the stdin file descriptor. If it's dropped,
/// subsequent newly created file descriptors may reuse the stdin file
/// descriptor number, confusing code that assumes that the stdin file
/// descriptor number is only used by stdin.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stdin.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stdin.3.html
#[inline]
pub unsafe fn take_stdin() -> OwnedFd {
    OwnedFd::from_raw_fd(imp::io::STDIN_FILENO as RawFd)
}

/// `STDOUT_FILENO`—Standard output, borrowed.
///
/// # Safety
///
/// The stdout file descriptor can be closed in which case the file descriptor
/// index value could be dynamically reused, potentially on a different thread.
/// Typically, it is only safe to call this from within `main` or in the
/// vicinity, where one knows there aren't any other threads yet and nothing
/// else has closed stdout.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stdout.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stdout.3.html
#[inline]
pub unsafe fn stdout() -> BorrowedFd<'static> {
    BorrowedFd::borrow_raw_fd(imp::io::STDOUT_FILENO as RawFd)
}

/// `STDOUT_FILENO`—Standard output, owned.
///
/// # Safety
///
/// This acquires ownership of the stdout file descriptor. If it's dropped,
/// subsequent newly created file descriptors may reuse the stdout file
/// descriptor number, confusing code that assumes that the stdout file
/// descriptor number is only used by stdout.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stdout.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stdout.3.html
#[inline]
pub unsafe fn take_stdout() -> OwnedFd {
    OwnedFd::from_raw_fd(imp::io::STDOUT_FILENO as RawFd)
}

/// `STDERR_FILENO`—Standard error, borrowed.
///
/// # Safety
///
/// The stderr file descriptor can be closed in which case the file descriptor
/// index value could be dynamically reused, potentially on a different thread.
/// Typically, it is only safe to call this from within `main` or in the
/// vicinity, where one knows there aren't any other threads yet and nothing
/// else has closed stderr.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stderr.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stderr.3.html
#[inline]
pub unsafe fn stderr() -> BorrowedFd<'static> {
    BorrowedFd::borrow_raw_fd(imp::io::STDERR_FILENO as RawFd)
}

/// `STDERR_FILENO`—Standard error, owned.
///
/// # Safety
///
/// This acquires ownership of the stderr file descriptor. If it's dropped,
/// subsequent newly created file descriptors may reuse the stderr file
/// descriptor number, confusing code that assumes that the stderr file
/// descriptor number is only used by stderr.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/stderr.html
/// [Linux]: https://man7.org/linux/man-pages/man3/stderr.3.html
#[inline]
pub unsafe fn take_stderr() -> OwnedFd {
    OwnedFd::from_raw_fd(imp::io::STDERR_FILENO as RawFd)
}
