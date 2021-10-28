//! Low-level implementation details for libc-like runtime libraries.
//!
//! These functions are for implementing thread-local storage (TLS),
//! managing threads, loaded libraries, and other process-wide resources.
//! Most of rsix doesn't care about what other libraries are linked into
//! the program or what they're doing, but the features in this module
//! generally can only be used by one entity within a process.
//!
//! The API for these functions is not stable, and this module is
//! `doc(hidden)`.
//!
//! # Safety
//!
//! This module is intended to be used for implementing a runtime library
//! such as libc. Use of these features for any other purpose is likely
//! to create serious problems.

#![allow(unsafe_code)]

use crate::path::Arg;
use crate::process::Pid;
use crate::{imp, io};
use std::borrow::Cow;
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

/// `fork()`—Creates a new process by duplicating the calling process.
///
/// On success, the PID of the child process is returned in the parent,
/// and `Pid::NONE` is returned in the child.
///
/// Unlike its libc counterpart,
/// this function does not call handlers registered with [`pthread_atfork`],
/// and does not initializes the `pthread` data structures in the child process.
///
/// # Safety
///
/// If the parent has multiple threads, fork creates a child process containing a copy of all the memory of all the threads,
/// but with only one actual thread, so objects in memory such as mutexes may be in unusable states.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/fork.2.html
/// [`pthread_atfork`]: https://man7.org/linux/man-pages/man3/pthread_atfork.3.html
pub unsafe fn fork() -> io::Result<Pid> {
    imp::syscalls::fork()
}

/// Executes the program pointed to by `path`, with the arguments `args`.
///
/// The first argument, by convention,
/// should be the filename associated with the file being executed.
pub fn execv<P: Arg>(path: P, args: &[P]) -> io::Result<()> {
    let arg_vec: Vec<Cow<'_, CStr>> = args
        .into_iter()
        .map(Arg::as_cow_c_str)
        .collect::<io::Result<_>>()?;
    path.into_with_c_str(|path_cstr| imp::syscalls::execv(path_cstr, &arg_vec))
}
