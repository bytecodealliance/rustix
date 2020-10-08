#[cfg(not(target_os = "wasi"))]
use {crate::zero_ok, std::mem::MaybeUninit};

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
pub fn clock_getres(id: ClockId) -> timespec {
    let mut timespec = MaybeUninit::<timespec>::uninit();
    zero_ok(unsafe { libc::clock_getres(id as libc::clockid_t, timespec.as_mut_ptr()) }).unwrap();
    unsafe { timespec.assume_init() }
}

/// `clock_gettime(id)`
#[cfg(not(target_os = "wasi"))]
pub fn clock_gettime(id: ClockId) -> timespec {
    let mut timespec = MaybeUninit::<timespec>::uninit();
    zero_ok(unsafe { libc::clock_gettime(id as libc::clockid_t, timespec.as_mut_ptr()) }).unwrap();
    unsafe { timespec.assume_init() }
}
