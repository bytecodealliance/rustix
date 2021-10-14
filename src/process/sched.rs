use crate::imp;
use crate::io;
use crate::io::Error;
use crate::process::Pid;

/// CpuSet represent a bit-mask of CPUs.
/// CpuSets are used by sched_setaffinity and
/// sched_getaffinity for example.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CpuSet {
    cpu_set: imp::process::RawCpuSet,
}

impl CpuSet {
    /// Create a new and empty CpuSet.
    pub fn new() -> CpuSet {
        CpuSet {
            #[allow(unsafe_code)]
            cpu_set: unsafe { std::mem::zeroed() },
        }
    }

    /// Test to see if a CPU is in the CpuSet.
    /// `field` is the CPU id to test
    pub fn is_set(&self, field: usize) -> io::Result<bool> {
        if field >= CpuSet::MAX_CPU {
            Err(Error::INVAL)
        } else {
            Ok(imp::syscalls::CPU_ISSET(field, &self.cpu_set))
        }
    }

    /// Add a CPU to CpuSet.
    /// `field` is the CPU id to add
    pub fn set(&mut self, field: usize) -> io::Result<()> {
        if field >= CpuSet::MAX_CPU {
            Err(Error::INVAL)
        } else {
            Ok(imp::syscalls::CPU_SET(field, &mut self.cpu_set))
        }
    }

    /// Remove a CPU from CpuSet.
    /// `field` is the CPU id to remove
    pub fn unset(&mut self, field: usize) -> io::Result<()> {
        if field >= CpuSet::MAX_CPU {
            Err(Error::INVAL)
        } else {
            Ok(imp::syscalls::CPU_CLR(field, &mut self.cpu_set))
        }
    }

    /// Count the number of CPUs set in the CpuSet
    #[cfg(target_os = "linux")]
    pub fn count(&self) -> u32 {
        imp::syscalls::CPU_COUNT(&self.cpu_set)
    }

    /// Zeroies the CpuSet
    pub fn clear(&mut self) {
        imp::syscalls::CPU_ZERO(&mut self.cpu_set)
    }

    /// Return the maximum number of CPU in CpuSet
    pub const MAX_CPU: usize = { 8 * std::mem::size_of::<imp::process::RawCpuSet>() };
}

/// `sched_setaffinity` set a thread's CPU affinity mask
/// ([`sched_setaffinity(2)`](https://man7.org/linux/man-pages/man2/sched_setaffinity.2.html))
///
/// `pid` is the thread ID to update.
/// If pid is zero, then the calling thread is updated.
///
/// The `cpuset` argument specifies the set of CPUs on which the thread
/// will be eligible to run.
#[inline]
pub fn sched_setaffinity(pid: Pid, cpuset: &CpuSet) -> io::Result<()> {
    imp::syscalls::sched_setaffinity(pid, &cpuset.cpu_set)
}

/// `sched_getaffinity` get a thread's CPU affinity mask
/// ([`sched_getaffinity(2)`](https://man7.org/linux/man-pages/man2/sched_getaffinity.2.html))
///
/// `pid` is the thread ID to check.
/// If pid is zero, then the calling thread is checked.
///
/// Returned `cpuset` is the set of CPUs on which the thread
/// is eligible to run.
#[inline]
pub fn sched_getaffinity(pid: Pid) -> io::Result<CpuSet> {
    let mut cpuset = CpuSet::new();
    imp::syscalls::sched_getaffinity(pid, &mut cpuset.cpu_set).and(Ok(cpuset))
}
