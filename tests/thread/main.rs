//! Tests for [`rustix::thread`].

#![cfg(feature = "thread")]
#![cfg(not(windows))]

#[cfg(not(target_os = "redox"))]
mod clocks;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod id;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod libcap;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod prctl;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod setns;
