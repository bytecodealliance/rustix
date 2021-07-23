//! Thread-associated operations.

#[cfg(any(target_os = "android", target_os = "linux"))]
mod id;

#[cfg(any(target_os = "android", target_os = "linux"))]
pub use id::gettid;
