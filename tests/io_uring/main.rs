//! Tests for [`rustix::io_uring`].

#![cfg(feature = "io_uring")]

#[cfg(linux_kernel)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod register;
