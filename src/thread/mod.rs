//! Thread-associated operations.

#[cfg(not(target_os = "redox"))]
mod clock;
#[cfg(linux_kernel)]
mod futex;
#[cfg(linux_kernel)]
mod id;
#[cfg(linux_kernel)]
mod libcap;
#[cfg(linux_kernel)]
mod prctl;
#[cfg(linux_kernel)]
mod setns;

#[cfg(not(target_os = "redox"))]
pub use clock::*;
#[cfg(linux_kernel)]
pub use futex::{
    futex_cmp_requeue, futex_cmp_requeue_pi, futex_fd, futex_lock_pi, futex_lock_pi2,
    futex_requeue, futex_trylock_pi, futex_unlock_pi, futex_wait, futex_wait_bitset,
    futex_wait_requeue_pi, futex_wake, futex_wake_bitset, futex_wake_op, FutexFlags,
    FUTEX_OWNER_DIED, FUTEX_WAITERS,
};
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
