use rustix::time::{clock_gettime, ClockId};

#[test]
fn test_wall_clock() {
    let a = clock_gettime(ClockId::Realtime);

    // Test that the timespec is valid; there's not much else we can say.
    assert!(a.tv_nsec < 1_000_000_000);
}
