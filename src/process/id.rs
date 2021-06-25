use crate::imp;

/// `getuid()`
#[inline]
#[must_use]
pub fn getuid() -> u32 {
    imp::syscalls::getuid()
}

/// `geteuid()`
#[inline]
#[must_use]
pub fn geteuid() -> u32 {
    imp::syscalls::geteuid()
}

/// `getgid()`
#[inline]
#[must_use]
pub fn getgid() -> u32 {
    imp::syscalls::getgid()
}

/// `getegid()`
#[inline]
#[must_use]
pub fn getegid() -> u32 {
    imp::syscalls::getegid()
}

/// `getpid()`
#[inline]
#[must_use]
pub fn getpid() -> u32 {
    imp::syscalls::getpid()
}

/// `getppid()`
#[inline]
#[must_use]
pub fn getppid() -> u32 {
    imp::syscalls::getppid()
}
