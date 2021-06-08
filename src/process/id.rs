/// `getuid()`
#[cfg(libc)]
#[inline]
#[must_use]
pub fn getuid() -> u32 {
    unsafe { libc::getuid() }
}

/// `getuid()`
#[cfg(linux_raw)]
#[inline]
#[must_use]
pub fn getuid() -> u32 {
    crate::linux_raw::getuid() as u32
}

/// `getgid()`
#[cfg(libc)]
#[inline]
#[must_use]
pub fn getgid() -> u32 {
    unsafe { libc::getgid() }
}

/// `getgid()`
#[cfg(linux_raw)]
#[inline]
#[must_use]
pub fn getgid() -> u32 {
    crate::linux_raw::getgid() as u32
}

/// `getpid()`
#[cfg(libc)]
#[inline]
#[must_use]
pub fn getpid() -> u32 {
    let pid: i32 = unsafe { libc::getpid() };
    pid as u32
}

/// `getpid()`
#[cfg(linux_raw)]
#[inline]
#[must_use]
pub fn getpid() -> u32 {
    crate::linux_raw::getpid() as u32
}
