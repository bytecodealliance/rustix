//! Test all the `ClockId` clocks, which should never fail.

#![cfg(not(any(apple, target_os = "wasi")))]

#[cfg(not(any(solarish, target_os = "netbsd", target_os = "redox")))]
use rustix::time::{clock_gettime, ClockId};

/// Attempt to test that the boot clock is monotonic. Time may or may not
/// advance, but it shouldn't regress.
#[cfg(any(
    freebsdlike,
    linux_kernel,
    target_os = "fuchsia",
    target_os = "openbsd"
))]
#[test]
fn test_boottime_clock() {
    use rustix::time::{clock_gettime_dynamic, DynamicClockId};

    if let Ok(a) = clock_gettime_dynamic(DynamicClockId::Boottime) {
        if let Ok(b) = clock_gettime_dynamic(DynamicClockId::Boottime) {
            if b.tv_sec == a.tv_sec {
                assert!(b.tv_nsec >= a.tv_nsec);
            } else {
                assert!(b.tv_sec > a.tv_sec);
            }
        }
    }
}

/// Attempt to test that the uptime clock is monotonic. Time may or may not
/// advance, but it shouldn't regress.
#[cfg(any(freebsdlike, target_os = "openbsd"))]
#[test]
fn test_uptime_clock() {
    let a = clock_gettime(ClockId::Uptime);
    let b = clock_gettime(ClockId::Uptime);
    if b.tv_sec == a.tv_sec {
        assert!(b.tv_nsec >= a.tv_nsec);
    } else {
        assert!(b.tv_sec > a.tv_sec);
    }
}

/// Attempt to test that the process CPU-time clock is monotonic. Time may or
/// may not advance, but it shouldn't regress.
#[cfg(not(any(solarish, target_os = "netbsd", target_os = "redox")))]
#[test]
fn test_process_cputime_clock() {
    let a = clock_gettime(ClockId::ProcessCPUTime);
    let b = clock_gettime(ClockId::ProcessCPUTime);
    if b.tv_sec == a.tv_sec {
        assert!(b.tv_nsec >= a.tv_nsec);
    } else {
        assert!(b.tv_sec > a.tv_sec);
    }
}

/// Attempt to test that the thread CPU-time clock is monotonic. Time may or
/// may not advance, but it shouldn't regress.
#[cfg(not(any(solarish, target_os = "netbsd", target_os = "redox")))]
#[test]
fn test_thread_cputime_clock() {
    let a = clock_gettime(ClockId::ThreadCPUTime);
    let b = clock_gettime(ClockId::ThreadCPUTime);
    if b.tv_sec == a.tv_sec {
        assert!(b.tv_nsec >= a.tv_nsec);
    } else {
        assert!(b.tv_sec > a.tv_sec);
    }
}

#[cfg(any(linux_kernel, target_os = "freebsd"))]
#[test]
fn test_realtime_coarse_clock() {
    let a = clock_gettime(ClockId::RealtimeCoarse);

    // Test that the timespec is valid; there's not much else we can say.
    assert!(a.tv_nsec < 1_000_000_000);
}

/// Attempt to test that the coarse monotonic clock is monotonic. Time may or
/// may not advance, but it shouldn't regress.
#[cfg(any(linux_kernel, target_os = "freebsd"))]
#[test]
fn test_monotonic_coarse_clock() {
    let a = clock_gettime(ClockId::MonotonicCoarse);
    let b = clock_gettime(ClockId::MonotonicCoarse);
    if b.tv_sec == a.tv_sec {
        assert!(b.tv_nsec >= a.tv_nsec);
    } else {
        assert!(b.tv_sec > a.tv_sec);
    }
}

/// Attempt to test that the raw monotonic clock is monotonic. Time may or
/// may not advance, but it shouldn't regress.
#[cfg(linux_kernel)]
#[test]
fn test_monotonic_raw_clock() {
    let a = clock_gettime(ClockId::MonotonicRaw);
    let b = clock_gettime(ClockId::MonotonicRaw);
    if b.tv_sec == a.tv_sec {
        assert!(b.tv_nsec >= a.tv_nsec);
    } else {
        assert!(b.tv_sec > a.tv_sec);
    }
}
