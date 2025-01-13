//! Thread-associated operations.

#[cfg(not(target_os = "redox"))]
mod clock;
#[cfg(linux_kernel)]
pub mod futex;
#[cfg(linux_kernel)]
mod id;
#[cfg(linux_kernel)]
mod libcap;
#[cfg(linux_kernel)]
mod membarrier;
#[cfg(linux_kernel)]
mod prctl;
#[cfg(any(freebsdlike, linux_kernel, target_os = "fuchsia"))]
mod sched;
mod sched_yield;
#[cfg(linux_kernel)]
mod setns;

#[cfg(linux_kernel)]
pub use crate::thread::futex::{
    Flags as FutexFlags, OWNER_DIED as FUTEX_OWNER_DIED, WAITERS as FUTEX_WAITERS,
};
#[cfg(not(target_os = "redox"))]
pub use clock::*;
#[cfg(linux_kernel)]
pub use id::*;
#[cfg(linux_kernel)]
pub use libcap::{capabilities, set_capabilities, CapabilityFlags, CapabilitySets};
#[cfg(linux_kernel)]
pub use membarrier::*;
#[cfg(linux_kernel)]
pub use prctl::*;
#[cfg(any(freebsdlike, linux_kernel, target_os = "fuchsia"))]
pub use sched::*;
pub use sched_yield::sched_yield;
#[cfg(linux_kernel)]
pub use setns::*;
