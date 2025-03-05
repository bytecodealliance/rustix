use rustix::io;
use rustix::time::{ClockId, Timespec, clock_settime};

#[test]
fn test_settime() {
    // Monotonic clocks are never settable.
    match clock_settime(
        ClockId::Monotonic,
        Timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
    ) {
        Err(io::Errno::INVAL | io::Errno::PERM) => (),
        _otherwise => panic!(),
    }
}
