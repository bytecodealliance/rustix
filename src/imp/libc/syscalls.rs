#[cfg(not(any(
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
use super::conv::ret_u32;
#[cfg(any(target_os = "android", target_os = "linux"))]
use super::conv::{syscall_ret, syscall_ret_owned_fd, syscall_ret_ssize_t};
#[cfg(not(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox"
)))]
use super::fs::Advice;
#[cfg(not(any(
    target_os = "redox",
    target_os = "wasi",
    target_os = "macos",
    target_os = "ios"
)))]
use super::fs::Dev;
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "openbsd")))]
use super::fs::FallocateFlags;
#[cfg(any(target_os = "android", target_os = "linux"))]
use super::fs::ResolveFlags;
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "wasi")))]
// not implemented in libc for netbsd yet
use super::fs::StatFs;
#[cfg(all(target_os = "linux", target_env = "gnu"))]
use super::fs::{Statx, StatxFlags};
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "wasi")))]
use super::io::PipeFlags;
#[cfg(not(any(target_os = "wasi", target_os = "redox")))]
use super::net::{
    decode_sockaddr, AcceptFlags, AddressFamily, Protocol, RecvFlags, SendFlags, Shutdown,
    SocketAddr, SocketAddrUnix, SocketAddrV4, SocketAddrV6, SocketType,
};
#[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
use super::offset::libc_fallocate;
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "wasi")))]
use super::offset::libc_fstatfs;
#[cfg(not(target_os = "wasi"))]
use super::offset::libc_mmap;
#[cfg(not(any(
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "redox",
)))]
use super::offset::libc_posix_fadvise;
#[cfg(not(any(
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "redox",
    target_os = "linux",
    target_os = "android",
    target_os = "fuchsia",
)))]
use super::offset::libc_posix_fallocate;
#[cfg(target_os = "linux")]
use super::rand::GetRandomFlags;
use super::{
    conv::{borrowed_fd, ret, ret_c_int, ret_off_t, ret_owned_fd, ret_redundant_fd, ret_ssize_t},
    fs::{Access, FdFlags, Mode, OFlags, Stat},
    io::PollFd,
    offset::{libc_fstat, libc_fstatat, libc_lseek, libc_off_t, libc_pread, libc_pwrite},
    time::Timespec,
};
#[cfg(all(target_pointer_width = "64", target_os = "linux", target_env = "gnu"))]
use super::{
    io::ReadWriteFlags,
    offset::{libc_preadv2, libc_pwritev2},
};
use crate::{as_ptr, io};
use errno::errno;
use io_lifetimes::{BorrowedFd, OwnedFd};
#[cfg(not(any(target_os = "wasi", target_os = "fuchsia")))]
use std::ffi::OsString;
#[cfg(target_os = "linux")]
use std::mem::transmute;
#[cfg(all(unix, not(target_os = "fuchsia")))]
use std::os::unix::ffi::OsStringExt;
#[cfg(target_os = "wasi")]
use std::os::wasi::ffi::OsStringExt;
#[cfg(not(any(target_os = "redox", target_os = "wasi",)))]
use std::ptr::null_mut;
#[cfg(not(any(target_os = "redox", target_env = "newlib")))]
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{
    cmp::min,
    convert::TryInto,
    ffi::CStr,
    io::{IoSlice, IoSliceMut, SeekFrom},
    mem::{size_of, MaybeUninit},
    os::raw::{c_int, c_void},
};
#[cfg(not(target_os = "redox"))]
use {
    super::conv::c_str,
    super::fs::AtFlags,
    super::offset::{libc_openat, libc_preadv, libc_pwritev},
    crate::time::NanosleepRelativeResult,
};
#[cfg(any(target_os = "ios", target_os = "macos"))]
use {
    super::conv::nonnegative_ret,
    super::fs::{copyfile_state_t, CloneFlags, CopyfileFlags},
    std::path::PathBuf,
};
#[cfg(any(target_os = "android", target_os = "linux"))]
use {super::fs::MemfdFlags, super::io::UserFaultFdFlags};
#[cfg(not(target_os = "wasi"))]
use {
    super::io::{DupFlags, MapFlags, ProtFlags, Termios, Winsize},
    super::time::{ClockId, DynamicClockId},
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

#[cfg(all(target_pointer_width = "64", target_os = "linux", target_env = "gnu"))]
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

#[cfg(all(target_pointer_width = "64", target_os = "linux", target_env = "gnu"))]
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

#[cfg(not(target_os = "redox"))]
pub(crate) fn openat(
    dirfd: BorrowedFd<'_>,
    path: &CStr,
    oflags: OFlags,
    mode: Mode,
) -> io::Result<OwnedFd> {
    unsafe {
        ret_owned_fd(libc_openat(
            borrowed_fd(dirfd),
            c_str(path),
            oflags.bits(),
            libc::c_uint::from(mode.bits()),
        ))
    }
}

#[cfg(not(target_os = "redox"))]
#[inline]
pub(crate) fn readlinkat(dirfd: BorrowedFd<'_>, path: &CStr, buf: &mut [u8]) -> io::Result<usize> {
    unsafe {
        ret_ssize_t(libc::readlinkat(
            borrowed_fd(dirfd),
            c_str(path),
            buf.as_mut_ptr().cast::<libc::c_char>(),
            buf.len(),
        ))
        .map(|nread| nread as usize)
    }
}

#[cfg(not(target_os = "redox"))]
pub(crate) fn mkdirat(dirfd: BorrowedFd<'_>, path: &CStr, mode: Mode) -> io::Result<()> {
    unsafe { ret(libc::mkdirat(borrowed_fd(dirfd), c_str(path), mode.bits())) }
}

#[cfg(not(target_os = "redox"))]
pub(crate) fn linkat(
    old_dirfd: BorrowedFd<'_>,
    old_path: &CStr,
    new_dirfd: BorrowedFd<'_>,
    new_path: &CStr,
    flags: AtFlags,
) -> io::Result<()> {
    unsafe {
        ret(libc::linkat(
            borrowed_fd(old_dirfd),
            c_str(old_path),
            borrowed_fd(new_dirfd),
            c_str(new_path),
            flags.bits(),
        ))
    }
}

#[cfg(not(target_os = "redox"))]
pub(crate) fn unlinkat(dirfd: BorrowedFd<'_>, path: &CStr, flags: AtFlags) -> io::Result<()> {
    unsafe {
        ret(libc::unlinkat(
            borrowed_fd(dirfd),
            c_str(path),
            flags.bits(),
        ))
    }
}

#[cfg(not(target_os = "redox"))]
pub(crate) fn renameat(
    old_dirfd: BorrowedFd<'_>,
    old_path: &CStr,
    new_dirfd: BorrowedFd<'_>,
    new_path: &CStr,
) -> io::Result<()> {
    unsafe {
        ret(libc::renameat(
            borrowed_fd(old_dirfd),
            c_str(old_path),
            borrowed_fd(new_dirfd),
            c_str(new_path),
        ))
    }
}

#[cfg(not(target_os = "redox"))]
pub(crate) fn symlinkat(
    old_path: &CStr,
    new_dirfd: BorrowedFd<'_>,
    new_path: &CStr,
) -> io::Result<()> {
    unsafe {
        ret(libc::symlinkat(
            c_str(old_path),
            borrowed_fd(new_dirfd),
            c_str(new_path),
        ))
    }
}

#[cfg(not(target_os = "redox"))]
pub(crate) fn statat(dirfd: BorrowedFd<'_>, path: &CStr, flags: AtFlags) -> io::Result<Stat> {
    let mut stat = MaybeUninit::<Stat>::uninit();
    unsafe {
        ret(libc_fstatat(
            borrowed_fd(dirfd),
            c_str(path),
            stat.as_mut_ptr(),
            flags.bits(),
        ))?;
        Ok(stat.assume_init())
    }
}

#[cfg(not(any(target_os = "redox", target_os = "emscripten")))]
pub(crate) fn accessat(
    dirfd: BorrowedFd<'_>,
    path: &CStr,
    access: Access,
    flags: AtFlags,
) -> io::Result<()> {
    unsafe {
        ret(libc::faccessat(
            borrowed_fd(dirfd),
            c_str(path),
            access.bits(),
            flags.bits(),
        ))
    }
}

#[cfg(target_os = "emscripten")]
pub(crate) fn accessat(
    _dirfd: BorrowedFd<'_>,
    _path: &CStr,
    _access: Access,
    _flags: AtFlags,
) -> io::Result<()> {
    Ok(())
}

#[cfg(not(target_os = "redox"))]
pub(crate) fn utimensat(
    dirfd: BorrowedFd<'_>,
    path: &CStr,
    times: &[Timespec; 2],
    flags: AtFlags,
) -> io::Result<()> {
    unsafe {
        ret(libc::utimensat(
            borrowed_fd(dirfd),
            c_str(path),
            times.as_ptr(),
            flags.bits(),
        ))
    }
}

#[cfg(not(any(
    target_os = "android",
    target_os = "linux",
    target_os = "wasi",
    target_os = "redox"
)))]
pub(crate) fn chmodat(dirfd: BorrowedFd<'_>, path: &CStr, mode: Mode) -> io::Result<()> {
    unsafe {
        ret(libc::fchmodat(
            borrowed_fd(dirfd),
            c_str(path),
            mode.bits(),
            0,
        ))
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
pub(crate) fn chmodat(dirfd: BorrowedFd<'_>, path: &CStr, mode: Mode) -> io::Result<()> {
    // Note that Linux's `fchmodat` does not have a flags argument.
    unsafe {
        syscall_ret(libc::syscall(
            libc::SYS_fchmodat,
            borrowed_fd(dirfd),
            c_str(path),
            mode.bits(),
        ))
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) fn fclonefileat(
    srcfd: BorrowedFd<'_>,
    dst_dirfd: BorrowedFd<'_>,
    dst: &CStr,
    flags: CloneFlags,
) -> io::Result<()> {
    syscall! {
        fn fclonefileat(
            srcfd: BorrowedFd<'_>,
            dst_dirfd: BorrowedFd<'_>,
            dst: *const libc::c_char,
            flags: libc::c_int
        ) -> libc::c_int
    }

    unsafe { ret(fclonefileat(srcfd, dst_dirfd, c_str(dst), flags.bits())) }
}

#[cfg(not(any(
    target_os = "redox",
    target_os = "wasi",
    target_os = "macos",
    target_os = "ios"
)))]
pub(crate) fn mknodat(dirfd: BorrowedFd<'_>, path: &CStr, mode: Mode, dev: Dev) -> io::Result<()> {
    unsafe {
        ret(libc::mknodat(
            borrowed_fd(dirfd),
            c_str(path),
            mode.bits(),
            dev,
        ))
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
pub(crate) fn copy_file_range(
    fd_in: BorrowedFd<'_>,
    off_in: Option<&mut u64>,
    fd_out: BorrowedFd<'_>,
    off_out: Option<&mut u64>,
    len: u64,
) -> io::Result<u64> {
    assert_eq!(size_of::<libc::loff_t>(), size_of::<u64>());

    let mut off_in_val: libc::loff_t = 0;
    let mut off_out_val: libc::loff_t = 0;
    // Silently cast; we'll get `EINVAL` if the value is negative.
    let off_in_ptr = if let Some(off_in) = &off_in {
        off_in_val = (**off_in) as i64;
        &mut off_in_val
    } else {
        null_mut()
    };
    let off_out_ptr = if let Some(off_out) = &off_out {
        off_out_val = (**off_out) as i64;
        &mut off_out_val
    } else {
        null_mut()
    };
    let len: usize = len.try_into().unwrap_or(usize::MAX);
    let copied = unsafe {
        syscall_ret_ssize_t(libc::syscall(
            libc::SYS_copy_file_range,
            borrowed_fd(fd_in),
            off_in_ptr,
            borrowed_fd(fd_out),
            off_out_ptr,
            len,
            0, // no flags are defined yet
        ))?
    };
    if let Some(off_in) = off_in {
        *off_in = off_in_val as u64;
    }
    if let Some(off_out) = off_out {
        *off_out = off_out_val as u64;
    }
    Ok(copied as u64)
}

#[cfg(not(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox"
)))]
pub(crate) fn fadvise(fd: BorrowedFd<'_>, offset: u64, len: u64, advice: Advice) -> io::Result<()> {
    let offset = offset as i64;
    let len = len as i64;

    // FreeBSD returns `EINVAL` on invalid offsets; emulate the POSIX behavior.
    #[cfg(target_os = "freebsd")]
    let offset = if (offset as i64) < 0 {
        i64::MAX
    } else {
        offset
    };

    // FreeBSD returns `EINVAL` on overflow; emulate the POSIX behavior.
    #[cfg(target_os = "freebsd")]
    let len = if len > 0 && offset.checked_add(len).is_none() {
        i64::MAX - offset
    } else {
        len
    };

    let err = unsafe { libc_posix_fadvise(borrowed_fd(fd), offset, len, advice as libc::c_int) };

    // `posix_fadvise` returns its error status rather than using `errno`.
    return if err == 0 {
        Ok(())
    } else {
        Err(io::Error(err))
    };
}

pub(crate) fn fcntl_getfd(fd: BorrowedFd<'_>) -> io::Result<FdFlags> {
    unsafe {
        ret_c_int(libc::fcntl(borrowed_fd(fd), libc::F_GETFD)).map(FdFlags::from_bits_truncate)
    }
}

pub(crate) fn fcntl_setfd(fd: BorrowedFd<'_>, flags: FdFlags) -> io::Result<()> {
    unsafe { ret(libc::fcntl(borrowed_fd(fd), libc::F_SETFD, flags.bits())) }
}

pub(crate) fn fcntl_getfl(fd: BorrowedFd<'_>) -> io::Result<OFlags> {
    unsafe {
        ret_c_int(libc::fcntl(borrowed_fd(fd), libc::F_GETFL)).map(OFlags::from_bits_truncate)
    }
}

pub(crate) fn fcntl_setfl(fd: BorrowedFd<'_>, flags: OFlags) -> io::Result<()> {
    unsafe { ret(libc::fcntl(borrowed_fd(fd), libc::F_SETFL, flags.bits())) }
}

#[cfg(not(any(
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
pub(crate) fn fcntl_get_seals(fd: BorrowedFd<'_>) -> io::Result<u32> {
    unsafe { ret_u32(libc::fcntl(borrowed_fd(fd), libc::F_GET_SEALS)) }
}

#[cfg(not(target_os = "wasi"))]
pub(crate) fn fcntl_dupfd_cloexec(fd: BorrowedFd<'_>) -> io::Result<OwnedFd> {
    unsafe { ret_owned_fd(libc::fcntl(borrowed_fd(fd), libc::F_DUPFD_CLOEXEC)) }
}

pub(crate) fn seek(fd: BorrowedFd<'_>, pos: SeekFrom) -> io::Result<u64> {
    let (whence, offset): (libc::c_int, libc_off_t) = match pos {
        SeekFrom::Start(pos) => {
            let pos: u64 = pos;
            // Silently cast; we'll get `EINVAL` if the value is negative.
            (libc::SEEK_SET, pos as i64)
        }
        SeekFrom::End(offset) => (libc::SEEK_END, offset),
        SeekFrom::Current(offset) => (libc::SEEK_CUR, offset),
    };
    let offset = unsafe { ret_off_t(libc_lseek(borrowed_fd(fd), offset, whence))? };
    Ok(offset as u64)
}

pub(crate) fn tell(fd: BorrowedFd<'_>) -> io::Result<u64> {
    let offset = unsafe { ret_off_t(libc_lseek(borrowed_fd(fd), 0, libc::SEEK_CUR))? };
    Ok(offset as u64)
}

#[cfg(not(any(target_os = "android", target_os = "linux", target_os = "wasi")))]
pub(crate) fn fchmod(fd: BorrowedFd<'_>, mode: Mode) -> io::Result<()> {
    unsafe { ret(libc::fchmod(borrowed_fd(fd), mode.bits())) }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
pub(crate) fn fchmod(fd: BorrowedFd<'_>, mode: Mode) -> io::Result<()> {
    // Use `libc::syscall` rather than `libc::fchmod` because some libc
    // implementations, such as musl, add extra logic to `fchmod` to emulate
    // support for `O_PATH`, which uses `/proc` outside our control and
    // interferes with our own use of `O_PATH`.
    unsafe {
        syscall_ret(libc::syscall(
            libc::SYS_fchmod,
            borrowed_fd(fd),
            mode.bits(),
        ))
    }
}

pub(crate) fn fstat(fd: BorrowedFd<'_>) -> io::Result<Stat> {
    let mut stat = MaybeUninit::<Stat>::uninit();
    unsafe {
        ret(libc_fstat(borrowed_fd(fd), stat.as_mut_ptr()))?;
        Ok(stat.assume_init())
    }
}

#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "wasi")))] // not implemented in libc for netbsd yet
pub(crate) fn fstatfs(fd: BorrowedFd<'_>) -> io::Result<StatFs> {
    let mut statfs = MaybeUninit::<StatFs>::uninit();
    unsafe {
        ret(libc_fstatfs(borrowed_fd(fd), statfs.as_mut_ptr()))?;
        Ok(statfs.assume_init())
    }
}

pub(crate) fn futimens(fd: BorrowedFd<'_>, times: &[Timespec; 2]) -> io::Result<()> {
    unsafe { ret(libc::futimens(borrowed_fd(fd), times.as_ptr())) }
}

#[cfg(not(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox"
)))]
pub(crate) fn fallocate(
    fd: BorrowedFd<'_>,
    mode: FallocateFlags,
    offset: u64,
    len: u64,
) -> io::Result<()> {
    // Silently cast; we'll get `EINVAL` if the value is negative.
    let offset = offset as i64;
    let len = len as i64;

    #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
    unsafe {
        ret(libc_fallocate(borrowed_fd(fd), mode.bits(), offset, len))
    }

    #[cfg(not(any(target_os = "linux", target_os = "android", target_os = "fuchsia")))]
    {
        assert!(mode.is_empty());
        let err = unsafe { libc_posix_fallocate(borrowed_fd(fd), offset, len) };

        // `posix_fallocate` returns its error status rather than using `errno`.
        if err == 0 {
            Ok(())
        } else {
            Err(io::Error(err))
        }
    }
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
pub(crate) fn fallocate(
    fd: BorrowedFd<'_>,
    mode: FallocateFlags,
    offset: u64,
    len: u64,
) -> io::Result<()> {
    // Silently cast; we'll get `EINVAL` if the value is negative.
    let offset = offset as i64;
    let len = len as i64;

    assert!(mode.is_empty());

    let new_len = offset.checked_add(len).ok_or_else(|| io::Error::FBIG)?;
    let mut store = libc::fstore_t {
        fst_flags: libc::F_ALLOCATECONTIG,
        fst_posmode: libc::F_PEOFPOSMODE,
        fst_offset: 0,
        fst_length: new_len,
        fst_bytesalloc: 0,
    };
    unsafe {
        if libc::fcntl(borrowed_fd(fd), libc::F_PREALLOCATE, &store) == -1 {
            store.fst_flags = libc::F_ALLOCATEALL;
            let _ = ret_c_int(libc::fcntl(borrowed_fd(fd), libc::F_PREALLOCATE, &store))?;
        }
        ret(libc::ftruncate(borrowed_fd(fd), new_len))
    }
}

pub(crate) fn fsync(fd: BorrowedFd<'_>) -> io::Result<()> {
    unsafe { ret(libc::fsync(borrowed_fd(fd))) }
}

#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "redox")))]
pub(crate) fn fdatasync(fd: BorrowedFd<'_>) -> io::Result<()> {
    unsafe { ret(libc::fdatasync(borrowed_fd(fd))) }
}

pub(crate) fn ftruncate(fd: BorrowedFd<'_>, length: u64) -> io::Result<()> {
    let length = length.try_into().map_err(|_overflow_err| io::Error::FBIG)?;
    unsafe { ret(libc::ftruncate(borrowed_fd(fd), length)) }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
pub(crate) fn memfd_create(path: &CStr, flags: MemfdFlags) -> io::Result<OwnedFd> {
    unsafe {
        syscall_ret_owned_fd(libc::syscall(
            libc::SYS_memfd_create,
            c_str(path),
            flags.bits(),
        ))
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
pub(crate) fn openat2(
    dirfd: BorrowedFd<'_>,
    path: &CStr,
    oflags: OFlags,
    mode: Mode,
    resolve: ResolveFlags,
) -> io::Result<OwnedFd> {
    let oflags: i32 = oflags.bits();
    let open_how = OpenHow {
        oflag: u64::from(oflags as u32),
        mode: u64::from(mode.bits()),
        resolve: resolve.bits(),
    };

    unsafe {
        syscall_ret_owned_fd(libc::syscall(
            SYS_OPENAT2,
            borrowed_fd(dirfd),
            c_str(path),
            &open_how,
            SIZEOF_OPEN_HOW,
        ))
    }
}
#[cfg(all(
    target_pointer_width = "32",
    any(target_os = "android", target_os = "linux")
))]
const SYS_OPENAT2: i32 = 437;
#[cfg(all(
    target_pointer_width = "64",
    any(target_os = "android", target_os = "linux")
))]
const SYS_OPENAT2: i64 = 437;

#[cfg(any(target_os = "android", target_os = "linux"))]
#[repr(C)]
#[derive(Debug)]
struct OpenHow {
    oflag: u64,
    mode: u64,
    resolve: u64,
}
#[cfg(any(target_os = "android", target_os = "linux"))]
const SIZEOF_OPEN_HOW: usize = std::mem::size_of::<OpenHow>();

#[cfg(target_os = "linux")]
pub(crate) fn sendfile(
    out_fd: BorrowedFd<'_>,
    in_fd: BorrowedFd<'_>,
    offset: Option<&mut u64>,
    count: usize,
) -> io::Result<usize> {
    unsafe {
        let nsent = ret_ssize_t(libc::sendfile64(
            borrowed_fd(out_fd),
            borrowed_fd(in_fd),
            transmute(offset),
            count,
        ))?;
        Ok(nsent as usize)
    }
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub(crate) fn statx(
    dirfd: BorrowedFd<'_>,
    path: &CStr,
    flags: AtFlags,
    mask: StatxFlags,
) -> io::Result<Statx> {
    weakcall! {
        fn statx(
            dirfd: BorrowedFd<'_>,
            path: *const libc::c_char,
            flags: libc::c_int,
            mask: libc::c_uint,
            buf: *mut Statx
        ) -> libc::c_int
    }

    let mut statx_buf = MaybeUninit::<Statx>::uninit();
    unsafe {
        ret(statx(
            dirfd,
            c_str(path),
            flags.bits(),
            mask.bits(),
            statx_buf.as_mut_ptr(),
        ))?;
        Ok(statx_buf.assume_init())
    }
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
pub(crate) fn dup2(fd: BorrowedFd<'_>, new: BorrowedFd<'_>) -> io::Result<()> {
    unsafe { ret_redundant_fd(libc::dup2(borrowed_fd(fd), borrowed_fd(new))) }
}

#[cfg(not(any(
    target_os = "android",
    target_os = "macos",
    target_os = "ios",
    target_os = "redox",
    target_os = "wasi"
)))]
pub(crate) fn dup2_with(
    fd: BorrowedFd<'_>,
    new: BorrowedFd<'_>,
    flags: DupFlags,
) -> io::Result<()> {
    unsafe { ret_redundant_fd(libc::dup3(borrowed_fd(fd), borrowed_fd(new), flags.bits())) }
}

#[cfg(any(
    target_os = "android",
    target_os = "macos",
    target_os = "ios",
    target_os = "redox"
))]
pub(crate) fn dup2_with(
    fd: BorrowedFd<'_>,
    new: BorrowedFd<'_>,
    _flags: DupFlags,
) -> io::Result<()> {
    // Android 5.0 has dup3, but libc doesn't have bindings
    dup2(fd, new)
}

#[cfg(not(any(target_os = "wasi", target_os = "fuchsia")))]
pub(crate) fn ttyname(dirfd: BorrowedFd<'_>, reuse: OsString) -> io::Result<OsString> {
    // This code would benefit from having a better way to read into
    // uninitialized memory, but that requires `unsafe`.
    let mut buffer = reuse.into_vec();
    buffer.clear();
    buffer.resize(256, 0u8);

    loop {
        match unsafe {
            libc::ttyname_r(
                borrowed_fd(dirfd),
                buffer.as_mut_ptr().cast::<libc::c_char>(),
                buffer.len(),
            )
        } {
            libc::ERANGE => buffer.resize(buffer.len() * 2, 0u8),
            0 => {
                let len = buffer.iter().position(|x| *x == b'\0').unwrap();
                buffer.resize(len, 0u8);
                return Ok(OsString::from_vec(buffer));
            }
            errno => return Err(io::Error(errno)),
        }
    }
}

#[cfg(not(any(
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
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
    target_os = "netbsd"
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

#[cfg(not(target_os = "wasi"))]
pub(crate) unsafe fn munmap(ptr: *mut c_void, len: usize) -> io::Result<()> {
    ret(libc::munmap(ptr, len))
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
pub(crate) unsafe fn userfaultfd(flags: UserFaultFdFlags) -> io::Result<OwnedFd> {
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
        Ok((nread as usize, decode_sockaddr(storage.as_ptr(), len)))
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
            as_ptr(&addr.encode()).cast::<libc::sockaddr>(),
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
            as_ptr(&addr.encode()).cast::<libc::sockaddr>(),
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
            as_ptr(&addr.encode()).cast::<libc::sockaddr>(),
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
            protocol as c_int,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn bind_v4(sockfd: BorrowedFd<'_>, addr: &SocketAddrV4) -> io::Result<()> {
    unsafe {
        ret(libc::bind(
            borrowed_fd(sockfd),
            as_ptr(&addr.encode()).cast::<_>(),
            size_of::<libc::sockaddr_in>() as libc::socklen_t,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn bind_v6(sockfd: BorrowedFd<'_>, addr: &SocketAddrV6) -> io::Result<()> {
    unsafe {
        ret(libc::bind(
            borrowed_fd(sockfd),
            as_ptr(&addr.encode()).cast::<_>(),
            size_of::<libc::sockaddr_in6>() as libc::socklen_t,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn bind_unix(sockfd: BorrowedFd<'_>, addr: &SocketAddrUnix) -> io::Result<()> {
    unsafe {
        ret(libc::bind(
            borrowed_fd(sockfd),
            as_ptr(&addr.encode()).cast::<_>(),
            size_of::<libc::sockaddr_un>() as libc::socklen_t,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn connect_v4(sockfd: BorrowedFd<'_>, addr: &SocketAddrV4) -> io::Result<()> {
    unsafe {
        ret(libc::connect(
            borrowed_fd(sockfd),
            as_ptr(&addr.encode()).cast::<_>(),
            size_of::<libc::sockaddr_in>() as libc::socklen_t,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn connect_v6(sockfd: BorrowedFd<'_>, addr: &SocketAddrV6) -> io::Result<()> {
    unsafe {
        ret(libc::connect(
            borrowed_fd(sockfd),
            as_ptr(&addr.encode()).cast::<_>(),
            size_of::<libc::sockaddr_in6>() as libc::socklen_t,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn connect_unix(sockfd: BorrowedFd<'_>, addr: &SocketAddrUnix) -> io::Result<()> {
    unsafe {
        ret(libc::connect(
            borrowed_fd(sockfd),
            as_ptr(&addr.encode()).cast::<_>(),
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
        Ok((owned_fd, decode_sockaddr(storage.as_ptr(), len)))
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
        Ok((owned_fd, decode_sockaddr(storage.as_ptr(), len)))
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
pub(crate) fn getsockopt_socket_type(fd: BorrowedFd<'_>) -> io::Result<SocketType> {
    let mut buffer = MaybeUninit::<SocketType>::uninit();
    let mut out_len = size_of::<SocketType>() as libc::socklen_t;
    unsafe {
        ret(libc::getsockopt(
            borrowed_fd(fd),
            libc::SOL_SOCKET,
            libc::SO_TYPE,
            buffer.as_mut_ptr().cast::<libc::c_void>(),
            &mut out_len,
        ))?;
        assert_eq!(
            out_len as usize,
            size_of::<SocketType>(),
            "unexpected SocketType size"
        );
        Ok(buffer.assume_init())
    }
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
        Ok(decode_sockaddr(storage.as_ptr(), len))
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
        Ok(decode_sockaddr(storage.as_ptr(), len))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn socketpair(
    domain: AddressFamily,
    type_: SocketType,
    accept_flags: AcceptFlags,
    protocol: Protocol,
) -> io::Result<(OwnedFd, OwnedFd)> {
    unsafe {
        let mut fds = MaybeUninit::<[OwnedFd; 2]>::uninit();
        ret(libc::socketpair(
            domain.0 as c_int,
            type_.0 as c_int | accept_flags.bits(),
            protocol as c_int,
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

#[cfg(not(any(target_os = "wasi", target_os = "redox")))]
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
#[must_use]
pub(crate) fn clock_gettime_dynamic(id: DynamicClockId) -> io::Result<Timespec> {
    let mut timespec = MaybeUninit::<Timespec>::uninit();
    unsafe {
        let id: libc::clockid_t = match id {
            DynamicClockId::Known(id) => id as libc::clockid_t,

            #[cfg(any(target_os = "android", target_os = "linux"))]
            DynamicClockId::Dynamic(fd) => {
                use crate::io::AsRawFd;
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
    target_os = "macos",
    target_os = "ios",
    target_os = "ios",
    target_os = "redox",
    target_os = "freebsd", // FreeBSD 12 has clock_nanosleep, but libc targets FreeBSD 11.
    target_os = "openbsd",
    target_os = "emscripten",
    target_os = "wasi",
)))]
#[inline]
#[must_use]
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
    target_os = "macos",
    target_os = "ios",
    target_os = "ios",
    target_os = "redox",
    target_os = "freebsd", // FreeBSD 12 has clock_nanosleep, but libc targets FreeBSD 11.
    target_os = "openbsd",
    target_os = "emscripten",
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
#[must_use]
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

#[cfg(any(target_os = "ios", target_os = "macos"))]
pub(crate) unsafe fn fcopyfile(
    from: BorrowedFd<'_>,
    to: BorrowedFd<'_>,
    state: copyfile_state_t,
    flags: CopyfileFlags,
) -> io::Result<()> {
    extern "C" {
        fn fcopyfile(
            from: libc::c_int,
            to: libc::c_int,
            state: copyfile_state_t,
            flags: libc::c_uint,
        ) -> libc::c_int;
    }

    nonnegative_ret(fcopyfile(
        borrowed_fd(from),
        borrowed_fd(to),
        state,
        flags.bits(),
    ))
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
pub(crate) fn copyfile_state_alloc() -> io::Result<copyfile_state_t> {
    extern "C" {
        fn copyfile_state_alloc() -> copyfile_state_t;
    }

    let result = unsafe { copyfile_state_alloc() };
    if result.0.is_null() {
        Err(io::Error::last_os_error())
    } else {
        Ok(result)
    }
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
pub(crate) unsafe fn copyfile_state_free(state: copyfile_state_t) -> io::Result<()> {
    extern "C" {
        fn copyfile_state_free(state: copyfile_state_t) -> libc::c_int;
    }

    nonnegative_ret(copyfile_state_free(state))
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
const COPYFILE_STATE_COPIED: u32 = 8;

#[cfg(any(target_os = "ios", target_os = "macos"))]
pub(crate) unsafe fn copyfile_state_get_copied(state: copyfile_state_t) -> io::Result<u64> {
    let mut copied = MaybeUninit::<u64>::uninit();
    copyfile_state_get(state, COPYFILE_STATE_COPIED, copied.as_mut_ptr().cast())?;
    Ok(copied.assume_init())
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
pub(crate) unsafe fn copyfile_state_get(
    state: copyfile_state_t,
    flag: u32,
    dst: *mut libc::c_void,
) -> io::Result<()> {
    extern "C" {
        fn copyfile_state_get(
            state: copyfile_state_t,
            flag: u32,
            dst: *mut libc::c_void,
        ) -> libc::c_int;
    }

    nonnegative_ret(copyfile_state_get(state, flag, dst))
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
pub(crate) fn getpath(fd: BorrowedFd<'_>) -> io::Result<PathBuf> {
    // The use of PATH_MAX is generally not encouraged, but it
    // is inevitable in this case because macOS defines `fcntl` with
    // `F_GETPATH` in terms of `MAXPATHLEN`, and there are no
    // alternatives. If a better method is invented, it should be used
    // instead.
    let mut buf = vec![0; libc::PATH_MAX as usize];

    // From the macOS `fcntl` man page:
    // `F_GETPATH` - Get the path of the file descriptor `Fildes`. The argument
    //               must be a buffer of size `MAXPATHLEN` or greater.
    //
    // https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fcntl.2.html
    unsafe {
        ret(libc::fcntl(
            borrowed_fd(fd),
            libc::F_GETPATH,
            buf.as_mut_ptr(),
        ))?;
    }

    let l = buf.iter().position(|&c| c == 0).unwrap();
    buf.truncate(l as usize);
    buf.shrink_to_fit();
    Ok(PathBuf::from(std::ffi::OsString::from_vec(buf)))
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
pub(crate) fn fcntl_rdadvise(fd: BorrowedFd<'_>, offset: u64, len: u64) -> io::Result<()> {
    // From the macOS `fcntl` man page:
    // `F_RDADVISE` - Issue an advisory read async with no copy to user.
    //
    // The `F_RDADVISE` command operates on the following structure which holds
    // information passed from the user to the system:
    //
    // ```
    // struct radvisory {
    //      off_t   ra_offset;  /* offset into the file */
    //      int     ra_count;   /* size of the read     */
    // };
    // ```
    //
    // https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fcntl.2.html
    let ra_offset = match offset.try_into() {
        Ok(len) => len,
        // If this conversion fails, the user is providing an offset outside
        // any possible file extent, so just ignore it.
        Err(_) => return Ok(()),
    };
    let ra_count = match len.try_into() {
        Ok(len) => len,
        // If this conversion fails, the user is providing a dubiously large
        // hint which is unlikely to improve performance.
        Err(_) => return Ok(()),
    };
    unsafe {
        let radvisory = libc::radvisory {
            ra_offset,
            ra_count,
        };
        ret(libc::fcntl(borrowed_fd(fd), libc::F_RDADVISE, &radvisory))
    }
}

#[cfg(not(target_os = "wasi"))]
#[inline]
#[must_use]
pub(crate) fn getuid() -> u32 {
    unsafe { libc::getuid() }
}

#[cfg(not(target_os = "wasi"))]
#[inline]
#[must_use]
pub(crate) fn geteuid() -> u32 {
    unsafe { libc::geteuid() }
}

#[cfg(not(target_os = "wasi"))]
#[inline]
#[must_use]
pub(crate) fn getgid() -> u32 {
    unsafe { libc::getgid() }
}

#[cfg(not(target_os = "wasi"))]
#[inline]
#[must_use]
pub(crate) fn getegid() -> u32 {
    unsafe { libc::getegid() }
}

#[cfg(not(target_os = "wasi"))]
#[inline]
#[must_use]
pub(crate) fn getpid() -> u32 {
    let pid: i32 = unsafe { libc::getpid() };
    pid as u32
}

#[cfg(not(target_os = "wasi"))]
#[inline]
#[must_use]
pub(crate) fn getppid() -> u32 {
    let pid: i32 = unsafe { libc::getppid() };
    pid as u32
}

#[inline]
pub(crate) fn sched_yield() {
    unsafe {
        let _ = libc::sched_yield();
    }
}
