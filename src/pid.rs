#![allow(unsafe_code)]

use crate::backend::c;

/// A process identifier as a raw integer.
pub type RawPid = c::pid_t;
/// A non-zero process identifier as a raw non-zero integer.
pub type RawNonZeroPid = core::num::NonZeroI32;

/// `pid_t`—A non-zero Unix process ID.
///
/// This is a pid, and not a pidfd. It is not a file descriptor, and the
/// process it refers to could disappear at any time and be replaced by
/// another, unrelated, process.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Pid(RawNonZeroPid);

impl Pid {
    /// A `Pid` corresponding to the init process (pid 1).
    pub const INIT: Self = Self(
        // SAFETY: The init process' pid is always valid.
        unsafe { RawNonZeroPid::new_unchecked(1) },
    );

    /// Converts a `RawPid` into a `Pid`.
    ///
    /// # Safety
    ///
    /// `raw` must be the value of a valid Unix process ID, or zero.
    #[inline]
    pub const unsafe fn from_raw(raw: RawPid) -> Option<Self> {
        match RawNonZeroPid::new(raw) {
            Some(pid) => Some(Self(pid)),
            None => None,
        }
    }

    /// Converts a known non-zero `RawPid` into a `Pid`.
    ///
    /// # Safety
    ///
    /// `raw` must be the value of a valid Unix process ID. It must not be
    /// zero.
    #[inline]
    pub const unsafe fn from_raw_nonzero(raw: RawNonZeroPid) -> Self {
        Self(raw)
    }

    /// Creates a `Pid` holding the ID of the given child process.
    #[cfg(feature = "std")]
    #[inline]
    pub fn from_child(child: &std::process::Child) -> Self {
        let id = child.id();
        debug_assert_ne!(id, 0);

        // SAFETY: We know the returned ID is valid because it came directly
        // from an OS API.
        unsafe { Self::from_raw_nonzero(RawNonZeroPid::new_unchecked(id as _)) }
    }

    /// Converts a `Pid` into a `RawNonZeroPid`.
    #[inline]
    pub const fn as_raw_nonzero(self) -> RawNonZeroPid {
        self.0
    }

    /// Converts an `Option<Pid>` into a `RawPid`.
    #[inline]
    pub fn as_raw(pid: Option<Self>) -> RawPid {
        pid.map_or(0, |pid| pid.0.get())
    }

    /// Test whether this pid represents the init process (pid 0).
    #[inline]
    pub const fn is_init(self) -> bool {
        self.0.get() == Self::INIT.0.get()
    }
}

#[test]
fn test_sizes() {
    use core::mem::size_of;

    assert_eq!(size_of::<RawPid>(), size_of::<RawNonZeroPid>());
    assert_eq!(size_of::<RawPid>(), size_of::<Pid>());
}
