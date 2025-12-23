#[cfg(feature = "thread")]
use rustix::thread::nanosleep;
use rustix::time::{clock_gettime, ClockId, Timespec};

/// Attempt to test that the monotonic clock is monotonic. Time may or may not
/// advance, but it shouldn't regress.
#[test]
fn test_monotonic_clock() {
    let a = clock_gettime(ClockId::Monotonic);
    let b = clock_gettime(ClockId::Monotonic);
    if b.tv_sec == a.tv_sec {
        assert!(b.tv_nsec >= a.tv_nsec);
    } else {
        assert!(b.tv_sec > a.tv_sec);
    }
}

/// With the "thread" feature, we can sleep so that we're guaranteed that time
/// has advanced.
#[cfg(feature = "thread")]
#[test]
fn test_monotonic_clock_with_sleep_1s() {
    let a = clock_gettime(ClockId::Monotonic);
    let _rem = nanosleep(&Timespec {
        tv_sec: 1,
        tv_nsec: 0,
    });
    let b = clock_gettime(ClockId::Monotonic);
    assert!(b.tv_sec > a.tv_sec);
}

/// With the "thread" feature, we can sleep so that we're guaranteed that time
/// has advanced.
#[cfg(feature = "thread")]
#[test]
fn test_monotonic_clock_with_sleep_1ms() {
    let a = clock_gettime(ClockId::Monotonic);
    let _rem = nanosleep(&Timespec {
        tv_sec: 0,
        tv_nsec: 1_000_000,
    });
    let b = clock_gettime(ClockId::Monotonic);
    assert!(b.tv_sec >= a.tv_sec);
    assert!(b.tv_sec != a.tv_sec || b.tv_nsec > a.tv_nsec);
}

#[test]
fn test_monotonic_clock_vs_libc() {
    let mut before = unsafe { core::mem::zeroed::<libc::timespec>() };
    let r = unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut before) };
    assert_eq!(r, 0);

    let a = clock_gettime(ClockId::Monotonic);

    // Test that the timespec is valid.
    assert!(a.tv_nsec < 1_000_000_000);

    let mut after = unsafe { core::mem::zeroed::<libc::timespec>() };
    let r = unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut after) };
    assert_eq!(r, 0);

    #[allow(clippy::useless_conversion)]
    let before = Timespec {
        tv_sec: before.tv_sec.try_into().unwrap(),
        tv_nsec: before.tv_nsec.try_into().unwrap(),
    };
    #[allow(clippy::useless_conversion)]
    let after = Timespec {
        tv_sec: after.tv_sec.try_into().unwrap(),
        tv_nsec: after.tv_nsec.try_into().unwrap(),
    };

    assert!(before <= a);
    assert!(a <= after);
}
