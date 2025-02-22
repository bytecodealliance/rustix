use rustix::fd::AsFd as _;
use rustix::time::{clock_gettime_dynamic, ClockId, DynamicClockId};

#[test]
fn test_known_clocks() {
    clock_gettime_dynamic(DynamicClockId::Known(ClockId::Realtime)).unwrap();
    clock_gettime_dynamic(DynamicClockId::Known(ClockId::Monotonic)).unwrap();
}

#[test]
fn test_dynamic_clocks() {
    let file = std::fs::File::open("Cargo.toml").unwrap();
    assert!(clock_gettime_dynamic(DynamicClockId::Dynamic(file.as_fd())).is_err());
}

#[cfg(linux_kernel)]
#[test]
fn test_conditional_clocks() {
    let _ = clock_gettime_dynamic(DynamicClockId::Tai);
}
