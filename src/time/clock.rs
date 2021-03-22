use crate::zero_ok;
use std::mem::MaybeUninit;
#[cfg(not(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "ios",
    target_os = "redox",
    target_os = "freebsd",
    target_os = "emscripten",
    target_os = "wasi",
)))]
use std::ptr;

pub use libc::{timespec, UTIME_NOW, UTIME_OMIT};

/// `clockid_t`
#[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "wasi")))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum ClockId {
    /// `CLOCK_REALTIME`
    Realtime = libc::CLOCK_REALTIME,

    /// `CLOCK_MONOTONIC`
    Monotonic = libc::CLOCK_MONOTONIC,

    /// `CLOCK_PROCESS_CPUTIME_ID`
    #[cfg(not(target_os = "netbsd"))]
    ProcessCPUTime = libc::CLOCK_PROCESS_CPUTIME_ID,

    /// `CLOCK_THREAD_CPUTIME_ID`
    #[cfg(not(target_os = "netbsd"))]
    ThreadCPUTime = libc::CLOCK_THREAD_CPUTIME_ID,
}

/// `clockid_t`
#[cfg(any(target_os = "macos", target_os = "ios"))]
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

/// `clock_getres(id)`
#[cfg(not(target_os = "wasi"))]
#[inline]
#[must_use]
pub fn clock_getres(id: ClockId) -> timespec {
    let mut timespec = MaybeUninit::<timespec>::uninit();
    zero_ok(unsafe { libc::clock_getres(id as libc::clockid_t, timespec.as_mut_ptr()) }).unwrap();
    unsafe { timespec.assume_init() }
}

/// `clock_gettime(id)`
#[cfg(not(target_os = "wasi"))]
#[inline]
#[must_use]
pub fn clock_gettime(id: ClockId) -> timespec {
    let mut timespec = MaybeUninit::<timespec>::uninit();
    zero_ok(unsafe { libc::clock_gettime(id as libc::clockid_t, timespec.as_mut_ptr()) }).unwrap();
    unsafe { timespec.assume_init() }
}

/// `clock_nanosleep(id, 0, request, remain)`
#[cfg(not(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "ios",
    target_os = "redox",
    target_os = "freebsd", // FreeBSD 12 has clock_nanosleep, but libc targets FreeBSD 11.
    target_os = "emscripten",
    target_os = "wasi",
)))]
#[inline]
pub fn clock_nanosleep_relative(id: ClockId, request: &timespec) -> Result<(), timespec> {
    let mut remain = MaybeUninit::<timespec>::uninit();
    let flags = 0;
    zero_ok(unsafe {
        libc::clock_nanosleep(id as libc::clockid_t, flags, request, remain.as_mut_ptr())
    })
    .map_err(|_| unsafe { remain.assume_init() })
}

/// `clock_nanosleep(id, TIMER_ABSTIME, request, NULL)`
#[cfg(not(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "ios",
    target_os = "redox",
    target_os = "freebsd", // FreeBSD 12 has clock_nanosleep, but libc targets FreeBSD 11.
    target_os = "emscripten",
    target_os = "wasi",
)))]
#[inline]
pub fn clock_nanosleep_absolute(id: ClockId, request: &timespec) -> Result<(), ()> {
    let flags = libc::TIMER_ABSTIME;
    zero_ok(unsafe {
        libc::clock_nanosleep(id as libc::clockid_t, flags, request, ptr::null_mut())
    })
    .map_err(|_| ())
}

/// `nanosleep(request, remain)`
#[inline]
pub fn nanosleep(request: &timespec) -> Result<(), timespec> {
    let mut remain = MaybeUninit::<timespec>::uninit();
    zero_ok(unsafe { libc::nanosleep(request, remain.as_mut_ptr()) })
        .map_err(|_| unsafe { remain.assume_init() })
}
