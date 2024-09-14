//! Tests for [`rustix::event`].

#![cfg(feature = "event")]

#[cfg(not(feature = "rustc-dep-of-std"))] // TODO
#[cfg(feature = "net")]
#[cfg(linux_kernel)]
mod epoll;
#[cfg(not(windows))]
#[cfg(not(target_os = "wasi"))]
mod eventfd;
mod poll;
#[cfg(any(bsd, linux_kernel, windows))]
mod select;
