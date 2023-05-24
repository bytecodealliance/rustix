//! Tests for [`rustix::io`].

#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]

mod error;
#[cfg(not(windows))]
mod from_into;
#[cfg(not(target_os = "redox"))]
mod ioctl;
mod pipe;
#[cfg(not(windows))]
#[cfg(not(target_os = "redox"))] // redox doesn't have cwd/openat
#[cfg(not(target_os = "wasi"))] // wasi support for `S_IRUSR` etc. submitted to libc in #2264
mod read_write;
