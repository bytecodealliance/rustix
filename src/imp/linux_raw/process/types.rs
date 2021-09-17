use std::os::raw::c_int;

/// A command for use with [`membarrier`] and [`membarrier_cpu`].
///
/// For `MEMBARRIER_CMD_QUERY`, see [`membarrier_query`].
///
/// [`membarrier`]: crate::process::membarrier
/// [`membarrier_cpu`]: crate::process::membarrier_cpu
/// [`membarrier_query`]: crate::process::membarrier_query
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u32)]
pub enum MembarrierCommand {
    /// `MEMBARRIER_CMD_GLOBAL`
    #[doc(alias = "Shared")]
    #[doc(alias = "MEMBARRIER_CMD_SHARED")]
    Global = linux_raw_sys::v5_4::general::membarrier_cmd::MEMBARRIER_CMD_GLOBAL as _,
    /// `MEMBARRIER_CMD_GLOBAL_EXPEDITED`
    GlobalExpedited = linux_raw_sys::v5_4::general::membarrier_cmd::MEMBARRIER_CMD_GLOBAL_EXPEDITED as _,
    /// `MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED`
    RegisterGlobalExpedited = linux_raw_sys::v5_4::general::membarrier_cmd::MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED as _,
    /// `MEMBARRIER_CMD_PRIVATE_EXPEDITED`
    PrivateExpedited = linux_raw_sys::v5_4::general::membarrier_cmd::MEMBARRIER_CMD_PRIVATE_EXPEDITED as _,
    /// `MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED`
    RegisterPrivateExpedited = linux_raw_sys::v5_4::general::membarrier_cmd::MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED as _,
    /// `MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE`
    PrivateExpeditedSyncCore = linux_raw_sys::v5_4::general::membarrier_cmd::MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE as _,
    /// `MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE`
    RegisterPrivateExpeditedSyncCore =
        linux_raw_sys::v5_4::general::membarrier_cmd::MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE as _,
    /// `MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ` (since Linux 5.10)
    PrivateExpeditedRseq = linux_raw_sys::v5_11::general::membarrier_cmd::MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ as _,
    /// `MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ` (since Linux 5.10)
    RegisterPrivateExpeditedRseq =
        linux_raw_sys::v5_11::general::membarrier_cmd::MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ as _,
}

pub const EXIT_SUCCESS: c_int = 0;
pub const EXIT_FAILURE: c_int = 1;
pub const EXIT_SIGNALED_SIGABRT: c_int = 128 + linux_raw_sys::general::SIGABRT as i32;

pub type RawPid = u32;
pub type RawGid = u32;
pub type RawUid = u32;
pub type RawCpuid = u32;

pub type RawUname = linux_raw_sys::general::new_utsname;
