#[cfg(not(any(
    apple,
    freebsdlike,
    target_os = "emscripten",
    target_os = "haiku",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
use rustix::thread::{ClockId, clock_nanosleep_absolute, clock_nanosleep_relative};
#[cfg(not(target_os = "redox"))]
use {
    rustix::io,
    rustix::thread::{NanosleepRelativeResult, Timespec, nanosleep},
};

#[cfg(not(target_os = "redox"))]
#[test]
fn test_invalid_nanosleep() {
    match nanosleep(&Timespec {
        tv_sec: 0,
        tv_nsec: 1_000_000_000,
    }) {
        NanosleepRelativeResult::Err(io::Errno::INVAL) => (),
        otherwise => panic!("unexpected result: {:?}", otherwise),
    }
    match nanosleep(&Timespec {
        tv_sec: 0,
        tv_nsec: !0,
    }) {
        NanosleepRelativeResult::Err(io::Errno::INVAL) => (),
        otherwise => panic!("unexpected result: {:?}", otherwise),
    }
    match nanosleep(&Timespec {
        tv_sec: !0,
        tv_nsec: 1_000_000_000,
    }) {
        NanosleepRelativeResult::Err(io::Errno::INVAL) => (),
        otherwise => panic!("unexpected result: {:?}", otherwise),
    }
    match nanosleep(&Timespec {
        tv_sec: !0,
        tv_nsec: !0,
    }) {
        NanosleepRelativeResult::Err(io::Errno::INVAL) => (),
        otherwise => panic!("unexpected result: {:?}", otherwise),
    }
}

#[cfg(not(any(
    apple,
    freebsdlike,
    target_os = "emscripten",
    target_os = "haiku",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
#[cfg(not(target_os = "netbsd"))] // NetBSD doesn't seem to enforce valid timespecs.
#[test]
fn test_invalid_nanosleep_absolute() {
    match clock_nanosleep_absolute(
        ClockId::Monotonic,
        &Timespec {
            tv_sec: 0,
            tv_nsec: 1_000_000_000,
        },
    ) {
        Err(io::Errno::INVAL) => (),
        otherwise => panic!("unexpected result: {:?}", otherwise),
    }
    match clock_nanosleep_absolute(
        ClockId::Monotonic,
        &Timespec {
            tv_sec: 0,
            tv_nsec: !0,
        },
    ) {
        Err(io::Errno::INVAL) => (),
        otherwise => panic!("unexpected result: {:?}", otherwise),
    }
    match clock_nanosleep_absolute(
        ClockId::Monotonic,
        &Timespec {
            tv_sec: !0,
            tv_nsec: 1_000_000_000,
        },
    ) {
        Err(io::Errno::INVAL) => (),
        otherwise => panic!("unexpected result: {:?}", otherwise),
    }
    match clock_nanosleep_absolute(
        ClockId::Monotonic,
        &Timespec {
            tv_sec: !0,
            tv_nsec: !0,
        },
    ) {
        Err(io::Errno::INVAL) => (),
        otherwise => panic!("unexpected result: {:?}", otherwise),
    }
}

#[cfg(not(any(
    apple,
    freebsdlike,
    target_os = "emscripten",
    target_os = "haiku",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
#[test]
fn test_invalid_nanosleep_relative() {
    match clock_nanosleep_relative(
        ClockId::Monotonic,
        &Timespec {
            tv_sec: 0,
            tv_nsec: 1_000_000_000,
        },
    ) {
        NanosleepRelativeResult::Err(io::Errno::INVAL) => (),
        otherwise => panic!("unexpected result: {:?}", otherwise),
    }
    match clock_nanosleep_relative(
        ClockId::Monotonic,
        &Timespec {
            tv_sec: 0,
            tv_nsec: !0,
        },
    ) {
        NanosleepRelativeResult::Err(io::Errno::INVAL) => (),
        otherwise => panic!("unexpected result: {:?}", otherwise),
    }
    match clock_nanosleep_relative(
        ClockId::Monotonic,
        &Timespec {
            tv_sec: !0,
            tv_nsec: 1_000_000_000,
        },
    ) {
        NanosleepRelativeResult::Err(io::Errno::INVAL) => (),
        otherwise => panic!("unexpected result: {:?}", otherwise),
    }
    match clock_nanosleep_relative(
        ClockId::Monotonic,
        &Timespec {
            tv_sec: !0,
            tv_nsec: !0,
        },
    ) {
        NanosleepRelativeResult::Err(io::Errno::INVAL) => (),
        otherwise => panic!("unexpected result: {:?}", otherwise),
    }
}

#[cfg(not(target_os = "redox"))]
#[test]
fn test_zero_nanosleep() {
    match nanosleep(&Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    }) {
        NanosleepRelativeResult::Ok => (),
        otherwise => panic!("unexpected result: {:?}", otherwise),
    }
}

#[cfg(not(any(
    apple,
    freebsdlike,
    target_os = "emscripten",
    target_os = "haiku",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
#[test]
fn test_zero_nanosleep_absolute() {
    match clock_nanosleep_absolute(
        ClockId::Monotonic,
        &Timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
    ) {
        Ok(()) => (),
        otherwise => panic!("unexpected result: {:?}", otherwise),
    }
}

#[cfg(not(any(
    apple,
    freebsdlike,
    target_os = "emscripten",
    target_os = "haiku",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
#[test]
fn test_zero_nanosleep_relative() {
    match clock_nanosleep_relative(
        ClockId::Monotonic,
        &Timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
    ) {
        NanosleepRelativeResult::Ok => (),
        otherwise => panic!("unexpected result: {:?}", otherwise),
    }
}
