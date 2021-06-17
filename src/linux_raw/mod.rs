#![allow(dead_code)]

mod arch;
mod conv;

use crate::io;
#[cfg(target_pointer_width = "64")]
use crate::linux_raw::conv::ret_u64;
use arch::{
    syscall0_readonly, syscall1, syscall1_noreturn, syscall1_readonly, syscall2, syscall2_readonly,
    syscall3, syscall3_readonly, syscall4, syscall4_readonly, syscall5, syscall5_readonly,
    syscall6, syscall6_readonly,
};
use conv::{
    borrowed_fd, by_mut, by_ref, c_int, c_str, c_uint, clockid_t, loff_t, loff_t_from_u64,
    opt_c_str, opt_mut, opt_ref, out, owned_fd, ret, ret_c_int, ret_c_uint, ret_owned_fd,
    ret_usize, ret_void_star, slice_addr, slice_as_mut_ptr, socklen_t, umode_t, void_star,
};
use io_lifetimes::{BorrowedFd, OwnedFd};
use linux_raw_sys::{
    general::{
        __NR_accept, __NR_accept4, __NR_bind, __NR_chdir, __NR_clock_getres, __NR_clock_gettime,
        __NR_clock_nanosleep, __NR_close, __NR_connect, __NR_dup, __NR_exit_group, __NR_faccessat,
        __NR_fadvise64, __NR_fallocate, __NR_fchmod, __NR_fchmodat, __NR_fcntl, __NR_fdatasync,
        __NR_fstat, __NR_fstatfs, __NR_fsync, __NR_ftruncate, __NR_getcwd, __NR_getdents64,
        __NR_getegid, __NR_geteuid, __NR_getgid, __NR_getpeername, __NR_getpid, __NR_getppid,
        __NR_getsockname, __NR_getsockopt, __NR_getuid, __NR_ioctl, __NR_linkat, __NR_listen,
        __NR_lseek, __NR_mkdirat, __NR_mmap, __NR_nanosleep, __NR_newfstatat, __NR_openat,
        __NR_pipe, __NR_pipe2, __NR_poll, __NR_pread64, __NR_preadv, __NR_pwrite64, __NR_pwritev,
        __NR_read, __NR_readlinkat, __NR_readv, __NR_recvfrom, __NR_renameat, __NR_sched_yield,
        __NR_sendto, __NR_setsockopt, __NR_shutdown, __NR_socket, __NR_socketpair, __NR_symlinkat,
        __NR_unlinkat, __NR_utimensat, __NR_write, __NR_writev,
    },
    general::{
        __kernel_clockid_t, __kernel_gid_t, __kernel_loff_t, __kernel_pid_t,
        __kernel_sockaddr_storage, __kernel_uid_t, socklen_t, stat, statfs64, termios, timespec,
        umode_t, winsize,
    },
    general::{
        AT_FDCWD, AT_REMOVEDIR, AT_SYMLINK_NOFOLLOW, FIONREAD, F_DUPFD, F_DUPFD_CLOEXEC, F_GETFD,
        F_GETFL, F_GETLEASE, F_GETOWN, F_GETSIG, F_SETFD, F_SETFL, TCGETS, TIMER_ABSTIME,
        TIOCGWINSZ,
    },
    v5_11::{general::__NR_openat2, general::open_how},
    v5_4::{
        general::statx,
        general::{
            __NR_copy_file_range, __NR_getrandom, __NR_memfd_create, __NR_preadv2, __NR_pwritev2,
            __NR_statx, __NR_userfaultfd,
        },
        general::{F_GETPIPE_SZ, F_GET_SEALS, F_SETPIPE_SZ},
    },
};
use std::{
    ffi::CStr,
    io::{IoSlice, IoSliceMut},
    mem::{size_of, MaybeUninit},
    os::raw::{c_char, c_int, c_uint, c_void},
};
#[cfg(target_pointer_width = "32")]
use {
    crate::linux_raw::conv::{hi, lo},
    linux_raw_sys::{errno::EINVAL, unistd::__NR_clock_nanosleep_time64},
};

#[inline]
pub(crate) fn exit_group(code: c_int) -> ! {
    unsafe { syscall1_noreturn(__NR_exit_group, c_int(code)) }
}

#[inline]
pub(crate) fn close(fd: OwnedFd) {
    unsafe {
        let _ = syscall1_readonly(__NR_close, owned_fd(fd));
    }
}

#[inline]
pub(crate) fn open(filename: &CStr, flags: c_int, mode: umode_t) -> io::Result<OwnedFd> {
    unsafe {
        ret_owned_fd(syscall4_readonly(
            __NR_openat,
            c_int(AT_FDCWD),
            c_str(filename),
            c_int(flags),
            umode_t(mode),
        ))
    }
}

#[inline]
pub(crate) fn openat(
    dirfd: BorrowedFd<'_>,
    filename: &CStr,
    flags: c_uint,
    mode: umode_t,
) -> io::Result<OwnedFd> {
    unsafe {
        ret_owned_fd(syscall4_readonly(
            __NR_openat,
            borrowed_fd(dirfd),
            c_str(filename),
            c_uint(flags),
            umode_t(mode),
        ))
    }
}

#[inline]
pub(crate) fn openat2(
    dirfd: BorrowedFd<'_>,
    pathname: &CStr,
    flags: u64,
    mode: u64,
    resolve: u64,
) -> io::Result<OwnedFd> {
    unsafe {
        ret_owned_fd(syscall4_readonly(
            __NR_openat2,
            borrowed_fd(dirfd),
            c_str(pathname),
            by_ref(&open_how {
                flags,
                mode,
                resolve,
            }),
            size_of::<open_how>(),
        ))
    }
}

#[inline]
pub(crate) fn clock_gettime(which_clock: __kernel_clockid_t) -> timespec {
    unsafe {
        // TODO: Linux makes clock_gettime available through the vDSO, which is
        // faster, but more complex to locate and parse before we can call it.
        let mut result = MaybeUninit::uninit();
        syscall2(__NR_clock_gettime, clockid_t(which_clock), out(&mut result));
        result.assume_init()
    }
}

#[inline]
pub(crate) fn clock_getres(which_clock: __kernel_clockid_t) -> timespec {
    unsafe {
        let mut result = MaybeUninit::uninit();
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
        ret_usize(syscall6(
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
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_usize(syscall3(
            __NR_readv,
            borrowed_fd(fd),
            slice_addr(bufs),
            bufs.len(),
        ))
    }
    #[cfg(target_pointer_width = "64")]
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
        ret_usize(syscall6(
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
    flags: c_uint,
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
            c_uint(flags),
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
            c_uint(flags),
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
        ret_usize(syscall6_readonly(
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
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_usize(syscall6_readonly(
            __NR_writev,
            borrowed_fd(fd),
            slice_addr(bufs),
            bufs.len(),
        ))
    }
    #[cfg(target_pointer_width = "64")]
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
        ret_usize(syscall6_readonly(
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
    flags: c_uint,
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
            c_uint(flags),
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
            c_uint(flags),
        ))
    }
}

#[inline]
pub(crate) fn chmod(filename: &CStr, mode: umode_t) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            __NR_fchmodat,
            c_int(AT_FDCWD),
            c_str(filename),
            umode_t(mode),
        ))
    }
}

#[inline]
pub(crate) fn fchmodat(dirfd: BorrowedFd<'_>, filename: &CStr, mode: umode_t) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            __NR_fchmodat,
            borrowed_fd(dirfd),
            c_str(filename),
            umode_t(mode),
        ))
    }
}

#[inline]
pub(crate) fn fchmod(fd: BorrowedFd<'_>, mode: umode_t) -> io::Result<()> {
    unsafe {
        ret(syscall2_readonly(
            __NR_fchmod,
            borrowed_fd(fd),
            umode_t(mode),
        ))
    }
}

#[inline]
pub(crate) fn seek(fd: BorrowedFd<'_>, offset: i64, whence: c_uint) -> io::Result<u64> {
    #[cfg(target_pointer_width = "32")]
    {
        let mut result = MaybeUninit::uninit();
        ret(syscall5(
            __NR__llseek,
            borrowed_fd(fd),
            hi(offset),
            lo(offset),
            out(&mut result),
            c_uint(whence),
        ))
        .map(|()| unsafe { result.assume_init() })
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
pub(crate) fn fallocate(fd: BorrowedFd, mode: c_int, offset: u64, len: u64) -> io::Result<()> {
    unsafe {
        ret(syscall4_readonly(
            __NR_fallocate,
            borrowed_fd(fd),
            c_int(mode),
            loff_t_from_u64(offset),
            loff_t_from_u64(len),
        ))
    }
}

#[inline]
pub(crate) fn fadvise(fd: BorrowedFd<'_>, pos: u64, len: u64, advice: c_int) -> io::Result<()> {
    unsafe {
        ret(syscall4_readonly(
            __NR_fadvise64,
            borrowed_fd(fd),
            loff_t_from_u64(pos),
            loff_t_from_u64(len),
            c_int(advice),
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
pub(crate) fn fstat(fd: BorrowedFd<'_>) -> io::Result<stat> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let mut result = MaybeUninit::uninit();
        ret(syscall2(__NR_fstat64, borrowed_fd(fd), out(&mut result)))
            .map(|()| result.assume_init())
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        let mut result = MaybeUninit::uninit();
        ret(syscall2(__NR_fstat, borrowed_fd(fd), out(&mut result))).map(|()| result.assume_init())
    }
}

#[inline]
pub(crate) fn stat(filename: &CStr) -> io::Result<stat> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let mut result = MaybeUninit::uninit();
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
        let mut result = MaybeUninit::uninit();
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
pub(crate) fn fstatat(dirfd: BorrowedFd<'_>, filename: &CStr, flags: c_uint) -> io::Result<stat> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let mut result = MaybeUninit::uninit();
        ret(syscall4(
            __NR_fstatat64,
            borrowed_fd(dirfd),
            c_str(filename),
            out(&mut result),
            c_uint(flags),
        ))
        .map(|()| result.assume_init())
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        let mut result = MaybeUninit::uninit();
        ret(syscall4(
            __NR_newfstatat,
            borrowed_fd(dirfd),
            c_str(filename),
            out(&mut result),
            c_uint(flags),
        ))
        .map(|()| result.assume_init())
    }
}

#[inline]
pub(crate) fn lstat(filename: &CStr) -> io::Result<stat> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let mut result = MaybeUninit::uninit();
        ret(syscall3(
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
        let mut result = MaybeUninit::uninit();
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
    flags: c_uint,
    mask: c_uint,
) -> io::Result<statx> {
    unsafe {
        let mut statx_buf = MaybeUninit::uninit();
        ret(syscall5(
            __NR_statx,
            borrowed_fd(dirfd),
            c_str(pathname),
            c_uint(flags),
            c_uint(mask),
            out(&mut statx_buf),
        ))
        .map(|()| statx_buf.assume_init())
    }
}

#[inline]
pub(crate) fn fstatfs(fd: BorrowedFd<'_>) -> io::Result<statfs64> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let mut result = MaybeUninit::uninit();
        ret(syscall2(
            __NR_fstatfs64,
            borrowed_fd(fd),
            size_of::<statfs>(),
            out(&mut result),
        ))
        .map(|()| result.assume_init())
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        let mut result = MaybeUninit::uninit();
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
pub(crate) fn fcntl_getfd(fd: BorrowedFd<'_>) -> io::Result<c_uint> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_c_uint(syscall2_readonly(
            __NR_fcntl64,
            borrowed_fd(fd),
            c_uint(F_GETFD),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_c_uint(syscall2_readonly(
            __NR_fcntl,
            borrowed_fd(fd),
            c_uint(F_GETFD),
        ))
    }
}

#[inline]
pub(crate) fn fcntl_setfd(fd: BorrowedFd<'_>, flags: c_uint) -> io::Result<()> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret(syscall3_readonly(
            __NR_fcntl64,
            borrowed_fd(fd),
            c_uint(F_SETFD),
            c_uint(flags),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret(syscall3_readonly(
            __NR_fcntl,
            borrowed_fd(fd),
            c_uint(F_SETFD),
            c_uint(flags),
        ))
    }
}

#[inline]
pub(crate) fn fcntl_getfl(fd: BorrowedFd<'_>) -> io::Result<c_uint> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_c_uint(syscall2_readonly(
            __NR_fcntl64,
            borrowed_fd(fd),
            c_uint(F_GETFL),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_c_uint(syscall2_readonly(
            __NR_fcntl,
            borrowed_fd(fd),
            c_uint(F_GETFL),
        ))
    }
}

#[inline]
pub(crate) fn fcntl_setfl(fd: BorrowedFd<'_>, flags: c_uint) -> io::Result<()> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret(syscall3_readonly(
            __NR_fcntl64,
            borrowed_fd(fd),
            c_uint(F_SETFL),
            c_uint(flags),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret(syscall3_readonly(
            __NR_fcntl,
            borrowed_fd(fd),
            c_uint(F_SETFL),
            c_uint(flags),
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
pub(crate) fn fcntl_get_seals(fd: BorrowedFd<'_>) -> io::Result<c_int> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret_c_int(syscall2_readonly(
            __NR_fcntl64,
            borrowed_fd(fd),
            c_uint(F_GET_SEALS),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret_c_int(syscall2_readonly(
            __NR_fcntl,
            borrowed_fd(fd),
            c_uint(F_GET_SEALS),
        ))
    }
}

#[inline]
pub(crate) fn rename(oldname: &CStr, newname: &CStr) -> io::Result<()> {
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
pub(crate) fn unlinkat(dirfd: BorrowedFd<'_>, pathname: &CStr, flags: c_uint) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            __NR_unlinkat,
            borrowed_fd(dirfd),
            c_str(pathname),
            c_uint(flags),
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
    flags: c_uint,
) -> io::Result<()> {
    unsafe {
        ret(syscall5_readonly(
            __NR_linkat,
            borrowed_fd(old_dirfd),
            c_str(oldname),
            borrowed_fd(new_dirfd),
            c_str(newname),
            c_uint(flags),
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
pub(crate) fn mkdir(pathname: &CStr, mode: umode_t) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            __NR_mkdirat,
            c_int(AT_FDCWD),
            c_str(pathname),
            umode_t(mode),
        ))
    }
}

#[inline]
pub(crate) fn mkdirat(dirfd: BorrowedFd<'_>, pathname: &CStr, mode: umode_t) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            __NR_mkdirat,
            borrowed_fd(dirfd),
            c_str(pathname),
            umode_t(mode),
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
pub(crate) fn pipe2(filedes: &mut [OwnedFd; 2], flags: c_int) -> io::Result<()> {
    unsafe { ret(syscall2(__NR_pipe2, slice_addr(filedes), c_int(flags))) }
}

#[inline]
pub(crate) fn pipe(filedes: &mut [OwnedFd; 2]) -> io::Result<()> {
    #[cfg(target_arch = "aarch64")]
    {
        pipe2(filedes, 0)
    }
    #[cfg(any(target_arch = "mips", target_arch = "mips64"))]
    {
        todo!("On MIPS pipe returns multiple values")
    }
    #[cfg(not(any(target_arch = "aarch64", target_arch = "mips", target_arch = "mips64")))]
    unsafe {
        ret(syscall1(__NR_pipe, slice_addr(filedes)))
    }
}

#[inline]
pub(crate) fn getrandom(buf: &mut [u8], flags: c_uint) -> io::Result<usize> {
    unsafe {
        ret_usize(syscall3(
            __NR_getrandom,
            slice_as_mut_ptr(buf),
            buf.len(),
            c_uint(flags),
        ))
    }
}

#[inline]
pub(crate) fn socket(family: c_uint, typ: c_uint, protocol: c_uint) -> io::Result<OwnedFd> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret_owned_fd(syscall3_readonly(
            __NR_socket,
            c_uint(family),
            c_uint(typ),
            c_uint(protocol),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_owned_fd(syscall2_readonly(
            __NR_socketcall,
            SYS_SOCKET,
            slice_addr(&[c_uint(family), c_uint(typ), c_uint(protocol), 0, 0, 0]),
        ))
    }
}

#[inline]
pub(crate) fn socketpair(
    family: c_int,
    typ: c_uint,
    protocol: c_int,
) -> io::Result<(OwnedFd, OwnedFd)> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut result = MaybeUninit::<[OwnedFd; 2]>::uninit();
        ret(syscall4(
            __NR_socketpair,
            c_int(family),
            c_uint(typ),
            c_int(protocol),
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
            SYS_SOCKETPAIR,
            slice_addr(&[
                c_int(family),
                c_uint(typ),
                c_int(protocol),
                out(&mut result),
                0,
                0,
            ]),
        ))
        .map(|()| {
            let [fd0, fd1] = result.assume_init();
            (fd0, fd1)
        })
    }
}

#[inline]
pub(crate) fn accept(
    fd: BorrowedFd<'_>,
    addr: &mut __kernel_sockaddr_storage,
    addrlen: &mut socklen_t,
) -> io::Result<OwnedFd> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret_owned_fd(syscall3(
            __NR_accept,
            borrowed_fd(fd),
            by_mut(addr),
            by_mut(addrlen),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_owned_fd(syscall2(
            __NR_socketcall,
            SYS_ACCEPT,
            slice_addr(&[borrwed_fd(fd), by_mut(addr), by_mut(addrlen), 0, 0, 0]),
        ))
    }
}

#[inline]
pub(crate) fn accept4(
    fd: BorrowedFd<'_>,
    addr: &mut __kernel_sockaddr_storage,
    addrlen: &mut socklen_t,
    flags: c_int,
) -> io::Result<OwnedFd> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret_owned_fd(syscall4(
            __NR_accept4,
            borrowed_fd(fd),
            by_ref(addr),
            by_mut(addrlen),
            c_int(flags),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_owned_fd(syscall2(
            __NR_socketcall,
            SYS_ACCEPT4,
            slice_addr(&[borrwed_fd(fd), by_mut(addr), by_mut(addrlen), flags, 0, 0]),
        ))
    }
}

#[inline]
pub(crate) fn connect(
    fd: BorrowedFd<'_>,
    addr: &__kernel_sockaddr_storage,
    addrlen: socklen_t,
) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall3_readonly(
            __NR_connect,
            borrowed_fd(fd),
            by_ref(addr),
            socklen_t(addrlen),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            __NR_socketcall,
            SYS_CONNECT,
            slice_addr(&[borrwed_fd(fd), by_ref(addr), socklen_t(addrlen), 0, 0, 0]),
        ))
    }
}

#[inline]
pub(crate) fn shutdown(fd: BorrowedFd<'_>, how: c_int) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall2(__NR_shutdown, borrowed_fd(fd), c_int(how)))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            __NR_socketcall,
            SYS_SHUTDOWN,
            &[borrwed_fd(fd), c_int(how), 0, 0, 0, 0],
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
            SYS_SETSOCKOPT,
            slice_addr(&[
                borrwed_fd(fd),
                c_int(level),
                c_int(name),
                slice_addr(value),
                socklen_t(optlen),
                0,
            ]),
        ))
    }
}

#[inline]
pub(crate) fn getsockopt(
    fd: BorrowedFd<'_>,
    level: c_int,
    name: c_int,
    value: &mut [u8],
    optlen: &mut socklen_t,
) -> io::Result<c_int> {
    assert!((*optlen as usize) <= value.len());

    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret_c_int(syscall5(
            __NR_getsockopt,
            borrowed_fd(fd),
            c_int(level),
            c_int(name),
            slice_addr(value),
            by_mut(optlen),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_c_int(syscall2(
            __NR_socketcall,
            SYS_GETSOCKOPT,
            &[
                borrwed_fd(fd),
                c_int(level),
                c_int(name),
                slice_addr(value),
                by_mut(optlen),
                0,
            ],
        ))
    }
}

#[inline]
pub(crate) fn send(fd: BorrowedFd<'_>, buf: &[u8], flags: c_uint) -> io::Result<usize> {
    sendto(fd, buf, flags, None, 0)
}

#[inline]
pub(crate) fn sendto(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: c_uint,
    addr: Option<&__kernel_sockaddr_storage>,
    addrlen: socklen_t,
) -> io::Result<usize> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret_usize(syscall6_readonly(
            __NR_sendto,
            borrowed_fd(fd),
            slice_addr(buf),
            buf.len(),
            c_uint(flags),
            opt_ref(addr),
            socklen_t(addrlen),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_usize(syscall2_readonly(
            __NR_socketcall,
            SYS_SENDTO,
            slice_addr(&[
                borrwed_fd(fd),
                slice_addr(buf),
                buf.len(),
                c_uint(flags),
                opt_ref(addr),
                socklen_t(addrlen),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn recv(fd: BorrowedFd<'_>, buf: &mut [u8], flags: c_uint) -> io::Result<usize> {
    recvfrom(fd, buf, flags, None, None)
}

#[inline]
pub(crate) fn recvfrom(
    fd: BorrowedFd<'_>,
    buf: &mut [u8],
    flags: c_uint,
    addr: Option<&mut __kernel_sockaddr_storage>,
    addrlen: Option<&mut socklen_t>,
) -> io::Result<usize> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret_usize(syscall6(
            __NR_recvfrom,
            borrowed_fd(fd),
            slice_as_mut_ptr(buf),
            buf.len(),
            c_uint(flags),
            opt_mut(addr),
            opt_mut(addrlen),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_usize(syscall2(
            __NR_socketcall,
            SYS_RECVFROM,
            slice_addr(&[
                borrwed_fd(fd),
                slice_as_mut_ptr(buf),
                buf.len(),
                c_uint(flags),
                opt_mut(addr),
                opt_mut(addrlen),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn getpeername(
    fd: BorrowedFd<'_>,
    addr: &mut __kernel_sockaddr_storage,
    addrlen: &mut socklen_t,
) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall3(
            __NR_getpeername,
            borrowed_fd(fd),
            by_mut(addr),
            by_mut(addrlen),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2(
            __NR_socketcall,
            SYS_GETPEERNAME,
            slice_addr(&[borrwed_fd(fd), by_mut(addr), by_mut(addrlen), 0, 0, 0]),
        ))
    }
}

#[inline]
pub(crate) fn getsockname(
    fd: BorrowedFd<'_>,
    addr: &mut __kernel_sockaddr_storage,
    addrlen: &mut socklen_t,
) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall3(
            __NR_getsockname,
            borrowed_fd(fd),
            by_ref(addr),
            by_mut(addrlen),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2(
            __NR_socketcall,
            SYS_GETSOCKNAME,
            slice_addr(&[borrowed_fd(fd), by_mut(addr), by_mut(addrlen), 0, 0, 0]),
        ))
    }
}

#[inline]
pub(crate) fn bind(
    fd: BorrowedFd<'_>,
    addr: &__kernel_sockaddr_storage,
    addrlen: socklen_t,
) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall3_readonly(
            __NR_bind,
            borrowed_fd(fd),
            by_ref(addr),
            socklen_t(addrlen),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            __NR_socketcall,
            SYS_BIND,
            slice_addr(&[borrwed_fd(fd), by_ref(addr), socklen_t(addrlen), 0, 0, 0]),
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
            SYS_LISTEN,
            slice_addr(&[borrwed_fd(fd), c_int(backlog), 0, 0, 0, 0]),
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
    prot: c_int,
    flags: c_int,
    fd: BorrowedFd<'_>,
    offset: u64,
) -> io::Result<*mut c_void> {
    #[cfg(target_pointer_width = "32")]
    {
        ret_void_star(syscall6(
            __NR_mmap2,
            void_star(addr),
            length,
            c_int(prot),
            c_int(flags),
            borrowed_fd(fd),
            (offset / 4096)
                .try_into()
                .map_err(|_| io::Error::from_raw_os_error(EINVAL))?,
        ))
    }
    #[cfg(target_pointer_width = "64")]
    {
        ret_void_star(syscall6(
            __NR_mmap,
            void_star(addr),
            length,
            c_int(prot),
            c_int(flags),
            borrowed_fd(fd),
            loff_t_from_u64(offset),
        ))
    }
}

#[inline]
pub(crate) fn utimensat(
    dirfd: BorrowedFd<'_>,
    pathname: Option<&CStr>,
    utimes: &[timespec; 2],
    flags: c_uint,
) -> io::Result<()> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret(syscall4_readonly(
            __NR_utimensat_time64,
            borrowed_fd(dirfd),
            opt_c_str(pathname),
            slice_addr(utimes),
            c_uint(flags),
        ))
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        ret(syscall4_readonly(
            __NR_utimensat,
            borrowed_fd(dirfd),
            opt_c_str(pathname),
            slice_addr(utimes),
            c_uint(flags),
        ))
    }
}

#[inline]
pub(crate) fn nanosleep(req: &timespec) -> Result<(), timespec> {
    unsafe {
        let mut rem = MaybeUninit::uninit();
        ret(syscall2(__NR_nanosleep, by_ref(req), out(&mut rem))).map_err(|_err| rem.assume_init())
    }
}

#[inline]
pub(crate) fn clock_nanosleep_relative(
    id: __kernel_clockid_t,
    req: &timespec,
) -> Result<(), timespec> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        let mut rem = MaybeUninit::uninit();
        ret(syscall4(
            __NR_clock_nanosleep_time64,
            clockid_t(id),
            c_int(0),
            by_ref(req),
            out(&mut rem),
        ))
        .map_err(|_err| rmpt.assume_init())
    }
    #[cfg(target_pointer_width = "64")]
    unsafe {
        let mut rem = MaybeUninit::uninit();
        ret(syscall4(
            __NR_clock_nanosleep,
            clockid_t(id),
            c_int(0),
            by_ref(req),
            out(&mut rem),
        ))
        .map_err(|_err| rem.assume_init())
    }
}

#[inline]
pub(crate) fn clock_nanosleep_absolute(id: __kernel_clockid_t, req: &timespec) -> io::Result<()> {
    #[cfg(target_pointer_width = "32")]
    unsafe {
        ret(syscall4_readonly(
            __NR_clock_nanosleep_time64,
            clockid_t(id),
            c_uint(TIMER_ABSTIME),
            by_ref(req),
            0usize,
        ))
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
pub(crate) fn ioctl_tiocgwinsz(fd: BorrowedFd) -> io::Result<winsize> {
    unsafe {
        let mut result = MaybeUninit::uninit();
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
pub(crate) fn ioctl_tcgets(fd: BorrowedFd) -> io::Result<termios> {
    unsafe {
        let mut result = MaybeUninit::uninit();
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
pub(crate) fn faccessat(dirfd: BorrowedFd<'_>, pathname: &CStr, mode: c_uint) -> io::Result<()> {
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
    off_in: Option<&mut __kernel_loff_t>,
    fd_out: BorrowedFd<'_>,
    off_out: Option<&mut __kernel_loff_t>,
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

#[repr(C)]
#[derive(Debug, Clone)]
pub(crate) struct PollFd<'fd> {
    pub(crate) fd: BorrowedFd<'fd>,
    pub(crate) events: u16,
    pub(crate) revents: u16,
}

#[inline]
pub(crate) fn poll(fds: &mut [PollFd<'_>], timeout: c_int) -> io::Result<usize> {
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
pub(crate) fn memfd_create(name: &CStr, flags: c_uint) -> io::Result<OwnedFd> {
    unsafe { ret_owned_fd(syscall2(__NR_memfd_create, c_str(name), c_uint(flags))) }
}

#[inline]
pub(crate) unsafe fn userfaultfd(flags: c_uint) -> io::Result<OwnedFd> {
    ret_owned_fd(syscall1(__NR_userfaultfd, c_uint(flags)))
}

#[inline]
pub(crate) fn getpid() -> __kernel_pid_t {
    unsafe { syscall0_readonly(__NR_getpid) as __kernel_pid_t }
}

#[inline]
pub(crate) fn getppid() -> __kernel_pid_t {
    unsafe { syscall0_readonly(__NR_getppid) as __kernel_pid_t }
}

#[inline]
pub(crate) fn getgid() -> __kernel_gid_t {
    unsafe { syscall0_readonly(__NR_getgid) as __kernel_gid_t }
}

#[inline]
pub(crate) fn getegid() -> __kernel_gid_t {
    unsafe { syscall0_readonly(__NR_getegid) as __kernel_gid_t }
}

#[inline]
pub(crate) fn getuid() -> __kernel_uid_t {
    unsafe { syscall0_readonly(__NR_getuid) as __kernel_uid_t }
}

#[inline]
pub(crate) fn geteuid() -> __kernel_uid_t {
    unsafe { syscall0_readonly(__NR_geteuid) as __kernel_uid_t }
}
