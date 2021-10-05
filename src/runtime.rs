//! Implementation details for thread-local storage (TLS).
//!
//! # Safety
//!
//! This module is intended to be used for implementing `libpthread`. Use
//! of these features for any other purpose is likely to conflict with
//! `libpthread`.

#![allow(unsafe_code)]

use crate::process::Pid;
use crate::{imp, io};
use std::ffi::{c_void, CStr};

#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn set_thread_area(u_info: &mut UserDesc) -> io::Result<()> {
    imp::syscalls::tls::set_thread_area(u_info)
}

#[cfg(target_arch = "arm")]
#[inline]
pub unsafe fn arm_set_tls(data: *mut c_void) -> io::Result<()> {
    imp::syscalls::tls::arm_set_tls(data)
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn set_fs(data: *mut c_void) {
    imp::syscalls::tls::set_fs(data)
}

#[inline]
pub unsafe fn set_tid_address(data: *mut c_void) -> Pid {
    imp::syscalls::tls::set_tid_address(data)
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
pub use imp::thread::tls::UserDesc;

/// `syscall(SYS_exit, status)`—Exit the current thread.
///
/// # Safety
///
/// This is a very low-level feature for implementing threading libraries.
#[inline]
pub unsafe fn exit_thread(status: i32) -> ! {
    imp::syscalls::tls::exit_thread(status)
}

/// Return fields from the main executable segment headers ("phdrs") relevant
/// to initializing TLS provided to the program at startup.
#[inline]
pub fn startup_tls_info() -> StartupTlsInfo {
    imp::thread::tls::startup_tls_info()
}

/// `(getauxval(AT_PHDR), getauxval(AT_PHNUM))`—Returns the address and
/// number of ELF segment headers for the main executable.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man3/getauxval.3.html
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
#[inline]
pub fn exe_phdrs() -> (*const c_void, usize) {
    imp::process::exe_phdrs()
}

pub use imp::thread::tls::StartupTlsInfo;
