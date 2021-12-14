//! libc syscalls supporting `rustix::process`.

#[inline]
pub(crate) fn sched_yield() {
    todo!("sched_yield")
}

#[inline]
pub(crate) fn exit_group(code: i32) -> ! {
    todo!("exit_group")
}
