//! Tests for [`rustix::pipe`].

#![cfg(feature = "pipe")]
#![cfg(not(windows))]

mod basic;
mod splice;
mod tee;
