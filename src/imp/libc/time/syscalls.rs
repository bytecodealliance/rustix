//! libc syscalls supporting `rustix::time`.

use super::super::c;
use super::super::conv::ret;
use super::Timespec;
#[cfg(not(target_os = "wasi"))]
use super::{ClockId, DynamicClockId};
use crate::io;
use core::mem::MaybeUninit;

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[inline]
#[must_use]
pub(crate) fn clock_getres(id: ClockId) -> Timespec {
    let mut timespec = MaybeUninit::<Timespec>::uninit();
    unsafe {
        let _ = c::clock_getres(id as c::clockid_t, timespec.as_mut_ptr());
        timespec.assume_init()
    }
}

#[cfg(not(target_os = "wasi"))]
#[inline]
#[must_use]
pub(crate) fn clock_gettime(id: ClockId) -> Timespec {
    let mut timespec = MaybeUninit::<Timespec>::uninit();
    // Use `unwrap()` here because `clock_getres` can fail if the clock itself
    // overflows a number of seconds, but if that happens, the monotonic clocks
    // can't maintain their invariants, or the realtime clocks aren't properly
    // configured.
    unsafe {
        ret(c::clock_gettime(id as c::clockid_t, timespec.as_mut_ptr())).unwrap();
        timespec.assume_init()
    }
}

#[cfg(not(target_os = "wasi"))]
#[inline]
pub(crate) fn clock_gettime_dynamic(id: DynamicClockId<'_>) -> io::Result<Timespec> {
    let mut timespec = MaybeUninit::<Timespec>::uninit();
    unsafe {
        let id: c::clockid_t = match id {
            DynamicClockId::Known(id) => id as c::clockid_t,

            #[cfg(any(target_os = "android", target_os = "linux"))]
            DynamicClockId::Dynamic(fd) => {
                use crate::fd::AsRawFd;
                const CLOCKFD: i32 = 3;
                (!fd.as_raw_fd() << 3) | CLOCKFD
            }

            #[cfg(not(any(target_os = "android", target_os = "linux")))]
            DynamicClockId::Dynamic(_fd) => {
                // Dynamic clocks are not supported on this platform.
                return Err(io::Error::INVAL);
            }

            #[cfg(any(target_os = "android", target_os = "linux"))]
            DynamicClockId::RealtimeAlarm => c::CLOCK_REALTIME_ALARM,

            #[cfg(any(target_os = "android", target_os = "linux"))]
            DynamicClockId::Tai => c::CLOCK_TAI,

            #[cfg(any(target_os = "android", target_os = "linux"))]
            DynamicClockId::Boottime => c::CLOCK_BOOTTIME,

            #[cfg(any(target_os = "android", target_os = "linux"))]
            DynamicClockId::BoottimeAlarm => c::CLOCK_BOOTTIME_ALARM,
        };

        ret(c::clock_gettime(id as c::clockid_t, timespec.as_mut_ptr()))?;

        Ok(timespec.assume_init())
    }
}
