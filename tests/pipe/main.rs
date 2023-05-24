//! Tests for [`rustix::pipe`].

#![cfg(feature = "pipe")]
#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]

mod basic;
mod splice;
mod tee;
