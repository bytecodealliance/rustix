use crate::backend::c;

/// A command for use with [`membarrier`] and [`membarrier_cpu`].
///
/// For `MEMBARRIER_CMD_QUERY`, see [`membarrier_query`].
///
/// [`membarrier`]: crate::process::membarrier
/// [`membarrier_cpu`]: crate::process::membarrier_cpu
/// [`membarrier_query`]: crate::process::membarrier_query
#[cfg(linux_kernel)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u32)]
pub enum MembarrierCommand {
    /// `MEMBARRIER_CMD_GLOBAL`
    #[doc(alias = "Shared")]
    #[doc(alias = "MEMBARRIER_CMD_SHARED")]
    Global = c::MEMBARRIER_CMD_GLOBAL as u32,
    /// `MEMBARRIER_CMD_GLOBAL_EXPEDITED`
    GlobalExpedited = c::MEMBARRIER_CMD_GLOBAL_EXPEDITED as u32,
    /// `MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED`
    RegisterGlobalExpedited = c::MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED as u32,
    /// `MEMBARRIER_CMD_PRIVATE_EXPEDITED`
    PrivateExpedited = c::MEMBARRIER_CMD_PRIVATE_EXPEDITED as u32,
    /// `MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED`
    RegisterPrivateExpedited = c::MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED as u32,
    /// `MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE`
    PrivateExpeditedSyncCore = c::MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE as u32,
    /// `MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE`
    RegisterPrivateExpeditedSyncCore =
        c::MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE as u32,
    /// `MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ` (since Linux 5.10)
    PrivateExpeditedRseq = c::MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ as u32,
    /// `MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ` (since Linux 5.10)
    RegisterPrivateExpeditedRseq = c::MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ as u32,
}

/// A resource value for use with [`getrlimit`], [`setrlimit`], and
/// [`prlimit`].
///
/// [`getrlimit`]: crate::process::getrlimit
/// [`setrlimit`]: crate::process::setrlimit
/// [`prlimit`]: crate::process::prlimit
#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Resource {
    /// `RLIMIT_CPU`
    Cpu = c::RLIMIT_CPU as c::c_int,
    /// `RLIMIT_FSIZE`
    Fsize = c::RLIMIT_FSIZE as c::c_int,
    /// `RLIMIT_DATA`
    Data = c::RLIMIT_DATA as c::c_int,
    /// `RLIMIT_STACK`
    Stack = c::RLIMIT_STACK as c::c_int,
    /// `RLIMIT_CORE`
    #[cfg(not(target_os = "haiku"))]
    Core = c::RLIMIT_CORE as c::c_int,
    /// `RLIMIT_RSS`
    #[cfg(not(any(apple, solarish, target_os = "haiku")))]
    Rss = c::RLIMIT_RSS as c::c_int,
    /// `RLIMIT_NPROC`
    #[cfg(not(any(solarish, target_os = "haiku")))]
    Nproc = c::RLIMIT_NPROC as c::c_int,
    /// `RLIMIT_NOFILE`
    Nofile = c::RLIMIT_NOFILE as c::c_int,
    /// `RLIMIT_MEMLOCK`
    #[cfg(not(any(solarish, target_os = "aix", target_os = "haiku")))]
    Memlock = c::RLIMIT_MEMLOCK as c::c_int,
    /// `RLIMIT_AS`
    #[cfg(not(target_os = "openbsd"))]
    As = c::RLIMIT_AS as c::c_int,
    /// `RLIMIT_LOCKS`
    #[cfg(not(any(bsd, solarish, target_os = "aix", target_os = "haiku")))]
    Locks = c::RLIMIT_LOCKS as c::c_int,
    /// `RLIMIT_SIGPENDING`
    #[cfg(not(any(bsd, solarish, target_os = "aix", target_os = "haiku")))]
    Sigpending = c::RLIMIT_SIGPENDING as c::c_int,
    /// `RLIMIT_MSGQUEUE`
    #[cfg(not(any(bsd, solarish, target_os = "aix", target_os = "haiku")))]
    Msgqueue = c::RLIMIT_MSGQUEUE as c::c_int,
    /// `RLIMIT_NICE`
    #[cfg(not(any(bsd, solarish, target_os = "aix", target_os = "haiku")))]
    Nice = c::RLIMIT_NICE as c::c_int,
    /// `RLIMIT_RTPRIO`
    #[cfg(not(any(bsd, solarish, target_os = "aix", target_os = "haiku")))]
    Rtprio = c::RLIMIT_RTPRIO as c::c_int,
    /// `RLIMIT_RTTIME`
    #[cfg(not(any(
        bsd,
        solarish,
        target_os = "aix",
        target_os = "android",
        target_os = "emscripten",
        target_os = "haiku",
    )))]
    Rttime = c::RLIMIT_RTTIME as c::c_int,
}

#[cfg(apple)]
#[allow(non_upper_case_globals)]
impl Resource {
    /// `RLIMIT_RSS`
    pub const Rss: Self = Self::As;
}

pub const EXIT_SUCCESS: c::c_int = c::EXIT_SUCCESS;
pub const EXIT_FAILURE: c::c_int = c::EXIT_FAILURE;
#[cfg(not(target_os = "wasi"))]
pub const EXIT_SIGNALED_SIGABRT: c::c_int = 128 + c::SIGABRT;

/// A CPU identifier as a raw integer.
#[cfg(linux_kernel)]
pub type RawCpuid = u32;
#[cfg(target_os = "freebsd")]
pub type RawId = c::id_t;

#[cfg(any(linux_kernel, target_os = "dragonfly", target_os = "fuchsia"))]
pub(crate) type RawCpuSet = c::cpu_set_t;

#[cfg(any(linux_kernel, target_os = "dragonfly", target_os = "fuchsia"))]
#[inline]
pub(crate) fn raw_cpu_set_new() -> RawCpuSet {
    let mut set = unsafe { core::mem::zeroed() };
    super::cpu_set::CPU_ZERO(&mut set);
    set
}

#[cfg(any(linux_kernel, target_os = "fuchsia"))]
pub(crate) const CPU_SETSIZE: usize = c::CPU_SETSIZE as usize;
#[cfg(target_os = "dragonfly")]
pub(crate) const CPU_SETSIZE: usize = 256;
