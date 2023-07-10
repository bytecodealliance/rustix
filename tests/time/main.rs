//! Tests for [`rustix::time`].

#![cfg(feature = "time")]
#![cfg(not(any(windows, target_os = "espidf")))]

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
mod dynamic_clocks;
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
mod monotonic;
#[cfg(not(any(
    target_os = "redox",
    target_os = "wasi",
    all(apple, not(target_os = "macos"))
)))]
mod settime;
#[cfg(linux_kernel)]
mod timerfd;
mod timespec;
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
mod wall;
mod y2038;
