use rustix::time::{clock_gettime, ClockId, Timespec};

#[test]
fn test_wall_clock() {
    let mut before = unsafe { core::mem::zeroed::<libc::timespec>() };
    let r = unsafe { libc::clock_gettime(libc::CLOCK_REALTIME, &mut before) };
    assert_eq!(r, 0);

    let a = clock_gettime(ClockId::Realtime);

    // Test that the timespec is valid.
    assert!(a.tv_nsec < 1_000_000_000);

    let mut after = unsafe { core::mem::zeroed::<libc::timespec>() };
    let r = unsafe { libc::clock_gettime(libc::CLOCK_REALTIME, &mut after) };
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
