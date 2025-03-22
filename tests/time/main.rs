//! Tests for [`rustix::time`].

#![cfg(feature = "time")]
#![cfg(not(any(windows, target_os = "espidf")))]

mod clocks;
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
#[cfg(any(
    linux_kernel,
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "illumos",
    target_os = "netbsd"
))]
mod timerfd;
mod timespec;
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
mod wall;
mod y2038;
