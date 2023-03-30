use rustix::io;
use rustix::time::{clock_settime, ClockId, Timespec};

#[test]
fn test_settime() {
    // Monotonic clocks are never settable.
    assert_eq!(
        clock_settime(
            ClockId::Monotonic,
            Timespec {
                tv_sec: 0,
                tv_nsec: 0
            }
        ),
        Err(io::Errno::INVAL)
    );
}
