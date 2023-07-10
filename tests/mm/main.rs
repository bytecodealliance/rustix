//! Tests for [`rustix::mm`].

#![cfg(feature = "mm")]
#![cfg(not(target_os = "espidf"))]

#[cfg(not(any(windows, target_os = "wasi")))]
mod mlock;
#[cfg(not(any(windows, target_os = "wasi")))]
mod mmap;
#[cfg(not(any(windows, target_os = "wasi")))]
mod prot;
