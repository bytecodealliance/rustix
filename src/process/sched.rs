use crate::imp;

/// `sched_yield()`
#[inline]
pub fn sched_yield() {
    imp::syscalls::sched_yield()
}
