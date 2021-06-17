//! Process-associated operations.

#[cfg(not(target_os = "wasi"))] // WASI doesn't have get[gpu]id.
mod id;
mod sched;

#[cfg(not(target_os = "wasi"))]
pub use id::{getegid, geteuid, getgid, getpid, getppid, getuid};
pub use sched::sched_yield;

/// Re-export `EXIT_SUCCESS`.
#[cfg(libc)]
pub use libc::EXIT_SUCCESS;

/// Re-export `EXIT_SUCCESS`.
#[cfg(linux_raw)]
pub const EXIT_SUCCESS: std::os::raw::c_int = 0;

/// Re-export `EXIT_FAILURE`.
#[cfg(libc)]
pub use libc::EXIT_FAILURE;

/// Re-export `EXIT_FAILURE`.
#[cfg(linux_raw)]
pub const EXIT_FAILURE: std::os::raw::c_int = 1;
