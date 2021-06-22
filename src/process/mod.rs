//! Process-associated operations.

#[cfg(not(target_os = "wasi"))] // WASI doesn't have get[gpu]id.
mod id;
mod sched;

#[cfg(not(target_os = "wasi"))]
pub use id::{getegid, geteuid, getgid, getpid, getppid, getuid};
pub use sched::sched_yield;

/// `EXIT_SUCCESS` for use with [`exit`].
///
/// [`exit`]: std::process::exit
#[cfg(libc)]
pub use libc::EXIT_SUCCESS;

/// `EXIT_SUCCESS` for use with [`exit`].
///
/// [`exit`]: std::process::exit
#[cfg(linux_raw)]
pub const EXIT_SUCCESS: std::os::raw::c_int = 0;

/// `EXIT_FAILURE` for use with [`exit`].
///
/// [`exit`]: std::process::exit
#[cfg(libc)]
pub use libc::EXIT_FAILURE;

/// `EXIT_FAILURE` for use with [`exit`].
///
/// [`exit`]: std::process::exit
#[cfg(linux_raw)]
pub const EXIT_FAILURE: std::os::raw::c_int = 1;
