//! Tests for [`rustix::procfs`].

#![cfg(feature = "procfs")]
#![cfg(linux_kernel)]

mod basic;
