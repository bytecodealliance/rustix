mod auxv;
mod types;
mod wait;

#[cfg(target_vendor = "mustang")]
pub(crate) use auxv::init;
pub(crate) use auxv::{exe_phdrs, linux_execfn, linux_hwcap, page_size};
pub(super) use auxv::{exe_phdrs_slice, sysinfo_ehdr};
pub use types::{
    MembarrierCommand, RawCpuSet, RawCpuid, RawGid, RawPid, RawUid, RawUname, Resource,
    CPU_SETSIZE, EXIT_FAILURE, EXIT_SIGNALED_SIGABRT, EXIT_SUCCESS,
};
pub use wait::{
    WCONTINUED, WEXITSTATUS, WIFCONTINUED, WIFEXITED, WIFSIGNALED, WIFSTOPPED, WNOHANG, WSTOPSIG,
    WTERMSIG, WUNTRACED,
};
