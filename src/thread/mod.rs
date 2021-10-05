//! Thread-associated operations.

#[cfg(any(target_os = "android", target_os = "linux"))]
mod id;

#[cfg(linux_raw)]
mod futex;

#[cfg(linux_raw)]
pub use futex::{futex, FutexFlags, FutexOperation};

#[cfg(any(target_os = "android", target_os = "linux"))]
pub use id::gettid;
