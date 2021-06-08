#[cfg(all(
    libc,
    not(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "ios",
        target_os = "redox",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "emscripten",
        target_os = "wasi",
    ))
))]
use std::ptr;
#[cfg(libc)]
use {crate::zero_ok, std::mem::MaybeUninit};

#[cfg(libc)]
pub use libc::timespec;

#[cfg(linux_raw)]
pub use linux_raw_sys::general::timespec;

/// `clockid_t`
#[cfg(all(
    libc,
    not(any(target_os = "macos", target_os = "ios", target_os = "wasi"))
))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(i32)]
#[non_exhaustive]
pub enum ClockId {
    /// `CLOCK_REALTIME`
    Realtime = libc::CLOCK_REALTIME,

    /// `CLOCK_MONOTONIC`
    Monotonic = libc::CLOCK_MONOTONIC,

    /// `CLOCK_PROCESS_CPUTIME_ID`
    #[cfg(not(any(target_os = "netbsd", target_os = "openbsd")))]
    ProcessCPUTime = libc::CLOCK_PROCESS_CPUTIME_ID,

    /// `CLOCK_THREAD_CPUTIME_ID`
    #[cfg(not(any(target_os = "netbsd", target_os = "openbsd")))]
    ThreadCPUTime = libc::CLOCK_THREAD_CPUTIME_ID,
}

/// `clockid_t`
#[cfg(all(libc, any(target_os = "macos", target_os = "ios")))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum ClockId {
    /// `CLOCK_REALTIME`
    Realtime = libc::CLOCK_REALTIME,

    /// `CLOCK_MONOTONIC`
    Monotonic = libc::CLOCK_MONOTONIC,

    /// `CLOCK_PROCESS_CPUTIME_ID`
    ProcessCPUTime = libc::CLOCK_PROCESS_CPUTIME_ID,

    /// `CLOCK_THREAD_CPUTIME_ID`
    ThreadCPUTime = libc::CLOCK_THREAD_CPUTIME_ID,
}

/// `clockid_t`
#[cfg(linux_raw)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum ClockId {
    /// `CLOCK_REALTIME`
    Realtime = linux_raw_sys::general::CLOCK_REALTIME,

    /// `CLOCK_MONOTONIC`
    Monotonic = linux_raw_sys::general::CLOCK_MONOTONIC,

    /// `CLOCK_PROCESS_CPUTIME_ID`
    ProcessCPUTime = linux_raw_sys::general::CLOCK_PROCESS_CPUTIME_ID,

    /// `CLOCK_THREAD_CPUTIME_ID`
    ThreadCPUTime = linux_raw_sys::general::CLOCK_THREAD_CPUTIME_ID,
}

/// `clock_getres(id)`
#[cfg(all(libc, not(target_os = "wasi")))]
#[inline]
#[must_use]
pub fn clock_getres(id: ClockId) -> timespec {
    let mut timespec = MaybeUninit::<timespec>::uninit();
    zero_ok(unsafe { libc::clock_getres(id as libc::clockid_t, timespec.as_mut_ptr()) }).unwrap();
    unsafe { timespec.assume_init() }
}

/// `clock_getres(id)`
#[cfg(linux_raw)]
#[inline]
#[must_use]
pub fn clock_getres(id: ClockId) -> linux_raw_sys::general::timespec {
    crate::linux_raw::clock_getres(id as i32)
}

/// `clock_gettime(id)`
#[cfg(all(libc, not(target_os = "wasi")))]
#[inline]
#[must_use]
pub fn clock_gettime(id: ClockId) -> timespec {
    let mut timespec = MaybeUninit::<timespec>::uninit();
    zero_ok(unsafe { libc::clock_gettime(id as libc::clockid_t, timespec.as_mut_ptr()) }).unwrap();
    unsafe { timespec.assume_init() }
}

/// `clock_gettime(id)`
#[cfg(linux_raw)]
#[inline]
#[must_use]
pub fn clock_gettime(id: ClockId) -> linux_raw_sys::general::timespec {
    crate::linux_raw::clock_gettime(id as i32)
}

/// `clock_nanosleep(id, 0, request, remain)`
#[cfg(all(libc, not(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "ios",
    target_os = "redox",
    target_os = "freebsd", // FreeBSD 12 has clock_nanosleep, but libc targets FreeBSD 11.
    target_os = "openbsd",
    target_os = "emscripten",
    target_os = "wasi",
))))]
#[inline]
pub fn clock_nanosleep_relative(id: ClockId, request: &timespec) -> Result<(), timespec> {
    let mut remain = MaybeUninit::<timespec>::uninit();
    let flags = 0;
    zero_ok(unsafe {
        libc::clock_nanosleep(id as libc::clockid_t, flags, request, remain.as_mut_ptr())
    })
    .map_err(|_| unsafe { remain.assume_init() })
}

/// `clock_nanosleep(id, 0, request, remain)`
#[cfg(linux_raw)]
#[inline]
pub fn clock_nanosleep_relative(
    id: ClockId,
    request: &linux_raw_sys::general::timespec,
) -> Result<(), linux_raw_sys::general::timespec> {
    crate::linux_raw::clock_nanosleep_relative(id as i32, request)
}

/// `clock_nanosleep(id, TIMER_ABSTIME, request, NULL)`
#[cfg(all(libc, not(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "ios",
    target_os = "redox",
    target_os = "freebsd", // FreeBSD 12 has clock_nanosleep, but libc targets FreeBSD 11.
    target_os = "openbsd",
    target_os = "emscripten",
    target_os = "wasi",
))))]
#[inline]
pub fn clock_nanosleep_absolute(id: ClockId, request: &timespec) -> Result<(), ()> {
    let flags = libc::TIMER_ABSTIME;
    zero_ok(unsafe {
        libc::clock_nanosleep(id as libc::clockid_t, flags, request, ptr::null_mut())
    })
    .map_err(|_| ())
}

/// `clock_nanosleep(id, TIMER_ABSTIME, request, NULL)`
#[cfg(linux_raw)]
#[inline]
pub fn clock_nanosleep_absolute(
    id: ClockId,
    request: &linux_raw_sys::general::timespec,
) -> Result<(), ()> {
    crate::linux_raw::clock_nanosleep_absolute(id as i32, request).map_err(|_err| ())
}

/// `nanosleep(request, remain)`
#[cfg(libc)]
#[inline]
pub fn nanosleep(request: &timespec) -> Result<(), timespec> {
    let mut remain = MaybeUninit::<timespec>::uninit();
    zero_ok(unsafe { libc::nanosleep(request, remain.as_mut_ptr()) })
        .map_err(|_| unsafe { remain.assume_init() })
}

/// `nanosleep(request, remain)`
#[cfg(linux_raw)]
#[inline]
pub fn nanosleep(
    request: &linux_raw_sys::general::timespec,
) -> Result<(), linux_raw_sys::general::timespec> {
    crate::linux_raw::nanosleep(request)
}
