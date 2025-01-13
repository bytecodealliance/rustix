#[cfg(linux_kernel)]
#[test]
fn test_cpu_set() {
    let set = rustix::thread::sched_getaffinity(None).unwrap();

    let mut count = 0;
    for i in 0..rustix::thread::CpuSet::MAX_CPU {
        if set.is_set(i) {
            count += 1;
        }
    }

    assert_eq!(count, set.count());
}
