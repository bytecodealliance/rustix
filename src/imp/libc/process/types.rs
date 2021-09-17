use libc::c_int;

/// A command for use with [`membarrier`] and [`membarrier_cpu`].
///
/// For `MEMBARRIER_CMD_QUERY`, see [`membarrier_query`].
///
/// [`membarrier`]: crate::process::membarrier
/// [`membarrier_cpu`]: crate::process::membarrier_cpu
/// [`membarrier_query`]: crate::process::membarrier_query
// TODO: These are not yet exposed through libc, so we define the
// constants ourselves.
#[cfg(any(target_os = "android", target_os = "linux"))]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u32)]
pub enum MembarrierCommand {
    /// `MEMBARRIER_CMD_GLOBAL`
    #[doc(alias = "Shared")]
    #[doc(alias = "MEMBARRIER_CMD_SHARED")]
    Global = 1,
    /// `MEMBARRIER_CMD_GLOBAL_EXPEDITED`
    GlobalExpedited = 2,
    /// `MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED`
    RegisterGlobalExpedited = 4,
    /// `MEMBARRIER_CMD_PRIVATE_EXPEDITED`
    PrivateExpedited = 8,
    /// `MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED`
    RegisterPrivateExpedited = 16,
    /// `MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE`
    PrivateExpeditedSyncCore = 32,
    /// `MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE`
    RegisterPrivateExpeditedSyncCore = 64,
    /// `MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ` (since Linux 5.10)
    PrivateExpeditedRseq = 128,
    /// `MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ` (since Linux 5.10)
    RegisterPrivateExpeditedRseq = 256,
}

pub const EXIT_SUCCESS: c_int = libc::EXIT_SUCCESS;
pub const EXIT_FAILURE: c_int = libc::EXIT_FAILURE;
#[cfg(not(target_os = "wasi"))]
pub const EXIT_SIGNALED_SIGABRT: c_int = 128 + libc::SIGABRT;

#[cfg(not(target_os = "wasi"))]
pub type RawPid = libc::pid_t;
#[cfg(not(target_os = "wasi"))]
pub type RawGid = libc::gid_t;
#[cfg(not(target_os = "wasi"))]
pub type RawUid = libc::uid_t;
#[cfg(any(target_os = "android", target_os = "linux"))]
pub type RawCpuid = u32;

#[cfg(not(target_os = "wasi"))]
pub type RawUname = libc::utsname;
