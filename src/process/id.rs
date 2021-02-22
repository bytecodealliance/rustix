/// `getuid()`
#[inline]
#[must_use]
pub fn getuid() -> u32 {
    unsafe { libc::getuid() }
}

/// `getgid()`
#[inline]
#[must_use]
pub fn getgid() -> u32 {
    unsafe { libc::getgid() }
}

/// `getpid()`
#[inline]
#[must_use]
pub fn getpid() -> u32 {
    let pid: i32 = unsafe { libc::getpid() };
    pid as u32
}
