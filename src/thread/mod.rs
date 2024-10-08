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
mod prctl;
#[cfg(linux_kernel)]
mod setns;

#[cfg(linux_kernel)]
pub use crate::thread::futex::{
    Flags as FutexFlags, OWNER_DIED as FUTEX_OWNER_DIED, WAITERS as FUTEX_WAITERS,
};
#[cfg(not(target_os = "redox"))]
pub use clock::*;
#[cfg(linux_kernel)]
pub use id::{
    gettid, set_thread_gid, set_thread_groups, set_thread_res_gid, set_thread_res_uid,
    set_thread_uid, Gid, Pid, RawGid, RawPid, RawUid, Uid,
};
#[cfg(linux_kernel)]
pub use libcap::{capabilities, set_capabilities, CapabilityFlags, CapabilitySets};
#[cfg(linux_kernel)]
pub use prctl::*;
#[cfg(linux_kernel)]
pub use setns::*;
