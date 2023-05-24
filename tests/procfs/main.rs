//! Tests for [`rustix::procfs`].

#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]
#![cfg(feature = "procfs")]
#![cfg(linux_kernel)]

mod basic;
