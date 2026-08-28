//! Thread-associated operations.

#[cfg(not(target_os = "redox"))]
mod clock;
#[cfg(linux_kernel)]
pub mod futex;
#[cfg(linux_kernel)]
mod id;
#[cfg(all(linux_kernel, linux_raw_dep))]
mod libcap;
#[cfg(linux_kernel)]
mod membarrier;
#[cfg(all(linux_kernel, linux_raw_dep))]
mod prctl;
#[cfg(any(freebsdlike, linux_kernel, target_os = "fuchsia"))]
mod sched;
mod sched_yield;
#[cfg(all(linux_kernel, linux_raw_dep))]
mod setns;

#[cfg(not(target_os = "redox"))]
pub use clock::*;
#[cfg(linux_kernel)]
pub use id::*;
#[cfg(linux_kernel)]
// #[expect(deprecated, reason = "CapabilityFlags is deprecated")]
#[allow(deprecated)]
#[cfg(all(linux_kernel, linux_raw_dep))]
pub use libcap::{capabilities, set_capabilities, CapabilityFlags, CapabilitySet, CapabilitySets};
#[cfg(linux_kernel)]
pub use membarrier::*;
#[cfg(all(linux_kernel, linux_raw_dep))]
pub use prctl::*;
#[cfg(any(freebsdlike, linux_kernel, target_os = "fuchsia"))]
pub use sched::*;
pub use sched_yield::sched_yield;
#[cfg(all(linux_kernel, linux_raw_dep))]
pub use setns::*;
