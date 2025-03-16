use rustix::time::{
    timerfd_create, timerfd_gettime, timerfd_settime, Itimerspec, TimerfdClockId, TimerfdFlags,
    TimerfdTimerFlags, Timespec,
};

#[test]
fn test_timerfd() {
    let fd = match timerfd_create(TimerfdClockId::Monotonic, TimerfdFlags::CLOEXEC) {
        Ok(fd) => fd,
        Err(rustix::io::Errno::NOSYS) => return,
        Err(err) => Err(err).unwrap(),
    };

    let set = Itimerspec {
        it_interval: Timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        it_value: Timespec {
            tv_sec: 1,
            tv_nsec: 2,
        },
    };
    let _old: Itimerspec = timerfd_settime(&fd, TimerfdTimerFlags::ABSTIME, &set).unwrap();

    // Wait for the timer to expire.
    let mut buf = [0_u8; 8];
    assert_eq!(rustix::io::read(&fd, &mut buf), Ok(8));
    assert!(u64::from_ne_bytes(buf) >= 1);

    let new = timerfd_gettime(&fd).unwrap();

    // The timer counts down.
    assert_eq!(set.it_interval.tv_sec, new.it_interval.tv_sec);
    assert_eq!(set.it_interval.tv_nsec, new.it_interval.tv_nsec);
    assert!(new.it_value.tv_sec <= set.it_value.tv_sec);
    assert!(
        new.it_value.tv_nsec < set.it_value.tv_nsec || new.it_value.tv_sec < set.it_value.tv_sec
    );
}

/// Similar, but set an interval for a repeated timer. Don't check that the
/// times are monotonic because that would race with the timer repeating.
#[test]
fn test_timerfd_with_interval() {
    let fd = match timerfd_create(TimerfdClockId::Monotonic, TimerfdFlags::CLOEXEC) {
        Ok(fd) => fd,
        Err(rustix::io::Errno::NOSYS) => return,
        Err(err) => Err(err).unwrap(),
    };

    // An `Itimerspec` with an initial value and an interval.
    //
    // For the interval, use a value of more than 200000 nanoseconds, as
    // illumos appears not to support values smaller than that.
    let set = Itimerspec {
        it_interval: Timespec {
            tv_sec: 0,
            tv_nsec: 200001,
        },
        it_value: Timespec {
            tv_sec: 1,
            tv_nsec: 7,
        },
    };
    let _old: Itimerspec = timerfd_settime(&fd, TimerfdTimerFlags::ABSTIME, &set).unwrap();

    // Wait for the timer to expire.
    let mut buf = [0_u8; 8];
    assert_eq!(rustix::io::read(&fd, &mut buf), Ok(8));
    assert!(u64::from_ne_bytes(buf) >= 1);

    let new = timerfd_gettime(&fd).unwrap();

    assert_eq!(set.it_interval.tv_sec, new.it_interval.tv_sec);
    assert_eq!(set.it_interval.tv_nsec, new.it_interval.tv_nsec);

    // Wait for the timer to expire again.
    let mut buf = [0_u8; 8];
    assert_eq!(rustix::io::read(&fd, &mut buf), Ok(8));
    assert!(u64::from_ne_bytes(buf) >= 1);

    let new = timerfd_gettime(&fd).unwrap();

    assert_eq!(set.it_interval.tv_sec, new.it_interval.tv_sec);
    assert_eq!(set.it_interval.tv_nsec, new.it_interval.tv_nsec);
}
