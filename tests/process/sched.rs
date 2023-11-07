use rustix::process::sched_yield;

#[test]
fn test_sched_yield() {
    // Just make sure we can call it.
    sched_yield();
}

#[cfg(any(linux_kernel, target_os = "dragonfly"))]
#[test]
fn test_sched_getcpu() {
    let n = rustix::process::sched_getcpu();
    assert!(n < rustix::process::CpuSet::MAX_CPU);
}
