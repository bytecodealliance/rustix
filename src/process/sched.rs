use crate::process::Pid;
use crate::{imp, io};

/// CpuSet represent a bit-mask of CPUs.
/// CpuSets are used by `sched_setaffinity` and
/// `sched_getaffinity` for example.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man3/CPU_SET.3.html
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CpuSet {
    cpu_set: imp::process::RawCpuSet,
}

impl CpuSet {
    /// Return the maximum number of CPU in CpuSet
    pub const MAX_CPU: usize = imp::process::CPU_SETSIZE;

    /// Create a new and empty CpuSet.
    #[inline]
    pub fn new() -> CpuSet {
        CpuSet {
            // This is a bit akward because idealy we would create
            // an unitilized `RawCpuSet` and call `CPU_ZERO` on it.
            // But this is impossible in Rust as all variable must
            // be initilized before use. So instead we do this in
            // one step by calling `mem::zeroed()`.
            #[allow(unsafe_code)]
            cpu_set: unsafe { core::mem::zeroed() },
        }
    }

    /// Test to see if a CPU is in the CpuSet.
    /// `field` is the CPU id to test
    #[inline]
    pub fn is_set(&self, field: usize) -> bool {
        imp::syscalls::CPU_ISSET(field, &self.cpu_set)
    }

    /// Add a CPU to CpuSet.
    /// `field` is the CPU id to add
    #[inline]
    pub fn set(&mut self, field: usize) {
        imp::syscalls::CPU_SET(field, &mut self.cpu_set)
    }

    /// Remove a CPU from CpuSet.
    /// `field` is the CPU id to remove
    #[inline]
    pub fn unset(&mut self, field: usize) {
        imp::syscalls::CPU_CLR(field, &mut self.cpu_set)
    }

    /// Count the number of CPUs set in the CpuSet
    #[cfg(target_os = "linux")]
    #[inline]
    pub fn count(&self) -> u32 {
        imp::syscalls::CPU_COUNT(&self.cpu_set)
    }

    /// Zeroies the CpuSet
    #[inline]
    pub fn clear(&mut self) {
        imp::syscalls::CPU_ZERO(&mut self.cpu_set)
    }
}

/// `sched_setaffinity` set a thread's CPU affinity mask
///
/// `pid` is the thread ID to update.
/// If pid is `Pid::NONE`, then the calling thread is updated.
///
/// The `CpuSet` argument specifies the set of CPUs on which the thread
/// will be eligible to run.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/sched_setaffinity.2.html
#[inline]
pub fn sched_setaffinity(pid: Pid, cpuset: &CpuSet) -> io::Result<()> {
    imp::syscalls::sched_setaffinity(pid, &cpuset.cpu_set)
}

/// `sched_getaffinity` get a thread's CPU affinity mask
///
/// `pid` is the thread ID to check.
/// If pid is `Pid::NONE`, then the calling thread is checked.
///
/// Returned `CpuSet` is the set of CPUs on which the thread
/// is eligible to run.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/sched_getaffinity.2.html
#[inline]
pub fn sched_getaffinity(pid: Pid) -> io::Result<CpuSet> {
    let mut cpuset = CpuSet::new();
    imp::syscalls::sched_getaffinity(pid, &mut cpuset.cpu_set).and(Ok(cpuset))
}
