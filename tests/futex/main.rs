//! Tests for [`rustix::futex`].

#![cfg(feature = "futex")]
#![cfg(linux_kernel)]

mod basic;
