#![cfg(not(any(target_os = "wasi", target_os = "redox")))]

use io_lifetimes::AsFd;
use posish::time::{clock_gettime_dynamic, DynamicClockId, ClockId};

#[test]
fn test_known_clocks() {
    clock_gettime_dynamic(DynamicClockId::Known(ClockId::Realtime)).unwrap();
    clock_gettime_dynamic(DynamicClockId::Known(ClockId::Monotonic)).unwrap();
}

#[test]
fn test_dynamic_clocks() {
    let file = std::fs::File::open("Cargo.toml").unwrap();
    clock_gettime_dynamic(DynamicClockId::Dynamic(file.as_fd())).unwrap_err();
}

#[cfg(any(target_os = "android", target_os = "linux"))]
#[test]
fn test_conditional_clocks() {
    let _ = clock_gettime_dynamic(DynamicClockId::Tai);
}
