//! Utilities to help with buffering.

#![allow(unsafe_code)]

use core::mem::MaybeUninit;
use core::slice;

/// Split an uninitialized slice into initialized and uninitialized parts.
///
/// # Safety
///
/// At least `init` items must be initialized.
#[inline]
pub(super) unsafe fn split_init<T>(
    buf: &mut [MaybeUninit<T>],
    init: usize,
) -> (&mut [T], &mut [MaybeUninit<T>]) {
    let (init, uninit) = buf.split_at_mut(init);
    let init = slice::from_raw_parts_mut(init.as_mut_ptr().cast::<T>(), init.len());
    (init, uninit)
}
