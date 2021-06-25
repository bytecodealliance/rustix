/// `struct timespec`
pub type Timespec = linux_raw_sys::general::__kernel_timespec;

pub type Secs = linux_raw_sys::general::__kernel_time64_t;
pub type Nsecs = i64;

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
