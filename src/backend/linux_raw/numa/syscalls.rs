//! linux_raw syscalls supporting `rustix::numa`.
//!
//! # Safety
//!
//! See the `rustix::backend` module documentation for details.

#![allow(unsafe_code)]
#![allow(clippy::undocumented_unsafe_blocks)]

use super::types::{Mode, ModeFlags};

use crate::backend::c;
use crate::backend::conv::{c_uint, pass_usize, ret, zero};
use crate::io;
use core::mem::MaybeUninit;

/// # Safety
///
/// `mbind` is primarily unsafe due to the `addr` parameter, as anything
/// working with memory pointed to by raw pointers is unsafe.
#[inline]
pub(crate) unsafe fn mbind(
    addr: *mut c::c_void,
    length: usize,
    mode: Mode,
    nodemask: &[u64],
    flags: ModeFlags,
) -> io::Result<()> {
    ret(syscall!(
        __NR_mbind,
        addr,
        pass_usize(length),
        mode,
        nodemask.as_ptr(),
        pass_usize(nodemask.len() * u64::BITS as usize),
        flags
    ))
}

/// # Safety
///
/// `set_mempolicy` is primarily unsafe due to the `addr` parameter,
/// as anything working with memory pointed to by raw pointers is
/// unsafe.
#[inline]
pub(crate) unsafe fn set_mempolicy(mode: Mode, nodemask: &[u64]) -> io::Result<()> {
    ret(syscall!(
        __NR_set_mempolicy,
        mode,
        nodemask.as_ptr(),
        pass_usize(nodemask.len() * u64::BITS as usize)
    ))
}

/// # Safety
///
/// `get_mempolicy` is primarily unsafe due to the `addr` parameter,
/// as anything working with memory pointed to by raw pointers is
/// unsafe.
#[inline]
pub(crate) unsafe fn get_mempolicy_node(addr: *mut c::c_void) -> io::Result<usize> {
    let mut mode = MaybeUninit::<usize>::uninit();

    ret(syscall!(
        __NR_get_mempolicy,
        &mut mode,
        zero(),
        zero(),
        addr,
        c_uint(linux_raw_sys::mempolicy::MPOL_F_NODE | linux_raw_sys::mempolicy::MPOL_F_ADDR)
    ))?;

    Ok(mode.assume_init())
}

#[inline]
pub(crate) fn get_mempolicy_next_node() -> io::Result<usize> {
    let mut mode = MaybeUninit::<usize>::uninit();

    unsafe {
        ret(syscall!(
            __NR_get_mempolicy,
            &mut mode,
            zero(),
            zero(),
            zero(),
            c_uint(linux_raw_sys::mempolicy::MPOL_F_NODE)
        ))?;

        Ok(mode.assume_init())
    }
}
