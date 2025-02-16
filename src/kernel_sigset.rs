//! The [`KernelSigSet`] type.

#![allow(unsafe_code)]
#![allow(non_camel_case_types)]

use crate::signal::Signal;
use core::fmt;
use linux_raw_sys::general::{kernel_sigset_t, _NSIG};

/// `kernel_sigset_t`—A set of signal numbers, as used by some syscalls.
///
/// Like [`SigSet`] but with only enough space for the signals currently known
/// to be used by the kernel. It is used in functions where Linux's
/// documentation states that the size "is currently required to have a fixed
/// architecture specific value (equal to `sizeof(kernel_sigset_t)`)".
///
/// This type is guaranteed to have a subset of the layout of `libc::sigset_t`.
///
/// libc implementations typically reserve some signal values for internal use.
/// In a process that contains a libc, some unsafe functions invoke undefined
/// behavior if passed a `KernelSigSet` that contains one of the signals that
/// the libc reserves.
///
/// [`SigSet`]: crate::sigset::SigSet
#[repr(transparent)]
#[derive(Clone)]
pub struct KernelSigSet(kernel_sigset_t);

impl KernelSigSet {
    /// Create a new empty `KernelSigSet`.
    pub const fn empty() -> Self {
        Self(kernel_sigset_t {
            #[cfg(target_pointer_width = "64")]
            sig: [0; 1],
            #[cfg(target_pointer_width = "32")]
            sig: [0; 2],
        })
    }

    /// Create a new `KernelSigSet` with all signals set.
    ///
    /// This includes signals which are typically reserved for libc.
    pub const fn all() -> Self {
        Self(kernel_sigset_t {
            #[cfg(target_pointer_width = "64")]
            sig: [!0; 1],
            #[cfg(target_pointer_width = "32")]
            sig: [!0; 2],
        })
    }

    /// Remove all signals.
    pub fn clear(&mut self) {
        *self = Self(kernel_sigset_t {
            sig: Default::default(),
        });
    }

    /// Insert a signal.
    pub fn insert(&mut self, sig: Signal) {
        let sigs_per_elt = core::mem::size_of_val(&self.0.sig[0]) * 8;

        let raw = (sig.as_raw().wrapping_sub(1)) as usize;
        self.0.sig[raw / sigs_per_elt] |= 1 << (raw % sigs_per_elt);
    }

    /// Insert all signals.
    pub fn insert_all(&mut self) {
        self.0.sig.fill(!0);
    }

    /// Remove a signal.
    pub fn remove(&mut self, sig: Signal) {
        let sigs_per_elt = core::mem::size_of_val(&self.0.sig[0]) * 8;

        let raw = (sig.as_raw().wrapping_sub(1)) as usize;
        self.0.sig[raw / sigs_per_elt] &= !(1 << (raw % sigs_per_elt));
    }

    /// Test whether a given signal is present.
    pub fn contains(&self, sig: Signal) -> bool {
        let sigs_per_elt = core::mem::size_of_val(&self.0.sig[0]) * 8;

        let raw = (sig.as_raw().wrapping_sub(1)) as usize;
        (self.0.sig[raw / sigs_per_elt] & (1 << (raw % sigs_per_elt))) != 0
    }
}

impl Default for KernelSigSet {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

impl fmt::Debug for KernelSigSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_set();

        // Surprisingly, `_NSIG` is inclusive.
        for i in 1..=_NSIG {
            // SAFETY: This value is non-zero, in range, and only used for
            // debug output.
            let sig = unsafe { Signal::from_raw_unchecked(i as _) };

            if self.contains(sig) {
                d.entry(&sig);
            }
        }

        d.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{KERNEL_SIGRTMAX, KERNEL_SIGRTMIN};
    use crate::sigset::SigSet;
    use core::mem::{align_of, size_of};

    #[test]
    fn test_assumptions() {
        assert!(KERNEL_SIGRTMAX as usize - 1 < size_of::<KernelSigSet>() * 8);
    }

    #[test]
    fn test_layouts() {
        assert!(size_of::<KernelSigSet>() <= size_of::<libc::sigset_t>());
        assert!(align_of::<KernelSigSet>() <= align_of::<libc::sigset_t>());
    }

    /// A bunch of signals for testing.
    const SIGS: [Signal; 31] = [
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
        unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN) },
        unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 7) },
        unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMAX) },
    ];

    #[test]
    fn test_ops_plain() {
        for sig in SIGS {
            let mut set = KernelSigSet::empty();
            for sig in SIGS {
                assert!(!set.contains(sig));
            }

            set.insert(sig);
            assert!(set.contains(sig));
            for sig in SIGS.iter().filter(|s| **s != sig) {
                assert!(!set.contains(*sig));
            }

            set.remove(sig);
            for sig in SIGS {
                assert!(!set.contains(sig));
            }
        }
    }

    #[test]
    fn test_clear() {
        let mut set = KernelSigSet::empty();
        for sig in SIGS {
            set.insert(sig);
        }

        set.clear();

        for sig in SIGS {
            assert!(!set.contains(sig));
        }
    }

    // We guarantee that `KernelSigSet` has a subset of the layout of `SigSet`.
    // Test this.
    #[test]
    fn test_sigset_layout_compatibility() {
        use crate::utils::as_ptr;

        let mut s = SigSet::empty();
        let mut k = KernelSigSet::empty();

        assert_eq!(
            unsafe {
                libc::memcmp(
                    as_ptr(&s).cast(),
                    as_ptr(&k).cast(),
                    (KERNEL_SIGRTMAX as usize + 7) / 8,
                )
            },
            0
        );

        for sig in SIGS {
            k.insert(sig);
            assert_ne!(
                unsafe {
                    libc::memcmp(
                        as_ptr(&s).cast(),
                        as_ptr(&k).cast(),
                        (KERNEL_SIGRTMAX as usize + 7) / 8,
                    )
                },
                0
            );
            s.insert(sig);
            assert_eq!(
                unsafe {
                    libc::memcmp(
                        as_ptr(&s).cast(),
                        as_ptr(&k).cast(),
                        (KERNEL_SIGRTMAX as usize + 7) / 8,
                    )
                },
                0
            );
            k.remove(sig);
            assert_ne!(
                unsafe {
                    libc::memcmp(
                        as_ptr(&s).cast(),
                        as_ptr(&k).cast(),
                        (KERNEL_SIGRTMAX as usize + 7) / 8,
                    )
                },
                0
            );
            s.remove(sig);
            assert_eq!(
                unsafe {
                    libc::memcmp(
                        as_ptr(&s).cast(),
                        as_ptr(&k).cast(),
                        (KERNEL_SIGRTMAX as usize + 7) / 8,
                    )
                },
                0
            );
        }
    }
}
