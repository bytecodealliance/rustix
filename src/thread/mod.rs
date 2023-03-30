//! Thread-associated operations.

#[cfg(not(target_os = "redox"))]
mod clock;
#[cfg(linux_raw)]
mod futex;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod id;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod libcap;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod prctl;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod setns;

#[cfg(not(target_os = "redox"))]
pub use clock::*;
#[cfg(linux_raw)]
pub use futex::{futex, FutexFlags, FutexOperation};
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use id::{gettid, set_thread_gid, set_thread_res_gid, set_thread_res_uid, set_thread_uid};
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use libcap::{capabilities, set_capabilities, CapabilityFlags, CapabilitySets};
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use prctl::*;
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use setns::*;
