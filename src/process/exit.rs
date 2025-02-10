use crate::backend;

/// `EXIT_SUCCESS` for use with [`exit`] or [`std::process::exit`].
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/stdlib.h.html
/// [Linux]: https://man7.org/linux/man-pages/man3/exit.3.html
pub const EXIT_SUCCESS: i32 = backend::c::EXIT_SUCCESS;

/// `EXIT_FAILURE` for use with [`exit`] or [`std::process::exit`].
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/stdlib.h.html
/// [Linux]: https://man7.org/linux/man-pages/man3/exit.3.html
pub const EXIT_FAILURE: i32 = backend::c::EXIT_FAILURE;

/// The exit status used by a process terminated with a [`Signal::ABORT`]
/// signal.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://tldp.org/LDP/abs/html/exitcodes.html
/// [`Signal::ABORT`]: crate::process::Signal::ABORT
#[cfg(not(any(target_os = "espidf", target_os = "wasi")))]
pub const EXIT_SIGNALED_SIGABRT: i32 = backend::c::EXIT_SIGNALED_SIGABRT;

/// Immediately exits the process. Exiting via this function does not unwind the
/// stack and does not call any further user code. This behavior is similar to
/// the POSIX/C `_Exit` and `_exit` functions.
///
/// Notably, this function does:
///  - *Not* flush any buffers, such as Rust or C standard output or files.
///  - *Not* call any destructors, neither in the form of stack unwinding, nor
///    any global destructors.
///  - *Not* call functions registered with [`atexit`] or [`at_quick_exit`]
///
/// In general, most code should call [`std::process::exit`] instead, if it is
/// available.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/stdlib.h.html
/// [Linux]: https://www.man7.org/linux/man-pages/man2/exit.2.html
/// [`atexit`]: https://www.man7.org/linux/man-pages/man3/atexit.3.html
/// [`at_quick_exit`]: https://en.cppreference.com/w/c/program/at_quick_exit
#[doc(alias = "_exit")]
#[inline]
pub fn immediate_exit(status: i32) -> ! {
    backend::process::syscalls::_exit(status);
}
