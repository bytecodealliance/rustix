//! Tests for [`rustix::pipe`].

#![cfg(feature = "pipe")]
#![cfg(not(windows))]

mod basic;
mod fcntl;
mod splice;
mod tee;
