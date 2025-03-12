//! linux_raw syscalls supporting `rustix::rand`.
//!
//! # Safety
//!
//! See the `rustix::backend` module documentation for details.
#![allow(unsafe_code, clippy::undocumented_unsafe_blocks)]

use crate::backend::conv::{buffer_len, buffer_ptr, ret_usize};
use crate::io;
use crate::rand::GetRandomFlags;
use core::mem::MaybeUninit;

#[inline]
pub(crate) unsafe fn getrandom(
    buf: *mut [MaybeUninit<u8>],
    flags: GetRandomFlags,
) -> io::Result<usize> {
    ret_usize(syscall!(
        __NR_getrandom,
        buffer_ptr(buf),
        buffer_len(buf),
        flags
    ))
}
