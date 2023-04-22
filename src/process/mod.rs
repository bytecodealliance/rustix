//! Process-associated operations.

#[cfg(not(target_os = "wasi"))]
mod chdir;
#[cfg(not(any(target_os = "fuchsia", target_os = "wasi")))]
mod chroot;
mod exit;
#[cfg(not(target_os = "wasi"))] // WASI doesn't have get[gpu]id.
mod id;
#[cfg(not(target_os = "wasi"))]
mod kill;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod membarrier;
#[cfg(target_os = "linux")]
mod pidfd;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod prctl;
#[cfg(not(any(target_os = "fuchsia", target_os = "wasi")))] // WASI doesn't have [gs]etpriority.
mod priority;
#[cfg(target_os = "freebsd")]
mod procctl;
#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
mod rlimit;
#[cfg(any(
    target_os = "android",
    target_os = "dragonfly",
    target_os = "fuchsia",
    target_os = "linux",
))]
mod sched;
mod sched_yield;
#[cfg(not(target_os = "wasi"))] // WASI doesn't have uname.
mod system;
#[cfg(not(target_os = "wasi"))] // WASI doesn't have umask.
mod umask;
#[cfg(not(target_os = "wasi"))]
mod wait;

#[cfg(not(target_os = "wasi"))]
pub use chdir::*;
#[cfg(not(any(target_os = "fuchsia", target_os = "wasi")))]
pub use chroot::*;
pub use exit::*;
#[cfg(not(target_os = "wasi"))]
pub use id::*;
#[cfg(not(target_os = "wasi"))]
pub use kill::*;
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use membarrier::*;
#[cfg(target_os = "linux")]
pub use pidfd::*;
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use prctl::*;
#[cfg(not(any(target_os = "fuchsia", target_os = "wasi")))]
pub use priority::*;
#[cfg(target_os = "freebsd")]
pub use procctl::*;
#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
pub use rlimit::*;
#[cfg(any(
    target_os = "android",
    target_os = "dragonfly",
    target_os = "fuchsia",
    target_os = "linux",
))]
pub use sched::*;
pub use sched_yield::sched_yield;
#[cfg(not(target_os = "wasi"))]
pub use system::*;
#[cfg(not(target_os = "wasi"))]
pub use umask::*;
#[cfg(not(target_os = "wasi"))]
pub use wait::*;

#[cfg(not(target_os = "wasi"))]
#[cfg(feature = "fs")]
pub(crate) use id::translate_fchown_args;
