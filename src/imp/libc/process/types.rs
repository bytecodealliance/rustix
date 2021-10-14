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

/// A resource value for use with [`getrlimit`].
///
/// [`getrlimit`]: crate::process::getrlimit
#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Resource {
    /// `RLIMIT_CPU`
    Cpu = libc::RLIMIT_CPU as c_int,
    /// `RLIMIT_FSIZE`
    Fsize = libc::RLIMIT_FSIZE as c_int,
    /// `RLIMIT_DATA`
    Data = libc::RLIMIT_DATA as c_int,
    /// `RLIMIT_STACK`
    Stack = libc::RLIMIT_STACK as c_int,
    /// `RLIMIT_CORE`
    Core = libc::RLIMIT_CORE as c_int,
    /// `RLIMIT_RSS`
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    Rss = libc::RLIMIT_RSS as c_int,
    /// `RLIMIT_NPROC`
    Nproc = libc::RLIMIT_NPROC as c_int,
    /// `RLIMIT_NOFILE`
    Nofile = libc::RLIMIT_NOFILE as c_int,
    /// `RLIMIT_MEMLOCK`
    Memlock = libc::RLIMIT_MEMLOCK as c_int,
    /// `RLIMIT_AS`
    #[cfg(not(target_os = "openbsd"))]
    As = libc::RLIMIT_AS as c_int,
    /// `RLIMIT_LOCKS`
    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    Locks = libc::RLIMIT_LOCKS as c_int,
    /// `RLIMIT_SIGPENDING`
    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    Sigpending = libc::RLIMIT_SIGPENDING as c_int,
    /// `RLIMIT_MSGQUEUE`
    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    Msgqueue = libc::RLIMIT_MSGQUEUE as c_int,
    /// `RLIMIT_NICE`
    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    Nice = libc::RLIMIT_NICE as c_int,
    /// `RLIMIT_RTPRIO`
    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    Rtprio = libc::RLIMIT_RTPRIO as c_int,
    /// `RLIMIT_RTTIME`
    #[cfg(not(any(
        target_os = "emscripten",
        target_os = "freebsd",
        target_os = "android",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    Rttime = libc::RLIMIT_RTTIME as c_int,
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
impl Resource {
    /// `RLIMIT_RSS`
    #[allow(non_upper_case_globals)]
    pub const Rss: Self = Self::As;
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

#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "fuchsia",
    target_os = "dragonfly"
))]
pub type RawCpuSet = libc::cpu_set_t;
