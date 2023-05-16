//! Tests for [`rustix::pty`].

#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]
#![cfg(feature = "pty")]

#[cfg(any(apple, linux_like, target_os = "freebsd", target_os = "fuchsia"))]
mod openpty;
