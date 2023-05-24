//! Tests for [`rustix::pipe`].

#![cfg(feature = "pipe")]
#![cfg(not(windows))]
#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]

mod basic;
mod splice;
mod tee;
