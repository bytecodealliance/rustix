use rustix::time::{
    timerfd_create, timerfd_gettime, timerfd_settime, Itimerspec, TimerfdClockId, TimerfdFlags,
    TimerfdTimerFlags, Timespec,
};

#[test]
fn test_timerfd() {
    let fd = timerfd_create(TimerfdClockId::Monotonic, TimerfdFlags::CLOEXEC).unwrap();

    let set = Itimerspec {
        it_interval: Timespec {
            tv_sec: 3,
            tv_nsec: 4,
        },
        it_value: Timespec {
            tv_sec: 1,
            tv_nsec: 2,
        },
    };
    let _old: Itimerspec = timerfd_settime(&fd, TimerfdTimerFlags::ABSTIME, &set).unwrap();

    // Wait for the timer to expire.
    let mut buf = [0u8; 8usize];
    assert_eq!(rustix::io::read(&fd, &mut buf), Ok(8));
    assert!(u64::from_ne_bytes(buf) >= 1);

    let new = timerfd_gettime(&fd).unwrap();

    assert_eq!(set.it_interval.tv_sec, new.it_interval.tv_sec);
    assert_eq!(set.it_interval.tv_nsec, new.it_interval.tv_nsec);
    assert!(new.it_value.tv_sec >= set.it_value.tv_sec);
    assert!(
        set.it_value.tv_nsec <= new.it_value.tv_nsec || set.it_value.tv_sec < new.it_value.tv_sec
    );
}
