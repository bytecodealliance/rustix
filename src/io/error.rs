//! The `Error` type, which is a minimal wrapper around an errno value.
//!
//! We define the errno constants as invididual `const`s instead of an
//! enum because we may not know about all of the host's errno values
//! and we don't want unrecognized values to create UB.

#![allow(missing_docs)]

use crate::imp;
use std::{error, fmt, result};

/// A specialized `Result` type for posish APIs.
pub type Result<T> = result::Result<T, Error>;

/// `errno`
///
/// The error type for posish APIs. This is similar to `std::io::Error`, but
/// only holds an OS error code, and no extra error value.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/errno.html
/// [Linux]: https://man7.org/linux/man-pages/man3/errno.3.html
pub use imp::io::Error;

impl Error {
    /// Shorthand for `std::io::Error::from(self).kind()`.
    #[inline]
    pub fn kind(self) -> std::io::ErrorKind {
        std::io::Error::from(self).kind()
    }

    /// Extract the raw OS error number from this error.
    #[inline]
    pub const fn raw_os_error(self) -> i32 {
        // This should be `i32::from` but that isn't a `const fn`.
        // Fortunately, we know `as i32` won't overflow.
        self.0 as i32
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::io::Error::from(*self).fmt(fmt)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::io::Error::from(*self).fmt(fmt)
    }
}

impl error::Error for Error {}

impl From<Error> for std::io::Error {
    #[inline]
    fn from(err: Error) -> Self {
        Self::from_raw_os_error(err.0 as _)
    }
}
