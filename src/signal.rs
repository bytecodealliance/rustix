//! Signal numbers.
//!
//! # Safety
//!
//! Some signal numbers are reserved by the libc.
//! [`Signal::from_raw_unchecked`] allows constructing `Signal` values with
//! arbitrary values. Users must avoid using these values to send or
//! consume signals in any way.
#![allow(unsafe_code)]

use crate::backend::c;
use core::num::NonZeroI32;

/// A signal number for use with [`kill_process`], [`kill_process_group`], and
/// [`kill_current_process_group`].
///
/// [`kill_process`]: crate::process::kill_process
/// [`kill_process_group`]: crate::process::kill_process_group
/// [`kill_current_process_group`]: crate::process::kill_current_process_group
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Signal(NonZeroI32);

/// Signal constants.
///
/// To construct `SIGRTMIN + n` “real-time” signal values, use
/// [`Signal::rt`].
// SAFETY: The libc-defined signal values are all non-zero.
#[rustfmt::skip]
impl Signal {
    /// `SIGHUP`
    pub const HUP: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGHUP) });
    /// `SIGINT`
    pub const INT: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGINT) });
    /// `SIGQUIT`
    pub const QUIT: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGQUIT) });
    /// `SIGILL`
    pub const ILL: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGILL) });
    /// `SIGTRAP`
    pub const TRAP: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGTRAP) });
    /// `SIGABRT`, aka `SIGIOT`
    #[doc(alias = "Iot")]
    #[doc(alias = "Abrt")]
    pub const ABORT: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGABRT) });
    /// `SIGBUS`
    pub const BUS: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGBUS) });
    /// `SIGFPE`
    pub const FPE: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGFPE) });
    /// `SIGKILL`
    pub const KILL: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGKILL) });
    /// `SIGUSR1`
    #[cfg(not(target_os = "vita"))]
    pub const USR1: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGUSR1) });
    /// `SIGSEGV`
    pub const SEGV: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGSEGV) });
    /// `SIGUSR2`
    #[cfg(not(target_os = "vita"))]
    pub const USR2: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGUSR2) });
    /// `SIGPIPE`
    pub const PIPE: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGPIPE) });
    /// `SIGALRM`
    #[doc(alias = "Alrm")]
    pub const ALARM: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGALRM) });
    /// `SIGTERM`
    pub const TERM: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGTERM) });
    /// `SIGSTKFLT`
    #[cfg(not(any(
        bsd,
        solarish,
        target_os = "aix",
        target_os = "haiku",
        target_os = "hurd",
        target_os = "nto",
        target_os = "vita",
        all(
            linux_kernel,
            any(
                target_arch = "mips",
                target_arch = "mips32r6",
                target_arch = "mips64",
                target_arch = "mips64r6",
                target_arch = "sparc",
                target_arch = "sparc64"
            ),
        )
    )))]
    pub const STKFLT: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGSTKFLT) });
    /// `SIGCHLD`
    #[cfg(not(target_os = "vita"))]
    #[doc(alias = "Chld")]
    pub const CHILD: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGCHLD) });
    /// `SIGCONT`
    #[cfg(not(target_os = "vita"))]
    pub const CONT: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGCONT) });
    /// `SIGSTOP`
    #[cfg(not(target_os = "vita"))]
    pub const STOP: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGSTOP) });
    /// `SIGTSTP`
    #[cfg(not(target_os = "vita"))]
    pub const TSTP: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGTSTP) });
    /// `SIGTTIN`
    #[cfg(not(target_os = "vita"))]
    pub const TTIN: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGTTIN) });
    /// `SIGTTOU`
    #[cfg(not(target_os = "vita"))]
    pub const TTOU: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGTTOU) });
    /// `SIGURG`
    #[cfg(not(target_os = "vita"))]
    pub const URG: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGURG) });
    /// `SIGXCPU`
    #[cfg(not(target_os = "vita"))]
    pub const XCPU: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGXCPU) });
    /// `SIGXFSZ`
    #[cfg(not(target_os = "vita"))]
    pub const XFSZ: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGXFSZ) });
    /// `SIGVTALRM`
    #[cfg(not(target_os = "vita"))]
    #[doc(alias = "Vtalrm")]
    pub const VTALARM: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGVTALRM) });
    /// `SIGPROF`
    #[cfg(not(target_os = "vita"))]
    pub const PROF: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGPROF) });
    /// `SIGWINCH`
    #[cfg(not(target_os = "vita"))]
    pub const WINCH: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGWINCH) });
    /// `SIGIO`, aka `SIGPOLL`
    #[doc(alias = "Poll")]
    #[cfg(not(any(target_os = "haiku", target_os = "vita")))]
    pub const IO: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGIO) });
    /// `SIGPWR`
    #[cfg(not(any(bsd, target_os = "haiku", target_os = "hurd", target_os = "vita")))]
    #[doc(alias = "Pwr")]
    pub const POWER: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGPWR) });
    /// `SIGSYS`, aka `SIGUNUSED`
    #[doc(alias = "Unused")]
    pub const SYS: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGSYS) });
    /// `SIGEMT`
    #[cfg(any(
        bsd,
        solarish,
        target_os = "aix",
        target_os = "hermit",
        all(
            linux_kernel,
            any(
                target_arch = "mips",
                target_arch = "mips32r6",
                target_arch = "mips64",
                target_arch = "mips64r6",
                target_arch = "sparc",
                target_arch = "sparc64"
            )
        )
    ))]
    pub const EMT: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGEMT) });
    /// `SIGINFO`
    #[cfg(bsd)]
    pub const INFO: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGINFO) });
    /// `SIGTHR`
    #[cfg(target_os = "freebsd")]
    #[doc(alias = "Lwp")]
    pub const THR: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGTHR) });
    /// `SIGLIBRT`
    #[cfg(target_os = "freebsd")]
    pub const LIBRT: Self = Self(unsafe { NonZeroI32::new_unchecked(c::SIGLIBRT) });
}

impl Signal {
    /// Convert a `Signal` to a raw signal number.
    ///
    /// To convert to a `NonZeroI32`, use [`Signal::as_raw_nonzero`].
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0.get()
    }

    /// Convert a `Signal` to a raw non-zero signal number.
    #[inline]
    pub const fn as_raw_nonzero(self) -> NonZeroI32 {
        self.0
    }

    /// `SIGRTMIN + n`—Convert a “real-time” signal offset into a `Signal`.
    ///
    /// This function adds `n` to the libc `SIGRTMIN` value to construct the
    /// raw signal value. If the result is greater than the platform `SIGRTMAX`
    /// value, it returns `None`.
    #[doc(alias = "SIGRTMIN", alias = "SIGRTMAX")]
    #[cfg(feature = "use-libc-sigrt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "use-libc-sigrt")))]
    #[cfg(any(linux_like, solarish, target_os = "hurd"))]
    pub fn rt(n: i32) -> Option<Self> {
        let min = libc_sigrt_min();
        let sig = min.wrapping_add(n);
        if sig >= min && sig <= libc_sigrt_max() {
            // SAFETY: Values at least `SIGRTMIN` will never be zero.
            let sig = unsafe { NonZeroI32::new_unchecked(sig) };
            Some(Self(sig))
        } else {
            None
        }
    }

    /// `SIGRTMIN`—Return the minimum “real-time” signal value.
    ///
    /// Use [`Signal::rt`] to construct `SIGRTMIN + n` values.
    #[doc(alias = "SIGRTMIN")]
    #[cfg(feature = "use-libc-sigrt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "use-libc-sigrt")))]
    #[cfg(any(linux_like, solarish, target_os = "hurd"))]
    pub fn rt_min() -> Self {
        // SAFETY: The libc is telling us this is the value it wants us to use.
        unsafe { Self::from_raw_unchecked(libc_sigrt_min()) }
    }

    /// `SIGRTMAX`—Return the maximum “real-time” signal value.
    #[doc(alias = "SIGRTMAX")]
    #[cfg(feature = "use-libc-sigrt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "use-libc-sigrt")))]
    #[cfg(any(linux_like, solarish, target_os = "hurd"))]
    pub fn rt_max() -> Self {
        // SAFETY: The libc is telling us this is the value it wants us to use.
        unsafe { Self::from_raw_unchecked(libc_sigrt_max()) }
    }

    /// Convert a raw signal number into a `Signal`.
    #[cfg(feature = "use-libc-sigrt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "use-libc-sigrt")))]
    #[cfg(any(linux_like, solarish, target_os = "hurd"))]
    pub fn from_raw(sig: i32) -> Option<Self> {
        if let Some(non_zero) = NonZeroI32::new(sig) {
            Self::from_raw_nonzero(non_zero)
        } else {
            None
        }
    }

    /// Convert a raw non-zero signal number into a `Signal`.
    #[cfg(feature = "use-libc-sigrt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "use-libc-sigrt")))]
    #[cfg(any(linux_like, solarish, target_os = "hurd"))]
    pub fn from_raw_nonzero(non_zero: NonZeroI32) -> Option<Self> {
        let sig = non_zero.get();
        match sig {
            c::SIGHUP => Some(Self::HUP),
            c::SIGINT => Some(Self::INT),
            c::SIGQUIT => Some(Self::QUIT),
            c::SIGILL => Some(Self::ILL),
            c::SIGTRAP => Some(Self::TRAP),
            c::SIGABRT => Some(Self::ABORT),
            c::SIGBUS => Some(Self::BUS),
            c::SIGFPE => Some(Self::FPE),
            c::SIGKILL => Some(Self::KILL),
            #[cfg(not(target_os = "vita"))]
            c::SIGUSR1 => Some(Self::USR1),
            c::SIGSEGV => Some(Self::SEGV),
            #[cfg(not(target_os = "vita"))]
            c::SIGUSR2 => Some(Self::USR2),
            c::SIGPIPE => Some(Self::PIPE),
            c::SIGALRM => Some(Self::ALARM),
            c::SIGTERM => Some(Self::TERM),
            #[cfg(not(any(
                bsd,
                solarish,
                target_os = "aix",
                target_os = "haiku",
                target_os = "hurd",
                target_os = "nto",
                target_os = "vita",
                all(
                    linux_kernel,
                    any(
                        target_arch = "mips",
                        target_arch = "mips32r6",
                        target_arch = "mips64",
                        target_arch = "mips64r6",
                        target_arch = "sparc",
                        target_arch = "sparc64"
                    ),
                )
            )))]
            c::SIGSTKFLT => Some(Self::STKFLT),
            #[cfg(not(target_os = "vita"))]
            c::SIGCHLD => Some(Self::CHILD),
            #[cfg(not(target_os = "vita"))]
            c::SIGCONT => Some(Self::CONT),
            #[cfg(not(target_os = "vita"))]
            c::SIGSTOP => Some(Self::STOP),
            #[cfg(not(target_os = "vita"))]
            c::SIGTSTP => Some(Self::TSTP),
            #[cfg(not(target_os = "vita"))]
            c::SIGTTIN => Some(Self::TTIN),
            #[cfg(not(target_os = "vita"))]
            c::SIGTTOU => Some(Self::TTOU),
            #[cfg(not(target_os = "vita"))]
            c::SIGURG => Some(Self::URG),
            #[cfg(not(target_os = "vita"))]
            c::SIGXCPU => Some(Self::XCPU),
            #[cfg(not(target_os = "vita"))]
            c::SIGXFSZ => Some(Self::XFSZ),
            #[cfg(not(target_os = "vita"))]
            c::SIGVTALRM => Some(Self::VTALARM),
            #[cfg(not(target_os = "vita"))]
            c::SIGPROF => Some(Self::PROF),
            #[cfg(not(target_os = "vita"))]
            c::SIGWINCH => Some(Self::WINCH),
            #[cfg(not(any(target_os = "haiku", target_os = "vita")))]
            c::SIGIO => Some(Self::IO),
            #[cfg(not(any(bsd, target_os = "haiku", target_os = "hurd", target_os = "vita")))]
            c::SIGPWR => Some(Self::POWER),
            c::SIGSYS => Some(Self::SYS),
            #[cfg(any(
                bsd,
                solarish,
                target_os = "aix",
                target_os = "hermit",
                all(
                    linux_kernel,
                    any(
                        target_arch = "mips",
                        target_arch = "mips32r6",
                        target_arch = "mips64",
                        target_arch = "mips64r6",
                        target_arch = "sparc",
                        target_arch = "sparc64"
                    )
                )
            ))]
            c::SIGEMT => Some(Self::EMT),
            #[cfg(bsd)]
            c::SIGINFO => Some(Self::INFO),
            #[cfg(target_os = "freebsd")]
            c::SIGTHR => Some(Self::THR),
            #[cfg(target_os = "freebsd")]
            c::SIGLIBRT => Some(Self::LIBRT),
            _ => {
                if sig >= libc_sigrt_min() && sig <= libc_sigrt_max() {
                    Some(Self(non_zero))
                } else {
                    None
                }
            }
        }
    }

    /// Convert a raw signal number into a `Signal` without checks.
    ///
    /// See [`Signal::from_raw`] to do this with checks.
    ///
    /// See [`Signal::from_raw_nonzero_unchecked`] if you already have a
    /// `NonZeroI32`.
    ///
    /// # Safety
    ///
    /// `sig` must be a valid and non-zero signal number.
    ///
    /// And, if `sig` is a signal number reserved by the libc, such as a value
    /// from [`SIGRTMIN`] to [`SIGRTMAX`] inclusive, then the resulting
    /// `Signal` must not be used to send any signals.
    ///
    /// [`SIGRTMIN`]: https://docs.rs/libc/latest/libc/fn.SIGRTMIN.html
    /// [`SIGRTMAX`]: https://docs.rs/libc/latest/libc/fn.SIGRTMAX.html
    #[inline]
    pub const unsafe fn from_raw_unchecked(sig: i32) -> Self {
        Self::from_raw_nonzero_unchecked(NonZeroI32::new_unchecked(sig))
    }

    /// Convert a raw non-zero signal number into a `Signal` without checks.
    ///
    /// See [`Signal::from_raw`] to do this with checks.
    ///
    /// # Safety
    ///
    /// `sig` must be a valid signal number.
    ///
    /// And, if `sig` is a signal number reserved by the libc, such as a value
    /// from [`SIGRTMIN`] to [`SIGRTMAX`] inclusive, then the resulting
    /// `Signal` must not be used to send any signals.
    ///
    /// [`SIGRTMIN`]: https://docs.rs/libc/latest/libc/fn.SIGRTMIN.html
    /// [`SIGRTMAX`]: https://docs.rs/libc/latest/libc/fn.SIGRTMAX.html
    #[inline]
    pub const unsafe fn from_raw_nonzero_unchecked(sig: NonZeroI32) -> Self {
        Self(sig)
    }
}

/// Return the libc `SIGRTMIN` value.
#[cfg(any(linux_like, solarish, target_os = "hurd"))]
#[cfg(feature = "use-libc-sigrt")]
fn libc_sigrt_min() -> i32 {
    // SAFETY: These are the ABI-compatible ways to obtain the `SIGRTMIN`
    // value.
    #[cfg(any(linux_like, target_os = "hurd"))]
    unsafe {
        extern "C" {
            fn __libc_current_sigrtmin() -> crate::ffi::c_int;
        }
        __libc_current_sigrtmin()
    }
    #[cfg(solarish)]
    unsafe {
        libc::SIGRTMIN()
    }
}

/// Return the libc `SIGRTMAX` value.
#[cfg(any(linux_like, solarish, target_os = "hurd"))]
#[cfg(feature = "use-libc-sigrt")]
fn libc_sigrt_max() -> i32 {
    // SAFETY: These are the ABI-compatible ways to obtain the `SIGRTMAX`
    // value.
    #[cfg(any(linux_like, target_os = "hurd"))]
    unsafe {
        extern "C" {
            fn __libc_current_sigrtmax() -> crate::ffi::c_int;
        }
        __libc_current_sigrtmax()
    }
    #[cfg(solarish)]
    unsafe {
        libc::SIGRTMAX()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        assert_eq!(Signal::HUP.as_raw(), libc::SIGHUP);
        unsafe {
            assert_eq!(Signal::from_raw_unchecked(libc::SIGHUP), Signal::HUP);
            assert_eq!(
                Signal::from_raw_nonzero_unchecked(NonZeroI32::new(libc::SIGHUP).unwrap()),
                Signal::HUP
            );
        }
    }

    #[cfg(any(linux_like, solarish, target_os = "hurd"))]
    #[cfg(feature = "use-libc-sigrt")]
    #[test]
    fn test_sigrt() {
        assert_eq!(libc::SIGRTMIN(), Signal::rt_min().as_raw());
        assert_eq!(libc::SIGRTMIN(), Signal::rt_min().as_raw_nonzero().get());
        assert_eq!(libc::SIGRTMAX(), Signal::rt_max().as_raw());
        assert_eq!(libc::SIGRTMAX(), Signal::rt_max().as_raw_nonzero().get());
        assert_eq!(Signal::rt(0).unwrap(), Signal::rt_min());
        // POSIX guarantees at least 8 values.
        assert_ne!(Signal::rt(7).unwrap(), Signal::rt_min());
        assert_ne!(Signal::rt(7).unwrap(), Signal::rt_max());
        assert_eq!(Signal::rt(7).unwrap().as_raw(), libc::SIGRTMIN() + 7);
        assert_eq!(
            Signal::rt(7).unwrap().as_raw_nonzero().get(),
            libc::SIGRTMIN() + 7
        );
        assert!(Signal::from_raw(0).is_none());
        for raw in libc::SIGRTMIN()..=libc::SIGRTMAX() {
            assert!(Signal::from_raw(raw).is_some());
        }
        assert!(Signal::from_raw(libc::SIGRTMIN() - 1).is_none());
        assert!(Signal::from_raw(libc::SIGRTMAX() + 1).is_none());
    }
}
