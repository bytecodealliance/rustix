//! Safe (where possible) wrappers around system calls.
//!
//! # Safety
//!
//! This file performs raw system calls, and sometimes passes them
//! uninitialized memory buffers. The signatures in this file are currently
//! manually maintained and must correspond with the signatures of the actual
//! Linux syscalls.
//!
//! Some of this could be auto-generated from the Linux header file
//! <linux/syscalls.h>, but we often need more information than it provides,
//! such as which pointers are array slices, out parameters, or in-out
//! parameters, which integers are owned or borrowed file descriptors, etc.
#![allow(unsafe_code)]
#![allow(dead_code)]

// There are a lot of filesystem and network system calls, so they're split
// out into separate files.
pub(crate) use super::fs::syscalls::*;
pub(crate) use super::net::syscalls::*;

#[cfg(target_pointer_width = "32")]
use super::arch::choose::syscall6_readonly;
use super::arch::choose::{
    syscall0_readonly, syscall1, syscall1_noreturn, syscall1_readonly, syscall2, syscall2_readonly,
    syscall3, syscall3_readonly, syscall4, syscall4_readonly, syscall5, syscall5_readonly,
    syscall6,
};
use super::c;
#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
use super::conv::opt_ref;
use super::conv::{
    borrowed_fd, by_mut, by_ref, c_int, c_str, c_uint, clockid_t, const_void_star, no_fd, out,
    pass_usize, raw_fd, ret, ret_c_int, ret_c_uint, ret_discarded_fd, ret_infallible, ret_owned_fd,
    ret_usize, ret_usize_infallible, ret_void_star, size_of, slice, slice_just_addr, slice_mut,
    void_star, zero,
};
use super::fd::{AsFd, BorrowedFd, RawFd};
#[cfg(feature = "procfs")]
use super::fs::Mode;
use super::io::{
    epoll, Advice as IoAdvice, DupFlags, EventfdFlags, MapFlags, MlockFlags, MprotectFlags,
    MremapFlags, PipeFlags, PollFd, ProtFlags, ReadWriteFlags, UserfaultfdFlags,
};
#[cfg(not(target_os = "wasi"))]
use super::io::{Termios, Winsize};
use super::net::{RecvFlags, SendFlags};
use super::process::{RawCpuSet, RawPid, RawUname, Resource, CPU_SETSIZE};
use super::rand::GetRandomFlags;
use super::reg::nr;
use super::thread::{FutexFlags, FutexOperation};
use super::time::{ClockId, Timespec};
use crate::ffi::ZStr;
use crate::io::{self, IoSlice, IoSliceMut, OwnedFd};
#[cfg(feature = "procfs")]
use crate::path::DecInt;
use crate::process::{
    Cpuid, Gid, MembarrierCommand, MembarrierQuery, Pid, Rlimit, Uid, WaitOptions, WaitStatus,
};
use crate::time::NanosleepRelativeResult;
use alloc::borrow::Cow;
use alloc::vec::Vec;
#[cfg(target_pointer_width = "32")]
use core::convert::TryInto;
use core::mem::{size_of_val, MaybeUninit};
#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
use linux_raw_sys::general::__NR_epoll_pwait;
#[cfg(not(any(target_arch = "aarch64", target_arch = "riscv64")))]
use linux_raw_sys::general::__NR_epoll_wait;
#[cfg(target_arch = "arm")]
use linux_raw_sys::general::{__ARM_NR_set_tls, __NR_mmap2};
#[cfg(target_arch = "x86_64")]
use linux_raw_sys::general::{__NR_arch_prctl, ARCH_SET_FS};
use linux_raw_sys::general::{
    __NR_chdir, __NR_clock_getres, __NR_clock_nanosleep, __NR_close, __NR_dup, __NR_dup3,
    __NR_epoll_create1, __NR_epoll_ctl, __NR_execve, __NR_exit, __NR_exit_group, __NR_fchdir,
    __NR_futex, __NR_getcwd, __NR_getpid, __NR_getppid, __NR_getpriority, __NR_gettid, __NR_ioctl,
    __NR_madvise, __NR_mlock, __NR_mprotect, __NR_munlock, __NR_munmap, __NR_nanosleep, __NR_pipe2,
    __NR_prctl, __NR_pread64, __NR_preadv, __NR_pwrite64, __NR_pwritev, __NR_read, __NR_readv,
    __NR_sched_getaffinity, __NR_sched_setaffinity, __NR_sched_yield, __NR_set_tid_address,
    __NR_setpriority, __NR_uname, __NR_wait4, __NR_write, __NR_writev, __kernel_gid_t,
    __kernel_pid_t, __kernel_timespec, __kernel_uid_t, epoll_event, EPOLL_CTL_ADD, EPOLL_CTL_DEL,
    EPOLL_CTL_MOD, FIONBIO, FIONREAD, PR_SET_NAME, SIGCHLD, TCGETS, TIMER_ABSTIME, TIOCEXCL,
    TIOCGWINSZ, TIOCNXCL,
};
#[cfg(not(any(target_arch = "aarch64", target_arch = "riscv64")))]
use linux_raw_sys::general::{__NR_dup2, __NR_pipe, __NR_poll};
#[cfg(not(any(target_arch = "x86", target_arch = "sparc", target_arch = "arm")))]
use linux_raw_sys::general::{__NR_getegid, __NR_geteuid, __NR_getgid, __NR_getuid};
#[cfg(any(target_arch = "x86", target_arch = "sparc", target_arch = "arm"))]
use linux_raw_sys::general::{__NR_getegid32, __NR_geteuid32, __NR_getgid32, __NR_getuid32};
#[cfg(target_arch = "x86")]
use linux_raw_sys::general::{__NR_mmap2, __NR_set_thread_area};
#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
use linux_raw_sys::general::{__NR_ppoll, sigset_t};
use linux_raw_sys::v5_11::general::__NR_mremap;
use linux_raw_sys::v5_4::general::{
    __NR_clone, __NR_eventfd2, __NR_getrandom, __NR_membarrier, __NR_mlock2, __NR_preadv2,
    __NR_prlimit64, __NR_pwritev2, __NR_userfaultfd,
};
#[cfg(target_pointer_width = "64")]
use {super::conv::loff_t_from_u64, linux_raw_sys::general::__NR_mmap};
#[cfg(target_pointer_width = "32")]
use {
    super::conv::{hi, lo},
    linux_raw_sys::{
        general::__NR_getrlimit,
        general::timespec as __kernel_old_timespec,
        v5_4::general::{__NR_clock_getres_time64, __NR_clock_nanosleep_time64, __NR_futex_time64},
    },
};

// `clock_gettime` has special optimizations via the vDSO.
pub(crate) use super::vdso_wrappers::{clock_gettime, clock_gettime_dynamic};

#[inline]
pub(crate) fn exit_group(code: c::c_int) -> ! {
    unsafe { syscall1_noreturn(nr(__NR_exit_group), c_int(code)) }
}

#[inline]
pub(crate) unsafe fn close(fd: RawFd) {
    let _ = syscall1_readonly(nr(__NR_close), raw_fd(fd));
}

#[inline]
pub(crate) fn clock_getres(which_clock: ClockId) -> __kernel_timespec {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let mut result = MaybeUninit::<__kernel_timespec>::uninit();
        let _ = ret(syscall2(
            nr(__NR_clock_getres_time64),
            clockid_t(which_clock),
            out(&mut result),
        ))
        .or_else(|err| {
            // See the comments in `rsix_clock_gettime_via_syscall` about
            // emulation.
            if err == io::Error::NOSYS {
                let mut old_result = MaybeUninit::<__kernel_old_timespec>::uninit();
                let res = ret(syscall2(
                    nr(__NR_clock_getres),
                    clockid_t(which_clock),
                    out(&mut old_result),
                ));
                let old_result = old_result.assume_init();
                *result.as_mut_ptr() = __kernel_timespec {
                    tv_sec: old_result.tv_sec.into(),
                    tv_nsec: old_result.tv_nsec.into(),
                };
                res
            } else {
                Err(err)
            }
        });
        result.assume_init()
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        let mut result = MaybeUninit::<__kernel_timespec>::uninit();
        let _ = syscall2(
            nr(__NR_clock_getres),
            clockid_t(which_clock),
            out(&mut result),
        );
        result.assume_init()
    }
}

#[inline]
pub(crate) fn read(fd: BorrowedFd<'_>, buf: &mut [u8]) -> io::Result<usize> {
    let (buf_addr_mut, buf_len) = slice_mut(buf);

    unsafe {
        ret_usize(syscall3(
            nr(__NR_read),
            borrowed_fd(fd),
            buf_addr_mut,
            buf_len,
        ))
    }
}

#[inline]
pub(crate) fn pread(fd: BorrowedFd<'_>, buf: &mut [u8], pos: u64) -> io::Result<usize> {
    let (buf_addr_mut, buf_len) = slice_mut(buf);

    // https://github.com/torvalds/linux/blob/fcadab740480e0e0e9fa9bd272acd409884d431a/arch/arm64/kernel/sys32.c#L75
    #[cfg(all(target_pointer_width = "32", target_arch = "arm"))]
    unsafe {
        ret_usize(syscall6(
            nr(__NR_pread64),
            borrowed_fd(fd),
            buf_addr_mut,
            buf_len,
            zero(),
            hi(pos),
            lo(pos),
        ))
    }
    #[cfg(all(target_pointer_width = "32", not(target_arch = "arm")))]
    unsafe {
        ret_usize(syscall5(
            nr(__NR_pread64),
            borrowed_fd(fd),
            buf_addr_mut,
            buf_len,
            hi(pos),
            lo(pos),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_usize(syscall4(
            nr(__NR_pread64),
            borrowed_fd(fd),
            buf_addr_mut,
            buf_len,
            loff_t_from_u64(pos),
        ))
    }
}

pub(crate) fn readv(fd: BorrowedFd<'_>, bufs: &[IoSliceMut]) -> io::Result<usize> {
    let (bufs_addr, bufs_len) = slice(bufs);

    unsafe {
        ret_usize(syscall3(
            nr(__NR_readv),
            borrowed_fd(fd),
            bufs_addr,
            bufs_len,
        ))
    }
}

pub(crate) fn preadv(fd: BorrowedFd<'_>, bufs: &[IoSliceMut], pos: u64) -> io::Result<usize> {
    let (bufs_addr, bufs_len) = slice(bufs);

    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_usize(syscall5(
            nr(__NR_preadv),
            borrowed_fd(fd),
            bufs_addr,
            bufs_len,
            hi(pos),
            lo(pos),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_usize(syscall4(
            nr(__NR_preadv),
            borrowed_fd(fd),
            bufs_addr,
            bufs_len,
            loff_t_from_u64(pos),
        ))
    }
}

pub(crate) fn preadv2(
    fd: BorrowedFd<'_>,
    bufs: &[IoSliceMut],
    pos: u64,
    flags: ReadWriteFlags,
) -> io::Result<usize> {
    let (bufs_addr, bufs_len) = slice(bufs);

    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_usize(syscall6(
            nr(__NR_preadv2),
            borrowed_fd(fd),
            bufs_addr,
            bufs_len,
            hi(pos),
            lo(pos),
            c_uint(flags.bits()),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_usize(syscall5(
            nr(__NR_preadv2),
            borrowed_fd(fd),
            bufs_addr,
            bufs_len,
            loff_t_from_u64(pos),
            c_uint(flags.bits()),
        ))
    }
}

#[inline]
pub(crate) fn write(fd: BorrowedFd<'_>, buf: &[u8]) -> io::Result<usize> {
    let (buf_addr, buf_len) = slice(buf);

    unsafe {
        ret_usize(syscall3_readonly(
            nr(__NR_write),
            borrowed_fd(fd),
            buf_addr,
            buf_len,
        ))
    }
}

#[inline]
pub(crate) fn pwrite(fd: BorrowedFd<'_>, buf: &[u8], pos: u64) -> io::Result<usize> {
    let (buf_addr, buf_len) = slice(buf);

    // https://github.com/torvalds/linux/blob/fcadab740480e0e0e9fa9bd272acd409884d431a/arch/arm64/kernel/sys32.c#L81-L83
    #[cfg(all(target_pointer_width = "32", target_arch = "arm"))]
    unsafe {
        ret_usize(syscall6_readonly(
            nr(__NR_pwrite64),
            borrowed_fd(fd),
            buf_addr,
            buf_len,
            zero(),
            hi(pos),
            lo(pos),
        ))
    }
    #[cfg(all(target_pointer_width = "32", not(target_arch = "arm")))]
    unsafe {
        ret_usize(syscall5_readonly(
            nr(__NR_pwrite64),
            borrowed_fd(fd),
            buf_addr,
            buf_len,
            hi(pos),
            lo(pos),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_usize(syscall4_readonly(
            nr(__NR_pwrite64),
            borrowed_fd(fd),
            buf_addr,
            buf_len,
            loff_t_from_u64(pos),
        ))
    }
}

#[inline]
pub(crate) fn writev(fd: BorrowedFd<'_>, bufs: &[IoSlice]) -> io::Result<usize> {
    let (bufs_addr, bufs_len) = slice(bufs);

    unsafe {
        ret_usize(syscall3_readonly(
            nr(__NR_writev),
            borrowed_fd(fd),
            bufs_addr,
            bufs_len,
        ))
    }
}

#[inline]
pub(crate) fn pwritev(fd: BorrowedFd<'_>, bufs: &[IoSlice], pos: u64) -> io::Result<usize> {
    let (bufs_addr, bufs_len) = slice(bufs);

    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_usize(syscall5_readonly(
            nr(__NR_pwritev),
            borrowed_fd(fd),
            bufs_addr,
            bufs_len,
            hi(pos),
            lo(pos),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_usize(syscall4_readonly(
            nr(__NR_pwritev),
            borrowed_fd(fd),
            bufs_addr,
            bufs_len,
            loff_t_from_u64(pos),
        ))
    }
}

#[inline]
pub(crate) fn pwritev2(
    fd: BorrowedFd<'_>,
    bufs: &[IoSlice],
    pos: u64,
    flags: ReadWriteFlags,
) -> io::Result<usize> {
    let (bufs_addr, bufs_len) = slice(bufs);

    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_usize(syscall6_readonly(
            nr(__NR_pwritev2),
            borrowed_fd(fd),
            bufs_addr,
            bufs_len,
            hi(pos),
            lo(pos),
            c_uint(flags.bits()),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_usize(syscall5_readonly(
            nr(__NR_pwritev2),
            borrowed_fd(fd),
            bufs_addr,
            bufs_len,
            loff_t_from_u64(pos),
            c_uint(flags.bits()),
        ))
    }
}

#[inline]
pub(crate) fn madvise(addr: *mut c::c_void, len: usize, advice: IoAdvice) -> io::Result<()> {
    unsafe {
        ret(syscall3(
            nr(__NR_madvise),
            void_star(addr),
            pass_usize(len),
            c_uint(advice as c::c_uint),
        ))
    }
}

#[inline]
pub(crate) fn pipe_with(flags: PipeFlags) -> io::Result<(OwnedFd, OwnedFd)> {
    #[cfg(any(target_arch = "mips", target_arch = "mips64"))]
    {
        todo!("On MIPS pipe returns multiple values")
    }
    #[cfg(not(any(target_arch = "mips", target_arch = "mips64")))]
    unsafe {
        let mut result = MaybeUninit::<[OwnedFd; 2]>::uninit();
        ret(syscall2(
            nr(__NR_pipe2),
            out(&mut result),
            c_uint(flags.bits()),
        ))?;
        let [p0, p1] = result.assume_init();
        Ok((p0, p1))
    }
}

#[inline]
pub(crate) fn pipe() -> io::Result<(OwnedFd, OwnedFd)> {
    #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
    {
        pipe_with(PipeFlags::empty())
    }
    #[cfg(any(target_arch = "mips", target_arch = "mips64"))]
    {
        todo!("On MIPS pipe returns multiple values")
    }
    #[cfg(not(any(
        target_arch = "aarch64",
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "riscv64"
    )))]
    unsafe {
        let mut result = MaybeUninit::<[OwnedFd; 2]>::uninit();
        ret(syscall1(nr(__NR_pipe), out(&mut result)))?;
        let [p0, p1] = result.assume_init();
        Ok((p0, p1))
    }
}

#[inline]
pub(crate) fn getrandom(buf: &mut [u8], flags: GetRandomFlags) -> io::Result<usize> {
    let (buf_addr_mut, buf_len) = slice_mut(buf);
    unsafe {
        ret_usize(syscall3(
            nr(__NR_getrandom),
            buf_addr_mut,
            buf_len,
            c_uint(flags.bits()),
        ))
    }
}

#[allow(non_snake_case)]
#[inline]
pub(crate) fn CPU_SET(cpu: usize, cpuset: &mut RawCpuSet) {
    let size_in_bits = 8 * size_of_val(&cpuset.bits[0]); // 32, 64 etc
    let (idx, offset) = (cpu / size_in_bits, cpu % size_in_bits);
    *cpuset.bits.get_mut(idx).unwrap_or_else(|| {
        panic!(
            "cpu out of bounds: the cpu max is {} but the cpu is {}",
            CPU_SETSIZE, cpu
        )
    }) |= 1 << offset;
}

#[allow(non_snake_case)]
#[inline]
pub(crate) fn CPU_ZERO(cpuset: &mut RawCpuSet) {
    cpuset.bits.fill(0)
}

#[allow(non_snake_case)]
#[inline]
pub(crate) fn CPU_CLR(cpu: usize, cpuset: &mut RawCpuSet) {
    let size_in_bits = 8 * size_of_val(&cpuset.bits[0]); // 32, 64 etc
    let (idx, offset) = (cpu / size_in_bits, cpu % size_in_bits);
    *cpuset.bits.get_mut(idx).unwrap_or_else(|| {
        panic!(
            "cpu out of bounds: the cpu max is {} but the cpu is {}",
            CPU_SETSIZE, cpu
        )
    }) &= !(1 << offset);
}

#[allow(non_snake_case)]
#[inline]
pub(crate) fn CPU_ISSET(cpu: usize, cpuset: &RawCpuSet) -> bool {
    let size_in_bits = 8 * size_of_val(&cpuset.bits[0]);
    let (idx, offset) = (cpu / size_in_bits, cpu % size_in_bits);
    0 != (cpuset.bits.get(idx).unwrap_or_else(|| {
        panic!(
            "cpu out of bounds: the cpu max is {} but the cpu is {}",
            CPU_SETSIZE, cpu
        )
    }) & (1 << offset))
}

#[allow(non_snake_case)]
#[inline]
pub(crate) fn CPU_COUNT_S(size: usize, cpuset: &RawCpuSet) -> u32 {
    let mut s: u32 = 0;
    let size_of_mask = size_of_val(&cpuset.bits[0]);
    for i in cpuset.bits[..(size / size_of_mask)].iter() {
        s += i.count_ones();
    }
    s
}

#[allow(non_snake_case)]
#[inline]
pub(crate) fn CPU_COUNT(cpuset: &RawCpuSet) -> u32 {
    CPU_COUNT_S(core::mem::size_of::<RawCpuSet>(), cpuset)
}

#[inline]
pub(crate) fn sched_getaffinity(pid: Pid, cpuset: &mut RawCpuSet) -> io::Result<()> {
    unsafe {
        ret(syscall3(
            nr(__NR_sched_getaffinity),
            c_uint(pid.as_raw()),
            size_of::<RawCpuSet, _>(),
            by_mut(&mut cpuset.bits),
        ))
    }
}

#[inline]
pub(crate) fn sched_setaffinity(pid: Pid, cpuset: &RawCpuSet) -> io::Result<()> {
    unsafe {
        ret(syscall3(
            nr(__NR_sched_setaffinity),
            c_uint(pid.as_raw()),
            size_of::<RawCpuSet, _>(),
            slice_just_addr(&cpuset.bits),
        ))
    }
}

#[inline]
pub(crate) fn sched_yield() {
    unsafe {
        let _ = syscall0_readonly(nr(__NR_sched_yield));
    }
}

/// # Safety
///
/// `mmap` is primarily unsafe due to the `addr` parameter, as anything working
/// with memory pointed to by raw pointers is unsafe.
#[inline]
pub(crate) unsafe fn mmap(
    addr: *mut c::c_void,
    length: usize,
    prot: ProtFlags,
    flags: MapFlags,
    fd: BorrowedFd<'_>,
    offset: u64,
) -> io::Result<*mut c::c_void> {
    #[cfg(target_pointer_width = "32")]
    {
        ret_void_star(syscall6(
            nr(__NR_mmap2),
            void_star(addr),
            pass_usize(length),
            c_uint(prot.bits()),
            c_uint(flags.bits()),
            borrowed_fd(fd),
            (offset / 4096)
                .try_into()
                .map(|scaled_offset| pass_usize(scaled_offset))
                .map_err(|_| io::Error::INVAL)?,
        ))
    }
    #[cfg(target_pointer_width = "64")]
    {
        ret_void_star(syscall6(
            nr(__NR_mmap),
            void_star(addr),
            pass_usize(length),
            c_uint(prot.bits()),
            c_uint(flags.bits()),
            borrowed_fd(fd),
            loff_t_from_u64(offset),
        ))
    }
}

/// # Safety
///
/// `mmap` is primarily unsafe due to the `addr` parameter, as anything working
/// with memory pointed to by raw pointers is unsafe.
#[inline]
pub(crate) unsafe fn mmap_anonymous(
    addr: *mut c::c_void,
    length: usize,
    prot: ProtFlags,
    flags: MapFlags,
) -> io::Result<*mut c::c_void> {
    #[cfg(target_pointer_width = "32")]
    {
        ret_void_star(syscall6(
            nr(__NR_mmap2),
            void_star(addr),
            pass_usize(length),
            c_uint(prot.bits()),
            c_uint(flags.bits() | linux_raw_sys::general::MAP_ANONYMOUS),
            no_fd(),
            pass_usize(0),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    {
        ret_void_star(syscall6(
            nr(__NR_mmap),
            void_star(addr),
            pass_usize(length),
            c_uint(prot.bits()),
            c_uint(flags.bits() | linux_raw_sys::general::MAP_ANONYMOUS),
            no_fd(),
            loff_t_from_u64(0),
        ))
    }
}

#[inline]
pub(crate) unsafe fn mprotect(
    ptr: *mut c::c_void,
    len: usize,
    flags: MprotectFlags,
) -> io::Result<()> {
    ret(syscall3(
        nr(__NR_mprotect),
        void_star(ptr),
        pass_usize(len),
        c_uint(flags.bits()),
    ))
}

/// # Safety
///
/// `munmap` is primarily unsafe due to the `addr` parameter, as anything
/// working with memory pointed to by raw pointers is unsafe.
#[inline]
pub(crate) unsafe fn munmap(addr: *mut c::c_void, length: usize) -> io::Result<()> {
    ret(syscall2(
        nr(__NR_munmap),
        void_star(addr),
        pass_usize(length),
    ))
}

/// # Safety
///
/// `mremap` is primarily unsafe due to the `old_address` parameter, as
/// anything working with memory pointed to by raw pointers is unsafe.
#[inline]
pub(crate) unsafe fn mremap(
    old_address: *mut c::c_void,
    old_size: usize,
    new_size: usize,
    flags: MremapFlags,
) -> io::Result<*mut c::c_void> {
    ret_void_star(syscall4(
        nr(__NR_mremap),
        void_star(old_address),
        pass_usize(old_size),
        pass_usize(new_size),
        c_uint(flags.bits()),
    ))
}

/// # Safety
///
/// `mremap_fixed` is primarily unsafe due to the `old_address` and
/// `new_address` parameters, as anything working with memory pointed to by raw
/// pointers is unsafe.
#[inline]
pub(crate) unsafe fn mremap_fixed(
    old_address: *mut c::c_void,
    old_size: usize,
    new_size: usize,
    flags: MremapFlags,
    new_address: *mut c::c_void,
) -> io::Result<*mut c::c_void> {
    ret_void_star(syscall5(
        nr(__NR_mremap),
        void_star(old_address),
        pass_usize(old_size),
        pass_usize(new_size),
        c_uint(flags.bits()),
        void_star(new_address),
    ))
}

/// # Safety
///
/// `mlock` operates on raw pointers and may round out to the nearest page
/// boundaries.
#[inline]
pub(crate) unsafe fn mlock(addr: *mut c::c_void, length: usize) -> io::Result<()> {
    ret(syscall2(
        nr(__NR_mlock),
        void_star(addr),
        pass_usize(length),
    ))
}

/// # Safety
///
/// `mlock_with` operates on raw pointers and may round out to the nearest page
/// boundaries.
#[inline]
pub(crate) unsafe fn mlock_with(
    addr: *mut c::c_void,
    length: usize,
    flags: MlockFlags,
) -> io::Result<()> {
    ret(syscall3(
        nr(__NR_mlock2),
        void_star(addr),
        pass_usize(length),
        c_uint(flags.bits()),
    ))
}

/// # Safety
///
/// `munlock` operates on raw pointers and may round out to the nearest page
/// boundaries.
#[inline]
pub(crate) unsafe fn munlock(addr: *mut c::c_void, length: usize) -> io::Result<()> {
    ret(syscall2(
        nr(__NR_munlock),
        void_star(addr),
        pass_usize(length),
    ))
}

pub(crate) fn membarrier_query() -> MembarrierQuery {
    unsafe {
        match ret_c_uint(syscall2(
            nr(__NR_membarrier),
            c_int(linux_raw_sys::v5_4::general::membarrier_cmd::MEMBARRIER_CMD_QUERY as _),
            c_uint(0),
        )) {
            Ok(query) => {
                // Safety: The safety of `from_bits_unchecked` is discussed
                // [here]. Our "source of truth" is Linux, and here, the
                // `query` value is coming from Linux, so we know it only
                // contains "source of truth" valid bits.
                //
                // [here]: https://github.com/bitflags/bitflags/pull/207#issuecomment-671668662
                MembarrierQuery::from_bits_unchecked(query)
            }
            Err(_) => MembarrierQuery::empty(),
        }
    }
}

pub(crate) fn membarrier(cmd: MembarrierCommand) -> io::Result<()> {
    unsafe {
        ret(syscall2(
            nr(__NR_membarrier),
            c_int(cmd as c::c_int),
            c_uint(0),
        ))
    }
}

pub(crate) fn membarrier_cpu(cmd: MembarrierCommand, cpu: Cpuid) -> io::Result<()> {
    unsafe {
        ret(syscall3(
            nr(__NR_membarrier),
            c_int(cmd as c::c_int),
            c_uint(
                linux_raw_sys::v5_11::general::membarrier_cmd_flag::MEMBARRIER_CMD_FLAG_CPU as _,
            ),
            c_uint(cpu.as_raw()),
        ))
    }
}

#[inline]
pub(crate) fn nanosleep(req: &__kernel_timespec) -> NanosleepRelativeResult {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let mut rem = MaybeUninit::<__kernel_timespec>::uninit();
        match ret(syscall4(
            nr(__NR_clock_nanosleep_time64),
            clockid_t(ClockId::Realtime),
            c_int(0),
            by_ref(req),
            out(&mut rem),
        ))
        .or_else(|err| {
            // See the comments in `rsix_clock_gettime_via_syscall` about
            // emulation.
            if err == io::Error::NOSYS {
                let old_req = __kernel_old_timespec {
                    tv_sec: req.tv_sec.try_into().map_err(|_| io::Error::INVAL)?,
                    tv_nsec: req.tv_nsec.try_into().map_err(|_| io::Error::INVAL)?,
                };
                let mut old_rem = MaybeUninit::<__kernel_old_timespec>::uninit();
                let res = ret(syscall2(
                    nr(__NR_nanosleep),
                    by_ref(&old_req),
                    out(&mut old_rem),
                ));
                let old_rem = old_rem.assume_init();
                *rem.as_mut_ptr() = __kernel_timespec {
                    tv_sec: old_rem.tv_sec.into(),
                    tv_nsec: old_rem.tv_nsec.into(),
                };
                res
            } else {
                Err(err)
            }
        }) {
            Ok(()) => NanosleepRelativeResult::Ok,
            Err(io::Error::INTR) => NanosleepRelativeResult::Interrupted(rem.assume_init()),
            Err(err) => NanosleepRelativeResult::Err(err),
        }
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        let mut rem = MaybeUninit::<__kernel_timespec>::uninit();
        match ret(syscall2(nr(__NR_nanosleep), by_ref(req), out(&mut rem))) {
            Ok(()) => NanosleepRelativeResult::Ok,
            Err(io::Error::INTR) => NanosleepRelativeResult::Interrupted(rem.assume_init()),
            Err(err) => NanosleepRelativeResult::Err(err),
        }
    }
}

#[inline]
pub(crate) fn clock_nanosleep_relative(
    id: ClockId,
    req: &__kernel_timespec,
) -> NanosleepRelativeResult {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let mut rem = MaybeUninit::<__kernel_timespec>::uninit();
        match ret(syscall4(
            nr(__NR_clock_nanosleep_time64),
            clockid_t(id),
            c_int(0),
            by_ref(req),
            out(&mut rem),
        ))
        .or_else(|err| {
            // See the comments in `rsix_clock_gettime_via_syscall` about
            // emulation.
            if err == io::Error::NOSYS {
                let old_req = __kernel_old_timespec {
                    tv_sec: req.tv_sec.try_into().map_err(|_| io::Error::INVAL)?,
                    tv_nsec: req.tv_nsec.try_into().map_err(|_| io::Error::INVAL)?,
                };
                let mut old_rem = MaybeUninit::<__kernel_old_timespec>::uninit();
                let res = ret(syscall4(
                    nr(__NR_clock_nanosleep),
                    clockid_t(id),
                    c_int(0),
                    by_ref(&old_req),
                    out(&mut old_rem),
                ));
                let old_rem = old_rem.assume_init();
                *rem.as_mut_ptr() = __kernel_timespec {
                    tv_sec: old_rem.tv_sec.into(),
                    tv_nsec: old_rem.tv_nsec.into(),
                };
                res
            } else {
                Err(err)
            }
        }) {
            Ok(()) => NanosleepRelativeResult::Ok,
            Err(io::Error::INTR) => NanosleepRelativeResult::Interrupted(rem.assume_init()),
            Err(err) => NanosleepRelativeResult::Err(err),
        }
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        let mut rem = MaybeUninit::<__kernel_timespec>::uninit();
        match ret(syscall4(
            nr(__NR_clock_nanosleep),
            clockid_t(id),
            c_int(0),
            by_ref(req),
            out(&mut rem),
        )) {
            Ok(()) => NanosleepRelativeResult::Ok,
            Err(io::Error::INTR) => NanosleepRelativeResult::Interrupted(rem.assume_init()),
            Err(err) => NanosleepRelativeResult::Err(err),
        }
    }
}

#[inline]
pub(crate) fn clock_nanosleep_absolute(id: ClockId, req: &__kernel_timespec) -> io::Result<()> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret(syscall4_readonly(
            nr(__NR_clock_nanosleep_time64),
            clockid_t(id),
            c_uint(TIMER_ABSTIME),
            by_ref(req),
            zero(),
        ))
        .or_else(|err| {
            // See the comments in `rsix_clock_gettime_via_syscall` about
            // emulation.
            if err == io::Error::NOSYS {
                let old_req = __kernel_old_timespec {
                    tv_sec: req.tv_sec.try_into().map_err(|_| io::Error::INVAL)?,
                    tv_nsec: req.tv_nsec.try_into().map_err(|_| io::Error::INVAL)?,
                };
                ret(syscall4_readonly(
                    nr(__NR_clock_nanosleep),
                    clockid_t(id),
                    c_int(0),
                    by_ref(&old_req),
                    zero(),
                ))
            } else {
                Err(err)
            }
        })
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret(syscall4_readonly(
            nr(__NR_clock_nanosleep),
            clockid_t(id),
            c_uint(TIMER_ABSTIME),
            by_ref(req),
            zero(),
        ))
    }
}

#[inline]
pub(crate) fn getcwd(buf: &mut [u8]) -> io::Result<usize> {
    let (buf_addr_mut, buf_len) = slice_mut(buf);
    unsafe { ret_usize(syscall2(nr(__NR_getcwd), buf_addr_mut, buf_len)) }
}

#[inline]
pub(crate) fn chdir(filename: &ZStr) -> io::Result<()> {
    unsafe { ret(syscall1_readonly(nr(__NR_chdir), c_str(filename))) }
}

#[inline]
pub(crate) fn fchdir(fd: BorrowedFd) -> io::Result<()> {
    unsafe { ret(syscall1_readonly(nr(__NR_fchdir), borrowed_fd(fd))) }
}

#[inline]
pub(crate) fn ioctl_fionread(fd: BorrowedFd) -> io::Result<u64> {
    unsafe {
        let mut result = MaybeUninit::<c::c_int>::uninit();
        ret(syscall3(
            nr(__NR_ioctl),
            borrowed_fd(fd),
            c_uint(FIONREAD),
            out(&mut result),
        ))
        .map(|()| result.assume_init() as u64)
    }
}

#[inline]
pub(crate) fn ioctl_fionbio(fd: BorrowedFd<'_>, value: bool) -> io::Result<()> {
    unsafe {
        let data = value as c::c_int;
        ret(syscall3(
            nr(__NR_ioctl),
            borrowed_fd(fd),
            c_uint(FIONBIO),
            by_ref(&data),
        ))
    }
}

#[inline]
pub(crate) fn ioctl_tiocgwinsz(fd: BorrowedFd) -> io::Result<Winsize> {
    unsafe {
        let mut result = MaybeUninit::<Winsize>::uninit();
        ret(syscall3(
            nr(__NR_ioctl),
            borrowed_fd(fd),
            c_uint(TIOCGWINSZ),
            out(&mut result),
        ))
        .map(|()| result.assume_init())
    }
}

#[inline]
pub(crate) fn ioctl_tiocexcl(fd: BorrowedFd) -> io::Result<()> {
    unsafe { ret(syscall2(nr(__NR_ioctl), borrowed_fd(fd), c_uint(TIOCEXCL))) }
}

#[inline]
pub(crate) fn ioctl_tiocnxcl(fd: BorrowedFd) -> io::Result<()> {
    unsafe { ret(syscall2(nr(__NR_ioctl), borrowed_fd(fd), c_uint(TIOCNXCL))) }
}

#[inline]
pub(crate) fn ioctl_tcgets(fd: BorrowedFd) -> io::Result<Termios> {
    unsafe {
        let mut result = MaybeUninit::<Termios>::uninit();
        ret(syscall3(
            nr(__NR_ioctl),
            borrowed_fd(fd),
            c_uint(TCGETS),
            out(&mut result),
        ))
        .map(|()| result.assume_init())
    }
}

#[inline]
pub(crate) fn dup(fd: BorrowedFd) -> io::Result<OwnedFd> {
    unsafe { ret_owned_fd(syscall1_readonly(nr(__NR_dup), borrowed_fd(fd))) }
}

#[inline]
pub(crate) fn dup2(fd: BorrowedFd, new: &OwnedFd) -> io::Result<()> {
    #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
    {
        dup2_with(fd, new, DupFlags::empty())
    }

    #[cfg(not(any(target_arch = "aarch64", target_arch = "riscv64")))]
    unsafe {
        ret_discarded_fd(syscall2_readonly(
            nr(__NR_dup2),
            borrowed_fd(fd),
            borrowed_fd(new.as_fd()),
        ))
    }
}

#[inline]
pub(crate) fn dup2_with(fd: BorrowedFd, new: &OwnedFd, flags: DupFlags) -> io::Result<()> {
    unsafe {
        ret_discarded_fd(syscall3_readonly(
            nr(__NR_dup3),
            borrowed_fd(fd),
            borrowed_fd(new.as_fd()),
            c_uint(flags.bits()),
        ))
    }
}

#[inline]
pub(crate) fn poll(fds: &mut [PollFd<'_>], timeout: c::c_int) -> io::Result<usize> {
    let (fds_addr_mut, fds_len) = slice_mut(fds);

    #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
    unsafe {
        let timeout = if timeout >= 0 {
            Some(Timespec {
                tv_sec: (timeout as i64) / 1000,
                tv_nsec: (timeout as i64) % 1000 * 1000000,
            })
        } else {
            None
        };
        ret_usize(syscall5(
            nr(__NR_ppoll),
            fds_addr_mut,
            fds_len,
            opt_ref(timeout.as_ref()),
            zero(),
            size_of::<sigset_t, _>(),
        ))
    }
    #[cfg(not(any(target_arch = "aarch64", target_arch = "riscv64")))]
    unsafe {
        ret_usize(syscall3(
            nr(__NR_poll),
            fds_addr_mut,
            fds_len,
            c_int(timeout),
        ))
    }
}

#[inline]
pub(crate) fn eventfd(initval: u32, flags: EventfdFlags) -> io::Result<OwnedFd> {
    unsafe {
        ret_owned_fd(syscall2(
            nr(__NR_eventfd2),
            c_uint(initval),
            c_uint(flags.bits()),
        ))
    }
}

#[inline]
pub(crate) unsafe fn userfaultfd(flags: UserfaultfdFlags) -> io::Result<OwnedFd> {
    ret_owned_fd(syscall1(nr(__NR_userfaultfd), c_uint(flags.bits())))
}

#[inline]
pub(crate) fn getpid() -> Pid {
    unsafe {
        let pid: i32 = ret_usize_infallible(syscall0_readonly(nr(__NR_getpid))) as __kernel_pid_t;
        Pid::from_raw(pid as u32)
    }
}

#[inline]
pub(crate) fn getppid() -> Pid {
    unsafe {
        let ppid: i32 = ret_usize_infallible(syscall0_readonly(nr(__NR_getppid))) as __kernel_pid_t;
        Pid::from_raw(ppid as u32)
    }
}

#[inline]
pub(crate) fn getgid() -> Gid {
    #[cfg(any(target_arch = "x86", target_arch = "sparc", target_arch = "arm"))]
    unsafe {
        let gid: i32 =
            (ret_usize_infallible(syscall0_readonly(nr(__NR_getgid32))) as __kernel_gid_t).into();
        Gid::from_raw(gid as u32)
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "sparc", target_arch = "arm")))]
    unsafe {
        let gid = ret_usize_infallible(syscall0_readonly(nr(__NR_getgid))) as __kernel_gid_t;
        Gid::from_raw(gid)
    }
}

#[inline]
pub(crate) fn getegid() -> Gid {
    #[cfg(any(target_arch = "x86", target_arch = "sparc", target_arch = "arm"))]
    unsafe {
        let gid: i32 =
            (ret_usize_infallible(syscall0_readonly(nr(__NR_getegid32))) as __kernel_gid_t).into();
        Gid::from_raw(gid as u32)
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "sparc", target_arch = "arm")))]
    unsafe {
        let gid = ret_usize_infallible(syscall0_readonly(nr(__NR_getegid))) as __kernel_gid_t;
        Gid::from_raw(gid)
    }
}

#[inline]
pub(crate) fn getuid() -> Uid {
    #[cfg(any(target_arch = "x86", target_arch = "sparc", target_arch = "arm"))]
    unsafe {
        let uid =
            (ret_usize_infallible(syscall0_readonly(nr(__NR_getuid32))) as __kernel_uid_t).into();
        Uid::from_raw(uid)
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "sparc", target_arch = "arm")))]
    unsafe {
        let uid = ret_usize_infallible(syscall0_readonly(nr(__NR_getuid))) as __kernel_uid_t;
        Uid::from_raw(uid)
    }
}

#[inline]
pub(crate) fn geteuid() -> Uid {
    #[cfg(any(target_arch = "x86", target_arch = "sparc", target_arch = "arm"))]
    unsafe {
        let uid: i32 =
            (ret_usize_infallible(syscall0_readonly(nr(__NR_geteuid32))) as __kernel_uid_t).into();
        Uid::from_raw(uid as u32)
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "sparc", target_arch = "arm")))]
    unsafe {
        let uid = ret_usize_infallible(syscall0_readonly(nr(__NR_geteuid))) as __kernel_uid_t;
        Uid::from_raw(uid)
    }
}

#[inline]
pub(crate) fn gettid() -> Pid {
    unsafe {
        let tid: i32 = ret_usize_infallible(syscall0_readonly(nr(__NR_gettid))) as __kernel_pid_t;
        Pid::from_raw(tid as u32)
    }
}

#[cfg(feature = "procfs")]
#[inline]
pub(crate) fn ttyname(fd: BorrowedFd<'_>, buf: &mut [u8]) -> io::Result<usize> {
    let fd_stat = fstat(fd)?;

    // Quick check: if `fd` isn't a character device, it's not a tty.
    if (fd_stat.st_mode & Mode::IFMT.bits()) != Mode::IFCHR.bits() {
        return Err(crate::io::Error::NOTTY);
    }

    // Check that `fd` is really a tty.
    ioctl_tiocgwinsz(fd)?;

    // Get a fd to '/proc/self/fd'.
    let proc_self_fd = io::proc_self_fd()?;

    // Gather the ttyname by reading the 'fd' file inside 'proc_self_fd'.
    let r = readlinkat(proc_self_fd, DecInt::from_fd(&fd).as_c_str(), buf)?;

    // If the number of bytes is equal to the buffer length, truncation may
    // have occurred. This check also ensures that we have enough space for
    // adding a NUL terminator.
    if r == buf.len() {
        return Err(io::Error::RANGE);
    }
    buf[r] = 0;

    // Check that the path we read refers to the same file as `fd`.
    let path = ZStr::from_bytes_with_nul(&buf[..=r]).unwrap();

    let path_stat = stat(path)?;
    if path_stat.st_dev != fd_stat.st_dev || path_stat.st_ino != fd_stat.st_ino {
        return Err(crate::io::Error::NODEV);
    }

    Ok(r)
}

#[inline]
pub(crate) fn isatty(fd: BorrowedFd<'_>) -> bool {
    // On error, Linux will return either `EINVAL` (2.6.32) or `ENOTTY`
    // (otherwise), because we assume we're never passing an invalid
    // file descriptor (which would get `EBADF`). Either way, an error
    // means we don't have a tty.
    ioctl_tiocgwinsz(fd).is_ok()
}

pub(crate) fn is_read_write(fd: BorrowedFd<'_>) -> io::Result<(bool, bool)> {
    let (mut read, mut write) = crate::fs::fd::_is_file_read_write(fd)?;
    let mut not_socket = false;
    if read {
        // Do a `recv` with `PEEK` and `DONTWAIT` for 1 byte. A 0 indicates
        // the read side is shut down; an `EWOULDBLOCK` indicates the read
        // side is still open.
        //
        // TODO: This code would benefit from having a better way to read into
        // uninitialized memory.
        let mut buf = [0];
        match recv(fd, &mut buf, RecvFlags::PEEK | RecvFlags::DONTWAIT) {
            Ok(0) => read = false,
            Err(err) => {
                #[allow(unreachable_patterns)] // EAGAIN may equal EWOULDBLOCK
                match err {
                    io::Error::AGAIN | io::Error::WOULDBLOCK => (),
                    io::Error::NOTSOCK => not_socket = true,
                    _ => return Err(err),
                }
            }
            Ok(_) => (),
        }
    }
    if write && !not_socket {
        // Do a `send` with `DONTWAIT` for 0 bytes. An `EPIPE` indicates
        // the write side is shut down.
        #[allow(unreachable_patterns)] // EAGAIN equals EWOULDBLOCK
        match send(fd, &[], SendFlags::DONTWAIT) {
            // TODO or-patterns when we don't need 1.51
            Err(io::Error::AGAIN) => (),
            Err(io::Error::WOULDBLOCK) => (),
            Err(io::Error::NOTSOCK) => (),
            Err(io::Error::PIPE) => write = false,
            Err(err) => return Err(err),
            Ok(_) => (),
        }
    }
    Ok((read, write))
}

#[inline]
pub(crate) fn epoll_create(flags: epoll::CreateFlags) -> io::Result<OwnedFd> {
    unsafe { ret_owned_fd(syscall1(nr(__NR_epoll_create1), c_uint(flags.bits()))) }
}

#[inline]
pub(crate) unsafe fn epoll_add(
    epfd: BorrowedFd<'_>,
    fd: c::c_int,
    event: &epoll_event,
) -> io::Result<()> {
    ret(syscall4(
        nr(__NR_epoll_ctl),
        borrowed_fd(epfd),
        c_uint(EPOLL_CTL_ADD),
        raw_fd(fd),
        by_ref(event),
    ))
}

#[inline]
pub(crate) unsafe fn epoll_mod(
    epfd: BorrowedFd<'_>,
    fd: c::c_int,
    event: &epoll_event,
) -> io::Result<()> {
    ret(syscall4(
        nr(__NR_epoll_ctl),
        borrowed_fd(epfd),
        c_uint(EPOLL_CTL_MOD),
        raw_fd(fd),
        by_ref(event),
    ))
}

#[inline]
pub(crate) unsafe fn epoll_del(epfd: BorrowedFd<'_>, fd: c::c_int) -> io::Result<()> {
    ret(syscall4(
        nr(__NR_epoll_ctl),
        borrowed_fd(epfd),
        c_uint(EPOLL_CTL_DEL),
        raw_fd(fd),
        zero(),
    ))
}

#[inline]
pub(crate) fn epoll_wait(
    epfd: BorrowedFd<'_>,
    events: *mut epoll_event,
    num_events: usize,
    timeout: c::c_int,
) -> io::Result<usize> {
    #[cfg(not(any(target_arch = "aarch64", target_arch = "riscv64")))]
    unsafe {
        ret_usize(syscall4(
            nr(__NR_epoll_wait),
            borrowed_fd(epfd),
            void_star(events.cast::<c::c_void>()),
            pass_usize(num_events),
            c_int(timeout),
        ))
    }
    #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
    unsafe {
        ret_usize(syscall5(
            nr(__NR_epoll_pwait),
            borrowed_fd(epfd),
            void_star(events.cast::<c::c_void>()),
            pass_usize(num_events),
            c_int(timeout),
            zero(),
        ))
    }
}

#[inline]
pub(crate) fn uname() -> RawUname {
    let mut uname = MaybeUninit::<RawUname>::uninit();
    unsafe {
        ret(syscall1(nr(__NR_uname), out(&mut uname))).unwrap();
        uname.assume_init()
    }
}

#[inline]
pub(crate) fn nice(inc: i32) -> io::Result<i32> {
    let priority = if inc > -40 && inc < 40 {
        inc + getpriority_process(Pid::NONE)?
    } else {
        inc
    }
    .clamp(-20, 19);
    setpriority_process(Pid::NONE, priority)?;
    Ok(priority)
}

#[inline]
pub(crate) fn getpriority_user(uid: Uid) -> io::Result<i32> {
    unsafe {
        Ok(20
            - ret_c_int(syscall2_readonly(
                nr(__NR_getpriority),
                c_uint(linux_raw_sys::general::PRIO_USER),
                c_uint(uid.as_raw()),
            ))?)
    }
}

#[inline]
pub(crate) fn getpriority_pgrp(pgid: Pid) -> io::Result<i32> {
    unsafe {
        Ok(20
            - ret_c_int(syscall2_readonly(
                nr(__NR_getpriority),
                c_uint(linux_raw_sys::general::PRIO_PGRP),
                c_uint(pgid.as_raw()),
            ))?)
    }
}

#[inline]
pub(crate) fn getpriority_process(pid: Pid) -> io::Result<i32> {
    unsafe {
        Ok(20
            - ret_c_int(syscall2_readonly(
                nr(__NR_getpriority),
                c_uint(linux_raw_sys::general::PRIO_PROCESS),
                c_uint(pid.as_raw()),
            ))?)
    }
}

#[inline]
pub(crate) fn setpriority_user(uid: Uid, priority: i32) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_setpriority),
            c_uint(linux_raw_sys::general::PRIO_USER),
            c_uint(uid.as_raw()),
            c_int(priority),
        ))
    }
}

#[inline]
pub(crate) fn setpriority_pgrp(pgid: Pid, priority: i32) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_setpriority),
            c_uint(linux_raw_sys::general::PRIO_PGRP),
            c_uint(pgid.as_raw()),
            c_int(priority),
        ))
    }
}

#[inline]
pub(crate) fn setpriority_process(pid: Pid, priority: i32) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_setpriority),
            c_uint(linux_raw_sys::general::PRIO_PROCESS),
            c_uint(pid.as_raw()),
            c_int(priority),
        ))
    }
}

// TODO: This could be de-multiplexed.
#[inline]
pub(crate) unsafe fn futex(
    uaddr: *mut u32,
    op: FutexOperation,
    flags: FutexFlags,
    val: u32,
    utime: *const Timespec,
    uaddr2: *mut u32,
    val3: u32,
) -> io::Result<usize> {
    #[cfg(target_pointer_width = "32")]
    {
        ret_usize(syscall6(
            nr(__NR_futex_time64),
            void_star(uaddr.cast()),
            c_uint(op as c::c_uint | flags.bits()),
            c_uint(val),
            const_void_star(utime.cast()),
            void_star(uaddr2.cast()),
            c_uint(val3),
        ))
        .or_else(|err| {
            // See the comments in `rsix_clock_gettime_via_syscall` about
            // emulation.
            if err == io::Error::NOSYS {
                let old_utime = __kernel_old_timespec {
                    tv_sec: (*utime).tv_sec.try_into().map_err(|_| io::Error::INVAL)?,
                    tv_nsec: (*utime).tv_nsec.try_into().map_err(|_| io::Error::INVAL)?,
                };
                ret_usize(syscall6(
                    nr(__NR_futex),
                    void_star(uaddr.cast()),
                    c_uint(op as c::c_uint | flags.bits()),
                    c_uint(val),
                    by_ref(&old_utime),
                    void_star(uaddr2.cast()),
                    c_uint(val3),
                ))
            } else {
                Err(err)
            }
        })
    }
    #[cfg(target_pointer_width = "64")]
    ret_usize(syscall6(
        nr(__NR_futex),
        void_star(uaddr.cast()),
        c_uint(op as c::c_uint | flags.bits()),
        c_uint(val),
        const_void_star(utime.cast()),
        void_star(uaddr2.cast()),
        c_uint(val3),
    ))
}

#[inline]
pub(crate) fn getrlimit(limit: Resource) -> Rlimit {
    let mut result = MaybeUninit::<linux_raw_sys::v5_4::general::rlimit64>::uninit();
    #[cfg(target_pointer_width = "32")]
    unsafe {
        match ret(syscall4(
            nr(__NR_prlimit64),
            c_uint(0),
            c_uint(limit as c::c_uint),
            void_star(core::ptr::null_mut()),
            out(&mut result),
        )) {
            Ok(()) => {
                let result = result.assume_init();
                let current =
                    if result.rlim_cur == linux_raw_sys::v5_4::general::RLIM64_INFINITY as _ {
                        None
                    } else {
                        Some(result.rlim_cur)
                    };
                let maximum =
                    if result.rlim_max == linux_raw_sys::v5_4::general::RLIM64_INFINITY as _ {
                        None
                    } else {
                        Some(result.rlim_max)
                    };
                Rlimit { current, maximum }
            }
            Err(e) => {
                debug_assert_eq!(e, io::Error::NOSYS);
                let mut result = MaybeUninit::<linux_raw_sys::general::rlimit>::uninit();
                ret_infallible(syscall2(
                    nr(__NR_getrlimit),
                    c_uint(limit as c::c_uint),
                    out(&mut result),
                ));
                let result = result.assume_init();
                let current = if result.rlim_cur == linux_raw_sys::general::RLIM_INFINITY as _ {
                    None
                } else {
                    result.rlim_cur.try_into().ok()
                };
                let maximum = if result.rlim_max == linux_raw_sys::general::RLIM_INFINITY as _ {
                    None
                } else {
                    result.rlim_cur.try_into().ok()
                };
                Rlimit { current, maximum }
            }
        }
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_infallible(syscall4(
            nr(__NR_prlimit64),
            c_uint(0),
            c_uint(limit as c::c_uint),
            void_star(core::ptr::null_mut()),
            out(&mut result),
        ));
        let result = result.assume_init();
        let current = if result.rlim_cur == linux_raw_sys::general::RLIM_INFINITY as _ {
            None
        } else {
            Some(result.rlim_cur)
        };
        let maximum = if result.rlim_max == linux_raw_sys::general::RLIM_INFINITY as _ {
            None
        } else {
            Some(result.rlim_max)
        };
        Rlimit { current, maximum }
    }
}

#[inline]
pub(crate) unsafe fn fork() -> io::Result<Pid> {
    let pid = ret_c_uint(syscall5_readonly(
        nr(__NR_clone),
        c_uint(SIGCHLD),
        zero(),
        zero(),
        zero(),
        zero(),
    ))?;
    Ok(Pid::from_raw(pid))
}

pub(crate) fn execve(
    path: &ZStr,
    args: &[Cow<'_, ZStr>],
    env_vars: &[Cow<'_, ZStr>],
) -> io::Result<()> {
    let argv: Vec<_> = args
        .iter()
        .map(|zstr| ZStr::as_ptr(zstr))
        .chain(core::iter::once(core::ptr::null()))
        .collect();
    let envs: Vec<_> = env_vars
        .iter()
        .map(|zstr| ZStr::as_ptr(zstr))
        .chain(core::iter::once(core::ptr::null()))
        .collect();
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_execve),
            c_str(path),
            slice_just_addr(&argv),
            slice_just_addr(&envs),
        ))
    }
}

#[inline]
pub(crate) fn waitpid(pid: RawPid, waitopts: WaitOptions) -> io::Result<Option<(Pid, WaitStatus)>> {
    unsafe {
        let mut status: u32 = 0;
        let pid = ret_c_uint(syscall4(
            nr(__NR_wait4),
            c_int(pid as _),
            by_mut(&mut status),
            c_int(waitopts.bits() as _),
            zero(),
        ))?;
        if pid == 0 {
            Ok(None)
        } else {
            Ok(Some((Pid::from_raw(pid), WaitStatus::new(status))))
        }
    }
}

pub(crate) mod tls {
    #[cfg(target_arch = "x86")]
    use super::super::thread::tls::UserDesc;
    use super::*;

    #[cfg(target_arch = "x86")]
    #[inline]
    pub(crate) unsafe fn set_thread_area(u_info: &mut UserDesc) -> io::Result<()> {
        ret(syscall1(nr(__NR_set_thread_area), by_mut(u_info)))
    }

    #[cfg(target_arch = "arm")]
    #[inline]
    pub(crate) unsafe fn arm_set_tls(data: *mut c::c_void) -> io::Result<()> {
        ret(syscall1(nr(__ARM_NR_set_tls), void_star(data)))
    }

    #[cfg(target_arch = "x86_64")]
    #[inline]
    pub(crate) unsafe fn set_fs(data: *mut c::c_void) {
        ret_infallible(syscall2(
            nr(__NR_arch_prctl),
            c_uint(ARCH_SET_FS),
            void_star(data),
        ))
    }

    #[inline]
    pub(crate) unsafe fn set_tid_address(data: *mut c::c_void) -> Pid {
        let tid: i32 = ret_usize_infallible(syscall1(nr(__NR_set_tid_address), void_star(data)))
            as __kernel_pid_t;
        Pid::from_raw(tid as u32)
    }

    #[inline]
    pub(crate) unsafe fn set_thread_name(name: &ZStr) -> io::Result<()> {
        ret(syscall2(nr(__NR_prctl), c_uint(PR_SET_NAME), c_str(name)))
    }

    #[inline]
    pub(crate) fn exit_thread(code: c::c_int) -> ! {
        unsafe { syscall1_noreturn(nr(__NR_exit), c_int(code)) }
    }
}
