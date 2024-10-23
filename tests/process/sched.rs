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

#[cfg(any(freebsdlike, linux_kernel))]
#[test]
fn test_sched_scheduler() {
    use rustix::process::{SchedParam, SchedPolicy};

    #[cfg(linux_kernel)]
    let policy = SchedPolicy::Batch;
    #[cfg(not(linux_kernel))]
    // we cannot change policy in *BSD because we do not have privilege
    let policy = SchedPolicy::default();

    // backup
    let original_policy = rustix::process::sched_getscheduler(None).unwrap();

    rustix::process::sched_setscheduler(None, policy, &SchedParam::default()).unwrap();
    assert_eq!(rustix::process::sched_getscheduler(None).unwrap(), policy);

    // restore
    rustix::process::sched_setscheduler(None, original_policy, &SchedParam::default()).unwrap();
}
