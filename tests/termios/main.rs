//! Tests for [`rustix::termios`].

#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]
#![cfg(feature = "termios")]

#[cfg(not(windows))]
mod isatty;
#[cfg(not(any(windows, target_os = "fuchsia")))]
#[cfg(feature = "procfs")]
mod ttyname;
