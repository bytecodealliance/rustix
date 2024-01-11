//! Test all the `ClockId` clocks, which should never fail.

#![cfg(not(any(apple, target_os = "wasi")))]

#[cfg(not(any(solarish, target_os = "netbsd", target_os = "redox")))]
use rustix::time::{clock_gettime, ClockId};

/// Attempt to test that the boot clock is monotonic. Time may or may not
/// advance, but it shouldn't regress.
#[cfg(any(
    linux_kernel,
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "openbsd"
))]
#[test]
fn test_boottime_clock() {
    use rustix::time::{clock_gettime_dynamic, DynamicClockId};

    let monotonic = clock_gettime(ClockId::Monotonic);

    if let Ok(a) = clock_gettime_dynamic(DynamicClockId::Boottime) {
        if let Ok(b) = clock_gettime_dynamic(DynamicClockId::Boottime) {
            if b.tv_sec == a.tv_sec {
                assert!(b.tv_nsec >= a.tv_nsec);
            } else {
                assert!(b.tv_sec > a.tv_sec);
            }
        }

        // Test that boot time is after monotonic.
        if a.tv_sec == monotonic.tv_sec {
            assert!(a.tv_nsec >= monotonic.tv_nsec);
        } else {
            assert!(a.tv_sec > monotonic.tv_sec);
        }
    }

    #[cfg(feature = "linux_4_11")]
    {
        let a = clock_gettime(ClockId::Boottime);
        let b = clock_gettime(ClockId::Boottime);

        if b.tv_sec == a.tv_sec {
            assert!(b.tv_nsec >= a.tv_nsec);
        } else {
            assert!(b.tv_sec > a.tv_sec);
        }

        // Test that boot time is after monotonic.
        if a.tv_sec == monotonic.tv_sec {
            assert!(a.tv_nsec >= monotonic.tv_nsec);
        } else {
            assert!(a.tv_sec > monotonic.tv_sec);
        }
    }
}

/// Attempt to test that the boot alarm clock is monotonic. Time may or may not
/// advance, but it shouldn't regress.
#[cfg(any(linux_kernel, target_os = "fuchsia"))]
#[test]
fn test_boottime_alarm_clock() {
    use rustix::time::{clock_gettime_dynamic, DynamicClockId};

    let monotonic = clock_gettime(ClockId::Monotonic);

    if let Ok(a) = clock_gettime_dynamic(DynamicClockId::BoottimeAlarm) {
        if let Ok(b) = clock_gettime_dynamic(DynamicClockId::BoottimeAlarm) {
            if b.tv_sec == a.tv_sec {
                assert!(b.tv_nsec >= a.tv_nsec);
            } else {
                assert!(b.tv_sec > a.tv_sec);
            }
        }

        // Test that boot alarm time is after monotonic.
        if a.tv_sec == monotonic.tv_sec {
            assert!(a.tv_nsec >= monotonic.tv_nsec);
        } else {
            assert!(a.tv_sec > monotonic.tv_sec);
        }
    }

    #[cfg(feature = "linux_4_11")]
    {
        let a = clock_gettime(ClockId::BoottimeAlarm);
        let b = clock_gettime(ClockId::BoottimeAlarm);

        if b.tv_sec == a.tv_sec {
            assert!(b.tv_nsec >= a.tv_nsec);
        } else {
            assert!(b.tv_sec > a.tv_sec);
        }

        // Test that boot alarm time is after monotonic.
        if a.tv_sec == monotonic.tv_sec {
            assert!(a.tv_nsec >= monotonic.tv_nsec);
        } else {
            assert!(a.tv_sec > monotonic.tv_sec);
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

#[cfg(linux_kernel)]
#[test]
fn test_realtime_alarm_clock() {
    use rustix::time::{clock_gettime_dynamic, DynamicClockId};

    if let Ok(a) = clock_gettime_dynamic(DynamicClockId::RealtimeAlarm) {
        // Test that the timespec is valid; there's not much else we can say.
        assert!(a.tv_nsec < 1_000_000_000);
    }

    #[cfg(feature = "linux_4_11")]
    {
        let a = clock_gettime(ClockId::RealtimeAlarm);

        // Test that the timespec is valid; there's not much else we can say.
        assert!(a.tv_nsec < 1_000_000_000);
    }
}

/// Attempt to test that the TAI clock is monotonic. Time may or may not
/// advance, but it shouldn't regress.
#[cfg(linux_kernel)]
#[test]
fn test_tai_clock() {
    use rustix::time::{clock_gettime_dynamic, DynamicClockId};

    let realtime = clock_gettime(ClockId::Realtime);

    if let Ok(a) = clock_gettime_dynamic(DynamicClockId::Tai) {
        if let Ok(b) = clock_gettime_dynamic(DynamicClockId::Tai) {
            if b.tv_sec == a.tv_sec {
                assert!(b.tv_nsec >= a.tv_nsec);
            } else {
                assert!(b.tv_sec > a.tv_sec);
            }
        }

        // Test that TAI time is after realtime.
        if a.tv_sec == realtime.tv_sec {
            assert!(a.tv_nsec >= realtime.tv_nsec);
        } else {
            assert!(a.tv_sec > realtime.tv_sec);
        }
    }

    #[cfg(feature = "linux_4_11")]
    {
        let a = clock_gettime(ClockId::Tai);
        let b = clock_gettime(ClockId::Tai);

        if b.tv_sec == a.tv_sec {
            assert!(b.tv_nsec >= a.tv_nsec);
        } else {
            assert!(b.tv_sec > a.tv_sec);
        }

        // Test that TAI time is after realtime.
        if a.tv_sec == realtime.tv_sec {
            assert!(a.tv_nsec >= realtime.tv_nsec);
        } else {
            assert!(a.tv_sec > realtime.tv_sec);
        }
    }
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
