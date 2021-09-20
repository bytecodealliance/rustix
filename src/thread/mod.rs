//! Thread-associated operations.

#[cfg(any(target_os = "android", target_os = "linux"))]
mod id;

#[cfg(linux_raw)]
#[doc(hidden)]
pub mod tls;

#[cfg(any(target_os = "android", target_os = "linux"))]
pub use id::gettid;
