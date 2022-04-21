mod auxv;
mod types;
mod wait;

pub(crate) mod cpu_set;
pub(crate) mod syscalls;

#[cfg(feature = "runtime")]
pub(crate) use auxv::exe_phdrs;
#[cfg(feature = "runtime")]
pub(super) use auxv::exe_phdrs_slice;
#[cfg(target_vendor = "mustang")]
pub(crate) use auxv::init;
#[cfg(any(feature = "time", target_arch = "x86"))]
pub(super) use auxv::sysinfo_ehdr;
pub(crate) use auxv::{clock_ticks_per_second, linux_execfn, linux_hwcap, page_size};
pub(crate) use types::{raw_cpu_set_new, RawCpuSet, RawUname, CPU_SETSIZE};
pub use types::{
    MembarrierCommand, RawCpuid, RawGid, RawNonZeroPid, RawPid, RawUid, Resource, Signal,
    EXIT_FAILURE, EXIT_SIGNALED_SIGABRT, EXIT_SUCCESS,
};
pub(crate) use wait::{
    WCONTINUED, WEXITSTATUS, WIFCONTINUED, WIFEXITED, WIFSIGNALED, WIFSTOPPED, WNOHANG, WSTOPSIG,
    WTERMSIG, WUNTRACED,
};
