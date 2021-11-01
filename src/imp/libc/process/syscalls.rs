#[cfg(not(any(target_os = "wasi", target_os = "fuchsia")))]
use super::super::conv::borrowed_fd;
#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
use super::super::conv::ret_infallible;
use super::super::conv::{c_str, ret, ret_c_int, ret_discarded_char_ptr};
#[cfg(any(target_os = "android", target_os = "linux"))]
use super::super::conv::{syscall_ret, syscall_ret_u32};
#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
use super::super::offset::libc_getrlimit;
#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
use super::super::offset::{libc_rlimit, LIBC_RLIM_INFINITY};
#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
use super::Resource;
#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "fuchsia",
    target_os = "dragonfly"
))]
use super::{RawCpuSet, CPU_SETSIZE};
#[cfg(not(target_os = "wasi"))]
use super::{RawPid, RawUname};
use crate::io;
#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
use crate::process::Rlimit;
#[cfg(any(target_os = "android", target_os = "linux"))]
use crate::process::{Cpuid, MembarrierCommand, MembarrierQuery};
#[cfg(not(target_os = "wasi"))]
use crate::process::{Gid, Pid, Uid, WaitOptions, WaitStatus};
#[cfg(not(any(target_os = "wasi", target_os = "fuchsia")))]
use io_lifetimes::BorrowedFd;
use libc::c_int;
#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
use std::convert::TryInto;
use std::ffi::CStr;
use std::mem::MaybeUninit;

#[cfg(not(target_os = "wasi"))]
pub(crate) fn chdir(path: &CStr) -> io::Result<()> {
    unsafe { ret(libc::chdir(c_str(path))) }
}

#[cfg(not(any(target_os = "wasi", target_os = "fuchsia")))]
pub(crate) fn fchdir(dirfd: BorrowedFd<'_>) -> io::Result<()> {
    unsafe { ret(libc::fchdir(borrowed_fd(dirfd))) }
}

#[cfg(not(target_os = "wasi"))]
pub(crate) fn getcwd(buf: &mut [u8]) -> io::Result<()> {
    unsafe { ret_discarded_char_ptr(libc::getcwd(buf.as_mut_ptr().cast::<_>(), buf.len())) }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
pub(crate) fn membarrier_query() -> MembarrierQuery {
    const MEMBARRIER_CMD_QUERY: u32 = 0;
    unsafe {
        match syscall_ret_u32(libc::syscall(libc::SYS_membarrier, MEMBARRIER_CMD_QUERY, 0)) {
            Ok(query) => MembarrierQuery::from_bits_unchecked(query),
            Err(_) => MembarrierQuery::empty(),
        }
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
pub(crate) fn membarrier(cmd: MembarrierCommand) -> io::Result<()> {
    unsafe { syscall_ret(libc::syscall(libc::SYS_membarrier, cmd as u32, 0)) }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
pub(crate) fn membarrier_cpu(cmd: MembarrierCommand, cpu: Cpuid) -> io::Result<()> {
    const MEMBARRIER_CMD_FLAG_CPU: u32 = 1;
    unsafe {
        syscall_ret(libc::syscall(
            libc::SYS_membarrier,
            cmd as u32,
            MEMBARRIER_CMD_FLAG_CPU,
            cpu.as_raw(),
        ))
    }
}

#[cfg(not(target_os = "wasi"))]
#[inline]
#[must_use]
pub(crate) fn getuid() -> Uid {
    unsafe {
        let uid = libc::getuid();
        Uid::from_raw(uid)
    }
}

#[cfg(not(target_os = "wasi"))]
#[inline]
#[must_use]
pub(crate) fn geteuid() -> Uid {
    unsafe {
        let uid = libc::geteuid();
        Uid::from_raw(uid)
    }
}

#[cfg(not(target_os = "wasi"))]
#[inline]
#[must_use]
pub(crate) fn getgid() -> Gid {
    unsafe {
        let gid = libc::getgid();
        Gid::from_raw(gid)
    }
}

#[cfg(not(target_os = "wasi"))]
#[inline]
#[must_use]
pub(crate) fn getegid() -> Gid {
    unsafe {
        let gid = libc::getegid();
        Gid::from_raw(gid)
    }
}

#[cfg(not(target_os = "wasi"))]
#[inline]
#[must_use]
pub(crate) fn getpid() -> Pid {
    unsafe {
        let pid = libc::getpid();
        Pid::from_raw(pid)
    }
}

#[cfg(not(target_os = "wasi"))]
#[inline]
#[must_use]
pub(crate) fn getppid() -> Pid {
    unsafe {
        let pid: i32 = libc::getppid();
        Pid::from_raw(pid)
    }
}

#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "fuchsia",
    target_os = "dragonfly"
))]
#[allow(non_snake_case)]
#[inline]
pub(crate) fn CPU_SET(cpu: usize, cpuset: &mut RawCpuSet) {
    if cpu >= CPU_SETSIZE {
        panic!(
            "cpu out of bounds: the cpu max is {} but the cpu is {}",
            CPU_SETSIZE, cpu
        )
    }
    unsafe { libc::CPU_SET(cpu, cpuset) }
}

#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "fuchsia",
    target_os = "dragonfly"
))]
#[allow(non_snake_case)]
#[inline]
pub(crate) fn CPU_ZERO(cpuset: &mut RawCpuSet) {
    unsafe { libc::CPU_ZERO(cpuset) }
}

#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "fuchsia",
    target_os = "dragonfly"
))]
#[allow(non_snake_case)]
#[inline]
pub(crate) fn CPU_CLR(cpu: usize, cpuset: &mut RawCpuSet) {
    if cpu >= CPU_SETSIZE {
        panic!(
            "cpu out of bounds: the cpu max is {} but the cpu is {}",
            CPU_SETSIZE, cpu
        )
    }
    unsafe { libc::CPU_CLR(cpu, cpuset) }
}

#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "fuchsia",
    target_os = "dragonfly"
))]
#[allow(non_snake_case)]
#[inline]
pub(crate) fn CPU_ISSET(cpu: usize, cpuset: &RawCpuSet) -> bool {
    if cpu >= CPU_SETSIZE {
        panic!(
            "cpu out of bounds: the cpu max is {} but the cpu is {}",
            CPU_SETSIZE, cpu
        )
    }
    unsafe { libc::CPU_ISSET(cpu, cpuset) }
}

#[cfg(any(target_os = "linux"))]
#[allow(non_snake_case)]
#[inline]
pub fn CPU_COUNT(cpuset: &RawCpuSet) -> u32 {
    unsafe { libc::CPU_COUNT(cpuset).try_into().unwrap() }
}

#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "fuchsia",
    target_os = "dragonfly"
))]
#[inline]
pub(crate) fn sched_getaffinity(pid: Pid, cpuset: &mut RawCpuSet) -> io::Result<()> {
    unsafe {
        ret(libc::sched_getaffinity(
            pid.as_raw() as _,
            std::mem::size_of::<RawCpuSet>(),
            cpuset,
        ))
    }
}

#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "fuchsia",
    target_os = "dragonfly"
))]
#[inline]
pub(crate) fn sched_setaffinity(pid: Pid, cpuset: &RawCpuSet) -> io::Result<()> {
    unsafe {
        ret(libc::sched_setaffinity(
            pid.as_raw() as _,
            std::mem::size_of::<RawCpuSet>(),
            cpuset,
        ))
    }
}

#[inline]
pub(crate) fn sched_yield() {
    unsafe {
        let _ = libc::sched_yield();
    }
}

#[cfg(not(target_os = "wasi"))]
#[inline]
pub(crate) fn uname() -> RawUname {
    let mut uname = MaybeUninit::<RawUname>::uninit();
    unsafe {
        ret(libc::uname(uname.as_mut_ptr())).unwrap();
        uname.assume_init()
    }
}

#[cfg(not(any(target_os = "fuchsia", target_os = "wasi")))]
#[inline]
pub(crate) fn nice(inc: i32) -> io::Result<i32> {
    errno::set_errno(errno::Errno(0));
    let r = unsafe { libc::nice(inc) };
    if errno::errno().0 != 0 {
        ret_c_int(r)
    } else {
        Ok(r)
    }
}

#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
#[inline]
pub(crate) fn getpriority_user(uid: Uid) -> io::Result<i32> {
    errno::set_errno(errno::Errno(0));
    let r = unsafe { libc::getpriority(libc::PRIO_USER, uid.as_raw() as _) };
    if errno::errno().0 != 0 {
        ret_c_int(r)
    } else {
        Ok(r)
    }
}

#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
#[inline]
pub(crate) fn getpriority_pgrp(pgid: Pid) -> io::Result<i32> {
    errno::set_errno(errno::Errno(0));
    let r = unsafe { libc::getpriority(libc::PRIO_PGRP, pgid.as_raw() as _) };
    if errno::errno().0 != 0 {
        ret_c_int(r)
    } else {
        Ok(r)
    }
}

#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
#[inline]
pub(crate) fn getpriority_process(pid: Pid) -> io::Result<i32> {
    errno::set_errno(errno::Errno(0));
    let r = unsafe { libc::getpriority(libc::PRIO_PROCESS, pid.as_raw() as _) };
    if errno::errno().0 != 0 {
        ret_c_int(r)
    } else {
        Ok(r)
    }
}

#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
#[inline]
pub(crate) fn setpriority_user(uid: Uid, priority: i32) -> io::Result<()> {
    unsafe {
        ret(libc::setpriority(
            libc::PRIO_USER,
            uid.as_raw() as _,
            priority,
        ))
    }
}

#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
#[inline]
pub(crate) fn setpriority_pgrp(pgid: Pid, priority: i32) -> io::Result<()> {
    unsafe {
        ret(libc::setpriority(
            libc::PRIO_PGRP,
            pgid.as_raw() as _,
            priority,
        ))
    }
}

#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
#[inline]
pub(crate) fn setpriority_process(pid: Pid, priority: i32) -> io::Result<()> {
    unsafe {
        ret(libc::setpriority(
            libc::PRIO_PROCESS,
            pid.as_raw() as _,
            priority,
        ))
    }
}

#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
#[inline]
pub(crate) fn getrlimit(limit: Resource) -> Rlimit {
    let mut result = MaybeUninit::<libc_rlimit>::uninit();
    unsafe {
        ret_infallible(libc_getrlimit(limit as _, result.as_mut_ptr()));
        let result = result.assume_init();
        let current = if result.rlim_cur == LIBC_RLIM_INFINITY {
            None
        } else {
            result.rlim_cur.try_into().ok()
        };
        let maximum = if result.rlim_max == LIBC_RLIM_INFINITY {
            None
        } else {
            result.rlim_max.try_into().ok()
        };
        Rlimit { current, maximum }
    }
}

#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn waitpid(pid: RawPid, waitopts: WaitOptions) -> io::Result<Option<(Pid, WaitStatus)>> {
    unsafe {
        let mut status: c_int = 0;
        let pid = ret_c_int(libc::waitpid(pid as _, &mut status, waitopts.bits() as _))?;
        if pid == 0 {
            Ok(None)
        } else {
            Ok(Some((Pid::from_raw(pid), WaitStatus::new(status as _))))
        }
    }
}
