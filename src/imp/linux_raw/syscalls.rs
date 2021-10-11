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

//! Functions which operate on file descriptors.

use super::arch::choose::{
    syscall0_readonly, syscall1, syscall1_noreturn, syscall1_readonly, syscall2, syscall2_readonly,
    syscall3, syscall3_readonly, syscall4, syscall4_readonly, syscall5, syscall5_readonly,
    syscall6, syscall6_readonly,
};
#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
use super::conv::opt_ref;
use super::conv::{
    borrowed_fd, by_mut, by_ref, c_int, c_str, c_uint, clockid_t, const_void_star, dev_t, mode_as,
    no_fd, oflags, oflags_for_open_how, opt_c_str, opt_mut, out, pass_usize, raw_fd, ret,
    ret_c_int, ret_c_uint, ret_discarded_fd, ret_infallible, ret_owned_fd, ret_usize,
    ret_usize_infallible, ret_void_star, size_of, slice, slice_just_addr, slice_mut, socklen_t,
    void_star, zero,
};
use super::fs::{
    Access, Advice as FsAdvice, AtFlags, FallocateFlags, FdFlags, FlockOperation, MemfdFlags, Mode,
    OFlags, RenameFlags, ResolveFlags, Stat, StatFs, StatxFlags,
};
use super::io::{
    epoll, Advice as IoAdvice, DupFlags, EventfdFlags, MapFlags, MlockFlags, MprotectFlags,
    MremapFlags, PipeFlags, PollFd, ProtFlags, ReadWriteFlags, UserfaultfdFlags,
};
#[cfg(not(target_os = "wasi"))]
use super::io::{Termios, Winsize};
use super::net::{
    encode_sockaddr_unix, encode_sockaddr_v4, encode_sockaddr_v6, read_sockaddr_os, AcceptFlags,
    AddressFamily, Protocol, RecvFlags, SendFlags, Shutdown, SocketAddr, SocketAddrUnix,
    SocketFlags, SocketType,
};
use super::process::{RawUname, Resource};
use super::rand::GetRandomFlags;
use super::reg::nr;
#[cfg(target_arch = "x86")]
use super::reg::{ArgReg, SocketArg};
use super::thread::{FutexFlags, FutexOperation};
use super::time::{ClockId, Timespec};
use crate::io;
use crate::io::{OwnedFd, RawFd};
use crate::path::DecInt;
use crate::process::{Cpuid, Gid, MembarrierCommand, MembarrierQuery, Pid, Rlimit, Uid};
use crate::time::NanosleepRelativeResult;
use io_lifetimes::{AsFd, BorrowedFd};
#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
use linux_raw_sys::general::__NR_epoll_pwait;
#[cfg(not(any(target_arch = "aarch64", target_arch = "riscv64")))]
use linux_raw_sys::general::__NR_epoll_wait;
#[cfg(all(target_pointer_width = "32", not(target_arch = "arm")))]
use linux_raw_sys::general::__NR_fadvise64_64;
#[cfg(not(any(target_arch = "riscv64")))]
use linux_raw_sys::general::__NR_renameat;
#[cfg(target_arch = "arm")]
use linux_raw_sys::general::{
    __ARM_NR_set_tls, __NR_arm_fadvise64_64 as __NR_fadvise64_64, __NR_mmap2,
};
#[cfg(not(target_arch = "x86"))]
use linux_raw_sys::general::{
    __NR_accept, __NR_accept4, __NR_bind, __NR_connect, __NR_getpeername, __NR_getsockname,
    __NR_getsockopt, __NR_listen, __NR_recvfrom, __NR_sendto, __NR_setsockopt, __NR_shutdown,
    __NR_socket, __NR_socketpair,
};
#[cfg(target_arch = "x86_64")]
use linux_raw_sys::general::{__NR_arch_prctl, ARCH_SET_FS};
use linux_raw_sys::general::{
    __NR_chdir, __NR_clock_getres, __NR_clock_nanosleep, __NR_close, __NR_dup, __NR_dup3,
    __NR_epoll_create1, __NR_epoll_ctl, __NR_exit, __NR_exit_group, __NR_faccessat, __NR_fallocate,
    __NR_fchdir, __NR_fchmod, __NR_fchmodat, __NR_fdatasync, __NR_flock, __NR_fsync, __NR_futex,
    __NR_getcwd, __NR_getdents64, __NR_getpid, __NR_getppid, __NR_getpriority, __NR_gettid,
    __NR_ioctl, __NR_linkat, __NR_madvise, __NR_mkdirat, __NR_mknodat, __NR_mlock, __NR_mprotect,
    __NR_munlock, __NR_munmap, __NR_nanosleep, __NR_openat, __NR_pipe2, __NR_prctl, __NR_pread64,
    __NR_preadv, __NR_pwrite64, __NR_pwritev, __NR_read, __NR_readlinkat, __NR_readv,
    __NR_sched_yield, __NR_set_tid_address, __NR_setpriority, __NR_symlinkat, __NR_uname,
    __NR_unlinkat, __NR_utimensat, __NR_write, __NR_writev, __kernel_gid_t, __kernel_pid_t,
    __kernel_timespec, __kernel_uid_t, epoll_event, sockaddr, sockaddr_in, sockaddr_in6,
    sockaddr_un, socklen_t, AT_FDCWD, AT_REMOVEDIR, AT_SYMLINK_NOFOLLOW, EPOLL_CTL_ADD,
    EPOLL_CTL_DEL, EPOLL_CTL_MOD, FIONBIO, FIONREAD, F_DUPFD, F_DUPFD_CLOEXEC, F_GETFD, F_GETFL,
    F_GETLEASE, F_GETOWN, F_GETSIG, F_SETFD, F_SETFL, PR_SET_NAME, TCGETS, TIMER_ABSTIME, TIOCEXCL,
    TIOCGWINSZ, TIOCNXCL,
};
#[cfg(not(any(target_arch = "aarch64", target_arch = "riscv64")))]
use linux_raw_sys::general::{__NR_dup2, __NR_open, __NR_pipe, __NR_poll};
#[cfg(not(any(target_arch = "x86", target_arch = "sparc", target_arch = "arm")))]
use linux_raw_sys::general::{__NR_getegid, __NR_geteuid, __NR_getgid, __NR_getuid};
#[cfg(any(target_arch = "x86", target_arch = "sparc", target_arch = "arm"))]
use linux_raw_sys::general::{__NR_getegid32, __NR_geteuid32, __NR_getgid32, __NR_getuid32};
#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
use linux_raw_sys::general::{__NR_ppoll, sigset_t};
#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "riscv64"
)))]
use linux_raw_sys::general::{__NR_recv, __NR_send};
use linux_raw_sys::v5_11::general::{__NR_mremap, __NR_openat2, open_how};
use linux_raw_sys::v5_4::general::{
    __NR_copy_file_range, __NR_eventfd2, __NR_getrandom, __NR_membarrier, __NR_memfd_create,
    __NR_mlock2, __NR_preadv2, __NR_prlimit64, __NR_pwritev2, __NR_renameat2, __NR_statx,
    __NR_userfaultfd, statx, F_GETPIPE_SZ, F_GET_SEALS, F_SETPIPE_SZ,
};
use std::convert::TryInto;
use std::ffi::CStr;
use std::io::{IoSlice, IoSliceMut, SeekFrom};
use std::mem::MaybeUninit;
use std::net::{SocketAddrV4, SocketAddrV6};
use std::os::raw::{c_int, c_uint, c_void};
#[cfg(target_arch = "x86")]
use {
    super::conv::x86_sys,
    linux_raw_sys::general::{
        __NR_mmap2, __NR_set_thread_area, __NR_socketcall, SYS_ACCEPT, SYS_ACCEPT4, SYS_BIND,
        SYS_CONNECT, SYS_GETPEERNAME, SYS_GETSOCKNAME, SYS_GETSOCKOPT, SYS_LISTEN, SYS_RECV,
        SYS_RECVFROM, SYS_SEND, SYS_SENDTO, SYS_SETSOCKOPT, SYS_SHUTDOWN, SYS_SOCKET,
        SYS_SOCKETPAIR,
    },
};
#[cfg(target_pointer_width = "32")]
use {
    super::conv::{hi, lo},
    linux_raw_sys::{
        general::timespec as __kernel_old_timespec,
        general::{
            __NR__llseek, __NR_fcntl64, __NR_fstat64, __NR_fstatat64, __NR_fstatfs64,
            __NR_ftruncate64, __NR_getrlimit, __NR_sendfile64,
        },
        v5_4::general::{
            __NR_clock_getres_time64, __NR_clock_nanosleep_time64, __NR_futex_time64,
            __NR_utimensat_time64,
        },
    },
};
#[cfg(target_pointer_width = "64")]
use {
    super::conv::{loff_t, loff_t_from_u64, ret_u64},
    linux_raw_sys::general::{
        __NR_fadvise64, __NR_fcntl, __NR_fstat, __NR_fstatfs, __NR_ftruncate, __NR_lseek,
        __NR_mmap, __NR_newfstatat, __NR_sendfile,
    },
};

// `clock_gettime` has special optimizations via the vDSO.
pub(crate) use super::vdso_wrappers::{clock_gettime, clock_gettime_dynamic};

#[inline]
pub(crate) fn exit_group(code: c_int) -> ! {
    unsafe { syscall1_noreturn(nr(__NR_exit_group), c_int(code)) }
}

#[inline]
pub(crate) unsafe fn close(fd: RawFd) {
    let _ = syscall1_readonly(nr(__NR_close), raw_fd(fd));
}

#[inline]
pub(crate) fn open(filename: &CStr, flags: OFlags, mode: Mode) -> io::Result<OwnedFd> {
    #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
    {
        openat(crate::fs::cwd().as_fd(), filename, flags, mode)
    }
    #[cfg(all(
        target_pointer_width = "32",
        not(any(target_arch = "aarch64", target_arch = "riscv64"))
    ))]
    unsafe {
        ret_owned_fd(syscall3_readonly(
            nr(__NR_open),
            c_str(filename),
            oflags(flags),
            mode_as(mode),
        ))
    }
    #[cfg(all(
        target_pointer_width = "64",
        not(any(target_arch = "aarch64", target_arch = "riscv64"))
    ))]
    unsafe {
        ret_owned_fd(syscall3_readonly(
            nr(__NR_open),
            c_str(filename),
            oflags(flags),
            mode_as(mode),
        ))
    }
}

#[inline]
pub(crate) fn openat(
    dirfd: BorrowedFd<'_>,
    filename: &CStr,
    flags: OFlags,
    mode: Mode,
) -> io::Result<OwnedFd> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_owned_fd(syscall4_readonly(
            nr(__NR_openat),
            borrowed_fd(dirfd),
            c_str(filename),
            oflags(flags),
            mode_as(mode),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_owned_fd(syscall4_readonly(
            nr(__NR_openat),
            borrowed_fd(dirfd),
            c_str(filename),
            oflags(flags),
            mode_as(mode),
        ))
    }
}

#[inline]
pub(crate) fn openat2(
    dirfd: BorrowedFd<'_>,
    pathname: &CStr,
    flags: OFlags,
    mode: Mode,
    resolve: ResolveFlags,
) -> io::Result<OwnedFd> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_owned_fd(syscall4_readonly(
            nr(__NR_openat2),
            borrowed_fd(dirfd),
            c_str(pathname),
            by_ref(&open_how {
                flags: oflags_for_open_how(flags),
                mode: u64::from(mode.bits()),
                resolve: resolve.bits(),
            }),
            size_of::<open_how, _>(),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_owned_fd(syscall4_readonly(
            nr(__NR_openat2),
            borrowed_fd(dirfd),
            c_str(pathname),
            by_ref(&open_how {
                flags: oflags_for_open_how(flags),
                mode: u64::from(mode.bits()),
                resolve: resolve.bits(),
            }),
            size_of::<open_how, _>(),
        ))
    }
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
pub(crate) fn chmod(filename: &CStr, mode: Mode) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_fchmodat),
            c_int(AT_FDCWD),
            c_str(filename),
            mode_as(mode),
        ))
    }
}

#[inline]
pub(crate) fn chmodat(dirfd: BorrowedFd<'_>, filename: &CStr, mode: Mode) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_fchmodat),
            borrowed_fd(dirfd),
            c_str(filename),
            mode_as(mode),
        ))
    }
}

#[inline]
pub(crate) fn fchmod(fd: BorrowedFd<'_>, mode: Mode) -> io::Result<()> {
    unsafe {
        ret(syscall2_readonly(
            nr(__NR_fchmod),
            borrowed_fd(fd),
            mode_as(mode),
        ))
    }
}

#[inline]
pub(crate) fn mknodat(
    dirfd: BorrowedFd<'_>,
    filename: &CStr,
    mode: Mode,
    dev: u64,
) -> io::Result<()> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret(syscall4_readonly(
            nr(__NR_mknodat),
            borrowed_fd(dirfd),
            c_str(filename),
            mode_as(mode),
            dev_t(dev)?,
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret(syscall4_readonly(
            nr(__NR_mknodat),
            borrowed_fd(dirfd),
            c_str(filename),
            mode_as(mode),
            dev_t(dev),
        ))
    }
}

#[inline]
pub(crate) fn seek(fd: BorrowedFd<'_>, pos: SeekFrom) -> io::Result<u64> {
    let (whence, offset) = match pos {
        SeekFrom::Start(pos) => {
            let pos: u64 = pos;
            // Silently cast; we'll get `EINVAL` if the value is negative.
            (linux_raw_sys::general::SEEK_SET, pos as i64)
        }
        SeekFrom::End(offset) => (linux_raw_sys::general::SEEK_END, offset),
        SeekFrom::Current(offset) => (linux_raw_sys::general::SEEK_CUR, offset),
    };
    _seek(fd, offset, whence)
}

#[inline]
pub(super) fn _seek(fd: BorrowedFd<'_>, offset: i64, whence: c_uint) -> io::Result<u64> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let mut result = MaybeUninit::<u64>::uninit();
        ret(syscall5(
            nr(__NR__llseek),
            borrowed_fd(fd),
            // Don't use the hi/lo functions here because Linux's llseek
            // takes its 64-bit argument differently from everything else.
            pass_usize((offset >> 32) as usize),
            pass_usize(offset as usize),
            out(&mut result),
            c_uint(whence),
        ))
        .map(|()| result.assume_init())
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_u64(syscall3_readonly(
            nr(__NR_lseek),
            borrowed_fd(fd),
            loff_t(offset),
            c_uint(whence),
        ))
    }
}

#[inline]
pub(crate) fn tell(fd: BorrowedFd<'_>) -> io::Result<u64> {
    _seek(fd, 0, linux_raw_sys::general::SEEK_CUR).map(|x| x as u64)
}

#[inline]
pub(crate) fn ftruncate(fd: BorrowedFd<'_>, length: u64) -> io::Result<()> {
    // https://github.com/torvalds/linux/blob/fcadab740480e0e0e9fa9bd272acd409884d431a/arch/arm64/kernel/sys32.c#L81-L83
    #[cfg(all(target_pointer_width = "32", target_arch = "arm"))]
    unsafe {
        ret(syscall4_readonly(
            nr(__NR_ftruncate64),
            borrowed_fd(fd),
            zero(),
            hi(length),
            lo(length),
        ))
    }
    #[cfg(all(target_pointer_width = "32", not(target_arch = "arm")))]
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_ftruncate64),
            borrowed_fd(fd),
            hi(length),
            lo(length),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret(syscall2_readonly(
            nr(__NR_ftruncate),
            borrowed_fd(fd),
            loff_t_from_u64(length),
        ))
    }
}

#[inline]
pub(crate) fn fallocate(
    fd: BorrowedFd,
    mode: FallocateFlags,
    offset: u64,
    len: u64,
) -> io::Result<()> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret(syscall6_readonly(
            nr(__NR_fallocate),
            borrowed_fd(fd),
            c_uint(mode.bits()),
            hi(offset),
            lo(offset),
            hi(len),
            lo(len),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret(syscall4_readonly(
            nr(__NR_fallocate),
            borrowed_fd(fd),
            c_uint(mode.bits()),
            loff_t_from_u64(offset),
            loff_t_from_u64(len),
        ))
    }
}

#[inline]
pub(crate) fn fadvise(fd: BorrowedFd<'_>, pos: u64, len: u64, advice: FsAdvice) -> io::Result<()> {
    // On arm and powerpc, the system calls are reordered so that the len and
    // pos argument pairs are aligned.
    #[cfg(any(target_arch = "arm", target_arch = "powerpc"))]
    unsafe {
        ret(syscall6_readonly(
            nr(__NR_fadvise64_64),
            borrowed_fd(fd),
            c_uint(advice as c_uint),
            hi(pos),
            lo(pos),
            hi(len),
            lo(len),
        ))
    }
    #[cfg(all(
        target_pointer_width = "32",
        not(any(target_arch = "arm", target_arch = "powerpc"))
    ))]
    unsafe {
        ret(syscall6_readonly(
            nr(__NR_fadvise64_64),
            borrowed_fd(fd),
            hi(pos),
            lo(pos),
            hi(len),
            lo(len),
            c_uint(advice as c_uint),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret(syscall4_readonly(
            nr(__NR_fadvise64),
            borrowed_fd(fd),
            loff_t_from_u64(pos),
            loff_t_from_u64(len),
            c_uint(advice as c_uint),
        ))
    }
}

#[inline]
pub(crate) fn madvise(addr: *mut c_void, len: usize, advice: IoAdvice) -> io::Result<()> {
    unsafe {
        ret(syscall3(
            nr(__NR_madvise),
            void_star(addr),
            pass_usize(len),
            c_uint(advice as c_uint),
        ))
    }
}

#[inline]
pub(crate) fn fsync(fd: BorrowedFd<'_>) -> io::Result<()> {
    unsafe { ret(syscall1_readonly(nr(__NR_fsync), borrowed_fd(fd))) }
}

#[inline]
pub(crate) fn fdatasync(fd: BorrowedFd<'_>) -> io::Result<()> {
    unsafe { ret(syscall1_readonly(nr(__NR_fdatasync), borrowed_fd(fd))) }
}

#[inline]
pub(crate) fn flock(fd: BorrowedFd<'_>, operation: FlockOperation) -> io::Result<()> {
    unsafe {
        ret(syscall2(
            nr(__NR_flock),
            borrowed_fd(fd),
            c_uint(operation as c_uint),
        ))
    }
}

#[inline]
pub(crate) fn fstat(fd: BorrowedFd<'_>) -> io::Result<Stat> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let mut result = MaybeUninit::<Stat>::uninit();
        ret(syscall2(
            nr(__NR_fstat64),
            borrowed_fd(fd),
            out(&mut result),
        ))
        .map(|()| result.assume_init())
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        let mut result = MaybeUninit::<Stat>::uninit();
        ret(syscall2(nr(__NR_fstat), borrowed_fd(fd), out(&mut result)))
            .map(|()| result.assume_init())
    }
}

#[inline]
pub(crate) fn stat(filename: &CStr) -> io::Result<Stat> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let mut result = MaybeUninit::<Stat>::uninit();
        ret(syscall4(
            nr(__NR_fstatat64),
            c_int(AT_FDCWD),
            c_str(filename),
            out(&mut result),
            c_uint(0),
        ))
        .map(|()| result.assume_init())
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        let mut result = MaybeUninit::<Stat>::uninit();
        ret(syscall4(
            nr(__NR_newfstatat),
            c_int(AT_FDCWD),
            c_str(filename),
            out(&mut result),
            c_uint(0),
        ))
        .map(|()| result.assume_init())
    }
}

#[inline]
pub(crate) fn statat(dirfd: BorrowedFd<'_>, filename: &CStr, flags: AtFlags) -> io::Result<Stat> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let mut result = MaybeUninit::<Stat>::uninit();
        ret(syscall4(
            nr(__NR_fstatat64),
            borrowed_fd(dirfd),
            c_str(filename),
            out(&mut result),
            c_uint(flags.bits()),
        ))
        .map(|()| result.assume_init())
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        let mut result = MaybeUninit::<Stat>::uninit();
        ret(syscall4(
            nr(__NR_newfstatat),
            borrowed_fd(dirfd),
            c_str(filename),
            out(&mut result),
            c_uint(flags.bits()),
        ))
        .map(|()| result.assume_init())
    }
}

#[inline]
pub(crate) fn lstat(filename: &CStr) -> io::Result<Stat> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let mut result = MaybeUninit::<Stat>::uninit();
        ret(syscall4(
            nr(__NR_fstatat64),
            c_int(AT_FDCWD),
            c_str(filename),
            out(&mut result),
            c_uint(AT_SYMLINK_NOFOLLOW),
        ))
        .map(|()| result.assume_init())
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        let mut result = MaybeUninit::<Stat>::uninit();
        ret(syscall4(
            nr(__NR_newfstatat),
            c_int(AT_FDCWD),
            c_str(filename),
            out(&mut result),
            c_uint(AT_SYMLINK_NOFOLLOW),
        ))
        .map(|()| result.assume_init())
    }
}

#[inline]
pub(crate) fn statx(
    dirfd: BorrowedFd<'_>,
    pathname: &CStr,
    flags: AtFlags,
    mask: StatxFlags,
) -> io::Result<statx> {
    unsafe {
        let mut statx_buf = MaybeUninit::<statx>::uninit();
        ret(syscall5(
            nr(__NR_statx),
            borrowed_fd(dirfd),
            c_str(pathname),
            c_uint(flags.bits()),
            c_uint(mask.bits()),
            out(&mut statx_buf),
        ))
        .map(|()| statx_buf.assume_init())
    }
}

#[inline]
pub(crate) fn fstatfs(fd: BorrowedFd<'_>) -> io::Result<StatFs> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let mut result = MaybeUninit::<StatFs>::uninit();
        ret(syscall3(
            nr(__NR_fstatfs64),
            borrowed_fd(fd),
            size_of::<StatFs, _>(),
            out(&mut result),
        ))
        .map(|()| result.assume_init())
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        let mut result = MaybeUninit::<StatFs>::uninit();
        ret(syscall2(
            nr(__NR_fstatfs),
            borrowed_fd(fd),
            out(&mut result),
        ))
        .map(|()| result.assume_init())
    }
}

#[inline]
pub(crate) fn readlink(path: &CStr, buf: &mut [u8]) -> io::Result<usize> {
    let (buf_addr_mut, buf_len) = slice_mut(buf);
    unsafe {
        ret_usize(syscall4_readonly(
            nr(__NR_readlinkat),
            c_int(AT_FDCWD),
            c_str(path),
            buf_addr_mut,
            buf_len,
        ))
    }
}

#[inline]
pub(crate) fn readlinkat(dirfd: BorrowedFd<'_>, path: &CStr, buf: &mut [u8]) -> io::Result<usize> {
    let (buf_addr_mut, buf_len) = slice_mut(buf);
    unsafe {
        ret_usize(syscall4_readonly(
            nr(__NR_readlinkat),
            borrowed_fd(dirfd),
            c_str(path),
            buf_addr_mut,
            buf_len,
        ))
    }
}

#[inline]
pub(crate) fn fcntl_dupfd(fd: BorrowedFd<'_>, min: RawFd) -> io::Result<OwnedFd> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_owned_fd(syscall3_readonly(
            nr(__NR_fcntl64),
            borrowed_fd(fd),
            c_uint(F_DUPFD),
            raw_fd(min),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_owned_fd(syscall3_readonly(
            nr(__NR_fcntl),
            borrowed_fd(fd),
            c_uint(F_DUPFD),
            raw_fd(min),
        ))
    }
}

#[inline]
pub(crate) fn fcntl_dupfd_cloexec(fd: BorrowedFd<'_>, min: RawFd) -> io::Result<OwnedFd> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_owned_fd(syscall3_readonly(
            nr(__NR_fcntl64),
            borrowed_fd(fd),
            c_uint(F_DUPFD_CLOEXEC),
            raw_fd(min),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_owned_fd(syscall3_readonly(
            nr(__NR_fcntl),
            borrowed_fd(fd),
            c_uint(F_DUPFD_CLOEXEC),
            raw_fd(min),
        ))
    }
}

#[inline]
pub(crate) fn fcntl_getfd(fd: BorrowedFd<'_>) -> io::Result<FdFlags> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_c_uint(syscall2_readonly(
            nr(__NR_fcntl64),
            borrowed_fd(fd),
            c_uint(F_GETFD),
        ))
        .map(FdFlags::from_bits_truncate)
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_c_uint(syscall2_readonly(
            nr(__NR_fcntl),
            borrowed_fd(fd),
            c_uint(F_GETFD),
        ))
        .map(FdFlags::from_bits_truncate)
    }
}

#[inline]
pub(crate) fn fcntl_setfd(fd: BorrowedFd<'_>, flags: FdFlags) -> io::Result<()> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_fcntl64),
            borrowed_fd(fd),
            c_uint(F_SETFD),
            c_uint(flags.bits()),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_fcntl),
            borrowed_fd(fd),
            c_uint(F_SETFD),
            c_uint(flags.bits()),
        ))
    }
}

#[inline]
pub(crate) fn fcntl_getfl(fd: BorrowedFd<'_>) -> io::Result<OFlags> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_c_uint(syscall2_readonly(
            nr(__NR_fcntl64),
            borrowed_fd(fd),
            c_uint(F_GETFL),
        ))
        .map(OFlags::from_bits_truncate)
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_c_uint(syscall2_readonly(
            nr(__NR_fcntl),
            borrowed_fd(fd),
            c_uint(F_GETFL),
        ))
        .map(OFlags::from_bits_truncate)
    }
}

#[inline]
pub(crate) fn fcntl_setfl(fd: BorrowedFd<'_>, flags: OFlags) -> io::Result<()> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_fcntl64),
            borrowed_fd(fd),
            c_uint(F_SETFL),
            oflags(flags),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_fcntl),
            borrowed_fd(fd),
            c_uint(F_SETFL),
            oflags(flags),
        ))
    }
}

#[inline]
pub(crate) fn fcntl_getlease(fd: BorrowedFd<'_>) -> io::Result<c_int> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_c_int(syscall2_readonly(
            nr(__NR_fcntl64),
            borrowed_fd(fd),
            c_uint(F_GETLEASE),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_c_int(syscall2_readonly(
            nr(__NR_fcntl),
            borrowed_fd(fd),
            c_uint(F_GETLEASE),
        ))
    }
}

#[inline]
pub(crate) fn fcntl_getown(fd: BorrowedFd<'_>) -> io::Result<c_int> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_c_int(syscall2_readonly(
            nr(__NR_fcntl64),
            borrowed_fd(fd),
            c_uint(F_GETOWN),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_c_int(syscall2_readonly(
            nr(__NR_fcntl),
            borrowed_fd(fd),
            c_uint(F_GETOWN),
        ))
    }
}

#[inline]
pub(crate) fn fcntl_getsig(fd: BorrowedFd<'_>) -> io::Result<c_int> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_c_int(syscall2_readonly(
            nr(__NR_fcntl64),
            borrowed_fd(fd),
            c_uint(F_GETSIG),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_c_int(syscall2_readonly(
            nr(__NR_fcntl),
            borrowed_fd(fd),
            c_uint(F_GETSIG),
        ))
    }
}

#[inline]
pub(crate) fn fcntl_getpipe_sz(fd: BorrowedFd<'_>) -> io::Result<usize> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_usize(syscall2_readonly(
            nr(__NR_fcntl64),
            borrowed_fd(fd),
            c_uint(F_GETPIPE_SZ),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_usize(syscall2_readonly(
            nr(__NR_fcntl),
            borrowed_fd(fd),
            c_uint(F_GETPIPE_SZ),
        ))
    }
}

#[inline]
pub(crate) fn fcntl_setpipe_sz(fd: BorrowedFd<'_>, size: c_int) -> io::Result<usize> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_usize(syscall3_readonly(
            nr(__NR_fcntl64),
            borrowed_fd(fd),
            c_uint(F_SETPIPE_SZ),
            c_int(size),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_usize(syscall3_readonly(
            nr(__NR_fcntl),
            borrowed_fd(fd),
            c_uint(F_SETPIPE_SZ),
            c_int(size),
        ))
    }
}

#[inline]
pub(crate) fn fcntl_get_seals(fd: BorrowedFd<'_>) -> io::Result<u32> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_c_int(syscall2_readonly(
            nr(__NR_fcntl64),
            borrowed_fd(fd),
            c_uint(F_GET_SEALS),
        ))
        .map(|seals| seals as u32)
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_c_int(syscall2_readonly(
            nr(__NR_fcntl),
            borrowed_fd(fd),
            c_uint(F_GET_SEALS),
        ))
        .map(|seals| seals as u32)
    }
}

#[inline]
pub(crate) fn rename(oldname: &CStr, newname: &CStr) -> io::Result<()> {
    #[cfg(target_arch = "riscv64")]
    unsafe {
        ret(syscall5_readonly(
            nr(__NR_renameat2),
            c_int(AT_FDCWD),
            c_str(oldname),
            c_int(AT_FDCWD),
            c_str(newname),
            c_uint(0),
        ))
    }
    #[cfg(not(target_arch = "riscv64"))]
    unsafe {
        ret(syscall4_readonly(
            nr(__NR_renameat),
            c_int(AT_FDCWD),
            c_str(oldname),
            c_int(AT_FDCWD),
            c_str(newname),
        ))
    }
}

#[inline]
pub(crate) fn renameat(
    old_dirfd: BorrowedFd<'_>,
    oldname: &CStr,
    new_dirfd: BorrowedFd<'_>,
    newname: &CStr,
) -> io::Result<()> {
    #[cfg(target_arch = "riscv64")]
    unsafe {
        ret(syscall5_readonly(
            nr(__NR_renameat2),
            borrowed_fd(old_dirfd),
            c_str(oldname),
            borrowed_fd(new_dirfd),
            c_str(newname),
            c_uint(0),
        ))
    }
    #[cfg(not(target_arch = "riscv64"))]
    unsafe {
        ret(syscall4_readonly(
            nr(__NR_renameat),
            borrowed_fd(old_dirfd),
            c_str(oldname),
            borrowed_fd(new_dirfd),
            c_str(newname),
        ))
    }
}

#[inline]
pub(crate) fn renameat2(
    old_dirfd: BorrowedFd<'_>,
    oldname: &CStr,
    new_dirfd: BorrowedFd<'_>,
    newname: &CStr,
    flags: RenameFlags,
) -> io::Result<()> {
    unsafe {
        ret(syscall5_readonly(
            nr(__NR_renameat2),
            borrowed_fd(old_dirfd),
            c_str(oldname),
            borrowed_fd(new_dirfd),
            c_str(newname),
            c_uint(flags.bits()),
        ))
    }
}

#[inline]
pub(crate) fn unlink(pathname: &CStr) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_unlinkat),
            c_int(AT_FDCWD),
            c_str(pathname),
            c_uint(0),
        ))
    }
}

#[inline]
pub(crate) fn unlinkat(dirfd: BorrowedFd<'_>, pathname: &CStr, flags: AtFlags) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_unlinkat),
            borrowed_fd(dirfd),
            c_str(pathname),
            c_uint(flags.bits()),
        ))
    }
}

#[inline]
pub(crate) fn rmdir(pathname: &CStr) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_unlinkat),
            c_int(AT_FDCWD),
            c_str(pathname),
            c_uint(AT_REMOVEDIR),
        ))
    }
}

#[inline]
pub(crate) fn link(oldname: &CStr, newname: &CStr) -> io::Result<()> {
    unsafe {
        ret(syscall5_readonly(
            nr(__NR_linkat),
            c_int(AT_FDCWD),
            c_str(oldname),
            c_int(AT_FDCWD),
            c_str(newname),
            c_uint(0),
        ))
    }
}

#[inline]
pub(crate) fn linkat(
    old_dirfd: BorrowedFd<'_>,
    oldname: &CStr,
    new_dirfd: BorrowedFd<'_>,
    newname: &CStr,
    flags: AtFlags,
) -> io::Result<()> {
    unsafe {
        ret(syscall5_readonly(
            nr(__NR_linkat),
            borrowed_fd(old_dirfd),
            c_str(oldname),
            borrowed_fd(new_dirfd),
            c_str(newname),
            c_uint(flags.bits()),
        ))
    }
}

#[inline]
pub(crate) fn symlink(oldname: &CStr, newname: &CStr) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_symlinkat),
            c_str(oldname),
            c_int(AT_FDCWD),
            c_str(newname),
        ))
    }
}

#[inline]
pub(crate) fn symlinkat(oldname: &CStr, dirfd: BorrowedFd<'_>, newname: &CStr) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_symlinkat),
            c_str(oldname),
            borrowed_fd(dirfd),
            c_str(newname),
        ))
    }
}

#[inline]
pub(crate) fn mkdir(pathname: &CStr, mode: Mode) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_mkdirat),
            c_int(AT_FDCWD),
            c_str(pathname),
            mode_as(mode),
        ))
    }
}

#[inline]
pub(crate) fn mkdirat(dirfd: BorrowedFd<'_>, pathname: &CStr, mode: Mode) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_mkdirat),
            borrowed_fd(dirfd),
            c_str(pathname),
            mode_as(mode),
        ))
    }
}

#[inline]
pub(crate) fn getdents(fd: BorrowedFd<'_>, dirent: &mut [u8]) -> io::Result<usize> {
    let (dirent_addr_mut, dirent_len) = slice_mut(dirent);

    unsafe {
        ret_usize(syscall3(
            nr(__NR_getdents64),
            borrowed_fd(fd),
            dirent_addr_mut,
            dirent_len,
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

#[inline]
pub(crate) fn socket(
    family: AddressFamily,
    type_: SocketType,
    protocol: Protocol,
) -> io::Result<OwnedFd> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret_owned_fd(syscall3_readonly(
            nr(__NR_socket),
            c_uint(family.0.into()),
            c_uint(type_.0),
            c_uint(protocol.0),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_owned_fd(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_SOCKET),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                c_uint(family.0.into()),
                c_uint(type_.0),
                c_uint(protocol.0),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn socket_with(
    family: AddressFamily,
    type_: SocketType,
    flags: SocketFlags,
    protocol: Protocol,
) -> io::Result<OwnedFd> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret_owned_fd(syscall3_readonly(
            nr(__NR_socket),
            c_uint(family.0.into()),
            c_uint(type_.0 | flags.bits()),
            c_uint(protocol.0),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_owned_fd(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_SOCKET),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                c_uint(family.0.into()),
                c_uint(type_.0 | flags.bits()),
                c_uint(protocol.0),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn socketpair(
    family: AddressFamily,
    type_: SocketType,
    flags: SocketFlags,
    protocol: Protocol,
) -> io::Result<(OwnedFd, OwnedFd)> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut result = MaybeUninit::<[OwnedFd; 2]>::uninit();
        ret(syscall4(
            nr(__NR_socketpair),
            c_uint(family.0.into()),
            c_uint(type_.0 | flags.bits()),
            c_uint(protocol.0),
            out(&mut result),
        ))
        .map(|()| {
            let [fd0, fd1] = result.assume_init();
            (fd0, fd1)
        })
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut result = MaybeUninit::<[OwnedFd; 2]>::uninit();
        ret(syscall2(
            nr(__NR_socketcall),
            x86_sys(SYS_SOCKETPAIR),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                c_uint(family.0.into()),
                c_uint(type_.0 | flags.bits()),
                c_uint(protocol.0),
                out(&mut result),
            ]),
        ))
        .map(|()| {
            let [fd0, fd1] = result.assume_init();
            (fd0, fd1)
        })
    }
}

#[inline]
pub(crate) fn accept(fd: BorrowedFd<'_>) -> io::Result<OwnedFd> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let fd = ret_owned_fd(syscall3(nr(__NR_accept), borrowed_fd(fd), zero(), zero()))?;
        Ok(fd)
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let fd = ret_owned_fd(syscall2(
            nr(__NR_socketcall),
            x86_sys(SYS_ACCEPT),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[borrowed_fd(fd), zero(), zero()]),
        ))?;
        Ok(fd)
    }
}

#[inline]
pub(crate) fn accept_with(fd: BorrowedFd<'_>, flags: AcceptFlags) -> io::Result<OwnedFd> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let fd = ret_owned_fd(syscall4(
            nr(__NR_accept4),
            borrowed_fd(fd),
            zero(),
            zero(),
            c_uint(flags.bits()),
        ))?;
        Ok(fd)
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let fd = ret_owned_fd(syscall2(
            nr(__NR_socketcall),
            x86_sys(SYS_ACCEPT4),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                zero(),
                zero(),
                c_uint(flags.bits()),
            ]),
        ))?;
        Ok(fd)
    }
}

#[inline]
pub(crate) fn acceptfrom(fd: BorrowedFd<'_>) -> io::Result<(OwnedFd, SocketAddr)> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut addrlen = std::mem::size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        let fd = ret_owned_fd(syscall3(
            nr(__NR_accept),
            borrowed_fd(fd),
            out(&mut storage),
            by_mut(&mut addrlen),
        ))?;
        Ok((
            fd,
            read_sockaddr_os(&storage.assume_init(), addrlen.try_into().unwrap()),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut addrlen = std::mem::size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        let fd = ret_owned_fd(syscall2(
            nr(__NR_socketcall),
            x86_sys(SYS_ACCEPT),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                out(&mut storage),
                by_mut(&mut addrlen),
            ]),
        ))?;
        Ok((
            fd,
            read_sockaddr_os(&storage.assume_init(), addrlen.try_into().unwrap()),
        ))
    }
}

#[inline]
pub(crate) fn acceptfrom_with(
    fd: BorrowedFd<'_>,
    flags: AcceptFlags,
) -> io::Result<(OwnedFd, SocketAddr)> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut addrlen = std::mem::size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        let fd = ret_owned_fd(syscall4(
            nr(__NR_accept4),
            borrowed_fd(fd),
            out(&mut storage),
            by_mut(&mut addrlen),
            c_uint(flags.bits()),
        ))?;
        Ok((
            fd,
            read_sockaddr_os(&storage.assume_init(), addrlen.try_into().unwrap()),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut addrlen = std::mem::size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        let fd = ret_owned_fd(syscall2(
            nr(__NR_socketcall),
            x86_sys(SYS_ACCEPT4),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                out(&mut storage),
                by_mut(&mut addrlen),
                c_uint(flags.bits()),
            ]),
        ))?;
        Ok((
            fd,
            read_sockaddr_os(&storage.assume_init(), addrlen.try_into().unwrap()),
        ))
    }
}

#[inline]
pub(crate) fn shutdown(fd: BorrowedFd<'_>, how: Shutdown) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall2(
            nr(__NR_shutdown),
            borrowed_fd(fd),
            c_uint(how as c_uint),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_SHUTDOWN),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[borrowed_fd(fd), c_uint(how as c_uint)]),
        ))
    }
}

#[inline]
pub(crate) fn send(fd: BorrowedFd<'_>, buf: &[u8], flags: SendFlags) -> io::Result<usize> {
    let (buf_addr, buf_len) = slice(buf);

    #[cfg(not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64"
    )))]
    unsafe {
        ret_usize(syscall4_readonly(
            nr(__NR_send),
            borrowed_fd(fd),
            buf_addr,
            buf_len,
            c_uint(flags.bits()),
        ))
    }
    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64"
    ))]
    unsafe {
        ret_usize(syscall6_readonly(
            nr(__NR_sendto),
            borrowed_fd(fd),
            buf_addr,
            buf_len,
            c_uint(flags.bits()),
            zero(),
            zero(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_usize(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_SEND),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                buf_addr,
                buf_len,
                c_uint(flags.bits()),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn sendto_v4(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrV4,
) -> io::Result<usize> {
    let (buf_addr, buf_len) = slice(buf);

    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret_usize(syscall6_readonly(
            nr(__NR_sendto),
            borrowed_fd(fd),
            buf_addr,
            buf_len,
            c_uint(flags.bits()),
            by_ref(&encode_sockaddr_v4(addr)),
            size_of::<sockaddr_in, _>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_usize(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_SENDTO),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                buf_addr,
                buf_len,
                c_uint(flags.bits()),
                by_ref(&encode_sockaddr_v4(addr)),
                size_of::<sockaddr_in, _>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn sendto_v6(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrV6,
) -> io::Result<usize> {
    let (buf_addr, buf_len) = slice(buf);

    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret_usize(syscall6_readonly(
            nr(__NR_sendto),
            borrowed_fd(fd),
            buf_addr,
            buf_len,
            c_uint(flags.bits()),
            by_ref(&encode_sockaddr_v6(addr)),
            size_of::<sockaddr_in6, _>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_usize(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_SENDTO),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                buf_addr,
                buf_len,
                c_uint(flags.bits()),
                by_ref(&encode_sockaddr_v6(addr)),
                size_of::<sockaddr_in6, _>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn sendto_unix(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrUnix,
) -> io::Result<usize> {
    let (buf_addr, buf_len) = slice(buf);

    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret_usize(syscall6_readonly(
            nr(__NR_sendto),
            borrowed_fd(fd),
            buf_addr,
            buf_len,
            c_uint(flags.bits()),
            by_ref(&encode_sockaddr_unix(addr)),
            size_of::<sockaddr_un, _>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_usize(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_SENDTO),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                buf_addr,
                buf_len,
                c_uint(flags.bits()),
                by_ref(&encode_sockaddr_unix(addr)),
                size_of::<sockaddr_un, _>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn recv(fd: BorrowedFd<'_>, buf: &mut [u8], flags: RecvFlags) -> io::Result<usize> {
    let (buf_addr_mut, buf_len) = slice_mut(buf);

    #[cfg(not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64"
    )))]
    unsafe {
        ret_usize(syscall4(
            nr(__NR_recv),
            borrowed_fd(fd),
            buf_addr_mut,
            buf_len,
            c_uint(flags.bits()),
        ))
    }
    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64"
    ))]
    unsafe {
        ret_usize(syscall6(
            nr(__NR_recvfrom),
            borrowed_fd(fd),
            buf_addr_mut,
            buf_len,
            c_uint(flags.bits()),
            zero(),
            zero(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_usize(syscall2(
            nr(__NR_socketcall),
            x86_sys(SYS_RECV),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                buf_addr_mut,
                buf_len,
                c_uint(flags.bits()),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn recvfrom(
    fd: BorrowedFd<'_>,
    buf: &mut [u8],
    flags: RecvFlags,
) -> io::Result<(usize, SocketAddr)> {
    let (buf_addr_mut, buf_len) = slice_mut(buf);

    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut addrlen = std::mem::size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        let nread = ret_usize(syscall6(
            nr(__NR_recvfrom),
            borrowed_fd(fd),
            buf_addr_mut,
            buf_len,
            c_uint(flags.bits()),
            out(&mut storage),
            by_mut(&mut addrlen),
        ))?;
        Ok((
            nread,
            read_sockaddr_os(&storage.assume_init(), addrlen.try_into().unwrap()),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut addrlen = std::mem::size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        let nread = ret_usize(syscall2(
            nr(__NR_socketcall),
            x86_sys(SYS_RECVFROM),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                buf_addr_mut,
                buf_len,
                c_uint(flags.bits()),
                out(&mut storage),
                by_mut(&mut addrlen),
            ]),
        ))?;
        Ok((
            nread,
            read_sockaddr_os(&storage.assume_init(), addrlen.try_into().unwrap()),
        ))
    }
}

#[inline]
pub(crate) fn getpeername(fd: BorrowedFd<'_>) -> io::Result<SocketAddr> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut addrlen = std::mem::size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        ret(syscall3(
            nr(__NR_getpeername),
            borrowed_fd(fd),
            out(&mut storage),
            by_mut(&mut addrlen),
        ))?;
        Ok(read_sockaddr_os(
            &storage.assume_init(),
            addrlen.try_into().unwrap(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut addrlen = std::mem::size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        ret(syscall2(
            nr(__NR_socketcall),
            x86_sys(SYS_GETPEERNAME),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                out(&mut storage),
                by_mut(&mut addrlen),
            ]),
        ))?;
        Ok(read_sockaddr_os(
            &storage.assume_init(),
            addrlen.try_into().unwrap(),
        ))
    }
}

#[inline]
pub(crate) fn getsockname(fd: BorrowedFd<'_>) -> io::Result<SocketAddr> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut addrlen = std::mem::size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        ret(syscall3(
            nr(__NR_getsockname),
            borrowed_fd(fd),
            out(&mut storage),
            by_mut(&mut addrlen),
        ))?;
        Ok(read_sockaddr_os(
            &storage.assume_init(),
            addrlen.try_into().unwrap(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut addrlen = std::mem::size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        ret(syscall2(
            nr(__NR_socketcall),
            x86_sys(SYS_GETSOCKNAME),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                out(&mut storage),
                by_mut(&mut addrlen),
            ]),
        ))?;
        Ok(read_sockaddr_os(
            &storage.assume_init(),
            addrlen.try_into().unwrap(),
        ))
    }
}

#[inline]
pub(crate) fn bind_v4(fd: BorrowedFd<'_>, addr: &SocketAddrV4) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_bind),
            borrowed_fd(fd),
            by_ref(&encode_sockaddr_v4(addr)),
            size_of::<sockaddr_in, _>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_BIND),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                by_ref(&encode_sockaddr_v4(addr)),
                size_of::<sockaddr_in, _>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn bind_v6(fd: BorrowedFd<'_>, addr: &SocketAddrV6) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_bind),
            borrowed_fd(fd),
            by_ref(&encode_sockaddr_v6(addr)),
            size_of::<sockaddr_in6, _>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_BIND),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                by_ref(&encode_sockaddr_v6(addr)),
                size_of::<sockaddr_in6, _>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn bind_unix(fd: BorrowedFd<'_>, addr: &SocketAddrUnix) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_bind),
            borrowed_fd(fd),
            by_ref(&encode_sockaddr_unix(addr)),
            size_of::<sockaddr_un, _>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_BIND),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                by_ref(&encode_sockaddr_unix(addr)),
                size_of::<sockaddr_un, _>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn connect_v4(fd: BorrowedFd<'_>, addr: &SocketAddrV4) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_connect),
            borrowed_fd(fd),
            by_ref(&encode_sockaddr_v4(addr)),
            size_of::<sockaddr_in, _>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_CONNECT),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                by_ref(&encode_sockaddr_v4(addr)),
                size_of::<sockaddr_in, _>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn connect_v6(fd: BorrowedFd<'_>, addr: &SocketAddrV6) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_connect),
            borrowed_fd(fd),
            by_ref(&encode_sockaddr_v6(addr)),
            size_of::<sockaddr_in6, _>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_CONNECT),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                by_ref(&encode_sockaddr_v6(addr)),
                size_of::<sockaddr_in6, _>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn connect_unix(fd: BorrowedFd<'_>, addr: &SocketAddrUnix) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_connect),
            borrowed_fd(fd),
            by_ref(&encode_sockaddr_unix(addr)),
            size_of::<sockaddr_un, _>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_CONNECT),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                by_ref(&encode_sockaddr_unix(addr)),
                size_of::<sockaddr_un, _>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn listen(fd: BorrowedFd<'_>, backlog: c_int) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall2_readonly(
            nr(__NR_listen),
            borrowed_fd(fd),
            c_int(backlog),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_LISTEN),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[borrowed_fd(fd), c_int(backlog)]),
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
    addr: *mut c_void,
    length: usize,
    prot: ProtFlags,
    flags: MapFlags,
    fd: BorrowedFd<'_>,
    offset: u64,
) -> io::Result<*mut c_void> {
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
    addr: *mut c_void,
    length: usize,
    prot: ProtFlags,
    flags: MapFlags,
) -> io::Result<*mut c_void> {
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
    ptr: *mut c_void,
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
pub(crate) unsafe fn munmap(addr: *mut c_void, length: usize) -> io::Result<()> {
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
    old_address: *mut c_void,
    old_size: usize,
    new_size: usize,
    flags: MremapFlags,
) -> io::Result<*mut c_void> {
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
    old_address: *mut c_void,
    old_size: usize,
    new_size: usize,
    flags: MremapFlags,
    new_address: *mut c_void,
) -> io::Result<*mut c_void> {
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
pub(crate) unsafe fn mlock(addr: *mut c_void, length: usize) -> io::Result<()> {
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
    addr: *mut c_void,
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
pub(crate) unsafe fn munlock(addr: *mut c_void, length: usize) -> io::Result<()> {
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
            c_int(cmd as c_int),
            c_uint(0),
        ))
    }
}

pub(crate) fn membarrier_cpu(cmd: MembarrierCommand, cpu: Cpuid) -> io::Result<()> {
    unsafe {
        ret(syscall3(
            nr(__NR_membarrier),
            c_int(cmd as c_int),
            c_uint(
                linux_raw_sys::v5_11::general::membarrier_cmd_flag::MEMBARRIER_CMD_FLAG_CPU as _,
            ),
            c_uint(cpu.as_raw()),
        ))
    }
}

#[inline]
pub(crate) fn utimensat(
    dirfd: BorrowedFd<'_>,
    pathname: &CStr,
    utimes: &[__kernel_timespec; 2],
    flags: AtFlags,
) -> io::Result<()> {
    _utimensat(dirfd, Some(pathname), utimes, flags)
}

#[inline]
fn _utimensat(
    dirfd: BorrowedFd<'_>,
    pathname: Option<&CStr>,
    utimes: &[__kernel_timespec; 2],
    flags: AtFlags,
) -> io::Result<()> {
    // The length of the array is fixed and not passed into the syscall.
    let utimes_addr = slice_just_addr(utimes);

    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret(syscall4_readonly(
            nr(__NR_utimensat_time64),
            borrowed_fd(dirfd),
            opt_c_str(pathname),
            utimes_addr,
            c_uint(flags.bits()),
        ))
        .or_else(|err| {
            // See the comments in `rsix_clock_gettime_via_syscall` about
            // emulation.
            if err == io::Error::NOSYS {
                let old_utimes = [
                    __kernel_old_timespec {
                        tv_sec: utimes[0].tv_sec.try_into().map_err(|_| io::Error::INVAL)?,
                        tv_nsec: utimes[0].tv_nsec.try_into().map_err(|_| io::Error::INVAL)?,
                    },
                    __kernel_old_timespec {
                        tv_sec: utimes[1].tv_sec.try_into().map_err(|_| io::Error::INVAL)?,
                        tv_nsec: utimes[1].tv_nsec.try_into().map_err(|_| io::Error::INVAL)?,
                    },
                ];
                // The length of the array is fixed and not passed into the syscall.
                let old_utimes_addr = slice_just_addr(&old_utimes);
                ret(syscall4_readonly(
                    nr(__NR_utimensat),
                    borrowed_fd(dirfd),
                    opt_c_str(pathname),
                    old_utimes_addr,
                    c_uint(flags.bits()),
                ))
            } else {
                Err(err)
            }
        })
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret(syscall4_readonly(
            nr(__NR_utimensat),
            borrowed_fd(dirfd),
            opt_c_str(pathname),
            utimes_addr,
            c_uint(flags.bits()),
        ))
    }
}

#[inline]
pub(crate) fn futimens(fd: BorrowedFd<'_>, times: &[Timespec; 2]) -> io::Result<()> {
    _utimensat(fd, None, times, AtFlags::empty())
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
pub(crate) fn chdir(filename: &CStr) -> io::Result<()> {
    unsafe { ret(syscall1_readonly(nr(__NR_chdir), c_str(filename))) }
}

#[inline]
pub(crate) fn fchdir(fd: BorrowedFd) -> io::Result<()> {
    unsafe { ret(syscall1_readonly(nr(__NR_fchdir), borrowed_fd(fd))) }
}

#[inline]
pub(crate) fn ioctl_fionread(fd: BorrowedFd) -> io::Result<u64> {
    unsafe {
        let mut result = MaybeUninit::<c_int>::uninit();
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
        let data = value as c_int;
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

pub(crate) fn accessat(
    dirfd: BorrowedFd<'_>,
    path: &CStr,
    access: Access,
    flags: AtFlags,
) -> io::Result<()> {
    if flags.is_empty()
        || (flags.bits() == linux_raw_sys::v5_11::general::AT_EACCESS
            && getuid() == geteuid()
            && getgid() == getegid())
    {
        return _accessat(dirfd, path, access.bits());
    }

    if flags.bits() != linux_raw_sys::v5_11::general::AT_EACCESS {
        return Err(io::Error::INVAL);
    }

    // TODO: Use faccessat2 in newer Linux versions.
    Err(io::Error::NOSYS)
}

#[inline]
fn _accessat(dirfd: BorrowedFd<'_>, pathname: &CStr, mode: c_uint) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_faccessat),
            borrowed_fd(dirfd),
            c_str(pathname),
            c_uint(mode),
        ))
    }
}

#[inline]
pub(crate) fn copy_file_range(
    fd_in: BorrowedFd<'_>,
    off_in: Option<&mut u64>,
    fd_out: BorrowedFd<'_>,
    off_out: Option<&mut u64>,
    len: u64,
) -> io::Result<u64> {
    let len: usize = len.try_into().unwrap_or(usize::MAX);
    _copy_file_range(fd_in, off_in, fd_out, off_out, len, 0).map(|result| result as u64)
}

#[inline]
fn _copy_file_range(
    fd_in: BorrowedFd<'_>,
    off_in: Option<&mut u64>,
    fd_out: BorrowedFd<'_>,
    off_out: Option<&mut u64>,
    len: usize,
    flags: c_uint,
) -> io::Result<usize> {
    unsafe {
        ret_usize(syscall6(
            nr(__NR_copy_file_range),
            borrowed_fd(fd_in),
            opt_mut(off_in),
            borrowed_fd(fd_out),
            opt_mut(off_out),
            pass_usize(len),
            c_uint(flags),
        ))
    }
}

#[inline]
pub(crate) fn poll(fds: &mut [PollFd<'_>], timeout: c_int) -> io::Result<usize> {
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
pub(crate) fn memfd_create(name: &CStr, flags: MemfdFlags) -> io::Result<OwnedFd> {
    unsafe {
        ret_owned_fd(syscall2(
            nr(__NR_memfd_create),
            c_str(name),
            c_uint(flags.bits()),
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
pub(crate) fn sendfile(
    out_fd: BorrowedFd<'_>,
    in_fd: BorrowedFd<'_>,
    offset: Option<&mut u64>,
    count: usize,
) -> io::Result<usize> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_usize(syscall4(
            nr(__NR_sendfile64),
            borrowed_fd(out_fd),
            borrowed_fd(in_fd),
            opt_mut(offset),
            pass_usize(count),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_usize(syscall4(
            nr(__NR_sendfile),
            borrowed_fd(out_fd),
            borrowed_fd(in_fd),
            opt_mut(offset),
            pass_usize(count),
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

#[inline]
pub(crate) fn ttyname(fd: BorrowedFd<'_>, buf: &mut [u8]) -> io::Result<()> {
    // Check that the fd is really a tty
    ioctl_tiocgwinsz(fd)?;

    // Get fd to '/proc/self/fd'
    let proc_self_fd = io::proc_self_fd()?;

    // Gatter the ttyname by reading the 'fd' file inside 'proc_self_fd'
    let r = readlinkat(proc_self_fd, DecInt::from_fd(&fd).as_c_str(), buf)?;

    // If the number of bytes is equal to the buffer length, truncation may
    // have occurred. This check also ensures that we have enough space for
    // adding a NUL terminator.
    if r == buf.len() {
        return Err(io::Error::RANGE);
    }
    buf[r] = 0;

    // Gatter the stat for the original fd and the newly gatter name
    let path = CStr::from_bytes_with_nul(&buf[..=r]).unwrap();
    let st1 = stat(path)?;
    let st2 = fstat(fd)?;

    // Finally check that the two stat(s) are equal
    if st1.st_dev != st2.st_dev || st1.st_ino != st2.st_ino {
        return Err(io::Error::NODEV);
    }

    Ok(())
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
    fd: c_int,
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
    fd: c_int,
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
pub(crate) unsafe fn epoll_del(epfd: BorrowedFd<'_>, fd: c_int) -> io::Result<()> {
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
    timeout: c_int,
) -> io::Result<usize> {
    #[cfg(not(any(target_arch = "aarch64", target_arch = "riscv64")))]
    unsafe {
        ret_usize(syscall4(
            nr(__NR_epoll_wait),
            borrowed_fd(epfd),
            void_star(events.cast::<c_void>()),
            pass_usize(num_events),
            c_int(timeout),
        ))
    }
    #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
    unsafe {
        ret_usize(syscall5(
            nr(__NR_epoll_pwait),
            borrowed_fd(epfd),
            void_star(events.cast::<c_void>()),
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
pub unsafe fn futex(
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
            c_uint(op as c_uint | flags.bits()),
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
                    c_uint(op as c_uint | flags.bits()),
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
        c_uint(op as c_uint | flags.bits()),
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
            c_uint(limit as c_uint),
            void_star(std::ptr::null_mut()),
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
                    c_uint(limit as c_uint),
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
            c_uint(limit as c_uint),
            void_star(std::ptr::null_mut()),
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

pub(crate) mod sockopt {
    use crate::io;
    use crate::net::sockopt::Timeout;
    use crate::net::{Ipv4Addr, Ipv6Addr, SocketType};
    use io_lifetimes::BorrowedFd;
    use std::convert::TryInto;
    use std::os::raw::{c_int, c_uint};
    use std::time::Duration;

    // TODO use Duration::ZERO when we don't need 1.51 support
    const DURATION_ZERO: Duration = Duration::from_secs(0);

    #[inline]
    fn getsockopt<T>(fd: BorrowedFd<'_>, level: u32, optname: u32) -> io::Result<T> {
        use super::*;
        #[cfg(not(target_arch = "x86"))]
        unsafe {
            let mut value = MaybeUninit::<T>::uninit();
            let mut optlen = std::mem::size_of::<T>();
            ret(syscall5(
                nr(__NR_getsockopt),
                borrowed_fd(fd),
                c_uint(level),
                c_uint(optname),
                out(&mut value),
                by_mut(&mut optlen),
            ))?;
            assert_eq!(
                optlen as usize,
                std::mem::size_of::<T>(),
                "unexpected getsockopt size"
            );
            Ok(value.assume_init())
        }
        #[cfg(target_arch = "x86")]
        unsafe {
            let mut value = MaybeUninit::<T>::uninit();
            let mut optlen = std::mem::size_of::<T>();
            ret(syscall2(
                nr(__NR_socketcall),
                x86_sys(SYS_GETSOCKOPT),
                slice_just_addr::<ArgReg<SocketArg>, _>(&[
                    borrowed_fd(fd),
                    c_uint(level),
                    c_uint(optname),
                    out(&mut value),
                    by_mut(&mut optlen),
                ]),
            ))?;
            assert_eq!(
                optlen as usize,
                std::mem::size_of::<T>(),
                "unexpected getsockopt size"
            );
            Ok(value.assume_init())
        }
    }

    #[inline]
    fn setsockopt<T>(fd: BorrowedFd<'_>, level: u32, optname: u32, value: T) -> io::Result<()> {
        use super::*;
        #[cfg(not(target_arch = "x86"))]
        unsafe {
            let optlen = std::mem::size_of::<T>().try_into().unwrap();
            ret(syscall5_readonly(
                nr(__NR_setsockopt),
                borrowed_fd(fd),
                c_uint(level),
                c_uint(optname),
                by_ref(&value),
                socklen_t(optlen),
            ))
        }
        #[cfg(target_arch = "x86")]
        unsafe {
            let optlen = std::mem::size_of::<T>().try_into().unwrap();
            ret(syscall2_readonly(
                nr(__NR_socketcall),
                x86_sys(SYS_SETSOCKOPT),
                slice_just_addr::<ArgReg<SocketArg>, _>(&[
                    borrowed_fd(fd),
                    c_uint(level),
                    c_uint(optname),
                    by_ref(&value),
                    socklen_t(optlen),
                ]),
            ))
        }
    }

    #[inline]
    pub(crate) fn get_socket_type(fd: BorrowedFd<'_>) -> io::Result<SocketType> {
        getsockopt(
            fd,
            linux_raw_sys::general::SOL_SOCKET as _,
            linux_raw_sys::general::SO_TYPE,
        )
    }

    #[inline]
    pub(crate) fn set_socket_reuseaddr(fd: BorrowedFd<'_>, reuseaddr: bool) -> io::Result<()> {
        setsockopt(
            fd,
            linux_raw_sys::general::SOL_SOCKET as _,
            linux_raw_sys::general::SO_REUSEADDR,
            from_bool(reuseaddr),
        )
    }

    #[inline]
    pub(crate) fn set_socket_broadcast(fd: BorrowedFd<'_>, broadcast: bool) -> io::Result<()> {
        setsockopt(
            fd,
            linux_raw_sys::general::SOL_SOCKET as _,
            linux_raw_sys::general::SO_BROADCAST,
            from_bool(broadcast),
        )
    }

    #[inline]
    pub(crate) fn get_socket_broadcast(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(
            fd,
            linux_raw_sys::general::SOL_SOCKET as _,
            linux_raw_sys::general::SO_BROADCAST,
        )
    }

    #[inline]
    pub(crate) fn set_socket_linger(
        fd: BorrowedFd<'_>,
        linger: Option<Duration>,
    ) -> io::Result<()> {
        let linger = linux_raw_sys::general::linger {
            l_onoff: linger.is_some() as c_int,
            l_linger: linger.unwrap_or_default().as_secs() as c_int,
        };
        setsockopt(
            fd,
            linux_raw_sys::general::SOL_SOCKET as _,
            linux_raw_sys::general::SO_LINGER,
            linger,
        )
    }

    #[inline]
    pub(crate) fn get_socket_linger(fd: BorrowedFd<'_>) -> io::Result<Option<Duration>> {
        let linger: linux_raw_sys::general::linger = getsockopt(
            fd,
            linux_raw_sys::general::SOL_SOCKET as _,
            linux_raw_sys::general::SO_LINGER,
        )?;
        Ok((linger.l_onoff != 0).then(|| Duration::from_secs(linger.l_linger as u64)))
    }

    #[inline]
    pub(crate) fn set_socket_passcred(fd: BorrowedFd<'_>, passcred: bool) -> io::Result<()> {
        setsockopt(
            fd,
            linux_raw_sys::general::SOL_SOCKET as _,
            linux_raw_sys::general::SO_PASSCRED,
            from_bool(passcred),
        )
    }

    #[inline]
    pub(crate) fn get_socket_passcred(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(
            fd,
            linux_raw_sys::general::SOL_SOCKET as _,
            linux_raw_sys::general::SO_PASSCRED,
        )
    }

    #[inline]
    pub(crate) fn set_socket_timeout(
        fd: BorrowedFd<'_>,
        id: Timeout,
        timeout: Option<Duration>,
    ) -> io::Result<()> {
        let timeout = match timeout {
            Some(timeout) => {
                if timeout == DURATION_ZERO {
                    return Err(io::Error::INVAL);
                }

                let mut timeout = linux_raw_sys::general::timeval {
                    tv_sec: timeout
                        .as_secs()
                        .try_into()
                        .unwrap_or(std::os::raw::c_long::MAX),
                    tv_usec: timeout.subsec_micros() as _,
                };
                if timeout.tv_sec == 0 && timeout.tv_usec == 0 {
                    timeout.tv_usec = 1;
                }
                timeout
            }
            None => linux_raw_sys::general::timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
        };
        let optname = match id {
            Timeout::Recv => linux_raw_sys::general::SO_RCVTIMEO,
            Timeout::Send => linux_raw_sys::general::SO_SNDTIMEO,
        };
        setsockopt(fd, linux_raw_sys::general::SOL_SOCKET, optname, timeout)
    }

    #[inline]
    pub(crate) fn get_socket_timeout(
        fd: BorrowedFd<'_>,
        id: Timeout,
    ) -> io::Result<Option<Duration>> {
        let optname = match id {
            Timeout::Recv => linux_raw_sys::general::SO_RCVTIMEO,
            Timeout::Send => linux_raw_sys::general::SO_SNDTIMEO,
        };
        let timeout: linux_raw_sys::general::timeval =
            getsockopt(fd, linux_raw_sys::general::SOL_SOCKET, optname)?;
        if timeout.tv_sec == 0 && timeout.tv_usec == 0 {
            Ok(None)
        } else {
            Ok(Some(
                Duration::from_secs(timeout.tv_sec as u64)
                    + Duration::from_micros(timeout.tv_usec as u64),
            ))
        }
    }

    #[inline]
    pub(crate) fn set_ip_ttl(fd: BorrowedFd<'_>, ttl: u32) -> io::Result<()> {
        setsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IP as _,
            linux_raw_sys::general::IP_TTL,
            ttl,
        )
    }

    #[inline]
    pub(crate) fn get_ip_ttl(fd: BorrowedFd<'_>) -> io::Result<u32> {
        getsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IP as _,
            linux_raw_sys::general::IP_TTL,
        )
    }

    #[inline]
    pub(crate) fn set_ipv6_v6only(fd: BorrowedFd<'_>, only_v6: bool) -> io::Result<()> {
        setsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IPV6 as _,
            linux_raw_sys::general::IPV6_V6ONLY,
            from_bool(only_v6),
        )
    }

    #[inline]
    pub(crate) fn get_ipv6_v6only(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IPV6 as _,
            linux_raw_sys::general::IPV6_V6ONLY,
        )
    }

    #[inline]
    pub(crate) fn set_ip_multicast_loop(
        fd: BorrowedFd<'_>,
        multicast_loop: bool,
    ) -> io::Result<()> {
        setsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IP as _,
            linux_raw_sys::general::IP_MULTICAST_LOOP,
            from_bool(multicast_loop),
        )
    }

    #[inline]
    pub(crate) fn get_ip_multicast_loop(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IP as _,
            linux_raw_sys::general::IP_MULTICAST_LOOP,
        )
    }

    #[inline]
    pub(crate) fn set_ip_multicast_ttl(fd: BorrowedFd<'_>, multicast_ttl: u32) -> io::Result<()> {
        setsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IP as _,
            linux_raw_sys::general::IP_MULTICAST_TTL,
            multicast_ttl,
        )
    }

    #[inline]
    pub(crate) fn get_ip_multicast_ttl(fd: BorrowedFd<'_>) -> io::Result<u32> {
        getsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IP as _,
            linux_raw_sys::general::IP_MULTICAST_TTL,
        )
    }

    #[inline]
    pub(crate) fn set_ipv6_multicast_loop(
        fd: BorrowedFd<'_>,
        multicast_loop: bool,
    ) -> io::Result<()> {
        setsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IPV6 as _,
            linux_raw_sys::general::IPV6_MULTICAST_LOOP,
            from_bool(multicast_loop),
        )
    }

    #[inline]
    pub(crate) fn get_ipv6_multicast_loop(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IPV6 as _,
            linux_raw_sys::general::IPV6_MULTICAST_LOOP,
        )
    }

    #[inline]
    pub(crate) fn set_ip_add_membership(
        fd: BorrowedFd<'_>,
        multiaddr: &Ipv4Addr,
        interface: &Ipv4Addr,
    ) -> io::Result<()> {
        let mreq = to_imr(multiaddr, interface);
        setsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IP as _,
            linux_raw_sys::general::IP_ADD_MEMBERSHIP,
            mreq,
        )
    }

    #[inline]
    pub(crate) fn set_ipv6_join_group(
        fd: BorrowedFd<'_>,
        multiaddr: &Ipv6Addr,
        interface: u32,
    ) -> io::Result<()> {
        let mreq = to_ipv6mr(multiaddr, interface);
        setsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IPV6 as _,
            linux_raw_sys::general::IPV6_ADD_MEMBERSHIP,
            mreq,
        )
    }

    #[inline]
    pub(crate) fn set_ip_drop_membership(
        fd: BorrowedFd<'_>,
        multiaddr: &Ipv4Addr,
        interface: &Ipv4Addr,
    ) -> io::Result<()> {
        let mreq = to_imr(multiaddr, interface);
        setsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IP as _,
            linux_raw_sys::general::IP_DROP_MEMBERSHIP,
            mreq,
        )
    }

    #[inline]
    pub(crate) fn set_ipv6_leave_group(
        fd: BorrowedFd<'_>,
        multiaddr: &Ipv6Addr,
        interface: u32,
    ) -> io::Result<()> {
        let mreq = to_ipv6mr(multiaddr, interface);
        setsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IPV6 as _,
            linux_raw_sys::general::IPV6_DROP_MEMBERSHIP,
            mreq,
        )
    }

    #[inline]
    pub(crate) fn set_tcp_nodelay(fd: BorrowedFd<'_>, nodelay: bool) -> io::Result<()> {
        setsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_TCP as _,
            linux_raw_sys::general::TCP_NODELAY,
            from_bool(nodelay),
        )
    }

    #[inline]
    pub(crate) fn get_tcp_nodelay(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_TCP as _,
            linux_raw_sys::general::TCP_NODELAY,
        )
    }

    #[inline]
    fn to_imr(multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> linux_raw_sys::general::ip_mreq {
        linux_raw_sys::general::ip_mreq {
            imr_multiaddr: to_imr_addr(multiaddr),
            imr_interface: to_imr_addr(interface),
        }
    }

    #[inline]
    fn to_imr_addr(addr: &Ipv4Addr) -> linux_raw_sys::general::in_addr {
        linux_raw_sys::general::in_addr {
            s_addr: u32::from_ne_bytes(addr.octets()),
        }
    }

    #[inline]
    fn to_ipv6mr(multiaddr: &Ipv6Addr, interface: u32) -> linux_raw_sys::general::ipv6_mreq {
        linux_raw_sys::general::ipv6_mreq {
            ipv6mr_multiaddr: to_ipv6mr_multiaddr(multiaddr),
            ipv6mr_ifindex: to_ipv6mr_interface(interface),
        }
    }

    #[inline]
    fn to_ipv6mr_multiaddr(multiaddr: &Ipv6Addr) -> linux_raw_sys::general::in6_addr {
        linux_raw_sys::general::in6_addr {
            in6_u: linux_raw_sys::general::in6_addr__bindgen_ty_1 {
                u6_addr8: multiaddr.octets(),
            },
        }
    }

    #[inline]
    fn to_ipv6mr_interface(interface: u32) -> c_int {
        interface as c_int
    }

    #[inline]
    fn from_bool(value: bool) -> c_uint {
        value as c_uint
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
    pub(crate) unsafe fn arm_set_tls(data: *mut c_void) -> io::Result<()> {
        ret(syscall1(nr(__ARM_NR_set_tls), void_star(data)))
    }

    #[cfg(target_arch = "x86_64")]
    #[inline]
    pub(crate) unsafe fn set_fs(data: *mut c_void) {
        ret_infallible(syscall2(
            nr(__NR_arch_prctl),
            c_uint(ARCH_SET_FS),
            void_star(data),
        ))
    }

    #[inline]
    pub(crate) unsafe fn set_tid_address(data: *mut c_void) -> Pid {
        let tid: i32 = ret_usize_infallible(syscall1(nr(__NR_set_tid_address), void_star(data)))
            as __kernel_pid_t;
        Pid::from_raw(tid as u32)
    }

    #[inline]
    pub(crate) unsafe fn set_thread_name(name: &CStr) -> io::Result<()> {
        ret(syscall2(nr(__NR_prctl), c_uint(PR_SET_NAME), c_str(name)))
    }

    #[inline]
    pub(crate) fn exit_thread(code: c_int) -> ! {
        unsafe { syscall1_noreturn(nr(__NR_exit), c_int(code)) }
    }
}
