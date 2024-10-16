//! Utilities to help with buffering.

#![allow(unsafe_code)]

use core::mem::MaybeUninit;
use core::slice;

/// Split an uninitialized byte slice into initialized and uninitialized parts.
///
/// # Safety
///
/// At least `init_len` bytes must be initialized.
#[inline]
pub(super) unsafe fn split_init(
    buf: &mut [MaybeUninit<u8>],
    init_len: usize,
) -> (&mut [u8], &mut [MaybeUninit<u8>]) {
    debug_assert!(init_len <= buf.len());
    let buf_ptr = buf.as_mut_ptr();
    let uninit_len = buf.len() - init_len;
    let init = slice::from_raw_parts_mut(buf_ptr.cast::<u8>(), init_len);
    let uninit = slice::from_raw_parts_mut(buf_ptr.add(init_len), uninit_len);
    (init, uninit)
}
