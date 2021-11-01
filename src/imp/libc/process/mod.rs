mod auxv;
mod types;

#[cfg(not(windows))]
pub(crate) mod syscalls;
pub(crate) use auxv::page_size;
#[cfg(any(target_os = "android", target_os = "linux"))]
pub(crate) use auxv::{linux_execfn, linux_hwcap};
#[cfg(not(target_os = "wasi"))]
pub use libc::{
    WCONTINUED, WEXITSTATUS, WIFCONTINUED, WIFEXITED, WIFSIGNALED, WIFSTOPPED, WNOHANG, WSTOPSIG,
    WTERMSIG, WUNTRACED,
};
#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
pub use types::Resource;
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use types::{MembarrierCommand, RawCpuid};
#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "fuchsia",
    target_os = "dragonfly"
))]
pub use types::{RawCpuSet, CPU_SETSIZE};
#[cfg(not(target_os = "wasi"))]
pub use types::{RawGid, RawPid, RawUid, RawUname, EXIT_SIGNALED_SIGABRT};
pub use types::{EXIT_FAILURE, EXIT_SUCCESS};
