//! Implementation details for thread-local storage (TLS).
//!
//! # Safety
//!
//! This module is intended to be used for implementing `libpthread`.

#![allow(unsafe_code)]

use crate::{imp, io};
use std::ffi::CStr;

#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn set_thread_area(u_info: &mut UserDesc) -> io::Result<()> {
    imp::syscalls::tls::set_thread_area(u_info)
}

#[cfg(target_arch = "arm")]
#[inline]
pub unsafe fn arm_set_tls(data: *mut std::ffi::c_void) -> io::Result<()> {
    imp::syscalls::tls::arm_set_tls(data)
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn set_fs(data: *mut std::ffi::c_void) {
    imp::syscalls::tls::set_fs(data)
}

/// `prctl(PR_SET_NAME, name)`
///
/// # References
///  - [Linux]: https://man7.org/linux/man-pages/man2/prctl.2.html
///
/// # Safety
///
/// This is a very low-level feature for implementing threading libraries.
/// See the references links above.
///
/// [Linux]: https://man7.org/linux/man-pages/man2/prctl.2.html
#[inline]
pub unsafe fn set_thread_name(name: &CStr) -> io::Result<()> {
    imp::syscalls::tls::set_thread_name(name)
}

#[cfg(target_arch = "x86")]
pub use imp::syscalls::tls::UserDesc;
