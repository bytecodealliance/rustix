//! Process-associated operations.

#[cfg(not(target_os = "wasi"))] // WASI doesn't have get[gpu]id.
mod id;
mod sched;

#[cfg(not(target_os = "wasi"))]
pub use id::{getgid, getpid, getuid};
pub use sched::sched_yield;
