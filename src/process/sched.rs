/// `sched_yield()`
#[cfg(libc)]
#[inline]
pub fn sched_yield() {
    unsafe {
        let _ = libc::sched_yield();
    }
}

/// `sched_yield()`
#[cfg(linux_raw)]
#[inline]
pub fn sched_yield() {
    crate::linux_raw::sched_yield()
}
