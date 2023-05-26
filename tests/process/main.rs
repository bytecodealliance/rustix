//! Tests for [`rustix::process`].

#![cfg(feature = "process")]
#![cfg(not(windows))]
#![cfg_attr(core_c_str, feature(core_c_str))]

mod cpu_set;
#[cfg(not(target_os = "wasi"))] // WASI doesn't have get[gpu]id.
mod id;
#[cfg(linux_kernel)]
mod membarrier;
#[cfg(target_os = "linux")]
mod pidfd;
#[cfg(linux_kernel)]
mod prctl;
#[cfg(not(any(target_os = "fuchsia", target_os = "wasi")))] // WASI doesn't have [gs]etpriority.
mod priority;
#[cfg(target_os = "freebsd")]
mod procctl;
#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
mod rlimit;
mod sched_yield;
#[cfg(not(target_os = "wasi"))] // WASI doesn't have umask.
mod umask;
#[cfg(not(target_os = "wasi"))] // WASI doesn't have waitpid.
mod wait;
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[cfg(feature = "fs")]
mod working_directory;
