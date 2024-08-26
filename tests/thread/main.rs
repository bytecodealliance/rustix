//! Tests for [`rustix::thread`].

#![cfg(feature = "thread")]
#![cfg(not(windows))]

#[cfg(not(target_os = "redox"))]
mod clocks;
#[cfg(linux_kernel)]
mod futex;
#[cfg(linux_kernel)]
mod id;
#[cfg(linux_kernel)]
mod libcap;
#[cfg(linux_kernel)]
mod prctl;
#[cfg(linux_kernel)]
mod setns;
