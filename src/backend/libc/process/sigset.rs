//! Handle signal sets.

use super::super::c;
use super::types::Signal;
use core::mem::MaybeUninit;

/// Initialize a signal set.
pub(crate) fn new_sigset() -> c::sigset_t {
    unsafe {
        let mut set = MaybeUninit::uninit();
        c::sigemptyset(set.as_mut_ptr());
        set.assume_init()
    }
}

/// Add a signal to a signal set.
pub(crate) fn add_sig(set: &mut c::sigset_t, sig: Signal) {
    unsafe {
        c::sigaddset(set, sig as _);
    }
}

/// Remove a signal from a signal set.
pub(crate) fn del_sig(set: &mut c::sigset_t, sig: Signal) {
    unsafe {
        c::sigdelset(set, sig as _);
    }
}

/// Check if a signal is in a signal set.
pub(crate) fn has_sig(set: &c::sigset_t, sig: Signal) -> bool {
    unsafe { c::sigismember(set, sig as _) != 0 }
}
