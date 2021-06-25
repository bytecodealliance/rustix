//! Process-associated operations.

use crate::imp;
use std::os::raw::c_int;

#[cfg(not(target_os = "wasi"))] // WASI doesn't have get[gpu]id.
mod id;
mod sched;

#[cfg(not(target_os = "wasi"))]
pub use id::{getegid, geteuid, getgid, getpid, getppid, getuid};
pub use sched::sched_yield;

/// `EXIT_SUCCESS` for use with [`exit`].
///
/// [`exit`]: std::process::exit
pub const EXIT_SUCCESS: c_int = imp::process::EXIT_SUCCESS;

/// `EXIT_FAILURE` for use with [`exit`].
///
/// [`exit`]: std::process::exit
pub const EXIT_FAILURE: c_int = imp::process::EXIT_FAILURE;
