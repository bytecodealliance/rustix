//! Tests for [`rustix::process`].

#![cfg(feature = "process")]
#![cfg(not(windows))]
#![cfg_attr(core_c_str, feature(core_c_str))]

#[cfg(not(any(
    target_os = "emscripten",
    target_os = "espidf",
    target_os = "fuchsia",
    target_os = "redox",
    target_os = "vita",
    target_os = "wasi"
)))]
#[cfg(feature = "fs")]
mod fcntl_getlk;
#[cfg(not(target_os = "wasi"))] // WASI doesn't have get[gpu]id.
mod id;
#[cfg(target_os = "linux")]
mod pidfd;
#[cfg(linux_kernel)]
mod prctl;
#[cfg(not(any(target_os = "fuchsia", target_os = "wasi")))] // WASI doesn't have [gs]etpriority.
mod priority;
#[cfg(freebsdlike)]
mod procctl;
#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
mod rlimit;
#[cfg(not(target_os = "wasi"))] // WASI doesn't have umask.
mod umask;
#[cfg(not(any(target_os = "espidf", target_os = "wasi")))] // WASI doesn't have waitpid.
mod wait;
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[cfg(feature = "fs")]
mod working_directory;
