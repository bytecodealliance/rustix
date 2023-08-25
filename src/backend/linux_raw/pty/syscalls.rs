//! linux_raw syscalls supporting `rustix::pty`.
//!
//! # Safety
//!
//! See the `rustix::backend` module documentation for details.
#![allow(unsafe_code)]
#![allow(clippy::undocumented_unsafe_blocks)]

use crate::backend::c;
use crate::backend::conv::{by_ref, c_uint, ret};
use crate::fd::BorrowedFd;
use crate::ffi::CString;
use crate::io;
use crate::path::DecInt;
use alloc::vec::Vec;
use core::mem::MaybeUninit;
use linux_raw_sys::ioctl::{TIOCGPTN, TIOCSPTLCK};

#[inline]
pub(crate) fn ptsname(fd: BorrowedFd, mut buffer: Vec<u8>) -> io::Result<CString> {
    unsafe {
        let mut n = MaybeUninit::<c::c_int>::uninit();
        ret(syscall!(__NR_ioctl, fd, c_uint(TIOCGPTN), &mut n))?;

        buffer.clear();
        buffer.extend_from_slice(b"/dev/pts/");
        buffer.extend_from_slice(DecInt::new(n.assume_init()).as_bytes());
        buffer.push(b'\0');
        Ok(CString::from_vec_with_nul_unchecked(buffer))
    }
}

#[inline]
pub(crate) fn unlockpt(fd: BorrowedFd) -> io::Result<()> {
    unsafe {
        ret(syscall_readonly!(
            __NR_ioctl,
            fd,
            c_uint(TIOCSPTLCK),
            by_ref(&0)
        ))
    }
}
