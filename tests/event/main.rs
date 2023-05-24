//! Tests for [`rustix::event`].

#![cfg(feature = "event")]
#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]

#[cfg(not(feature = "rustc-dep-of-std"))] // TODO
#[cfg(feature = "net")]
#[cfg(linux_kernel)]
mod epoll;
#[cfg(not(windows))]
#[cfg(not(target_os = "wasi"))]
mod eventfd;
mod poll;
