#[test]
fn test_timespec_layout() {
    #[cfg(not(target_os = "redox"))]
    #[cfg(feature = "fs")]
    use rustix::fs::{UTIME_NOW, UTIME_OMIT};
    use rustix::time::{Nsecs, Secs, Timespec};

    let tv_sec: Secs = 0;
    let tv_nsec: Nsecs = 0;
    let x = Timespec { tv_sec, tv_nsec };

    // Test that `Timespec` implements `Copy` and `Debug`.
    let _y = Timespec { tv_sec, tv_nsec };
    let _z = Timespec { tv_sec, tv_nsec };
    dbg!(x.tv_sec, x.tv_nsec);

    #[cfg(not(target_os = "redox"))]
    #[cfg(feature = "fs")]
    let _ = Timespec {
        tv_sec,
        tv_nsec: UTIME_NOW,
    };
    #[cfg(not(target_os = "redox"))]
    #[cfg(feature = "fs")]
    let _ = Timespec {
        tv_sec,
        tv_nsec: UTIME_OMIT,
    };
    let _ = Timespec { tv_sec, tv_nsec: 0 };
    let _ = Timespec {
        tv_sec,
        tv_nsec: 999_999_999,
    };
}
