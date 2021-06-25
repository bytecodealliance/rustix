/// `struct timespec`
pub type Timespec = libc::timespec;

#[allow(deprecated)]
pub type Secs = libc::time_t;
#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
pub type Nsecs = i64;
#[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))]
pub type Nsecs = std::os::raw::c_long;

/// `clockid_t`
#[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "wasi")))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(i32)]
#[non_exhaustive]
pub enum ClockId {
    /// `CLOCK_REALTIME`
    Realtime = libc::CLOCK_REALTIME,

    /// `CLOCK_MONOTONIC`
    Monotonic = libc::CLOCK_MONOTONIC,

    /// `CLOCK_PROCESS_CPUTIME_ID`
    #[cfg(not(any(target_os = "netbsd", target_os = "openbsd", target_os = "redox")))]
    ProcessCPUTime = libc::CLOCK_PROCESS_CPUTIME_ID,

    /// `CLOCK_THREAD_CPUTIME_ID`
    #[cfg(not(any(target_os = "netbsd", target_os = "openbsd", target_os = "redox")))]
    ThreadCPUTime = libc::CLOCK_THREAD_CPUTIME_ID,
}

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
