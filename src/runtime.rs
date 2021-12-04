//! Low-level implementation details for libc-like runtime libraries.
//!
//! These functions are for implementing thread-local storage (TLS),
//! managing threads, loaded libraries, and other process-wide resources.
//! Most of `rustix` doesn't care about what other libraries are linked into
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

use crate::ffi::ZStr;
#[cfg(linux_raw)]
use crate::process::Pid;
use crate::{imp, io, path};
use alloc::borrow::Cow;
use alloc::vec::Vec;
#[cfg(linux_raw)]
use core::ffi::c_void;

#[cfg(linux_raw)]
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn set_thread_area(u_info: &mut UserDesc) -> io::Result<()> {
    imp::syscalls::tls::set_thread_area(u_info)
}

#[cfg(linux_raw)]
#[cfg(target_arch = "arm")]
#[inline]
pub unsafe fn arm_set_tls(data: *mut c_void) -> io::Result<()> {
    imp::syscalls::tls::arm_set_tls(data)
}

#[cfg(linux_raw)]
#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn set_fs(data: *mut c_void) {
    imp::syscalls::tls::set_fs(data)
}

#[cfg(linux_raw)]
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
#[cfg(linux_raw)]
#[inline]
pub unsafe fn set_thread_name(name: &ZStr) -> io::Result<()> {
    imp::syscalls::tls::set_thread_name(name)
}

#[cfg(linux_raw)]
#[cfg(target_arch = "x86")]
pub use imp::thread::tls::UserDesc;

/// `syscall(SYS_exit, status)`—Exit the current thread.
///
/// # Safety
///
/// This is a very low-level feature for implementing threading libraries.
#[cfg(linux_raw)]
#[inline]
pub unsafe fn exit_thread(status: i32) -> ! {
    imp::syscalls::tls::exit_thread(status)
}

/// Exit all the threads in the current process' thread group.
///
/// This is equivalent to `_exit` and `_Exit` in libc.
///
/// Note that this does not all any `__cxa_atexit`, `atexit`, or any other
/// destructors. Most programs should use [`std::process::exit`] instead
/// of calling this directly.
///
/// # References
///  - [POSIX `_Exit`]
///  - [Linux `exit_group`]
///  - [Linux `_Exit`]
///
/// [POSIX `_Exit`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/_Exit.html
/// [Linux `exit_group`]: https://man7.org/linux/man-pages/man2/exit_group.2.html
/// [Linux `_Exit`]: https://man7.org/linux/man-pages/man2/exit.2.html
#[doc(alias = "_exit")]
#[doc(alias = "_Exit")]
#[inline]
pub fn exit_group(status: i32) -> ! {
    imp::syscalls::exit_group(status)
}

/// Return fields from the main executable segment headers ("phdrs") relevant
/// to initializing TLS provided to the program at startup.
#[cfg(linux_raw)]
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
#[cfg(linux_raw)]
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
#[inline]
pub fn exe_phdrs() -> (*const c_void, usize) {
    imp::process::exe_phdrs()
}

#[cfg(linux_raw)]
pub use imp::thread::tls::StartupTlsInfo;

/// `fork()`—Creates a new process by duplicating the calling process.
///
/// On success, the PID of the child process is returned in the parent, and
/// `None` is returned in the child.
///
/// If the parent has multiple threads, fork creates a child process containing
/// a copy of all the memory of all the threads, but with only one actual
/// thread. Mutexes held on threads other than the one that called `fork` in
/// the parent will appear in the child as if they are locked indefinitely,
/// and attempting to lock them may deadlock.
///
/// Unlike its libc counterpart, this function does not call handlers
/// registered with [`pthread_atfork`].
///
/// # Safety
///
/// This function does not update the threading runtime's data structures in
/// the child process, so higher-level APIs such as `pthread_self` may return
/// stale values in the child.
///
/// And because it doesn't call handlers registered with `pthread_atfork`,
/// random number generators such as those in the [rand] crate aren't
/// reinitialized in the child, so may generate the same values in the child
/// as in the parent.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/fork.2.html
/// [`pthread_atfork`]: https://man7.org/linux/man-pages/man3/pthread_atfork.3.html
/// [rand]: https://crates.io/crates/rand
#[cfg(linux_raw)]
pub unsafe fn fork() -> io::Result<Option<Pid>> {
    imp::syscalls::fork()
}

/// Executes the program pointed to by `path`, with the arguments `args`, and
/// the environment variables `env_vars`.
///
/// The first argument, by convention, should be the filename associated with
/// the file being executed.
#[cfg(not(target_os = "wasi"))]
pub fn execve<P: path::Arg, A: path::Arg, E: path::Arg>(
    path: P,
    args: &[A],
    env_vars: &[E],
) -> io::Result<()> {
    let arg_zstr: Vec<Cow<'_, ZStr>> = args
        .iter()
        .map(path::Arg::as_cow_z_str)
        .collect::<io::Result<_>>()?;
    let env_zstr: Vec<Cow<'_, ZStr>> = env_vars
        .iter()
        .map(path::Arg::as_cow_z_str)
        .collect::<io::Result<_>>()?;
    path.into_with_z_str(|path_zstr| _execve(path_zstr, &arg_zstr, &env_zstr))
}

#[cfg(not(target_os = "wasi"))]
fn _execve(path: &ZStr, arg_zstr: &[Cow<'_, ZStr>], env_zstr: &[Cow<'_, ZStr>]) -> io::Result<()> {
    let arg_ptrs: Vec<_> = arg_zstr
        .iter()
        .map(|zstr| ZStr::as_ptr(zstr).cast::<_>())
        .chain(core::iter::once(core::ptr::null()))
        .collect();
    let env_ptrs: Vec<_> = env_zstr
        .iter()
        .map(|zstr| ZStr::as_ptr(zstr).cast::<_>())
        .chain(core::iter::once(core::ptr::null()))
        .collect();
    unsafe { imp::syscalls::execve(path, &arg_ptrs, &env_ptrs) }
}
