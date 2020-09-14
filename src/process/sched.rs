use crate::zero_ok;

/// `sched_yield()`
#[inline]
pub fn sched_yield() {
    zero_ok(unsafe { libc::sched_yield() }).unwrap()
}
