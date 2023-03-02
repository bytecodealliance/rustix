//! Random-related operations.

#[cfg(any(target_os = "android", target_os = "linux"))]
mod getrandom;

#[cfg(any(target_os = "android", target_os = "linux"))]
pub use getrandom::{getrandom, GetRandomFlags};
