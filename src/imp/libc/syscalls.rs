// There are a lot of filesystem and network system calls, so they're split
// out into separate files.
pub(crate) use super::fs::syscalls::*;

#[cfg(any(target_os = "ios", target_os = "macos"))]
use super::conv::nonnegative_ret;
#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
use super::conv::ret_infallible;
use super::conv::{
    borrowed_fd, c_str, no_fd, ret, ret_c_int, ret_discarded_char_ptr, ret_discarded_fd,
    ret_owned_fd, ret_ssize_t,
};
#[cfg(any(target_os = "android", target_os = "linux"))]
use super::conv::{syscall_ret, syscall_ret_owned_fd, syscall_ret_u32};
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
use super::io::Advice as IoAdvice;
#[cfg(target_os = "linux")]
use super::io::MremapFlags;
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "wasi")))]
use super::io::PipeFlags;
use super::io::PollFd;
#[cfg(any(target_os = "android", target_os = "linux"))]
use super::io::{EventfdFlags, UserfaultfdFlags};
#[cfg(any(target_os = "android", target_os = "linux"))]
use super::io::{MlockFlags, ReadWriteFlags};
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
use super::net::{
    encode_sockaddr_unix, encode_sockaddr_v4, encode_sockaddr_v6, read_sockaddr_os, AcceptFlags,
    AddressFamily, Protocol, RecvFlags, SendFlags, Shutdown, SocketAddr, SocketAddrUnix,
    SocketFlags, SocketType,
};
#[cfg(not(any(target_os = "fuchsia", target_os = "wasi")))]
use super::offset::libc_getrlimit;
#[cfg(not(target_os = "wasi"))]
use super::offset::libc_mmap;
use super::offset::{libc_pread, libc_pwrite};
#[cfg(all(target_os = "linux", target_env = "gnu"))]
use super::offset::{libc_preadv2, libc_pwritev2};
#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
use super::offset::{libc_rlimit, LIBC_RLIM_INFINITY};
#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
use super::process::Resource;
#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "fuchsia",
    target_os = "dragonfly"
))]
use super::process::{RawCpuSet, CPU_SETSIZE};
#[cfg(not(target_os = "wasi"))]
use super::process::{RawPid, RawUname};
#[cfg(target_os = "linux")]
use super::rand::GetRandomFlags;
use super::time::Timespec;
use crate::as_ptr;
use crate::io::{self, OwnedFd, RawFd};
#[cfg(not(any(target_os = "fuchsia", target_os = "redox", target_os = "wasi")))]
use crate::process::Rlimit;
#[cfg(any(target_os = "android", target_os = "linux"))]
use crate::process::{Cpuid, MembarrierCommand, MembarrierQuery};
#[cfg(not(target_os = "wasi"))]
use crate::process::{Gid, Pid, Uid, WaitOptions, WaitStatus};
use errno::errno;
use io_lifetimes::{AsFd, BorrowedFd};
use libc::{c_int, c_void};
use std::cmp::min;
use std::convert::TryInto;
use std::ffi::CStr;
#[cfg(any(target_os = "ios", target_os = "macos"))]
use std::ffi::CString;
use std::io::{IoSlice, IoSliceMut};
use std::mem::{size_of, MaybeUninit};
use std::net::{SocketAddrV4, SocketAddrV6};
#[cfg(not(any(target_os = "redox", target_os = "wasi",)))]
use std::ptr::null_mut;
#[cfg(not(any(target_os = "redox", target_env = "newlib")))]
use std::sync::atomic::{AtomicUsize, Ordering};
#[cfg(not(target_os = "wasi"))]
use {
    super::io::{DupFlags, MapFlags, MprotectFlags, ProtFlags, Termios, Winsize},
    super::time::{ClockId, DynamicClockId},
};
#[cfg(not(target_os = "redox"))]
use {
    super::offset::{libc_preadv, libc_pwritev},
    crate::time::NanosleepRelativeResult,
};

pub(crate) fn read(fd: BorrowedFd<'_>, buf: &mut [u8]) -> io::Result<usize> {
    let nread = unsafe {
        ret_ssize_t(libc::read(
            borrowed_fd(fd),
            buf.as_mut_ptr().cast::<_>(),
            buf.len(),
        ))?
    };
    Ok(nread as usize)
}

pub(crate) fn write(fd: BorrowedFd<'_>, buf: &[u8]) -> io::Result<usize> {
    let nwritten = unsafe {
        ret_ssize_t(libc::write(
            borrowed_fd(fd),
            buf.as_ptr().cast::<_>(),
            buf.len(),
        ))?
    };
    Ok(nwritten as usize)
}

pub(crate) fn pread(fd: BorrowedFd<'_>, buf: &mut [u8], offset: u64) -> io::Result<usize> {
    // Silently cast; we'll get `EINVAL` if the value is negative.
    let offset = offset as i64;
    let nread = unsafe {
        ret_ssize_t(libc_pread(
            borrowed_fd(fd),
            buf.as_mut_ptr().cast::<_>(),
            buf.len(),
            offset,
        ))?
    };
    Ok(nread as usize)
}

pub(crate) fn pwrite(fd: BorrowedFd<'_>, buf: &[u8], offset: u64) -> io::Result<usize> {
    // Silently cast; we'll get `EINVAL` if the value is negative.
    let offset = offset as i64;
    let nwritten = unsafe {
        ret_ssize_t(libc_pwrite(
            borrowed_fd(fd),
            buf.as_ptr().cast::<_>(),
            buf.len(),
            offset,
        ))?
    };
    Ok(nwritten as usize)
}

pub(crate) fn readv(fd: BorrowedFd<'_>, bufs: &[IoSliceMut]) -> io::Result<usize> {
    let nread = unsafe {
        ret_ssize_t(libc::readv(
            borrowed_fd(fd),
            bufs.as_ptr().cast::<libc::iovec>(),
            min(bufs.len(), max_iov()) as c_int,
        ))?
    };
    Ok(nread as usize)
}

pub(crate) fn writev(fd: BorrowedFd<'_>, bufs: &[IoSlice]) -> io::Result<usize> {
    let nwritten = unsafe {
        ret_ssize_t(libc::writev(
            borrowed_fd(fd),
            bufs.as_ptr().cast::<libc::iovec>(),
            min(bufs.len(), max_iov()) as c_int,
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(not(target_os = "redox"))]
pub(crate) fn preadv(fd: BorrowedFd<'_>, bufs: &[IoSliceMut], offset: u64) -> io::Result<usize> {
    // Silently cast; we'll get `EINVAL` if the value is negative.
    let offset = offset as i64;
    let nread = unsafe {
        ret_ssize_t(libc_preadv(
            borrowed_fd(fd),
            bufs.as_ptr().cast::<libc::iovec>(),
            min(bufs.len(), max_iov()) as c_int,
            offset,
        ))?
    };
    Ok(nread as usize)
}

#[cfg(not(target_os = "redox"))]
pub(crate) fn pwritev(fd: BorrowedFd<'_>, bufs: &[IoSlice], offset: u64) -> io::Result<usize> {
    // Silently cast; we'll get `EINVAL` if the value is negative.
    let offset = offset as i64;
    let nwritten = unsafe {
        ret_ssize_t(libc_pwritev(
            borrowed_fd(fd),
            bufs.as_ptr().cast::<libc::iovec>(),
            min(bufs.len(), max_iov()) as c_int,
            offset,
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub(crate) fn preadv2(
    fd: BorrowedFd<'_>,
    bufs: &[IoSliceMut],
    offset: u64,
    flags: ReadWriteFlags,
) -> io::Result<usize> {
    // Silently cast; we'll get `EINVAL` if the value is negative.
    let offset = offset as i64;
    let nread = unsafe {
        ret_ssize_t(libc_preadv2(
            borrowed_fd(fd),
            bufs.as_ptr().cast::<libc::iovec>(),
            min(bufs.len(), max_iov()) as c_int,
            offset,
            flags.bits(),
        ))?
    };
    Ok(nread as usize)
}

/// At present, `libc` only has `preadv2` defined for glibc. On other
/// ABIs, `ReadWriteFlags` has no flags defined, and we use plain `preadv`.
#[cfg(any(
    target_os = "android",
    all(target_os = "linux", not(target_env = "gnu"))
))]
#[inline]
pub(crate) fn preadv2(
    fd: BorrowedFd<'_>,
    bufs: &[IoSliceMut],
    offset: u64,
    flags: ReadWriteFlags,
) -> io::Result<usize> {
    assert!(flags.is_empty());
    preadv(fd, bufs, offset)
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub(crate) fn pwritev2(
    fd: BorrowedFd<'_>,
    bufs: &[IoSlice],
    offset: u64,
    flags: ReadWriteFlags,
) -> io::Result<usize> {
    // Silently cast; we'll get `EINVAL` if the value is negative.
    let offset = offset as i64;
    let nwritten = unsafe {
        ret_ssize_t(libc_pwritev2(
            borrowed_fd(fd),
            bufs.as_ptr().cast::<libc::iovec>(),
            min(bufs.len(), max_iov()) as c_int,
            offset,
            flags.bits(),
        ))?
    };
    Ok(nwritten as usize)
}

/// At present, `libc` only has `pwritev2` defined for glibc. On other
/// ABIs, `ReadWriteFlags` has no flags defined, and we use plain `pwritev`.
#[cfg(any(
    target_os = "android",
    all(target_os = "linux", not(target_env = "gnu"))
))]
#[inline]
pub(crate) fn pwritev2(
    fd: BorrowedFd<'_>,
    bufs: &[IoSlice],
    offset: u64,
    flags: ReadWriteFlags,
) -> io::Result<usize> {
    assert!(flags.is_empty());
    pwritev(fd, bufs, offset)
}

// These functions are derived from Rust's library/std/src/sys/unix/fd.rs at
// revision 108e90ca78f052c0c1c49c42a22c85620be19712.

#[cfg(not(any(target_os = "redox", target_env = "newlib")))]
static LIM: AtomicUsize = AtomicUsize::new(0);

#[cfg(not(any(target_os = "redox", target_env = "newlib")))]
#[inline]
fn max_iov() -> usize {
    let mut lim = LIM.load(Ordering::Relaxed);
    if lim == 0 {
        lim = query_max_iov()
    }

    lim
}

#[cfg(not(any(target_os = "redox", target_env = "newlib")))]
fn query_max_iov() -> usize {
    let ret = unsafe { libc::sysconf(libc::_SC_IOV_MAX) };

    // 16 is the minimum value required by POSIX.
    let lim = if ret > 0 { ret as usize } else { 16 };
    LIM.store(lim, Ordering::Relaxed);
    lim
}

#[cfg(any(target_os = "redox", target_env = "newlib"))]
#[inline]
fn max_iov() -> usize {
    16 // The minimum value required by POSIX.
}

#[cfg(any(target_os = "android", target_os = "linux"))]
#[inline]
pub(crate) fn exit_group(code: c_int) -> ! {
    unsafe { libc::_exit(code) }
}

pub(crate) unsafe fn close(raw_fd: RawFd) {
    let _ = libc::close(raw_fd as c_int);
}

#[cfg(not(target_os = "wasi"))]
pub(crate) fn chdir(path: &CStr) -> io::Result<()> {
    unsafe { ret(libc::chdir(c_str(path))) }
}

#[cfg(not(any(target_os = "wasi", target_os = "fuchsia")))]
pub(crate) fn fchdir(dirfd: BorrowedFd<'_>) -> io::Result<()> {
    unsafe { ret(libc::fchdir(borrowed_fd(dirfd))) }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn madvise(addr: *mut c_void, len: usize, advice: IoAdvice) -> io::Result<()> {
    // On Linux platforms, `MADV_DONTNEED` has the same value as
    // `POSIX_MADV_DONTNEED` but different behavior. We remap it to a different
    // value, and check for it here.
    #[cfg(target_os = "linux")]
    if let IoAdvice::LinuxDontNeed = advice {
        return unsafe { ret(libc::madvise(addr, len, libc::MADV_DONTNEED)) };
    }

    #[cfg(not(target_os = "android"))]
    {
        let err = unsafe { libc::posix_madvise(addr, len, advice as libc::c_int) };

        // `posix_madvise` returns its error status rather than using `errno`.
        if err == 0 {
            Ok(())
        } else {
            Err(io::Error(err))
        }
    }

    #[cfg(target_os = "android")]
    {
        if let IoAdvice::DontNeed = advice {
            // Do nothing. Linux's `MADV_DONTNEED` isn't the same as
            // `POSIX_MADV_DONTNEED`, so just discard `MADV_DONTNEED`.
            Ok(())
        } else {
            unsafe { ret(libc::madvise(addr, len, advice as libc::c_int)) }
        }
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
pub(crate) fn eventfd(initval: u32, flags: EventfdFlags) -> io::Result<OwnedFd> {
    unsafe { syscall_ret_owned_fd(libc::syscall(libc::SYS_eventfd2, initval, flags.bits())) }
}

#[cfg(not(target_os = "redox"))]
pub(crate) fn ioctl_fionread(fd: BorrowedFd<'_>) -> io::Result<u64> {
    let mut nread = MaybeUninit::<libc::c_int>::uninit();
    unsafe {
        ret(libc::ioctl(
            borrowed_fd(fd),
            libc::FIONREAD,
            nread.as_mut_ptr(),
        ))?;
        // `FIONREAD` returns the number of bytes silently casted to a `c_int`,
        // even when this is lossy. The best we can do is convert it back to a
        // `u64` without sign-extending it back first.
        Ok(nread.assume_init() as libc::c_uint as u64)
    }
}

pub(crate) fn ioctl_fionbio(fd: BorrowedFd<'_>, value: bool) -> io::Result<()> {
    unsafe {
        let data = value as c_int;
        ret(libc::ioctl(borrowed_fd(fd), libc::FIONBIO, &data))
    }
}

pub(crate) fn isatty(fd: BorrowedFd<'_>) -> bool {
    let res = unsafe { libc::isatty(borrowed_fd(fd)) };
    if res == 0 {
        match errno().0 {
            #[cfg(not(any(target_os = "android", target_os = "linux")))]
            libc::ENOTTY => false,

            // Old Linux versions reportedly return `EINVAL`.
            // https://man7.org/linux/man-pages/man3/isatty.3.html#ERRORS
            #[cfg(any(target_os = "android", target_os = "linux"))]
            libc::ENOTTY | libc::EINVAL => false,

            // Darwin mysteriously returns `EOPNOTSUPP` sometimes.
            #[cfg(any(target_os = "ios", target_os = "macos"))]
            libc::EOPNOTSUPP => false,

            err => panic!("unexpected error from isatty: {:?}", err),
        }
    } else {
        true
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn is_read_write(fd: BorrowedFd<'_>) -> io::Result<(bool, bool)> {
    let (mut read, mut write) = crate::fs::fd::_is_file_read_write(fd)?;
    let mut not_socket = false;
    if read {
        // Do a `recv` with `PEEK` and `DONTWAIT` for 1 byte. A 0 indicates
        // the read side is shut down; an `EWOULDBLOCK` indicates the read
        // side is still open.
        match unsafe {
            libc::recv(
                borrowed_fd(fd),
                MaybeUninit::<[u8; 1]>::uninit()
                    .as_mut_ptr()
                    .cast::<libc::c_void>(),
                1,
                libc::MSG_PEEK | libc::MSG_DONTWAIT,
            )
        } {
            0 => read = false,
            -1 => {
                #[allow(unreachable_patterns)] // EAGAIN may equal EWOULDBLOCK
                match errno().0 {
                    libc::EAGAIN | libc::EWOULDBLOCK => (),
                    libc::ENOTSOCK => not_socket = true,
                    err => return Err(io::Error(err)),
                }
            }
            _ => (),
        }
    }
    if write && !not_socket {
        // Do a `send` with `DONTWAIT` for 0 bytes. An `EPIPE` indicates
        // the write side is shut down.
        match unsafe { libc::send(borrowed_fd(fd), [].as_ptr(), 0, libc::MSG_DONTWAIT) } {
            -1 => {
                #[allow(unreachable_patterns)] // EAGAIN may equal EWOULDBLOCK
                match errno().0 {
                    libc::EAGAIN | libc::EWOULDBLOCK => (),
                    libc::ENOTSOCK => (),
                    libc::EPIPE => write = false,
                    err => return Err(io::Error(err)),
                }
            }
            _ => (),
        }
    }
    Ok((read, write))
}

#[cfg(target_os = "wasi")]
pub(crate) fn is_read_write(_fd: BorrowedFd<'_>) -> io::Result<(bool, bool)> {
    todo!("Implement is_read_write for WASI in terms of fd_fdstat_get");
}

#[cfg(not(target_os = "wasi"))]
pub(crate) fn dup(fd: BorrowedFd<'_>) -> io::Result<OwnedFd> {
    unsafe { ret_owned_fd(libc::dup(borrowed_fd(fd))) }
}

#[cfg(not(target_os = "wasi"))]
pub(crate) fn dup2(fd: BorrowedFd<'_>, new: &OwnedFd) -> io::Result<()> {
    unsafe { ret_discarded_fd(libc::dup2(borrowed_fd(fd), borrowed_fd(new.as_fd()))) }
}

#[cfg(not(any(
    target_os = "android",
    target_os = "macos",
    target_os = "ios",
    target_os = "redox",
    target_os = "wasi"
)))]
pub(crate) fn dup2_with(fd: BorrowedFd<'_>, new: &OwnedFd, flags: DupFlags) -> io::Result<()> {
    unsafe {
        ret_discarded_fd(libc::dup3(
            borrowed_fd(fd),
            borrowed_fd(new.as_fd()),
            flags.bits(),
        ))
    }
}

#[cfg(any(
    target_os = "android",
    target_os = "macos",
    target_os = "ios",
    target_os = "redox"
))]
pub(crate) fn dup2_with(fd: BorrowedFd<'_>, new: &OwnedFd, _flags: DupFlags) -> io::Result<()> {
    // Android 5.0 has dup3, but libc doesn't have bindings
    dup2(fd, new)
}

#[cfg(not(target_os = "wasi"))]
pub(crate) fn getcwd(buf: &mut [u8]) -> io::Result<()> {
    unsafe { ret_discarded_char_ptr(libc::getcwd(buf.as_mut_ptr().cast::<_>(), buf.len())) }
}

#[cfg(not(any(target_os = "fuchsia", target_os = "wasi")))]
pub(crate) fn ttyname(dirfd: BorrowedFd<'_>, buf: &mut [u8]) -> io::Result<usize> {
    unsafe {
        ret(libc::ttyname_r(
            borrowed_fd(dirfd),
            buf.as_mut_ptr().cast::<_>(),
            buf.len(),
        ))?;
        Ok(CStr::from_ptr(buf.as_ptr().cast::<_>()).to_bytes().len())
    }
}

#[cfg(not(any(
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "wasi"
)))]
pub(crate) fn ioctl_tcgets(fd: BorrowedFd<'_>) -> io::Result<Termios> {
    let mut result = MaybeUninit::<Termios>::uninit();
    unsafe {
        ret(libc::ioctl(
            borrowed_fd(fd),
            libc::TCGETS,
            result.as_mut_ptr(),
        ))
        .map(|()| result.assume_init())
    }
}

#[cfg(any(
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
))]
pub(crate) fn ioctl_tcgets(fd: BorrowedFd<'_>) -> io::Result<Termios> {
    let mut result = MaybeUninit::<Termios>::uninit();
    unsafe {
        ret(libc::tcgetattr(borrowed_fd(fd), result.as_mut_ptr())).map(|()| result.assume_init())
    }
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
pub(crate) fn ioctl_fioclex(fd: BorrowedFd<'_>) -> io::Result<()> {
    unsafe { ret(libc::ioctl(borrowed_fd(fd), libc::FIOCLEX)) }
}

#[cfg(not(target_os = "wasi"))]
pub(crate) fn ioctl_tiocgwinsz(fd: BorrowedFd) -> io::Result<Winsize> {
    unsafe {
        let mut buf = MaybeUninit::<Winsize>::uninit();
        ret(libc::ioctl(
            borrowed_fd(fd),
            libc::TIOCGWINSZ.into(),
            buf.as_mut_ptr(),
        ))?;
        Ok(buf.assume_init())
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn ioctl_tiocexcl(fd: BorrowedFd) -> io::Result<()> {
    unsafe { ret(libc::ioctl(borrowed_fd(fd), libc::TIOCEXCL as _)) }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn ioctl_tiocnxcl(fd: BorrowedFd) -> io::Result<()> {
    unsafe { ret(libc::ioctl(borrowed_fd(fd), libc::TIOCNXCL as _)) }
}

/// # Safety
///
/// `mmap` is primarily unsafe due to the `addr` parameter, as anything working
/// with memory pointed to by raw pointers is unsafe.
#[cfg(not(target_os = "wasi"))]
pub(crate) unsafe fn mmap(
    ptr: *mut c_void,
    len: usize,
    prot: ProtFlags,
    flags: MapFlags,
    fd: BorrowedFd<'_>,
    offset: u64,
) -> io::Result<*mut c_void> {
    let res = libc_mmap(
        ptr,
        len,
        prot.bits(),
        flags.bits(),
        borrowed_fd(fd),
        offset as i64,
    );
    if res == libc::MAP_FAILED {
        Err(io::Error::last_os_error())
    } else {
        Ok(res)
    }
}

/// # Safety
///
/// `mmap` is primarily unsafe due to the `addr` parameter, as anything working
/// with memory pointed to by raw pointers is unsafe.
#[cfg(not(target_os = "wasi"))]
pub(crate) unsafe fn mmap_anonymous(
    ptr: *mut c_void,
    len: usize,
    prot: ProtFlags,
    flags: MapFlags,
) -> io::Result<*mut c_void> {
    let res = libc_mmap(
        ptr,
        len,
        prot.bits(),
        flags.bits() | libc::MAP_ANONYMOUS,
        no_fd(),
        0,
    );
    if res == libc::MAP_FAILED {
        Err(io::Error::last_os_error())
    } else {
        Ok(res)
    }
}

#[cfg(not(target_os = "wasi"))]
pub(crate) unsafe fn mprotect(
    ptr: *mut c_void,
    len: usize,
    flags: MprotectFlags,
) -> io::Result<()> {
    ret(libc::mprotect(ptr, len, flags.bits()))
}

#[cfg(not(target_os = "wasi"))]
pub(crate) unsafe fn munmap(ptr: *mut c_void, len: usize) -> io::Result<()> {
    ret(libc::munmap(ptr, len))
}

/// # Safety
///
/// `mremap` is primarily unsafe due to the `old_address` parameter, as
/// anything working with memory pointed to by raw pointers is unsafe.
#[cfg(target_os = "linux")]
pub(crate) unsafe fn mremap(
    old_address: *mut c_void,
    old_size: usize,
    new_size: usize,
    flags: MremapFlags,
) -> io::Result<*mut c_void> {
    let res = libc::mremap(old_address, old_size, new_size, flags.bits());
    if res == libc::MAP_FAILED {
        Err(io::Error::last_os_error())
    } else {
        Ok(res)
    }
}

/// # Safety
///
/// `mremap_fixed` is primarily unsafe due to the `old_address` and
/// `new_address` parameters, as anything working with memory pointed to by raw
/// pointers is unsafe.
#[cfg(target_os = "linux")]
pub(crate) unsafe fn mremap_fixed(
    old_address: *mut c_void,
    old_size: usize,
    new_size: usize,
    flags: MremapFlags,
    new_address: *mut c_void,
) -> io::Result<*mut c_void> {
    let res = libc::mremap(
        old_address,
        old_size,
        new_size,
        flags.bits() | libc::MAP_FIXED,
        new_address,
    );
    if res == libc::MAP_FAILED {
        Err(io::Error::last_os_error())
    } else {
        Ok(res)
    }
}

/// # Safety
///
/// `mlock` operates on raw pointers and may round out to the nearest page
/// boundaries.
#[cfg(not(target_os = "wasi"))]
#[inline]
pub(crate) unsafe fn mlock(addr: *mut c_void, length: usize) -> io::Result<()> {
    ret(libc::mlock(addr, length))
}

/// # Safety
///
/// `mlock_with` operates on raw pointers and may round out to the nearest page
/// boundaries.
#[cfg(any(target_os = "android", target_os = "linux"))]
#[inline]
pub(crate) unsafe fn mlock_with(
    addr: *mut c_void,
    length: usize,
    flags: MlockFlags,
) -> io::Result<()> {
    assert_eq!(flags.bits(), 0, "libc doesn't define `MLOCK_*` yet");
    ret(libc::mlock(addr, length))
}

/// # Safety
///
/// `munlock` operates on raw pointers and may round out to the nearest page
/// boundaries.
#[cfg(not(target_os = "wasi"))]
#[inline]
pub(crate) unsafe fn munlock(addr: *mut c_void, length: usize) -> io::Result<()> {
    ret(libc::munlock(addr, length))
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
pub(crate) fn pipe() -> io::Result<(OwnedFd, OwnedFd)> {
    unsafe {
        let mut result = MaybeUninit::<[OwnedFd; 2]>::uninit();
        ret(libc::pipe(result.as_mut_ptr().cast::<i32>()))?;
        let [p0, p1] = result.assume_init();
        Ok((p0, p1))
    }
}

#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "wasi")))]
pub(crate) fn pipe_with(flags: PipeFlags) -> io::Result<(OwnedFd, OwnedFd)> {
    unsafe {
        let mut result = MaybeUninit::<[OwnedFd; 2]>::uninit();
        ret(libc::pipe2(result.as_mut_ptr().cast::<i32>(), flags.bits()))?;
        let [p0, p1] = result.assume_init();
        Ok((p0, p1))
    }
}

#[inline]
pub(crate) fn poll(fds: &mut [PollFd<'_>], timeout: c_int) -> io::Result<usize> {
    let nfds = fds
        .len()
        .try_into()
        .map_err(|_convert_err| io::Error::INVAL)?;

    ret_c_int(unsafe { libc::poll(fds.as_mut_ptr().cast::<_>(), nfds, timeout) })
        .map(|nready| nready as usize)
}

#[cfg(any(target_os = "android", target_os = "linux"))]
pub(crate) unsafe fn userfaultfd(flags: UserfaultfdFlags) -> io::Result<OwnedFd> {
    syscall_ret_owned_fd(libc::syscall(libc::SYS_userfaultfd, flags.bits()))
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn recv(fd: BorrowedFd<'_>, buf: &mut [u8], flags: RecvFlags) -> io::Result<usize> {
    let nrecv = unsafe {
        ret_ssize_t(libc::recv(
            borrowed_fd(fd),
            buf.as_mut_ptr().cast::<_>(),
            buf.len(),
            flags.bits(),
        ))?
    };
    Ok(nrecv as usize)
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn send(fd: BorrowedFd<'_>, buf: &[u8], flags: SendFlags) -> io::Result<usize> {
    let nwritten = unsafe {
        ret_ssize_t(libc::send(
            borrowed_fd(fd),
            buf.as_ptr().cast::<_>(),
            buf.len(),
            flags.bits(),
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn recvfrom(
    fd: BorrowedFd<'_>,
    buf: &mut [u8],
    flags: RecvFlags,
) -> io::Result<(usize, SocketAddr)> {
    unsafe {
        let mut storage = MaybeUninit::<libc::sockaddr_storage>::uninit();
        let mut len = size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        let nread = ret_ssize_t(libc::recvfrom(
            borrowed_fd(fd),
            buf.as_mut_ptr().cast::<_>(),
            buf.len(),
            flags.bits(),
            storage.as_mut_ptr().cast::<_>(),
            &mut len,
        ))?;
        Ok((
            nread as usize,
            read_sockaddr_os(storage.as_ptr(), len.try_into().unwrap()),
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn sendto_v4(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrV4,
) -> io::Result<usize> {
    let nwritten = unsafe {
        ret_ssize_t(libc::sendto(
            borrowed_fd(fd),
            buf.as_ptr().cast::<_>(),
            buf.len(),
            flags.bits(),
            as_ptr(&encode_sockaddr_v4(addr)).cast::<libc::sockaddr>(),
            size_of::<SocketAddrV4>() as u32,
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn sendto_v6(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrV6,
) -> io::Result<usize> {
    let nwritten = unsafe {
        ret_ssize_t(libc::sendto(
            borrowed_fd(fd),
            buf.as_ptr().cast::<_>(),
            buf.len(),
            flags.bits(),
            as_ptr(&encode_sockaddr_v6(addr)).cast::<libc::sockaddr>(),
            size_of::<SocketAddrV6>() as u32,
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn sendto_unix(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrUnix,
) -> io::Result<usize> {
    let nwritten = unsafe {
        ret_ssize_t(libc::sendto(
            borrowed_fd(fd),
            buf.as_ptr().cast::<_>(),
            buf.len(),
            flags.bits(),
            as_ptr(&encode_sockaddr_unix(addr)).cast::<libc::sockaddr>(),
            size_of::<SocketAddrUnix>() as u32,
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn socket(
    domain: AddressFamily,
    type_: SocketType,
    protocol: Protocol,
) -> io::Result<OwnedFd> {
    unsafe {
        ret_owned_fd(libc::socket(
            domain.0 as c_int,
            type_.0 as c_int,
            protocol.0,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn socket_with(
    domain: AddressFamily,
    type_: SocketType,
    flags: SocketFlags,
    protocol: Protocol,
) -> io::Result<OwnedFd> {
    unsafe {
        ret_owned_fd(libc::socket(
            domain.0 as c_int,
            type_.0 as c_int | flags.bits(),
            protocol.0,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn bind_v4(sockfd: BorrowedFd<'_>, addr: &SocketAddrV4) -> io::Result<()> {
    unsafe {
        ret(libc::bind(
            borrowed_fd(sockfd),
            as_ptr(&encode_sockaddr_v4(addr)).cast::<_>(),
            size_of::<libc::sockaddr_in>() as libc::socklen_t,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn bind_v6(sockfd: BorrowedFd<'_>, addr: &SocketAddrV6) -> io::Result<()> {
    unsafe {
        ret(libc::bind(
            borrowed_fd(sockfd),
            as_ptr(&encode_sockaddr_v6(addr)).cast::<_>(),
            size_of::<libc::sockaddr_in6>() as libc::socklen_t,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn bind_unix(sockfd: BorrowedFd<'_>, addr: &SocketAddrUnix) -> io::Result<()> {
    unsafe {
        ret(libc::bind(
            borrowed_fd(sockfd),
            as_ptr(&encode_sockaddr_unix(addr)).cast::<_>(),
            size_of::<libc::sockaddr_un>() as libc::socklen_t,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn connect_v4(sockfd: BorrowedFd<'_>, addr: &SocketAddrV4) -> io::Result<()> {
    unsafe {
        ret(libc::connect(
            borrowed_fd(sockfd),
            as_ptr(&encode_sockaddr_v4(addr)).cast::<_>(),
            size_of::<libc::sockaddr_in>() as libc::socklen_t,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn connect_v6(sockfd: BorrowedFd<'_>, addr: &SocketAddrV6) -> io::Result<()> {
    unsafe {
        ret(libc::connect(
            borrowed_fd(sockfd),
            as_ptr(&encode_sockaddr_v6(addr)).cast::<_>(),
            size_of::<libc::sockaddr_in6>() as libc::socklen_t,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn connect_unix(sockfd: BorrowedFd<'_>, addr: &SocketAddrUnix) -> io::Result<()> {
    unsafe {
        ret(libc::connect(
            borrowed_fd(sockfd),
            as_ptr(&encode_sockaddr_unix(addr)).cast::<_>(),
            size_of::<libc::sockaddr_un>() as libc::socklen_t,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn listen(sockfd: BorrowedFd<'_>, backlog: c_int) -> io::Result<()> {
    unsafe { ret(libc::listen(borrowed_fd(sockfd), backlog)) }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn accept(sockfd: BorrowedFd<'_>) -> io::Result<OwnedFd> {
    unsafe {
        let owned_fd = ret_owned_fd(libc::accept(borrowed_fd(sockfd), null_mut(), null_mut()))?;
        Ok(owned_fd)
    }
}

#[cfg(not(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "redox",
    target_os = "wasi"
)))]
pub(crate) fn accept_with(sockfd: BorrowedFd<'_>, flags: AcceptFlags) -> io::Result<OwnedFd> {
    unsafe {
        let owned_fd = ret_owned_fd(libc::accept4(
            borrowed_fd(sockfd),
            null_mut(),
            null_mut(),
            flags.bits(),
        ))?;
        Ok(owned_fd)
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn acceptfrom(sockfd: BorrowedFd<'_>) -> io::Result<(OwnedFd, SocketAddr)> {
    unsafe {
        let mut storage = MaybeUninit::<libc::sockaddr_storage>::uninit();
        let mut len = size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        let owned_fd = ret_owned_fd(libc::accept(
            borrowed_fd(sockfd),
            storage.as_mut_ptr().cast::<_>(),
            &mut len,
        ))?;
        Ok((
            owned_fd,
            read_sockaddr_os(storage.as_ptr(), len.try_into().unwrap()),
        ))
    }
}

#[cfg(not(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "redox",
    target_os = "wasi"
)))]
pub(crate) fn acceptfrom_with(
    sockfd: BorrowedFd<'_>,
    flags: AcceptFlags,
) -> io::Result<(OwnedFd, SocketAddr)> {
    unsafe {
        let mut storage = MaybeUninit::<libc::sockaddr_storage>::uninit();
        let mut len = size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        let owned_fd = ret_owned_fd(libc::accept4(
            borrowed_fd(sockfd),
            storage.as_mut_ptr().cast::<_>(),
            &mut len,
            flags.bits(),
        ))?;
        Ok((
            owned_fd,
            read_sockaddr_os(storage.as_ptr(), len.try_into().unwrap()),
        ))
    }
}

/// Darwin lacks `accept4`, but does have `accept`. We define
/// `AcceptFlags` to have no flags, so we can discard it here.
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub(crate) fn accept_with(sockfd: BorrowedFd<'_>, _flags: AcceptFlags) -> io::Result<OwnedFd> {
    accept(sockfd)
}

/// Darwin lacks `accept4`, but does have `accept`. We define
/// `AcceptFlags` to have no flags, so we can discard it here.
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub(crate) fn acceptfrom_with(
    sockfd: BorrowedFd<'_>,
    _flags: AcceptFlags,
) -> io::Result<(OwnedFd, SocketAddr)> {
    acceptfrom(sockfd)
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn shutdown(sockfd: BorrowedFd<'_>, how: Shutdown) -> io::Result<()> {
    unsafe { ret(libc::shutdown(borrowed_fd(sockfd), how as c_int)) }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn getsockname(sockfd: BorrowedFd<'_>) -> io::Result<SocketAddr> {
    unsafe {
        let mut storage = MaybeUninit::<libc::sockaddr_storage>::uninit();
        let mut len = size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        ret(libc::getsockname(
            borrowed_fd(sockfd),
            storage.as_mut_ptr().cast::<_>(),
            &mut len,
        ))?;
        Ok(read_sockaddr_os(storage.as_ptr(), len.try_into().unwrap()))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn getpeername(sockfd: BorrowedFd<'_>) -> io::Result<SocketAddr> {
    unsafe {
        let mut storage = MaybeUninit::<libc::sockaddr_storage>::uninit();
        let mut len = size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        ret(libc::getpeername(
            borrowed_fd(sockfd),
            storage.as_mut_ptr().cast::<_>(),
            &mut len,
        ))?;
        Ok(read_sockaddr_os(storage.as_ptr(), len.try_into().unwrap()))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn socketpair(
    domain: AddressFamily,
    type_: SocketType,
    flags: SocketFlags,
    protocol: Protocol,
) -> io::Result<(OwnedFd, OwnedFd)> {
    unsafe {
        let mut fds = MaybeUninit::<[OwnedFd; 2]>::uninit();
        ret(libc::socketpair(
            domain.0 as c_int,
            type_.0 as c_int | flags.bits(),
            protocol.0,
            fds.as_mut_ptr().cast::<c_int>(),
        ))?;

        let [fd0, fd1] = fds.assume_init();
        Ok((fd0, fd1))
    }
}

#[cfg(target_os = "linux")]
pub(crate) fn getrandom(buf: &mut [u8], flags: GetRandomFlags) -> io::Result<usize> {
    let nread = unsafe {
        ret_ssize_t(libc::getrandom(
            buf.as_mut_ptr().cast::<_>(),
            buf.len(),
            flags.bits(),
        ))?
    };
    Ok(nread as usize)
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[inline]
#[must_use]
pub(crate) fn clock_getres(id: ClockId) -> Timespec {
    let mut timespec = MaybeUninit::<Timespec>::uninit();
    unsafe {
        let _ = libc::clock_getres(id as libc::clockid_t, timespec.as_mut_ptr());
        timespec.assume_init()
    }
}

#[cfg(not(target_os = "wasi"))]
#[inline]
#[must_use]
pub(crate) fn clock_gettime(id: ClockId) -> Timespec {
    let mut timespec = MaybeUninit::<Timespec>::uninit();
    // Use `unwrap()` here because `clock_getres` can fail if the clock itself
    // overflows a number of seconds, but if that happens, the monotonic clocks
    // can't maintain their invariants, or the realtime clocks aren't properly
    // configured.
    unsafe {
        ret(libc::clock_gettime(
            id as libc::clockid_t,
            timespec.as_mut_ptr(),
        ))
        .unwrap();
        timespec.assume_init()
    }
}

#[cfg(not(target_os = "wasi"))]
#[inline]
pub(crate) fn clock_gettime_dynamic(id: DynamicClockId) -> io::Result<Timespec> {
    let mut timespec = MaybeUninit::<Timespec>::uninit();
    unsafe {
        let id: libc::clockid_t = match id {
            DynamicClockId::Known(id) => id as libc::clockid_t,

            #[cfg(any(target_os = "android", target_os = "linux"))]
            DynamicClockId::Dynamic(fd) => {
                use io::AsRawFd;
                const CLOCKFD: i32 = 3;
                (!fd.as_raw_fd() << 3) | CLOCKFD
            }

            #[cfg(not(any(target_os = "android", target_os = "linux")))]
            DynamicClockId::Dynamic(_fd) => {
                // Dynamic clocks are not supported on this platform.
                return Err(io::Error::INVAL);
            }

            #[cfg(any(target_os = "android", target_os = "linux"))]
            DynamicClockId::RealtimeAlarm => libc::CLOCK_REALTIME_ALARM,

            #[cfg(any(target_os = "android", target_os = "linux"))]
            DynamicClockId::Tai => libc::CLOCK_TAI,

            #[cfg(any(target_os = "android", target_os = "linux"))]
            DynamicClockId::Boottime => libc::CLOCK_BOOTTIME,

            #[cfg(any(target_os = "android", target_os = "linux"))]
            DynamicClockId::BoottimeAlarm => libc::CLOCK_BOOTTIME_ALARM,
        };

        ret(libc::clock_gettime(
            id as libc::clockid_t,
            timespec.as_mut_ptr(),
        ))?;

        Ok(timespec.assume_init())
    }
}

#[cfg(not(any(
    target_os = "emscripten",
    target_os = "freebsd", // FreeBSD 12 has clock_nanosleep, but libc targets FreeBSD 11.
    target_os = "ios",
    target_os = "macos",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
#[inline]
pub(crate) fn clock_nanosleep_relative(id: ClockId, request: &Timespec) -> NanosleepRelativeResult {
    let mut remain = MaybeUninit::<Timespec>::uninit();
    let flags = 0;
    unsafe {
        match libc::clock_nanosleep(id as libc::clockid_t, flags, request, remain.as_mut_ptr()) {
            0 => NanosleepRelativeResult::Ok,
            err if err == io::Error::INTR.0 => {
                NanosleepRelativeResult::Interrupted(remain.assume_init())
            }
            err => NanosleepRelativeResult::Err(io::Error(err)),
        }
    }
}

#[cfg(not(any(
    target_os = "freebsd", // FreeBSD 12 has clock_nanosleep, but libc targets FreeBSD 11.
    target_os = "emscripten",
    target_os = "ios",
    target_os = "macos",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
#[inline]
pub(crate) fn clock_nanosleep_absolute(id: ClockId, request: &Timespec) -> io::Result<()> {
    let flags = libc::TIMER_ABSTIME;
    match unsafe { libc::clock_nanosleep(id as libc::clockid_t, flags, request, null_mut()) } {
        0 => Ok(()),
        err => Err(io::Error(err)),
    }
}

#[cfg(not(target_os = "redox"))]
#[inline]
pub(crate) fn nanosleep(request: &Timespec) -> NanosleepRelativeResult {
    let mut remain = MaybeUninit::<Timespec>::uninit();
    unsafe {
        match ret(libc::nanosleep(request, remain.as_mut_ptr())) {
            Ok(()) => NanosleepRelativeResult::Ok,
            Err(io::Error::INTR) => NanosleepRelativeResult::Interrupted(remain.assume_init()),
            Err(err) => NanosleepRelativeResult::Err(err),
        }
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

#[cfg(any(target_os = "android", target_os = "linux"))]
#[inline]
#[must_use]
pub(crate) fn gettid() -> Pid {
    unsafe {
        let tid: i32 = libc::gettid();
        Pid::from_raw(tid)
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

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) mod sockopt {
    use crate::net::sockopt::Timeout;
    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd"
    )))]
    use crate::net::Ipv6Addr;
    use crate::net::{Ipv4Addr, SocketType};
    use crate::{as_mut_ptr, io};
    use io_lifetimes::BorrowedFd;
    use std::convert::TryInto;
    use std::os::raw::{c_int, c_uint};
    use std::time::Duration;

    #[inline]
    fn getsockopt<T>(fd: BorrowedFd<'_>, level: i32, optname: i32) -> io::Result<T> {
        use super::*;
        unsafe {
            let mut value = MaybeUninit::<T>::uninit();
            let mut optlen = std::mem::size_of::<T>().try_into().unwrap();
            ret(libc::getsockopt(
                borrowed_fd(fd),
                level,
                optname,
                as_mut_ptr(&mut value).cast(),
                &mut optlen,
            ))?;
            assert_eq!(
                optlen as usize,
                size_of::<T>(),
                "unexpected getsockopt size"
            );
            Ok(value.assume_init())
        }
    }

    #[inline]
    fn setsockopt<T>(fd: BorrowedFd<'_>, level: i32, optname: i32, value: T) -> io::Result<()> {
        use super::*;
        unsafe {
            let optlen = std::mem::size_of::<T>().try_into().unwrap();
            ret(libc::setsockopt(
                borrowed_fd(fd),
                level,
                optname,
                as_ptr(&value).cast(),
                optlen,
            ))
        }
    }

    #[inline]
    pub(crate) fn get_socket_type(fd: BorrowedFd<'_>) -> io::Result<SocketType> {
        getsockopt(fd, libc::SOL_SOCKET as _, libc::SO_TYPE)
    }

    #[inline]
    pub(crate) fn set_socket_reuseaddr(fd: BorrowedFd<'_>, reuseaddr: bool) -> io::Result<()> {
        setsockopt(
            fd,
            libc::SOL_SOCKET as _,
            libc::SO_REUSEADDR,
            from_bool(reuseaddr),
        )
    }

    #[inline]
    pub(crate) fn set_socket_broadcast(fd: BorrowedFd<'_>, broadcast: bool) -> io::Result<()> {
        setsockopt(
            fd,
            libc::SOL_SOCKET as _,
            libc::SO_BROADCAST,
            from_bool(broadcast),
        )
    }

    #[inline]
    pub(crate) fn get_socket_broadcast(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(fd, libc::SOL_SOCKET as _, libc::SO_BROADCAST)
    }

    #[inline]
    pub(crate) fn set_socket_linger(
        fd: BorrowedFd<'_>,
        linger: Option<Duration>,
    ) -> io::Result<()> {
        let linger = libc::linger {
            l_onoff: linger.is_some() as c_int,
            l_linger: linger.unwrap_or_default().as_secs() as c_int,
        };
        setsockopt(fd, libc::SOL_SOCKET as _, libc::SO_LINGER, linger)
    }

    #[inline]
    pub(crate) fn get_socket_linger(fd: BorrowedFd<'_>) -> io::Result<Option<Duration>> {
        let linger: libc::linger = getsockopt(fd, libc::SOL_SOCKET as _, libc::SO_LINGER)?;
        Ok((linger.l_onoff != 0).then(|| Duration::from_secs(linger.l_linger as u64)))
    }

    #[cfg(any(target_os = "android", target_os = "linux"))]
    #[inline]
    pub(crate) fn set_socket_passcred(fd: BorrowedFd<'_>, passcred: bool) -> io::Result<()> {
        setsockopt(
            fd,
            libc::SOL_SOCKET as _,
            libc::SO_PASSCRED,
            from_bool(passcred),
        )
    }

    #[cfg(any(target_os = "android", target_os = "linux"))]
    #[inline]
    pub(crate) fn get_socket_passcred(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(fd, libc::SOL_SOCKET as _, libc::SO_PASSCRED)
    }

    #[inline]
    pub(crate) fn set_socket_timeout(
        fd: BorrowedFd<'_>,
        id: Timeout,
        timeout: Option<Duration>,
    ) -> io::Result<()> {
        let timeout = match timeout {
            Some(timeout) => {
                if timeout == Duration::ZERO {
                    return Err(io::Error::INVAL);
                }

                let tv_sec = timeout.as_secs().try_into();
                #[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))]
                let tv_sec = tv_sec.unwrap_or(std::os::raw::c_long::MAX);
                #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
                let tv_sec = tv_sec.unwrap_or(i64::MAX);

                let mut timeout = libc::timeval {
                    tv_sec,
                    tv_usec: timeout.subsec_micros() as _,
                };
                if timeout.tv_sec == 0 && timeout.tv_usec == 0 {
                    timeout.tv_usec = 1;
                }
                timeout
            }
            None => libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
        };
        let optname = match id {
            Timeout::Recv => libc::SO_RCVTIMEO,
            Timeout::Send => libc::SO_SNDTIMEO,
        };
        setsockopt(fd, libc::SOL_SOCKET, optname, timeout)
    }

    #[inline]
    pub(crate) fn get_socket_timeout(
        fd: BorrowedFd<'_>,
        id: Timeout,
    ) -> io::Result<Option<Duration>> {
        let optname = match id {
            Timeout::Recv => libc::SO_RCVTIMEO,
            Timeout::Send => libc::SO_SNDTIMEO,
        };
        let timeout: libc::timeval = getsockopt(fd, libc::SOL_SOCKET, optname)?;
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
        setsockopt(fd, libc::IPPROTO_IP as _, libc::IP_TTL, ttl)
    }

    #[inline]
    pub(crate) fn get_ip_ttl(fd: BorrowedFd<'_>) -> io::Result<u32> {
        getsockopt(fd, libc::IPPROTO_IP as _, libc::IP_TTL)
    }

    #[inline]
    pub(crate) fn set_ipv6_v6only(fd: BorrowedFd<'_>, only_v6: bool) -> io::Result<()> {
        setsockopt(
            fd,
            libc::IPPROTO_IPV6 as _,
            libc::IPV6_V6ONLY,
            from_bool(only_v6),
        )
    }

    #[inline]
    pub(crate) fn get_ipv6_v6only(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(fd, libc::IPPROTO_IPV6 as _, libc::IPV6_V6ONLY)
    }

    #[inline]
    pub(crate) fn set_ip_multicast_loop(
        fd: BorrowedFd<'_>,
        multicast_loop: bool,
    ) -> io::Result<()> {
        setsockopt(
            fd,
            libc::IPPROTO_IP as _,
            libc::IP_MULTICAST_LOOP,
            from_bool(multicast_loop),
        )
    }

    #[inline]
    pub(crate) fn get_ip_multicast_loop(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(fd, libc::IPPROTO_IP as _, libc::IP_MULTICAST_LOOP)
    }

    #[inline]
    pub(crate) fn set_ip_multicast_ttl(fd: BorrowedFd<'_>, multicast_ttl: u32) -> io::Result<()> {
        setsockopt(
            fd,
            libc::IPPROTO_IP as _,
            libc::IP_MULTICAST_TTL,
            multicast_ttl,
        )
    }

    #[inline]
    pub(crate) fn get_ip_multicast_ttl(fd: BorrowedFd<'_>) -> io::Result<u32> {
        getsockopt(fd, libc::IPPROTO_IP as _, libc::IP_MULTICAST_TTL)
    }

    #[inline]
    pub(crate) fn set_ipv6_multicast_loop(
        fd: BorrowedFd<'_>,
        multicast_loop: bool,
    ) -> io::Result<()> {
        setsockopt(
            fd,
            libc::IPPROTO_IPV6 as _,
            libc::IPV6_MULTICAST_LOOP,
            from_bool(multicast_loop),
        )
    }

    #[inline]
    pub(crate) fn get_ipv6_multicast_loop(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(fd, libc::IPPROTO_IPV6 as _, libc::IPV6_MULTICAST_LOOP)
    }

    #[inline]
    pub(crate) fn set_ip_add_membership(
        fd: BorrowedFd<'_>,
        multiaddr: &Ipv4Addr,
        interface: &Ipv4Addr,
    ) -> io::Result<()> {
        let mreq = to_imr(multiaddr, interface);
        setsockopt(fd, libc::IPPROTO_IP as _, libc::IP_ADD_MEMBERSHIP, mreq)
    }

    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    )))]
    #[inline]
    pub(crate) fn set_ipv6_join_group(
        fd: BorrowedFd<'_>,
        multiaddr: &Ipv6Addr,
        interface: u32,
    ) -> io::Result<()> {
        let mreq = to_ipv6mr(multiaddr, interface);
        setsockopt(fd, libc::IPPROTO_IPV6 as _, libc::IPV6_ADD_MEMBERSHIP, mreq)
    }

    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd"
    )))]
    #[inline]
    pub(crate) fn set_ip_drop_membership(
        fd: BorrowedFd<'_>,
        multiaddr: &Ipv4Addr,
        interface: &Ipv4Addr,
    ) -> io::Result<()> {
        let mreq = to_imr(multiaddr, interface);
        setsockopt(fd, libc::IPPROTO_IP as _, libc::IP_DROP_MEMBERSHIP, mreq)
    }

    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    #[inline]
    pub(crate) fn set_ipv6_leave_group(
        fd: BorrowedFd<'_>,
        multiaddr: &Ipv6Addr,
        interface: u32,
    ) -> io::Result<()> {
        let mreq = to_ipv6mr(multiaddr, interface);
        setsockopt(
            fd,
            libc::IPPROTO_IPV6 as _,
            libc::IPV6_DROP_MEMBERSHIP,
            mreq,
        )
    }

    #[inline]
    pub(crate) fn set_tcp_nodelay(fd: BorrowedFd<'_>, nodelay: bool) -> io::Result<()> {
        setsockopt(
            fd,
            libc::IPPROTO_TCP as _,
            libc::TCP_NODELAY,
            from_bool(nodelay),
        )
    }

    #[inline]
    pub(crate) fn get_tcp_nodelay(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(fd, libc::IPPROTO_TCP as _, libc::TCP_NODELAY)
    }

    #[inline]
    fn to_imr(multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> libc::ip_mreq {
        libc::ip_mreq {
            imr_multiaddr: to_imr_addr(multiaddr),
            imr_interface: to_imr_addr(interface),
        }
    }

    #[inline]
    fn to_imr_addr(addr: &Ipv4Addr) -> libc::in_addr {
        libc::in_addr {
            s_addr: u32::from_ne_bytes(addr.octets()),
        }
    }

    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd"
    )))]
    #[inline]
    fn to_ipv6mr(multiaddr: &Ipv6Addr, interface: u32) -> libc::ipv6_mreq {
        libc::ipv6_mreq {
            ipv6mr_multiaddr: to_ipv6mr_multiaddr(multiaddr),
            ipv6mr_interface: to_ipv6mr_interface(interface),
        }
    }

    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd"
    )))]
    #[inline]
    fn to_ipv6mr_multiaddr(multiaddr: &Ipv6Addr) -> libc::in6_addr {
        libc::in6_addr {
            s6_addr: multiaddr.octets(),
        }
    }

    #[cfg(target_os = "android")]
    #[inline]
    fn to_ipv6mr_interface(interface: u32) -> c_int {
        interface as c_int
    }

    #[cfg(not(any(
        target_os = "android",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd"
    )))]
    #[inline]
    fn to_ipv6mr_interface(interface: u32) -> c_uint {
        interface as c_uint
    }

    #[inline]
    fn from_bool(value: bool) -> c_uint {
        value as c_uint
    }
}
