//! Process configuration APIs.
//!
//! These values correspond to `sysconf` in POSIX, and the auxv array in Linux.
//! Despite the name "sysconf", these aren't *system* configuration parameters;
//! they're *process* configuration parameters, as they may differ between
//! different processes on the same system.

#[cfg(feature = "conf")]
mod auxv;
#[cfg(any(
    target_vendor = "mustang",
    not(any(target_env = "gnu", target_env = "musl"))
))]
mod init;

#[cfg(feature = "conf")]
#[cfg(not(target_os = "wasi"))]
pub use auxv::clock_ticks_per_second;
#[cfg(feature = "conf")]
pub use auxv::page_size;
#[cfg(feature = "conf")]
#[cfg(any(
    linux_raw,
    all(
        libc,
        any(
            all(target_os = "android", target_pointer_width = "64"),
            target_os = "linux"
        )
    )
))]
pub use auxv::{linux_execfn, linux_hwcap};
#[cfg(any(
    target_vendor = "mustang",
    not(any(target_env = "gnu", target_env = "musl"))
))]
pub use init::init;
