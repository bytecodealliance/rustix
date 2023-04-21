//! Tests for [`rustix::rand`].

#![cfg(feature = "rand")]
#![cfg(not(windows))]
#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]

#[cfg(any(target_os = "android", target_os = "linux"))]
mod getrandom;
