//! Signal set manipulation.
//!
//! Based off of the implementations here:
//! https://github.com/torvalds/linux/blob/master/include/linux/signal.h

use super::super::c;
use super::types::Signal;
use core::mem::size_of;

const NUM_WORDS: usize = size_of::<c::sigset_t>() / size_of::<c::c_ulong>();
const BITS_PER_WORD: usize = size_of::<c::c_ulong>() * 8;

/// Initialize a signal set.
pub(crate) fn new_sigset() -> c::sigset_t {
    c::sigset_t {
        sig: [0; NUM_WORDS],
    }
}

/// Add a signal to a signal set.
pub(crate) fn add_sig(set: &mut c::sigset_t, signal: Signal) {
    let index = (signal as usize) - 1;
    let word = index / BITS_PER_WORD;
    let bit = index % BITS_PER_WORD;
    set.sig[word as usize] |= 1 << (bit as c::c_ulong);
}

/// Remove a signal from a signal set.
pub(crate) fn del_sig(set: &mut c::sigset_t, signal: Signal) {
    let index = (signal as usize) - 1;
    let word = index / BITS_PER_WORD;
    let bit = index % BITS_PER_WORD;
    set.sig[word as usize] &= !(1 << (bit as c::c_ulong));
}

/// Tell if a signal is in a signal set.
pub(crate) fn has_sig(set: &c::sigset_t, signal: Signal) -> bool {
    let index = (signal as usize) - 1;
    let word = index / BITS_PER_WORD;
    let bit = index % BITS_PER_WORD;
    set.sig[word as usize] & (1 << (bit as c::c_ulong)) != 0
}
