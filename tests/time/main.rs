//! Tests for [`rustix::time`].

#![cfg(feature = "time")]
#![cfg(not(windows))]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]

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
#[cfg(any(target_os = "android", target_os = "linux"))]
mod timerfd;
mod timespec;
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
mod wall;
mod y2038;
