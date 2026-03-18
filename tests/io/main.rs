//! Tests for [`rustix::io`].

mod close;
#[cfg(not(any(windows, target_os = "vxworks")))]
mod dup;
mod error;
#[cfg(not(any(windows, target_os = "vxworks")))]
mod from_into;
#[cfg(not(any(target_os = "redox", target_os = "vxworks")))]
mod ioctl;
#[cfg(not(windows))]
#[cfg(not(target_os = "redox"))] // redox doesn't have cwd/openat
mod read_write;
