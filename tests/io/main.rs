//! Tests for [`rustix::io`].

mod close;
#[cfg(not(windows))]
mod dup;
mod error;
#[cfg(not(windows))]
mod from_into;
#[cfg(not(target_os = "redox"))]
mod ioctl;
#[cfg(not(windows))]
#[cfg(not(target_os = "redox"))] // redox doesn't have cwd/openat
mod read_write;
