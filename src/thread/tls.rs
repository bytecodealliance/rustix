//! Implementation details for thread-local storage.
//!
//! # Safety
//!
//! Don't use this module unless you are implementing `libpthread` yourself.

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

#[inline]
pub unsafe fn set_thread_name(name: &CStr) -> io::Result<()> {
    imp::syscalls::tls::set_thread_name(name)
}

#[cfg(target_arch = "x86")]
pub use imp::syscalls::tls::UserDesc;
