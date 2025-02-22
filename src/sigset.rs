//! The [`SigSet`] type.
//!
//! # Safety
//!
//! This code calls libc `sigset_t` functions.

#![allow(unsafe_code)]
#![allow(non_camel_case_types)]

#[cfg(all(linux_raw, feature = "runtime"))]
use crate::kernel_sigset::KernelSigSet;
use crate::signal::Signal;
#[cfg(linux_raw)]
use crate::utils::as_mut_ptr;
use core::fmt;
#[cfg(linux_raw)]
use core::mem::MaybeUninit;

// linux_raw backend: use a `sigset_t` that matches libc so that we support the
// same extensibility.
#[cfg(linux_raw)]
#[repr(C)]
#[derive(Clone)]
struct sigset_t {
    // The fields are `MaybeUninit` because this is guaranteed to match the
    // libc representation and libc implementations don't always initialize
    // fields beyond `_NSIG`.
    #[cfg(target_pointer_width = "32")]
    __val: [MaybeUninit<u32>; 32],
    #[cfg(target_pointer_width = "64")]
    __val: [MaybeUninit<u64>; 16],
}

// libc backend: use libc's `sigset_t`.
#[cfg(libc)]
use libc::sigset_t;

/// `sigset_t`—A set of signal numbers.
///
/// This type is guaranteed to have the same layout as `libc::sigset_t`.
///
/// libc implementations typically reserve some signal values for internal use.
/// In a process that contains a libc, some unsafe functions invoke undefined
/// behavior if passed a `SigSet` that contains one of the signals that the
/// libc reserves.
///
/// For additional operations such as [`SigSet::all`] and
/// [`SigSet::insert_all`], which are wrappers around `libc::sigfillset`, see
/// [rustix-libc-wrappers].
///
/// [`SigSet::all`]: https://docs.rs/rustix-libc-wrappers/*/rustix_libc_wrappers/trait.SigSetExt.html#tymethod.all
/// [`SigSet::insert_all`]: https://docs.rs/rustix-libc-wrappers/*/rustix_libc_wrappers/trait.SigSetExt.html#tymethod.insert_all
/// [rustix-libc-wrappers]: https://docs.rs/rustix-libc-wrappers
// This is `repr(transparent)` so that users such as rustix-libc-wrappers and
// c-scape can cast pointers to `libc::sigset_t` to pointers to `SigSet`.
#[repr(transparent)]
#[derive(Clone)]
#[doc(alias = "sigfillset")]
pub struct SigSet(sigset_t);

impl SigSet {
    /// Create a new empty `SigSet`.
    #[doc(alias = "sigemptyset")]
    pub fn empty() -> Self {
        #[cfg(linux_raw)]
        {
            Self(sigset_t {
                #[cfg(target_pointer_width = "32")]
                __val: [MaybeUninit::zeroed(); 32],
                #[cfg(target_pointer_width = "64")]
                __val: [MaybeUninit::zeroed(); 16],
            })
        }

        // SAFETY: Use `sigemptyset` from libc to initialize `new`.
        #[cfg(libc)]
        unsafe {
            let mut new = core::mem::MaybeUninit::<sigset_t>::uninit();
            let _r = libc::sigemptyset(new.as_mut_ptr());
            debug_assert_eq!(_r, 0);
            Self(new.assume_init())
        }
    }

    /// Remove all signals.
    #[doc(alias = "sigemptyset")]
    pub fn clear(&mut self) {
        #[cfg(linux_raw)]
        {
            self.0.__val.fill(MaybeUninit::zeroed());
        }

        // SAFETY: Use `sigemptyset` from libc.
        #[cfg(libc)]
        unsafe {
            let _r = libc::sigemptyset(&mut self.0);
            debug_assert_eq!(_r, 0);
        }
    }

    /// Insert a signal.
    #[doc(alias = "sigaddset")]
    pub fn insert(&mut self, sig: Signal) {
        // This assumes the `Signal` does not contain any of the values
        // reserved by libc, which is enforced by `Signal`'s API.
        #[cfg(linux_raw)]
        {
            let sigs_per_elt = core::mem::size_of_val(&self.0.__val[0]) * 8;

            let raw = (sig.as_raw().wrapping_sub(1)) as usize;
            let elt = unsafe { self.0.__val[raw / sigs_per_elt].assume_init_mut() };
            *elt |= 1 << (raw % sigs_per_elt);
        }

        // SAFETY: Use `sigaddset` from libc.
        #[cfg(libc)]
        unsafe {
            let _r = libc::sigaddset(&mut self.0, sig.as_raw());
            debug_assert_eq!(_r, 0);
        }
    }

    /// Remove a signal.
    #[doc(alias = "sigdelset")]
    pub fn remove(&mut self, sig: Signal) {
        // This assumes the `Signal` does not contain any of the values
        // reserved by libc, which is enforced by `Signal`'s API.
        #[cfg(linux_raw)]
        {
            let sigs_per_elt = core::mem::size_of_val(&self.0.__val[0]) * 8;

            let raw = (sig.as_raw().wrapping_sub(1)) as usize;
            let elt = unsafe { self.0.__val[raw / sigs_per_elt].assume_init_mut() };
            *elt &= !(1 << (raw % sigs_per_elt));
        }

        // SAFETY: Use `sigdelset` from libc.
        #[cfg(libc)]
        unsafe {
            let _r = libc::sigdelset(&mut self.0, sig.as_raw());
            debug_assert_eq!(_r, 0);
        }
    }

    /// Test whether a given signal is present.
    #[doc(alias = "sigismember")]
    pub fn contains(&self, sig: Signal) -> bool {
        #[cfg(linux_raw)]
        {
            let sigs_per_elt = core::mem::size_of_val(&self.0.__val[0]) * 8;

            let raw = (sig.as_raw().wrapping_sub(1)) as usize;
            let elt = unsafe { self.0.__val[raw / sigs_per_elt].assume_init_ref() };
            (*elt & (1 << (raw % sigs_per_elt))) != 0
        }

        // SAFETY: Use `sigismember` from libc.
        #[cfg(libc)]
        unsafe {
            // Treat `-1` as meaning the set does not contain the signal.
            libc::sigismember(&self.0, sig.as_raw()) == 1
        }
    }
}

#[cfg(all(linux_raw, feature = "runtime"))]
impl From<KernelSigSet> for SigSet {
    fn from(kernel: KernelSigSet) -> Self {
        let mut res = Self::empty();
        // SAFETY: We guarantee that `KernelSigSet` is a subset of the layout
        // of `SigSet`.
        unsafe {
            as_mut_ptr(&mut res).cast::<KernelSigSet>().write(kernel);
        }
        res
    }
}

impl fmt::Debug for SigSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_set();

        // Surprisingly, `_NSIG` is inclusive.
        #[cfg(linux_raw)]
        for i in 1..=linux_raw_sys::general::_NSIG {
            // SAFETY: This value is non-zero, in range, and only used for
            // debug output.
            let sig = unsafe { Signal::from_raw_unchecked(i as _) };

            if self.contains(sig) {
                d.entry(&sig);
            }
        }

        #[cfg(libc)]
        {
            // First check all the known signals.
            for i in 1..=libc::SIGRTMAX() {
                if unsafe { libc::sigismember(&self.0, i) } == 1 {
                    d.entry(&unsafe { Signal::from_raw_unchecked(i) });
                }
            }
            // Then check for any reserved signals above `libc::SIGRTMAX`.
            let mut i = libc::SIGRTMAX();
            loop {
                i += 1;
                match unsafe { libc::sigismember(&self.0, i) } {
                    1 => {
                        d.entry(&unsafe { Signal::from_raw_unchecked(i) });
                    }
                    0 => {}
                    // libc will return an error when we pass the upper bound.
                    _ => break,
                }
            }
        }

        d.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assumptions() {
        // POSIX guarantees at least 8 RT signals.
        assert!(libc::SIGRTMIN() + 8 <= libc::SIGRTMAX());

        assert!(libc::SIGRTMAX() as usize - 1 < core::mem::size_of::<libc::sigset_t>() * 8);
    }

    #[test]
    fn test_layouts() {
        assert_eq_size!(SigSet, libc::sigset_t);
        assert_eq_align!(SigSet, libc::sigset_t);
    }

    /// A bunch of signals for testing.
    fn sigs() -> [Signal; 31] {
        [
            Signal::HUP,
            Signal::INT,
            Signal::QUIT,
            Signal::ILL,
            Signal::TRAP,
            Signal::ABORT,
            Signal::BUS,
            Signal::FPE,
            Signal::KILL,
            Signal::USR1,
            Signal::SEGV,
            Signal::USR2,
            Signal::PIPE,
            Signal::ALARM,
            Signal::TERM,
            Signal::CHILD,
            Signal::CONT,
            Signal::STOP,
            Signal::TSTP,
            Signal::TTIN,
            Signal::TTOU,
            Signal::URG,
            Signal::XCPU,
            Signal::XFSZ,
            Signal::VTALARM,
            Signal::PROF,
            Signal::WINCH,
            Signal::SYS,
            unsafe { Signal::from_raw_unchecked(libc::SIGRTMIN()) },
            unsafe { Signal::from_raw_unchecked(libc::SIGRTMIN() + 7) },
            unsafe { Signal::from_raw_unchecked(libc::SIGRTMAX()) },
        ]
    }

    #[test]
    fn test_ops_plain() {
        for sig in sigs() {
            let mut set = SigSet::empty();
            for sig in sigs() {
                assert!(!set.contains(sig));
            }

            set.insert(sig);
            assert!(set.contains(sig));
            for sig in sigs().iter().filter(|s| **s != sig) {
                assert!(!set.contains(*sig));
            }

            set.remove(sig);
            for sig in sigs() {
                assert!(!set.contains(sig));
            }
        }
    }

    #[test]
    fn test_clear() {
        let mut set = SigSet::empty();
        for sig in sigs() {
            set.insert(sig);
        }

        set.clear();

        for sig in sigs() {
            assert!(!set.contains(sig));
        }
    }

    // io_uring libraries assume that libc's `sigset_t` matches the layout
    // of the Linux kernel's `kernel_sigset_t`. Test that rustix's layout
    // matches as well.
    #[test]
    fn test_libc_layout_compatibility() {
        use crate::utils::as_ptr;

        let mut lc = unsafe { core::mem::zeroed::<libc::sigset_t>() };
        let mut ru = SigSet::empty();
        let r = unsafe { libc::sigemptyset(&mut lc) };

        assert_eq!(r, 0);
        assert_eq!(
            unsafe {
                libc::memcmp(
                    as_ptr(&lc).cast(),
                    as_ptr(&ru).cast(),
                    (libc::SIGRTMAX() as usize + 7) / 8,
                )
            },
            0
        );

        for sig in sigs() {
            ru.insert(sig);
            assert_ne!(
                unsafe {
                    libc::memcmp(
                        as_ptr(&lc).cast(),
                        as_ptr(&ru).cast(),
                        (libc::SIGRTMAX() as usize + 7) / 8,
                    )
                },
                0
            );
            let r = unsafe { libc::sigaddset(&mut lc, sig.as_raw()) };
            assert_eq!(r, 0);
            assert_eq!(
                unsafe {
                    libc::memcmp(
                        as_ptr(&lc).cast(),
                        as_ptr(&ru).cast(),
                        (libc::SIGRTMAX() as usize + 7) / 8,
                    )
                },
                0
            );
            ru.remove(sig);
            assert_ne!(
                unsafe {
                    libc::memcmp(
                        as_ptr(&lc).cast(),
                        as_ptr(&ru).cast(),
                        (libc::SIGRTMAX() as usize + 7) / 8,
                    )
                },
                0
            );
            let r = unsafe { libc::sigdelset(&mut lc, sig.as_raw()) };
            assert_eq!(r, 0);
            assert_eq!(
                unsafe {
                    libc::memcmp(
                        as_ptr(&lc).cast(),
                        as_ptr(&ru).cast(),
                        (libc::SIGRTMAX() as usize + 7) / 8,
                    )
                },
                0
            );
        }
    }
}
