#[cfg(any(target_os = "android", target_os = "linux"))]
use super::super::conv::syscall_ret_owned_fd;
use super::super::conv::{
    borrowed_fd, no_fd, ret, ret_c_int, ret_discarded_fd, ret_owned_fd, ret_ssize_t,
};
use super::super::fd::{AsFd, BorrowedFd, RawFd};
#[cfg(not(target_os = "wasi"))]
use super::super::offset::libc_mmap;
use super::super::offset::{libc_pread, libc_pwrite};
#[cfg(not(target_os = "redox"))]
use super::super::offset::{libc_preadv, libc_pwritev};
#[cfg(all(target_os = "linux", target_env = "gnu"))]
use super::super::offset::{libc_preadv2, libc_pwritev2};
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
use super::Advice as IoAdvice;
#[cfg(target_os = "linux")]
use super::MremapFlags;
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "wasi")))]
use super::PipeFlags;
use super::PollFd;
#[cfg(not(target_os = "wasi"))]
use super::{DupFlags, MapFlags, MprotectFlags, ProtFlags, Termios, Winsize};
#[cfg(any(target_os = "android", target_os = "linux"))]
use super::{EventfdFlags, UserfaultfdFlags};
#[cfg(any(target_os = "android", target_os = "linux"))]
use super::{MlockFlags, ReadWriteFlags};
use crate::io::{self, OwnedFd};
use errno::errno;
use libc::{c_int, c_void};
use std::cmp::min;
use std::convert::TryInto;
#[cfg(not(any(target_os = "fuchsia", target_os = "wasi")))]
use std::ffi::CStr;
use std::io::{IoSlice, IoSliceMut};
use std::mem::MaybeUninit;
#[cfg(not(any(target_os = "redox", target_env = "newlib")))]
use std::sync::atomic::{AtomicUsize, Ordering};

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

pub(crate) unsafe fn close(raw_fd: RawFd) {
    let _ = libc::close(raw_fd as c_int);
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
