//! Tests for [`rustix::time`].

#![cfg(not(windows))]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]

mod dynamic_clocks;
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
mod monotonic;
mod timespec;
mod y2038;
