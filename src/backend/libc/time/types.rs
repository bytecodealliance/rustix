#[cfg(any(
    linux_kernel,
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "illumos",
    target_os = "netbsd"
))]
use crate::backend::c;
#[cfg(any(
    linux_kernel,
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "illumos",
    target_os = "netbsd"
))]
use crate::time::Itimerspec;
#[cfg(any(
    linux_kernel,
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "illumos",
    target_os = "netbsd"
))]
#[cfg(fix_y2038)]
use crate::timespec::LibcTimespec;
#[cfg(any(
    linux_kernel,
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "illumos",
    target_os = "netbsd"
))]
use bitflags::bitflags;

/// On most platforms, `LibcItimerspec` is just `Itimerspec`.
#[cfg(any(
    linux_kernel,
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "illumos",
    target_os = "netbsd"
))]
#[cfg(not(fix_y2038))]
pub(crate) type LibcItimerspec = Itimerspec;

/// On 32-bit glibc platforms, `LibcTimespec` differs from `Timespec`, so we
/// define our own struct, with bidirectional `From` impls.
#[cfg(any(
    linux_kernel,
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "illumos",
    target_os = "netbsd"
))]
#[cfg(fix_y2038)]
#[repr(C)]
#[derive(Debug, Clone)]
pub(crate) struct LibcItimerspec {
    pub it_interval: LibcTimespec,
    pub it_value: LibcTimespec,
}

#[cfg(any(
    linux_kernel,
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "illumos",
    target_os = "netbsd"
))]
#[cfg(fix_y2038)]
impl From<LibcItimerspec> for Itimerspec {
    #[inline]
    fn from(t: LibcItimerspec) -> Self {
        Self {
            it_interval: t.it_interval.into(),
            it_value: t.it_value.into(),
        }
    }
}

#[cfg(any(
    linux_kernel,
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "illumos",
    target_os = "netbsd"
))]
#[cfg(fix_y2038)]
impl From<Itimerspec> for LibcItimerspec {
    #[inline]
    fn from(t: Itimerspec) -> Self {
        Self {
            it_interval: t.it_interval.into(),
            it_value: t.it_value.into(),
        }
    }
}

#[cfg(any(
    linux_kernel,
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "illumos",
    target_os = "netbsd"
))]
#[cfg(not(fix_y2038))]
pub(crate) fn as_libc_itimerspec_ptr(itimerspec: &Itimerspec) -> *const c::itimerspec {
    #[cfg(test)]
    {
        assert_eq_size!(Itimerspec, c::itimerspec);
    }
    crate::utils::as_ptr(itimerspec).cast::<c::itimerspec>()
}

#[cfg(any(
    linux_kernel,
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "illumos",
    target_os = "netbsd"
))]
#[cfg(not(fix_y2038))]
pub(crate) fn as_libc_itimerspec_mut_ptr(
    itimerspec: &mut core::mem::MaybeUninit<Itimerspec>,
) -> *mut c::itimerspec {
    #[cfg(test)]
    {
        assert_eq_size!(Itimerspec, c::itimerspec);
    }
    itimerspec.as_mut_ptr().cast::<c::itimerspec>()
}

#[cfg(any(
    linux_kernel,
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "illumos",
    target_os = "netbsd"
))]
bitflags! {
    /// `TFD_*` flags for use with [`timerfd_create`].
    ///
    /// [`timerfd_create`]: crate::time::timerfd_create
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct TimerfdFlags: u32 {
        /// `TFD_NONBLOCK`
        #[doc(alias = "TFD_NONBLOCK")]
        const NONBLOCK = bitcast!(c::TFD_NONBLOCK);

        /// `TFD_CLOEXEC`
        #[doc(alias = "TFD_CLOEXEC")]
        const CLOEXEC = bitcast!(c::TFD_CLOEXEC);

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

#[cfg(any(
    linux_kernel,
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "illumos",
    target_os = "netbsd"
))]
bitflags! {
    /// `TFD_TIMER_*` flags for use with [`timerfd_settime`].
    ///
    /// [`timerfd_settime`]: crate::time::timerfd_settime
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct TimerfdTimerFlags: u32 {
        /// `TFD_TIMER_ABSTIME`
        #[doc(alias = "TFD_TIMER_ABSTIME")]
        const ABSTIME = bitcast!(c::TFD_TIMER_ABSTIME);

        /// `TFD_TIMER_CANCEL_ON_SET`
        #[cfg(not(target_os = "fuchsia"))]
        #[doc(alias = "TFD_TIMER_CANCEL_ON_SET")]
        const CANCEL_ON_SET = bitcast!(c::TFD_TIMER_CANCEL_ON_SET);

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

/// `CLOCK_*` constants for use with [`timerfd_create`].
///
/// [`timerfd_create`]: crate::time::timerfd_create
#[cfg(any(
    linux_kernel,
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "illumos",
    target_os = "netbsd"
))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
#[non_exhaustive]
pub enum TimerfdClockId {
    /// `CLOCK_REALTIME`—A clock that tells the “real” time.
    ///
    /// This is a clock that tells the amount of time elapsed since the Unix
    /// epoch, 1970-01-01T00:00:00Z. The clock is externally settable, so it is
    /// not monotonic. Successive reads may see decreasing times, so it isn't
    /// reliable for measuring durations.
    #[doc(alias = "CLOCK_REALTIME")]
    Realtime = bitcast!(c::CLOCK_REALTIME),

    /// `CLOCK_MONOTONIC`—A clock that tells an abstract time.
    ///
    /// Unlike `Realtime`, this clock is not based on a fixed known epoch, so
    /// individual times aren't meaningful. However, since it isn't settable,
    /// it is reliable for measuring durations.
    ///
    /// This clock does not advance while the system is suspended; see
    /// `Boottime` for a clock that does.
    #[doc(alias = "CLOCK_MONOTONIC")]
    Monotonic = bitcast!(c::CLOCK_MONOTONIC),

    /// `CLOCK_BOOTTIME`—Like `Monotonic`, but advances while suspended.
    ///
    /// This clock is similar to `Monotonic`, but does advance while the system
    /// is suspended.
    #[doc(alias = "CLOCK_BOOTTIME")]
    #[cfg(any(linux_kernel, target_os = "fuchsia", target_os = "openbsd"))]
    Boottime = bitcast!(c::CLOCK_BOOTTIME),

    /// `CLOCK_REALTIME_ALARM`—Like `Realtime`, but wakes a suspended system.
    ///
    /// This clock is like `Realtime`, but can wake up a suspended system.
    ///
    /// Use of this clock requires the `CAP_WAKE_ALARM` Linux capability.
    #[cfg(linux_kernel)]
    #[doc(alias = "CLOCK_REALTIME_ALARM")]
    RealtimeAlarm = bitcast!(c::CLOCK_REALTIME_ALARM),

    /// `CLOCK_BOOTTIME_ALARM`—Like `Boottime`, but wakes a suspended system.
    ///
    /// This clock is like `Boottime`, but can wake up a suspended system.
    ///
    /// Use of this clock requires the `CAP_WAKE_ALARM` Linux capability.
    #[cfg(any(linux_kernel, target_os = "cygwin", target_os = "fuchsia"))]
    #[doc(alias = "CLOCK_BOOTTIME_ALARM")]
    BoottimeAlarm = bitcast!(c::CLOCK_BOOTTIME_ALARM),
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[cfg(any(
        linux_kernel,
        target_os = "freebsd",
        target_os = "fuchsia",
        target_os = "illumos",
        target_os = "netbsd"
    ))]
    #[test]
    fn test_types() {
        assert_eq_size!(TimerfdFlags, c::c_int);
        assert_eq_size!(TimerfdTimerFlags, c::c_int);
    }
}
