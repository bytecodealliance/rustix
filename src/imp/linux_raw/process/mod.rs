mod auxv;
mod types;

#[cfg(target_vendor = "mustang")]
pub(crate) use auxv::init;
pub(super) use auxv::{exe_phdrs, sysinfo_ehdr};
pub(crate) use auxv::{linux_execfn, linux_hwcap, page_size};
pub use types::{
    MembarrierCommand, RawCpuid, RawGid, RawPid, RawUid, RawUname, Resource, EXIT_FAILURE,
    EXIT_SIGNALED_SIGABRT, EXIT_SUCCESS,
};
