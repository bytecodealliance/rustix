//! Tests for [`rustix::rand`].

#![cfg(feature = "rand")]
#![cfg(not(windows))]

#[cfg(linux_kernel)]
mod getrandom;
