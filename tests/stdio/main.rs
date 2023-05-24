//! Tests for [`rustix::stdio`].

#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]

#[cfg(not(feature = "rustc-dep-of-std"))]
#[cfg(not(windows))]
#[cfg(not(target_os = "wasi"))]
mod dup2_to_replace_stdio;
