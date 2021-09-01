//! Unix user, group, and process identiiers.
//!
//! # Safety
//!
//! The `Uid`, `Gid`, and `Uid` types can be constructed from raw integers,
//! which is marked safe for similar reasons as [`FromRawFd::from_raw_fd`].
//!
//! [`FromRawFd::from_raw_fd`]: https://doc.rust-lang.org/std/os/unix/io/trait.FromRawFd.html#tymethod.from_raw_fd
#![allow(unsafe_code)]

use crate::imp;

/// The raw integer value of a Unix user ID.
pub use imp::process::RawUid;

/// The raw integer value of a Unix group ID.
pub use imp::process::RawGid;

/// The raw integer value of a Unix process ID.
pub use imp::process::RawPid;

/// `uid_t`—A Unix user ID.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Uid(RawUid);

/// `gid_t`—A Unix group ID.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Gid(RawGid);

/// `pid_t`—A Unix process ID.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Pid(RawPid);

impl Uid {
    /// A `Uid` corresponding to the root user (uid 0).
    pub const ROOT: Self = Self(0);

    /// Converts a `RawUid` into a `Uid`.
    ///
    /// # Safety
    ///
    /// `raw` must be the value of a valid Unix user ID.
    #[inline]
    pub const unsafe fn from_raw(raw: RawUid) -> Self {
        Self(raw)
    }

    /// Converts a `Uid` into a `RawUid`.
    #[inline]
    pub const fn as_raw(self) -> RawUid {
        self.0
    }
}

impl Gid {
    /// A `Gid` corresponding to the root group (gid 0).
    pub const ROOT: Self = Self(0);

    /// Converts a `RawGid` into a `Gid`.
    ///
    /// # Safety
    ///
    /// `raw` must be the value of a valid Unix group ID.
    #[inline]
    pub const unsafe fn from_raw(raw: RawGid) -> Self {
        Self(raw)
    }

    /// Converts a `Gid` into a `RawGid`.
    #[inline]
    pub const fn as_raw(self) -> RawGid {
        self.0
    }
}

impl Pid {
    /// A `Pid` corresponding to the init process (pid 1).
    pub const INIT: Self = Self(1);
    /// A `Pid` corresponding to no process (pid 0).
    pub const NONE: Self = Self(0);

    /// Converts a `RawPid` into a `Pid`.
    ///
    /// # Safety
    ///
    /// `raw` must be the value of a valid Unix process ID.
    #[inline]
    pub const unsafe fn from_raw(raw: RawPid) -> Self {
        Self(raw)
    }

    /// Converts a `Pid` into a `RawPid`.
    #[inline]
    pub const fn as_raw(self) -> RawPid {
        self.0
    }
}

/// `getuid()`—Returns the process' real user ID.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getuid.html
/// [Linux]: https://man7.org/linux/man-pages/man2/getuid.2.html
#[inline]
#[must_use]
pub fn getuid() -> Uid {
    imp::syscalls::getuid()
}

/// `geteuid()`—Returns the process' effective user ID.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/geteuid.html
/// [Linux]: https://man7.org/linux/man-pages/man2/geteuid.2.html
#[inline]
#[must_use]
pub fn geteuid() -> Uid {
    imp::syscalls::geteuid()
}

/// `getgid()`—Returns the process' real group ID.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getgid.html
/// [Linux]: https://man7.org/linux/man-pages/man2/getgid.2.html
#[inline]
#[must_use]
pub fn getgid() -> Gid {
    imp::syscalls::getgid()
}

/// `getegid()`—Returns the process' effective group ID.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getegid.html
/// [Linux]: https://man7.org/linux/man-pages/man2/getegid.2.html
#[inline]
#[must_use]
pub fn getegid() -> Gid {
    imp::syscalls::getegid()
}

/// `getpid()`—Returns the process' ID.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpid.html
/// [Linux]: https://man7.org/linux/man-pages/man2/getpid.2.html
#[inline]
#[must_use]
pub fn getpid() -> Pid {
    imp::syscalls::getpid()
}

/// `getppid()`—Returns the parent process' ID.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getppid.html
/// [Linux]: https://man7.org/linux/man-pages/man2/getppid.2.html
#[inline]
#[must_use]
pub fn getppid() -> Pid {
    imp::syscalls::getppid()
}
