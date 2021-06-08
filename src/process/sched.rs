#[cfg(libc)]
use crate::zero_ok;

/// `sched_yield()`
#[cfg(libc)]
#[inline]
pub fn sched_yield() {
    zero_ok(unsafe { libc::sched_yield() }).unwrap()
}

/// `sched_yield()`
#[cfg(linux_raw)]
#[inline]
pub fn sched_yield() {
    crate::linux_raw::sched_yield()
}
