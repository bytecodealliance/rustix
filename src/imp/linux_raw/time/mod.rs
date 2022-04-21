mod types;

#[cfg(any(feature = "time", target_arch = "x86"))]
pub(crate) mod syscalls;

pub use types::{
    ClockId, DynamicClockId, Itimerspec, Nsecs, Secs, TimerfdClockId, TimerfdFlags,
    TimerfdTimerFlags, Timespec,
};
