//! `Timespec` and related types, which are used by multiple public API
//! modules.

#![allow(dead_code)]

#[cfg(not(fix_y2038))]
use crate::backend::c;
#[allow(unused)]
use crate::ffi;
#[cfg(not(fix_y2038))]
use core::ptr::null;

/// `struct timespec`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Timespec {
    /// Seconds.
    pub tv_sec: Secs,

    /// Nanoseconds. Must be less than 1_000_000_000.
    pub tv_nsec: Nsecs,
}

/// A type for the `tv_sec` field of [`Timespec`].
pub type Secs = i64;

/// A type for the `tv_sec` field of [`Timespec`].
#[cfg(any(
    fix_y2038,
    linux_raw,
    all(libc, target_arch = "x86_64", target_pointer_width = "32")
))]
pub type Nsecs = i64;

/// A type for the `tv_nsec` field of [`Timespec`].
#[cfg(all(
    not(fix_y2038),
    libc,
    not(all(target_arch = "x86_64", target_pointer_width = "32"))
))]
pub type Nsecs = ffi::c_long;

/// On 32-bit glibc platforms, `timespec` has anonymous padding fields, which
/// Rust doesn't support yet (see `unnamed_fields`), so we define our own
/// struct with explicit padding, with bidirectional `From` impls.
#[cfg(fix_y2038)]
#[repr(C)]
#[derive(Debug, Clone)]
pub(crate) struct LibcTimespec {
    pub(crate) tv_sec: Secs,

    #[cfg(target_endian = "big")]
    padding: core::mem::MaybeUninit<u32>,

    pub(crate) tv_nsec: i32,

    #[cfg(target_endian = "little")]
    padding: core::mem::MaybeUninit<u32>,
}

#[cfg(fix_y2038)]
impl From<LibcTimespec> for Timespec {
    #[inline]
    fn from(t: LibcTimespec) -> Self {
        Self {
            tv_sec: t.tv_sec,
            tv_nsec: t.tv_nsec as _,
        }
    }
}

#[cfg(fix_y2038)]
impl From<Timespec> for LibcTimespec {
    #[inline]
    fn from(t: Timespec) -> Self {
        Self {
            tv_sec: t.tv_sec,
            tv_nsec: t.tv_nsec as _,
            padding: core::mem::MaybeUninit::uninit(),
        }
    }
}

#[cfg(not(fix_y2038))]
pub(crate) fn as_libc_timespec_ptr(timespec: &Timespec) -> *const c::timespec {
    #[cfg(test)]
    {
        assert_eq_size!(Timespec, c::timespec);
    }
    crate::utils::as_ptr(timespec).cast::<c::timespec>()
}

#[cfg(not(fix_y2038))]
pub(crate) fn as_libc_timespec_mut_ptr(
    timespec: &mut core::mem::MaybeUninit<Timespec>,
) -> *mut c::timespec {
    #[cfg(test)]
    {
        assert_eq_size!(Timespec, c::timespec);
    }
    timespec.as_mut_ptr().cast::<c::timespec>()
}

#[cfg(not(fix_y2038))]
pub(crate) fn option_as_libc_timespec_ptr(timespec: Option<&Timespec>) -> *const c::timespec {
    match timespec {
        None => null(),
        Some(timespec) => as_libc_timespec_ptr(timespec),
    }
}

/// As described [here], Apple platforms may return a negative nanoseconds
/// value in some cases; adjust it so that nanoseconds is always in
/// `0..1_000_000_000`.
///
/// [here]: https://github.com/rust-lang/rust/issues/108277#issuecomment-1787057158
#[cfg(apple)]
#[inline]
pub(crate) fn fix_negative_nsecs(secs: &mut i64, mut nsecs: i32) -> i32 {
    #[cold]
    fn adjust(secs: &mut i64, nsecs: i32) -> i32 {
        assert!(nsecs >= -1_000_000_000);
        assert!(*secs < 0);
        assert!(*secs > i64::MIN);
        *secs -= 1;
        nsecs + 1_000_000_000
    }

    if nsecs < 0 {
        nsecs = adjust(secs, nsecs);
    }
    nsecs
}

#[cfg(apple)]
#[test]
fn test_negative_timestamps() {
    let mut secs = -59;
    let mut nsecs = -900_000_000;
    nsecs = fix_negative_nsecs(&mut secs, nsecs);
    assert_eq!(secs, -60);
    assert_eq!(nsecs, 100_000_000);
    nsecs = fix_negative_nsecs(&mut secs, nsecs);
    assert_eq!(secs, -60);
    assert_eq!(nsecs, 100_000_000);
}

#[test]
fn test_sizes() {
    assert_eq_size!(Secs, u64);
    const_assert!(core::mem::size_of::<Timespec>() >= core::mem::size_of::<(u64, u32)>());
    const_assert!(core::mem::size_of::<Nsecs>() >= 4);

    let mut t = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    // `tv_nsec` needs to be able to hold nanoseconds up to a second.
    t.tv_nsec = 999_999_999_u32 as _;
    assert_eq!(t.tv_nsec as u64, 999_999_999_u64);

    // `tv_sec` needs to be able to hold more than 32-bits of seconds.
    t.tv_sec = 0x1_0000_0000_u64 as _;
    assert_eq!(t.tv_sec as u64, 0x1_0000_0000_u64);
}

// Test that our workarounds are needed.
#[cfg(fix_y2038)]
#[test]
#[allow(deprecated)]
fn test_fix_y2038() {
    assert_eq_size!(libc::time_t, u32);
}

#[cfg(not(fix_y2038))]
#[test]
fn timespec_layouts() {
    use crate::backend::c;
    check_renamed_type!(Timespec, timespec);
    #[cfg(linux_raw)]
    assert_eq_size!(Timespec, linux_raw_sys::general::__kernel_timespec);
}
