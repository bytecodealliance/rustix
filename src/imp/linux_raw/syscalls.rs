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
    borrowed_fd, by_mut, by_ref, c_int, c_str, c_uint, clockid_t, dev_t, mode_as, oflags,
    opt_c_str, opt_mut, out, raw_fd, ret, ret_c_int, ret_c_uint, ret_discarded_fd, ret_owned_fd,
    ret_usize, ret_void_star, slice_addr, slice_as_mut_ptr, socklen_t, void_star,
};
use super::fs::{
    Access, Advice, AtFlags, FallocateFlags, FdFlags, MemfdFlags, Mode, OFlags, ResolveFlags,
    StatFs, StatxFlags,
};
use super::io::{
    epoll, DupFlags, EventfdFlags, MapFlags, PipeFlags, PollFd, ProtFlags, ReadWriteFlags,
    UserfaultfdFlags,
};
#[cfg(not(target_os = "wasi"))]
use super::io::{Termios, Winsize};
use super::net::{
    decode_sockaddr, AcceptFlags, AddressFamily, Protocol, RecvFlags, SendFlags, Shutdown,
    SocketAddr, SocketAddrUnix, SocketAddrV4, SocketAddrV6, SocketType,
};
use super::rand::GetRandomFlags;
use super::time::ClockId;
use super::{fs::Stat, time::Timespec};
use crate::io;
use crate::io::RawFd;
use crate::time::NanosleepRelativeResult;
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
use linux_raw_sys::general::__NR_epoll_pwait;
#[cfg(not(any(target_arch = "aarch64", target_arch = "riscv64")))]
use linux_raw_sys::general::__NR_epoll_wait;
#[cfg(not(any(target_arch = "riscv64")))]
use linux_raw_sys::general::__NR_renameat;
#[cfg(any(target_arch = "riscv64"))]
use linux_raw_sys::general::__NR_renameat2;
#[cfg(not(target_arch = "x86"))]
use linux_raw_sys::general::{
    __NR_accept, __NR_accept4, __NR_bind, __NR_connect, __NR_getpeername, __NR_getsockname,
    __NR_getsockopt, __NR_listen, __NR_recvfrom, __NR_sendto, __NR_setsockopt, __NR_shutdown,
    __NR_socket, __NR_socketpair,
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
use linux_raw_sys::{
    general::{
        __NR_chdir, __NR_clock_getres, __NR_clock_nanosleep, __NR_close, __NR_dup, __NR_dup3,
        __NR_epoll_create1, __NR_epoll_ctl, __NR_exit_group, __NR_faccessat, __NR_fallocate,
        __NR_fchmod, __NR_fchmodat, __NR_fdatasync, __NR_fsync, __NR_getcwd, __NR_getdents64,
        __NR_getpid, __NR_getppid, __NR_ioctl, __NR_linkat, __NR_mkdirat, __NR_mknodat,
        __NR_munmap, __NR_nanosleep, __NR_openat, __NR_pipe2, __NR_pread64, __NR_preadv,
        __NR_pwrite64, __NR_pwritev, __NR_read, __NR_readlinkat, __NR_readv, __NR_sched_yield,
        __NR_symlinkat, __NR_unlinkat, __NR_utimensat, __NR_write, __NR_writev,
    },
    general::{
        __kernel_gid_t, __kernel_pid_t, __kernel_timespec, __kernel_uid_t, epoll_event, sockaddr,
        sockaddr_in, sockaddr_in6, sockaddr_un, socklen_t,
    },
    general::{
        AT_FDCWD, AT_REMOVEDIR, AT_SYMLINK_NOFOLLOW, EPOLL_CTL_ADD, EPOLL_CTL_DEL, EPOLL_CTL_MOD,
        FIONBIO, FIONREAD, F_DUPFD, F_DUPFD_CLOEXEC, F_GETFD, F_GETFL, F_GETLEASE, F_GETOWN,
        F_GETSIG, F_SETFD, F_SETFL, TCGETS, TIMER_ABSTIME, TIOCGWINSZ,
    },
    v5_11::{general::__NR_openat2, general::open_how},
    v5_4::{
        general::statx,
        general::{
            __NR_copy_file_range, __NR_eventfd2, __NR_getrandom, __NR_memfd_create, __NR_preadv2,
            __NR_pwritev2, __NR_statx, __NR_userfaultfd,
        },
        general::{F_GETPIPE_SZ, F_GET_SEALS, F_SETPIPE_SZ},
    },
};
use std::{
    convert::TryInto,
    ffi::CStr,
    io::{IoSlice, IoSliceMut, SeekFrom},
    mem::{size_of, MaybeUninit},
    os::raw::{c_char, c_int, c_uint, c_void},
};
#[cfg(target_arch = "x86")]
use {
    super::conv::x86_sys,
    linux_raw_sys::general::{
        __NR_mmap2, __NR_socketcall, SYS_ACCEPT, SYS_ACCEPT4, SYS_BIND, SYS_CONNECT,
        SYS_GETPEERNAME, SYS_GETSOCKNAME, SYS_GETSOCKOPT, SYS_LISTEN, SYS_RECV, SYS_RECVFROM,
        SYS_SEND, SYS_SENDTO, SYS_SETSOCKOPT, SYS_SHUTDOWN, SYS_SOCKET, SYS_SOCKETPAIR,
    },
};
#[cfg(target_pointer_width = "32")]
use {
    super::conv::{hi, lo},
    linux_raw_sys::{
        errno::EINVAL,
        general::timespec as __kernel_old_timespec,
        general::{
            __NR__llseek, __NR_fadvise64_64, __NR_fcntl64, __NR_fstat64, __NR_fstatat64,
            __NR_fstatfs64, __NR_ftruncate64, __NR_sendfile64,
        },
        v5_4::general::{
            __NR_clock_getres_time64, __NR_clock_nanosleep_time64, __NR_utimensat_time64,
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
    unsafe { syscall1_noreturn(__NR_exit_group, c_int(code)) }
}

#[inline]
pub(crate) unsafe fn close(raw_fd: RawFd) {
    let _ = syscall1_readonly(__NR_close, raw_fd as isize as usize);
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
            __NR_open,
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
            __NR_open,
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
            __NR_openat,
            borrowed_fd(dirfd),
            c_str(filename),
            oflags(flags),
            mode_as(mode),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_owned_fd(syscall4_readonly(
            __NR_openat,
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
            __NR_openat2,
            borrowed_fd(dirfd),
            c_str(pathname),
            by_ref(&open_how {
                flags: oflags(flags) as u64,
                mode: u64::from(mode.bits()),
                resolve: resolve.bits(),
            }),
            size_of::<open_how>(),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_owned_fd(syscall4_readonly(
            __NR_openat2,
            borrowed_fd(dirfd),
            c_str(pathname),
            by_ref(&open_how {
                flags: oflags(flags) as u64,
                mode: u64::from(mode.bits()),
                resolve: resolve.bits(),
            }),
            size_of::<open_how>(),
        ))
    }
}

#[inline]
pub(crate) fn clock_getres(which_clock: ClockId) -> __kernel_timespec {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let mut result = MaybeUninit::<__kernel_timespec>::uninit();
        let _ = ret(syscall2(
            __NR_clock_getres_time64,
            clockid_t(which_clock),
            out(&mut result),
        ))
        .or_else(|err| {
            // See the comments in `posish_clock_gettime_via_syscall` about
            // emulation.
            if err == io::Error::NOSYS {
                let mut old_result = MaybeUninit::<__kernel_old_timespec>::uninit();
                let res = ret(syscall2(
                    __NR_clock_getres,
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
        syscall2(__NR_clock_getres, clockid_t(which_clock), out(&mut result));
        result.assume_init()
    }
}

#[inline]
pub(crate) fn read(fd: BorrowedFd<'_>, buffer: &mut [u8]) -> io::Result<usize> {
    unsafe {
        ret_usize(syscall3(
            __NR_read,
            borrowed_fd(fd),
            slice_as_mut_ptr(buffer),
            buffer.len(),
        ))
    }
}

#[inline]
pub(crate) fn pread(fd: BorrowedFd<'_>, buffer: &[u8], pos: u64) -> io::Result<usize> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_usize(syscall5(
            __NR_pread64,
            borrowed_fd(fd),
            slice_addr(buffer),
            buffer.len(),
            hi(pos),
            lo(pos),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_usize(syscall4(
            __NR_pread64,
            borrowed_fd(fd),
            slice_addr(buffer),
            buffer.len(),
            loff_t_from_u64(pos),
        ))
    }
}

pub(crate) fn readv(fd: BorrowedFd<'_>, bufs: &[IoSliceMut]) -> io::Result<usize> {
    unsafe {
        ret_usize(syscall3(
            __NR_readv,
            borrowed_fd(fd),
            slice_addr(bufs),
            bufs.len(),
        ))
    }
}

pub(crate) fn preadv(fd: BorrowedFd<'_>, bufs: &[IoSliceMut], pos: u64) -> io::Result<usize> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_usize(syscall5(
            __NR_preadv,
            borrowed_fd(fd),
            slice_addr(bufs),
            bufs.len(),
            hi(pos),
            lo(pos),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_usize(syscall4(
            __NR_preadv,
            borrowed_fd(fd),
            slice_addr(bufs),
            bufs.len(),
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
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_usize(syscall6(
            __NR_preadv2,
            borrowed_fd(fd),
            slice_addr(bufs),
            bufs.len(),
            hi(pos),
            lo(pos),
            c_uint(flags.bits()),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_usize(syscall5(
            __NR_preadv2,
            borrowed_fd(fd),
            slice_addr(bufs),
            bufs.len(),
            loff_t_from_u64(pos),
            c_uint(flags.bits()),
        ))
    }
}

#[inline]
pub(crate) fn write(fd: BorrowedFd<'_>, buffer: &[u8]) -> io::Result<usize> {
    unsafe {
        ret_usize(syscall3_readonly(
            __NR_write,
            borrowed_fd(fd),
            slice_addr(buffer),
            buffer.len(),
        ))
    }
}

#[inline]
pub(crate) fn pwrite(fd: BorrowedFd<'_>, buffer: &[u8], pos: u64) -> io::Result<usize> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_usize(syscall5_readonly(
            __NR_pwrite64,
            borrowed_fd(fd),
            slice_addr(buffer),
            buffer.len(),
            hi(pos),
            lo(pos),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_usize(syscall4_readonly(
            __NR_pwrite64,
            borrowed_fd(fd),
            slice_addr(buffer),
            buffer.len(),
            loff_t_from_u64(pos),
        ))
    }
}

#[inline]
pub(crate) fn writev(fd: BorrowedFd<'_>, bufs: &[IoSlice]) -> io::Result<usize> {
    unsafe {
        ret_usize(syscall3_readonly(
            __NR_writev,
            borrowed_fd(fd),
            slice_addr(bufs),
            bufs.len(),
        ))
    }
}

#[inline]
pub(crate) fn pwritev(fd: BorrowedFd<'_>, bufs: &[IoSlice], pos: u64) -> io::Result<usize> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_usize(syscall5_readonly(
            __NR_pwritev,
            borrowed_fd(fd),
            slice_addr(bufs),
            bufs.len(),
            hi(pos),
            lo(pos),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_usize(syscall4_readonly(
            __NR_pwritev,
            borrowed_fd(fd),
            slice_addr(bufs),
            bufs.len(),
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
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_usize(syscall6_readonly(
            __NR_pwritev2,
            borrowed_fd(fd),
            slice_addr(bufs),
            bufs.len(),
            hi(pos),
            lo(pos),
            c_uint(flags.bits()),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_usize(syscall5_readonly(
            __NR_pwritev2,
            borrowed_fd(fd),
            slice_addr(bufs),
            bufs.len(),
            loff_t_from_u64(pos),
            c_uint(flags.bits()),
        ))
    }
}

#[inline]
pub(crate) fn chmod(filename: &CStr, mode: Mode) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            __NR_fchmodat,
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
            __NR_fchmodat,
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
            __NR_fchmod,
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
            __NR_mknodat,
            borrowed_fd(dirfd),
            c_str(filename),
            mode_as(mode),
            dev_t(dev)?,
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret(syscall4_readonly(
            __NR_mknodat,
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
            __NR__llseek,
            borrowed_fd(fd),
            // Don't use the hi/lo functions here because Linux's llseek
            // takes its 64-bit argument differently from everything else.
            (offset >> 32) as usize,
            offset as usize,
            out(&mut result),
            c_uint(whence),
        ))
        .map(|()| result.assume_init())
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_u64(syscall3_readonly(
            __NR_lseek,
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
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret(syscall3_readonly(
            __NR_ftruncate64,
            borrowed_fd(fd),
            hi(length),
            lo(length),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret(syscall2_readonly(
            __NR_ftruncate,
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
            __NR_fallocate,
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
            __NR_fallocate,
            borrowed_fd(fd),
            c_uint(mode.bits()),
            loff_t_from_u64(offset),
            loff_t_from_u64(len),
        ))
    }
}

#[inline]
pub(crate) fn fadvise(fd: BorrowedFd<'_>, pos: u64, len: u64, advice: Advice) -> io::Result<()> {
    // On arm and powerpc, the system calls are reordered so that the len and
    // pos argument pairs are aligned.
    #[cfg(any(target_arch = "arm", target_arch = "powerpc"))]
    unsafe {
        ret(syscall6_readonly(
            __NR_fadvise64_64,
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
            __NR_fadvise64_64,
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
            __NR_fadvise64,
            borrowed_fd(fd),
            loff_t_from_u64(pos),
            loff_t_from_u64(len),
            c_uint(advice as c_uint),
        ))
    }
}

#[inline]
pub(crate) fn fsync(fd: BorrowedFd<'_>) -> io::Result<()> {
    unsafe { ret(syscall1_readonly(__NR_fsync, borrowed_fd(fd))) }
}

#[inline]
pub(crate) fn fdatasync(fd: BorrowedFd<'_>) -> io::Result<()> {
    unsafe { ret(syscall1_readonly(__NR_fdatasync, borrowed_fd(fd))) }
}

#[inline]
pub(crate) fn fstat(fd: BorrowedFd<'_>) -> io::Result<Stat> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let mut result = MaybeUninit::<Stat>::uninit();
        ret(syscall2(__NR_fstat64, borrowed_fd(fd), out(&mut result)))
            .map(|()| result.assume_init())
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        let mut result = MaybeUninit::<Stat>::uninit();
        ret(syscall2(__NR_fstat, borrowed_fd(fd), out(&mut result))).map(|()| result.assume_init())
    }
}

#[inline]
pub(crate) fn stat(filename: &CStr) -> io::Result<Stat> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let mut result = MaybeUninit::<Stat>::uninit();
        ret(syscall4(
            __NR_fstatat64,
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
            __NR_newfstatat,
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
            __NR_fstatat64,
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
            __NR_newfstatat,
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
            __NR_fstatat64,
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
            __NR_newfstatat,
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
            __NR_statx,
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
            __NR_fstatfs64,
            borrowed_fd(fd),
            size_of::<StatFs>(),
            out(&mut result),
        ))
        .map(|()| result.assume_init())
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        let mut result = MaybeUninit::<StatFs>::uninit();
        ret(syscall2(__NR_fstatfs, borrowed_fd(fd), out(&mut result)))
            .map(|()| result.assume_init())
    }
}

#[inline]
pub(crate) fn readlink(path: &CStr, buf: &mut [u8]) -> io::Result<usize> {
    unsafe {
        ret_usize(syscall4_readonly(
            __NR_readlinkat,
            c_int(AT_FDCWD),
            c_str(path),
            slice_as_mut_ptr(buf),
            buf.len(),
        ))
    }
}

#[inline]
pub(crate) fn readlinkat(dirfd: BorrowedFd<'_>, path: &CStr, buf: &mut [u8]) -> io::Result<usize> {
    unsafe {
        ret_usize(syscall4_readonly(
            __NR_readlinkat,
            borrowed_fd(dirfd),
            c_str(path),
            slice_as_mut_ptr(buf),
            buf.len(),
        ))
    }
}

#[inline]
pub(crate) fn fcntl_dupfd(fd: BorrowedFd<'_>) -> io::Result<OwnedFd> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_owned_fd(syscall2_readonly(
            __NR_fcntl64,
            borrowed_fd(fd),
            c_uint(F_DUPFD),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_owned_fd(syscall2_readonly(
            __NR_fcntl,
            borrowed_fd(fd),
            c_uint(F_DUPFD),
        ))
    }
}

#[inline]
pub(crate) fn fcntl_dupfd_cloexec(fd: BorrowedFd<'_>) -> io::Result<OwnedFd> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_owned_fd(syscall2_readonly(
            __NR_fcntl64,
            borrowed_fd(fd),
            c_uint(F_DUPFD_CLOEXEC),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_owned_fd(syscall2_readonly(
            __NR_fcntl,
            borrowed_fd(fd),
            c_uint(F_DUPFD_CLOEXEC),
        ))
    }
}

#[inline]
pub(crate) fn fcntl_getfd(fd: BorrowedFd<'_>) -> io::Result<FdFlags> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_c_uint(syscall2_readonly(
            __NR_fcntl64,
            borrowed_fd(fd),
            c_uint(F_GETFD),
        ))
        .map(FdFlags::from_bits_truncate)
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_c_uint(syscall2_readonly(
            __NR_fcntl,
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
            __NR_fcntl64,
            borrowed_fd(fd),
            c_uint(F_SETFD),
            c_uint(flags.bits()),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret(syscall3_readonly(
            __NR_fcntl,
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
            __NR_fcntl64,
            borrowed_fd(fd),
            c_uint(F_GETFL),
        ))
        .map(OFlags::from_bits_truncate)
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_c_uint(syscall2_readonly(
            __NR_fcntl,
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
            __NR_fcntl64,
            borrowed_fd(fd),
            c_uint(F_SETFL),
            oflags(flags),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret(syscall3_readonly(
            __NR_fcntl,
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
            __NR_fcntl64,
            borrowed_fd(fd),
            c_uint(F_GETLEASE),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_c_int(syscall2_readonly(
            __NR_fcntl,
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
            __NR_fcntl64,
            borrowed_fd(fd),
            c_uint(F_GETOWN),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_c_int(syscall2_readonly(
            __NR_fcntl,
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
            __NR_fcntl64,
            borrowed_fd(fd),
            c_uint(F_GETSIG),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_c_int(syscall2_readonly(
            __NR_fcntl,
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
            __NR_fcntl64,
            borrowed_fd(fd),
            c_uint(F_GETPIPE_SZ),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_usize(syscall2_readonly(
            __NR_fcntl,
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
            __NR_fcntl64,
            borrowed_fd(fd),
            c_uint(F_SETPIPE_SZ),
            c_int(size),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_usize(syscall3_readonly(
            __NR_fcntl,
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
            __NR_fcntl64,
            borrowed_fd(fd),
            c_uint(F_GET_SEALS),
        ))
        .map(|seals| seals as u32)
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_c_int(syscall2_readonly(
            __NR_fcntl,
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
            __NR_renameat2,
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
            __NR_renameat,
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
            __NR_renameat2,
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
            __NR_renameat,
            borrowed_fd(old_dirfd),
            c_str(oldname),
            borrowed_fd(new_dirfd),
            c_str(newname),
        ))
    }
}

#[inline]
pub(crate) fn unlink(pathname: &CStr) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            __NR_unlinkat,
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
            __NR_unlinkat,
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
            __NR_unlinkat,
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
            __NR_linkat,
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
            __NR_linkat,
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
            __NR_symlinkat,
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
            __NR_symlinkat,
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
            __NR_mkdirat,
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
            __NR_mkdirat,
            borrowed_fd(dirfd),
            c_str(pathname),
            mode_as(mode),
        ))
    }
}

#[inline]
pub(crate) fn getdents(fd: BorrowedFd<'_>, dirent: &mut [u8]) -> io::Result<usize> {
    unsafe {
        ret_usize(syscall3(
            __NR_getdents64,
            borrowed_fd(fd),
            slice_as_mut_ptr(dirent),
            dirent.len(),
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
        ret(syscall2(__NR_pipe2, out(&mut result), c_uint(flags.bits())))?;
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
        ret(syscall1(__NR_pipe, out(&mut result)))?;
        let [p0, p1] = result.assume_init();
        Ok((p0, p1))
    }
}

#[inline]
pub(crate) fn getrandom(buf: &mut [u8], flags: GetRandomFlags) -> io::Result<usize> {
    unsafe {
        ret_usize(syscall3(
            __NR_getrandom,
            slice_as_mut_ptr(buf),
            buf.len(),
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
            __NR_socket,
            c_uint(family.0.into()),
            c_uint(type_.0),
            c_uint(protocol as u32),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_owned_fd(syscall2_readonly(
            __NR_socketcall,
            x86_sys(SYS_SOCKET),
            slice_addr(&[
                c_uint(family.0.into()),
                c_uint(type_.0),
                c_uint(protocol as u32),
                0,
                0,
                0,
            ]),
        ))
    }
}

#[inline]
pub(crate) fn socketpair(
    family: AddressFamily,
    type_: SocketType,
    accept_flags: AcceptFlags,
    protocol: Protocol,
) -> io::Result<(OwnedFd, OwnedFd)> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut result = MaybeUninit::<[OwnedFd; 2]>::uninit();
        ret(syscall4(
            __NR_socketpair,
            c_uint(family.0.into()),
            c_uint(type_.0 | accept_flags.bits()),
            c_uint(protocol as c_uint),
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
            __NR_socketcall,
            x86_sys(SYS_SOCKETPAIR),
            slice_addr(&[
                c_uint(family.0.into()),
                c_uint(type_.0 | accept_flags.bits()),
                c_uint(protocol as u32),
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
        let fd = ret_owned_fd(syscall3(__NR_accept, borrowed_fd(fd), 0, 0))?;
        Ok(fd)
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let fd = ret_owned_fd(syscall2(
            __NR_socketcall,
            x86_sys(SYS_ACCEPT),
            slice_addr(&[borrowed_fd(fd), 0, 0]),
        ))?;
        Ok(fd)
    }
}

#[inline]
pub(crate) fn accept_with(fd: BorrowedFd<'_>, flags: AcceptFlags) -> io::Result<OwnedFd> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let fd = ret_owned_fd(syscall4(
            __NR_accept4,
            borrowed_fd(fd),
            0,
            0,
            c_uint(flags.bits()),
        ))?;
        Ok(fd)
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let fd = ret_owned_fd(syscall2(
            __NR_socketcall,
            x86_sys(SYS_ACCEPT4),
            slice_addr(&[borrowed_fd(fd), 0, 0, c_uint(flags.bits())]),
        ))?;
        Ok(fd)
    }
}

#[inline]
pub(crate) fn acceptfrom(fd: BorrowedFd<'_>) -> io::Result<(OwnedFd, SocketAddr)> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut addrlen = size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        let fd = ret_owned_fd(syscall3(
            __NR_accept,
            borrowed_fd(fd),
            out(&mut storage),
            by_mut(&mut addrlen),
        ))?;
        Ok((fd, decode_sockaddr(&storage.assume_init(), addrlen)))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut addrlen = size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        let fd = ret_owned_fd(syscall2(
            __NR_socketcall,
            x86_sys(SYS_ACCEPT),
            slice_addr(&[borrowed_fd(fd), out(&mut storage), by_mut(&mut addrlen)]),
        ))?;
        Ok((fd, decode_sockaddr(&storage.assume_init(), addrlen)))
    }
}

#[inline]
pub(crate) fn acceptfrom_with(
    fd: BorrowedFd<'_>,
    flags: AcceptFlags,
) -> io::Result<(OwnedFd, SocketAddr)> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut addrlen = size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        let fd = ret_owned_fd(syscall4(
            __NR_accept4,
            borrowed_fd(fd),
            out(&mut storage),
            by_mut(&mut addrlen),
            c_uint(flags.bits()),
        ))?;
        Ok((fd, decode_sockaddr(&storage.assume_init(), addrlen)))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut addrlen = size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        let fd = ret_owned_fd(syscall2(
            __NR_socketcall,
            x86_sys(SYS_ACCEPT4),
            slice_addr(&[
                borrowed_fd(fd),
                out(&mut storage),
                by_mut(&mut addrlen),
                c_uint(flags.bits()),
            ]),
        ))?;
        Ok((fd, decode_sockaddr(&storage.assume_init(), addrlen)))
    }
}

#[inline]
pub(crate) fn shutdown(fd: BorrowedFd<'_>, how: Shutdown) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall2(
            __NR_shutdown,
            borrowed_fd(fd),
            c_uint(how as c_uint),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            __NR_socketcall,
            x86_sys(SYS_SHUTDOWN),
            slice_addr(&[borrowed_fd(fd), c_uint(how as c_uint), 0, 0, 0, 0]),
        ))
    }
}

#[inline]
pub(crate) fn setsockopt(
    fd: BorrowedFd<'_>,
    level: c_int,
    name: c_int,
    value: &[u8],
    optlen: socklen_t,
) -> io::Result<c_int> {
    assert!((optlen as usize) <= value.len());

    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret_c_int(syscall5_readonly(
            __NR_setsockopt,
            borrowed_fd(fd),
            c_int(level),
            c_int(name),
            slice_addr(value),
            socklen_t(optlen),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_c_int(syscall2_readonly(
            __NR_socketcall,
            x86_sys(SYS_SETSOCKOPT),
            slice_addr(&[
                borrowed_fd(fd),
                c_int(level),
                c_int(name),
                slice_addr(value),
                socklen_t(optlen),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn getsockopt_socket_type(fd: BorrowedFd<'_>) -> io::Result<SocketType> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut type_ = MaybeUninit::<u32>::uninit();
        let mut optlen = size_of::<u32>();
        ret(syscall5(
            __NR_getsockopt,
            borrowed_fd(fd),
            c_uint(linux_raw_sys::general::SOL_SOCKET),
            c_uint(linux_raw_sys::general::SO_TYPE),
            out(&mut type_),
            by_mut(&mut optlen),
        ))?;
        Ok(SocketType(type_.assume_init()))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut type_ = MaybeUninit::<u32>::uninit();
        let mut optlen = size_of::<u32>();
        ret(syscall2(
            __NR_socketcall,
            x86_sys(SYS_GETSOCKOPT),
            slice_addr(&[
                borrowed_fd(fd),
                c_uint(linux_raw_sys::general::SOL_SOCKET),
                c_uint(linux_raw_sys::general::SO_TYPE),
                out(&mut type_),
                by_mut(&mut optlen),
            ]),
        ))?;
        Ok(SocketType(type_.assume_init()))
    }
}

#[inline]
pub(crate) fn send(fd: BorrowedFd<'_>, buf: &[u8], flags: SendFlags) -> io::Result<usize> {
    #[cfg(not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64"
    )))]
    unsafe {
        ret_usize(syscall4_readonly(
            __NR_send,
            borrowed_fd(fd),
            slice_addr(buf),
            buf.len(),
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
            __NR_sendto,
            borrowed_fd(fd),
            slice_addr(buf),
            buf.len(),
            c_uint(flags.bits()),
            0,
            0,
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_usize(syscall2_readonly(
            __NR_socketcall,
            x86_sys(SYS_SEND),
            slice_addr(&[
                borrowed_fd(fd),
                slice_addr(buf),
                buf.len(),
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
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret_usize(syscall6_readonly(
            __NR_sendto,
            borrowed_fd(fd),
            slice_addr(buf),
            buf.len(),
            c_uint(flags.bits()),
            by_ref(&addr.encode()),
            size_of::<sockaddr_in>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_usize(syscall2_readonly(
            __NR_socketcall,
            x86_sys(SYS_SENDTO),
            slice_addr(&[
                borrowed_fd(fd),
                slice_addr(buf),
                buf.len(),
                c_uint(flags.bits()),
                by_ref(&addr.encode()),
                size_of::<sockaddr_in>(),
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
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret_usize(syscall6_readonly(
            __NR_sendto,
            borrowed_fd(fd),
            slice_addr(buf),
            buf.len(),
            c_uint(flags.bits()),
            by_ref(&addr.encode()),
            size_of::<sockaddr_in6>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_usize(syscall2_readonly(
            __NR_socketcall,
            x86_sys(SYS_SENDTO),
            slice_addr(&[
                borrowed_fd(fd),
                slice_addr(buf),
                buf.len(),
                c_uint(flags.bits()),
                by_ref(&addr.encode()),
                size_of::<sockaddr_in6>(),
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
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret_usize(syscall6_readonly(
            __NR_sendto,
            borrowed_fd(fd),
            slice_addr(buf),
            buf.len(),
            c_uint(flags.bits()),
            by_ref(&addr.encode()),
            size_of::<sockaddr_un>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_usize(syscall2_readonly(
            __NR_socketcall,
            x86_sys(SYS_SENDTO),
            slice_addr(&[
                borrowed_fd(fd),
                slice_addr(buf),
                buf.len(),
                c_uint(flags.bits()),
                by_ref(&addr.encode()),
                size_of::<sockaddr_un>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn recv(fd: BorrowedFd<'_>, buf: &mut [u8], flags: RecvFlags) -> io::Result<usize> {
    #[cfg(not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64"
    )))]
    unsafe {
        ret_usize(syscall4(
            __NR_recv,
            borrowed_fd(fd),
            slice_as_mut_ptr(buf),
            buf.len(),
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
            __NR_recvfrom,
            borrowed_fd(fd),
            slice_as_mut_ptr(buf),
            buf.len(),
            c_uint(flags.bits()),
            0,
            0,
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_usize(syscall2(
            __NR_socketcall,
            x86_sys(SYS_RECV),
            slice_addr(&[
                borrowed_fd(fd),
                slice_as_mut_ptr(buf),
                buf.len(),
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
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut addrlen = size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        let nread = ret_usize(syscall6(
            __NR_recvfrom,
            borrowed_fd(fd),
            slice_as_mut_ptr(buf),
            buf.len(),
            c_uint(flags.bits()),
            out(&mut storage),
            by_mut(&mut addrlen),
        ))?;
        Ok((nread, decode_sockaddr(&storage.assume_init(), addrlen)))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut addrlen = size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        let nread = ret_usize(syscall2(
            __NR_socketcall,
            x86_sys(SYS_RECVFROM),
            slice_addr(&[
                borrowed_fd(fd),
                slice_as_mut_ptr(buf),
                buf.len(),
                c_uint(flags.bits()),
                out(&mut storage),
                by_mut(&mut addrlen),
            ]),
        ))?;
        Ok((nread, decode_sockaddr(&storage.assume_init(), addrlen)))
    }
}

#[inline]
pub(crate) fn getpeername(fd: BorrowedFd<'_>) -> io::Result<SocketAddr> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut addrlen = size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        ret(syscall3(
            __NR_getpeername,
            borrowed_fd(fd),
            out(&mut storage),
            by_mut(&mut addrlen),
        ))?;
        Ok(decode_sockaddr(&storage.assume_init(), addrlen))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut addrlen = size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        ret(syscall2(
            __NR_socketcall,
            x86_sys(SYS_GETPEERNAME),
            slice_addr(&[borrowed_fd(fd), out(&mut storage), by_mut(&mut addrlen)]),
        ))?;
        Ok(decode_sockaddr(&storage.assume_init(), addrlen))
    }
}

#[inline]
pub(crate) fn getsockname(fd: BorrowedFd<'_>) -> io::Result<SocketAddr> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut addrlen = size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        ret(syscall3(
            __NR_getsockname,
            borrowed_fd(fd),
            out(&mut storage),
            by_mut(&mut addrlen),
        ))?;
        Ok(decode_sockaddr(&storage.assume_init(), addrlen))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut addrlen = size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        ret(syscall2(
            __NR_socketcall,
            x86_sys(SYS_GETSOCKNAME),
            slice_addr(&[borrowed_fd(fd), out(&mut storage), by_mut(&mut addrlen)]),
        ))?;
        Ok(decode_sockaddr(&storage.assume_init(), addrlen))
    }
}

#[inline]
pub(crate) fn bind_v4(fd: BorrowedFd<'_>, addr: &SocketAddrV4) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall3_readonly(
            __NR_bind,
            borrowed_fd(fd),
            by_ref(&addr.encode()),
            size_of::<sockaddr_in>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            __NR_socketcall,
            x86_sys(SYS_BIND),
            slice_addr(&[
                borrowed_fd(fd),
                by_ref(&addr.encode()),
                size_of::<sockaddr_in>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn bind_v6(fd: BorrowedFd<'_>, addr: &SocketAddrV6) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall3_readonly(
            __NR_bind,
            borrowed_fd(fd),
            by_ref(&addr.encode()),
            size_of::<sockaddr_in6>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            __NR_socketcall,
            x86_sys(SYS_BIND),
            slice_addr(&[
                borrowed_fd(fd),
                by_ref(&addr.encode()),
                size_of::<sockaddr_in6>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn bind_unix(fd: BorrowedFd<'_>, addr: &SocketAddrUnix) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall3_readonly(
            __NR_bind,
            borrowed_fd(fd),
            by_ref(&addr.encode()),
            size_of::<sockaddr_un>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            __NR_socketcall,
            x86_sys(SYS_BIND),
            slice_addr(&[
                borrowed_fd(fd),
                by_ref(&addr.encode()),
                size_of::<sockaddr_un>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn connect_v4(fd: BorrowedFd<'_>, addr: &SocketAddrV4) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall3_readonly(
            __NR_connect,
            borrowed_fd(fd),
            by_ref(&addr.encode()),
            size_of::<sockaddr_in>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            __NR_socketcall,
            x86_sys(SYS_CONNECT),
            slice_addr(&[
                borrowed_fd(fd),
                by_ref(&addr.encode()),
                size_of::<sockaddr_in>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn connect_v6(fd: BorrowedFd<'_>, addr: &SocketAddrV6) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall3_readonly(
            __NR_connect,
            borrowed_fd(fd),
            by_ref(&addr.encode()),
            size_of::<sockaddr_in6>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            __NR_socketcall,
            x86_sys(SYS_CONNECT),
            slice_addr(&[
                borrowed_fd(fd),
                by_ref(&addr.encode()),
                size_of::<sockaddr_in6>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn connect_unix(fd: BorrowedFd<'_>, addr: &SocketAddrUnix) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall3_readonly(
            __NR_connect,
            borrowed_fd(fd),
            by_ref(&addr.encode()),
            size_of::<sockaddr_un>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            __NR_socketcall,
            x86_sys(SYS_CONNECT),
            slice_addr(&[
                borrowed_fd(fd),
                by_ref(&addr.encode()),
                size_of::<sockaddr_un>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn listen(fd: BorrowedFd<'_>, backlog: c_int) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall2_readonly(
            __NR_listen,
            borrowed_fd(fd),
            c_int(backlog),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            __NR_socketcall,
            x86_sys(SYS_LISTEN),
            slice_addr(&[borrowed_fd(fd), c_int(backlog), 0, 0, 0, 0]),
        ))
    }
}

#[inline]
pub(crate) fn sched_yield() {
    unsafe {
        let _ = syscall0_readonly(__NR_sched_yield);
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
            __NR_mmap2,
            void_star(addr),
            length,
            c_uint(prot.bits()),
            c_uint(flags.bits()),
            borrowed_fd(fd),
            (offset / 4096)
                .try_into()
                .map_err(|_| io::Error(EINVAL as _))?,
        ))
    }
    #[cfg(target_pointer_width = "64")]
    {
        ret_void_star(syscall6(
            __NR_mmap,
            void_star(addr),
            length,
            c_uint(prot.bits()),
            c_uint(flags.bits()),
            borrowed_fd(fd),
            loff_t_from_u64(offset),
        ))
    }
}

/// # Safety
///
/// `munmap` is primarily unsafe due to the `addr` parameter, as anything
/// working with memory pointed to by raw pointers is unsafe.
#[inline]
pub(crate) unsafe fn munmap(addr: *mut c_void, length: usize) -> io::Result<()> {
    ret(syscall2(__NR_munmap, void_star(addr), length))
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
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret(syscall4_readonly(
            __NR_utimensat_time64,
            borrowed_fd(dirfd),
            opt_c_str(pathname),
            slice_addr(utimes),
            c_uint(flags.bits()),
        ))
        .or_else(|err| {
            // See the comments in `posish_clock_gettime_via_syscall` about
            // emulation.
            if err == io::Error::NOSYS {
                let old_utimes = [
                    __kernel_old_timespec {
                        tv_sec: utimes[0]
                            .tv_sec
                            .try_into()
                            .map_err(|_| io::Error(EINVAL as _))?,
                        tv_nsec: utimes[0]
                            .tv_nsec
                            .try_into()
                            .map_err(|_| io::Error(EINVAL as _))?,
                    },
                    __kernel_old_timespec {
                        tv_sec: utimes[1]
                            .tv_sec
                            .try_into()
                            .map_err(|_| io::Error(EINVAL as _))?,
                        tv_nsec: utimes[1]
                            .tv_nsec
                            .try_into()
                            .map_err(|_| io::Error(EINVAL as _))?,
                    },
                ];
                ret(syscall4_readonly(
                    __NR_utimensat,
                    borrowed_fd(dirfd),
                    opt_c_str(pathname),
                    slice_addr(&old_utimes),
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
            __NR_utimensat,
            borrowed_fd(dirfd),
            opt_c_str(pathname),
            slice_addr(utimes),
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
            __NR_clock_nanosleep_time64,
            clockid_t(ClockId::Realtime),
            c_int(0),
            by_ref(req),
            out(&mut rem),
        ))
        .or_else(|err| {
            // See the comments in `posish_clock_gettime_via_syscall` about
            // emulation.
            if err == io::Error::NOSYS {
                let old_req = __kernel_old_timespec {
                    tv_sec: req.tv_sec.try_into().map_err(|_| io::Error(EINVAL as _))?,
                    tv_nsec: req.tv_nsec.try_into().map_err(|_| io::Error(EINVAL as _))?,
                };
                let mut old_rem = MaybeUninit::<__kernel_old_timespec>::uninit();
                let res = ret(syscall2(
                    __NR_nanosleep,
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
        match ret(syscall2(__NR_nanosleep, by_ref(req), out(&mut rem))) {
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
            __NR_clock_nanosleep_time64,
            clockid_t(id),
            c_int(0),
            by_ref(req),
            out(&mut rem),
        ))
        .or_else(|err| {
            // See the comments in `posish_clock_gettime_via_syscall` about
            // emulation.
            if err == io::Error::NOSYS {
                let old_req = __kernel_old_timespec {
                    tv_sec: req.tv_sec.try_into().map_err(|_| io::Error(EINVAL as _))?,
                    tv_nsec: req.tv_nsec.try_into().map_err(|_| io::Error(EINVAL as _))?,
                };
                let mut old_rem = MaybeUninit::<__kernel_old_timespec>::uninit();
                let res = ret(syscall4(
                    __NR_clock_nanosleep,
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
            __NR_clock_nanosleep,
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
            __NR_clock_nanosleep_time64,
            clockid_t(id),
            c_uint(TIMER_ABSTIME),
            by_ref(req),
            0usize,
        ))
        .or_else(|err| {
            // See the comments in `posish_clock_gettime_via_syscall` about
            // emulation.
            if err == io::Error::NOSYS {
                let old_req = __kernel_old_timespec {
                    tv_sec: req.tv_sec.try_into().map_err(|_| io::Error(EINVAL as _))?,
                    tv_nsec: req.tv_nsec.try_into().map_err(|_| io::Error(EINVAL as _))?,
                };
                ret(syscall4_readonly(
                    __NR_clock_nanosleep,
                    clockid_t(id),
                    c_int(0),
                    by_ref(&old_req),
                    0usize,
                ))
            } else {
                Err(err)
            }
        })
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret(syscall4_readonly(
            __NR_clock_nanosleep,
            clockid_t(id),
            c_uint(TIMER_ABSTIME),
            by_ref(req),
            0usize,
        ))
    }
}

#[inline]
pub(crate) fn getcwd(buf: &mut [c_char]) -> io::Result<usize> {
    unsafe { ret_usize(syscall2(__NR_getcwd, slice_as_mut_ptr(buf), buf.len())) }
}

#[inline]
pub(crate) fn chdir(filename: &CStr) -> io::Result<()> {
    unsafe { ret(syscall1_readonly(__NR_chdir, c_str(filename))) }
}

#[inline]
pub(crate) fn ioctl_fionread(fd: BorrowedFd) -> io::Result<u64> {
    unsafe {
        let mut result = MaybeUninit::<c_int>::uninit();
        ret(syscall3(
            __NR_ioctl,
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
            __NR_ioctl,
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
            __NR_ioctl,
            borrowed_fd(fd),
            c_uint(TIOCGWINSZ),
            out(&mut result),
        ))
        .map(|()| result.assume_init())
    }
}

#[inline]
pub(crate) fn ioctl_tcgets(fd: BorrowedFd) -> io::Result<Termios> {
    unsafe {
        let mut result = MaybeUninit::<Termios>::uninit();
        ret(syscall3(
            __NR_ioctl,
            borrowed_fd(fd),
            c_uint(TCGETS),
            out(&mut result),
        ))
        .map(|()| result.assume_init())
    }
}

#[inline]
pub(crate) fn dup(fd: BorrowedFd) -> io::Result<OwnedFd> {
    unsafe { ret_owned_fd(syscall1_readonly(__NR_dup, borrowed_fd(fd))) }
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
            __NR_dup2,
            borrowed_fd(fd),
            borrowed_fd(new.as_fd()),
        ))
    }
}

#[inline]
pub(crate) fn dup2_with(fd: BorrowedFd, new: &OwnedFd, flags: DupFlags) -> io::Result<()> {
    unsafe {
        ret_discarded_fd(syscall3_readonly(
            __NR_dup3,
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
            __NR_faccessat,
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
            __NR_copy_file_range,
            borrowed_fd(fd_in),
            opt_mut(off_in),
            borrowed_fd(fd_out),
            opt_mut(off_out),
            len,
            c_uint(flags),
        ))
    }
}

#[inline]
pub(crate) fn poll(fds: &mut [PollFd<'_>], timeout: c_int) -> io::Result<usize> {
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
            __NR_ppoll,
            slice_as_mut_ptr(fds),
            fds.len(),
            opt_ref(timeout.as_ref()),
            0,
            size_of::<sigset_t>(),
        ))
    }
    #[cfg(not(any(target_arch = "aarch64", target_arch = "riscv64")))]
    unsafe {
        ret_usize(syscall3(
            __NR_poll,
            slice_as_mut_ptr(fds),
            fds.len(),
            c_int(timeout),
        ))
    }
}

#[inline]
pub(crate) fn memfd_create(name: &CStr, flags: MemfdFlags) -> io::Result<OwnedFd> {
    unsafe {
        ret_owned_fd(syscall2(
            __NR_memfd_create,
            c_str(name),
            c_uint(flags.bits()),
        ))
    }
}

#[inline]
pub(crate) fn eventfd(initval: u32, flags: EventfdFlags) -> io::Result<OwnedFd> {
    unsafe {
        ret_owned_fd(syscall2(
            __NR_eventfd2,
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
            __NR_sendfile64,
            borrowed_fd(out_fd),
            borrowed_fd(in_fd),
            opt_mut(offset),
            count,
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_usize(syscall4(
            __NR_sendfile,
            borrowed_fd(out_fd),
            borrowed_fd(in_fd),
            opt_mut(offset),
            count,
        ))
    }
}

#[inline]
pub(crate) unsafe fn userfaultfd(flags: UserfaultfdFlags) -> io::Result<OwnedFd> {
    ret_owned_fd(syscall1(__NR_userfaultfd, c_uint(flags.bits())))
}

#[inline]
pub(crate) fn getpid() -> u32 {
    let gid: i32 = unsafe { syscall0_readonly(__NR_getpid) as __kernel_pid_t };
    gid as u32
}

#[inline]
pub(crate) fn getppid() -> u32 {
    let ppid: i32 = unsafe { syscall0_readonly(__NR_getppid) as __kernel_pid_t };
    ppid as u32
}

#[inline]
pub(crate) fn getgid() -> u32 {
    #[cfg(any(target_arch = "x86", target_arch = "sparc", target_arch = "arm"))]
    unsafe {
        (syscall0_readonly(__NR_getgid32) as __kernel_gid_t).into()
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "sparc", target_arch = "arm")))]
    unsafe {
        syscall0_readonly(__NR_getgid) as __kernel_gid_t
    }
}

#[inline]
pub(crate) fn getegid() -> u32 {
    #[cfg(any(target_arch = "x86", target_arch = "sparc", target_arch = "arm"))]
    unsafe {
        (syscall0_readonly(__NR_getegid32) as __kernel_gid_t).into()
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "sparc", target_arch = "arm")))]
    unsafe {
        syscall0_readonly(__NR_getegid) as __kernel_gid_t
    }
}

#[inline]
pub(crate) fn getuid() -> u32 {
    #[cfg(any(target_arch = "x86", target_arch = "sparc", target_arch = "arm"))]
    unsafe {
        let uid = syscall0_readonly(__NR_getuid32) as __kernel_uid_t;
        uid as u32
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "sparc", target_arch = "arm")))]
    unsafe {
        let uid = syscall0_readonly(__NR_getuid) as __kernel_uid_t;
        uid as u32
    }
}

#[inline]
pub(crate) fn geteuid() -> u32 {
    #[cfg(any(target_arch = "x86", target_arch = "sparc", target_arch = "arm"))]
    unsafe {
        (syscall0_readonly(__NR_geteuid32) as __kernel_uid_t).into()
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "sparc", target_arch = "arm")))]
    unsafe {
        syscall0_readonly(__NR_geteuid) as __kernel_uid_t
    }
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
            Err(io::Error::AGAIN | io::Error::WOULDBLOCK) => (),
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
    unsafe { ret_owned_fd(syscall1(__NR_epoll_create1, c_uint(flags.bits()))) }
}

#[inline]
pub(crate) unsafe fn epoll_add(
    epfd: BorrowedFd<'_>,
    fd: c_int,
    event: &epoll_event,
) -> io::Result<()> {
    ret(syscall4(
        __NR_epoll_ctl,
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
        __NR_epoll_ctl,
        borrowed_fd(epfd),
        c_uint(EPOLL_CTL_MOD),
        raw_fd(fd),
        by_ref(event),
    ))
}

#[inline]
pub(crate) unsafe fn epoll_del(epfd: BorrowedFd<'_>, fd: c_int) -> io::Result<()> {
    ret(syscall4(
        __NR_epoll_ctl,
        borrowed_fd(epfd),
        c_uint(EPOLL_CTL_DEL),
        raw_fd(fd),
        0,
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
            __NR_epoll_wait,
            borrowed_fd(epfd),
            events as usize,
            num_events,
            c_int(timeout),
        ))
    }
    #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
    unsafe {
        ret_usize(syscall5(
            __NR_epoll_pwait,
            borrowed_fd(epfd),
            events as usize,
            num_events,
            c_int(timeout),
            0,
        ))
    }
}
