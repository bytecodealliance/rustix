use rustix::thread::sched_yield;

#[test]
fn test_sched_yield() {
    // Just make sure we can call it.
    sched_yield();
}

#[cfg(any(linux_kernel, target_os = "dragonfly"))]
#[test]
fn test_sched_getcpu() {
    let n = rustix::thread::sched_getcpu();
    assert!(n < rustix::thread::CpuSet::MAX_CPU);
}
