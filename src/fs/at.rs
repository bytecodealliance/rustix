//! POSIX-style `*at` functions.

#[cfg(any(target_os = "macos", target_os = "ios"))]
use crate::fs::CloneFlags;
#[cfg(not(any(
    target_os = "redox",
    target_os = "wasi",
    target_os = "macos",
    target_os = "ios"
)))]
use crate::fs::Dev;
#[cfg(all(libc, any(target_os = "android", target_os = "linux")))]
use crate::libc::conv::syscall_ret;
use crate::{
    fs::{Access, AtFlags, Mode, OFlags, Stat},
    io, path,
    time::Timespec,
};
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
#[cfg(all(
    libc,
    not(any(
        target_os = "android",
        target_os = "linux",
        target_os = "emscripten",
        target_os = "l4re"
    ))
))]
use libc::{fstatat as libc_fstatat, openat as libc_openat};
#[cfg(all(
    libc,
    any(
        target_os = "android",
        target_os = "linux",
        target_os = "emscripten",
        target_os = "l4re"
    )
))]
use libc::{fstatat64 as libc_fstatat, openat64 as libc_openat};
use std::ffi::{CStr, OsString};
#[cfg(unix)]
use std::os::unix::ffi::OsStringExt;
#[cfg(target_os = "wasi")]
use std::os::wasi::ffi::OsStringExt;
#[cfg(libc)]
use {
    crate::libc::conv::{borrowed_fd, c_str, ret, ret_owned_fd, ret_ssize_t},
    std::mem::MaybeUninit,
};

/// `openat(dirfd, path, oflags, mode)`
#[inline]
pub fn openat<P: path::Arg, Fd: AsFd>(
    dirfd: &Fd,
    path: P,
    oflags: OFlags,
    mode: Mode,
) -> io::Result<OwnedFd> {
    let dirfd = dirfd.as_fd();
    path.into_with_c_str(|path| _openat(dirfd, &path, oflags, mode))
}

#[cfg(libc)]
fn _openat(dirfd: BorrowedFd<'_>, path: &CStr, oflags: OFlags, mode: Mode) -> io::Result<OwnedFd> {
    unsafe {
        ret_owned_fd(libc_openat(
            borrowed_fd(dirfd),
            c_str(path),
            oflags.bits(),
            libc::c_uint::from(mode.bits()),
        ))
    }
}

#[cfg(linux_raw)]
#[inline]
fn _openat(dirfd: BorrowedFd<'_>, path: &CStr, oflags: OFlags, mode: Mode) -> io::Result<OwnedFd> {
    crate::linux_raw::openat(dirfd, path, oflags.bits(), mode.bits() as u16)
}

/// `readlinkat(fd, path)`
///
/// If `reuse` is non-empty, reuse its buffer to store the result if possible.
#[inline]
pub fn readlinkat<P: path::Arg, Fd: AsFd>(
    dirfd: &Fd,
    path: P,
    reuse: OsString,
) -> io::Result<OsString> {
    let dirfd = dirfd.as_fd();
    path.into_with_c_str(|path| _readlinkat(dirfd, &path, reuse))
}

#[cfg(libc)]
fn _readlinkat(dirfd: BorrowedFd<'_>, path: &CStr, reuse: OsString) -> io::Result<OsString> {
    // This code would benefit from having a better way to read into
    // uninitialized memory, but that requires `unsafe`.
    let mut buffer = reuse.into_vec();
    buffer.clear();
    buffer.resize(256, 0u8);

    loop {
        let nread = unsafe {
            ret_ssize_t(libc::readlinkat(
                borrowed_fd(dirfd),
                c_str(path),
                buffer.as_mut_ptr().cast::<libc::c_char>(),
                buffer.len(),
            ))?
        };

        let nread = nread as usize;
        assert!(nread <= buffer.len());
        if nread < buffer.len() {
            buffer.resize(nread, 0u8);
            return Ok(OsString::from_vec(buffer));
        }
        buffer.resize(buffer.len() * 2, 0u8);
    }
}

#[cfg(linux_raw)]
fn _readlinkat(dirfd: BorrowedFd<'_>, path: &CStr, reuse: OsString) -> io::Result<OsString> {
    // This code would benefit from having a better way to read into
    // uninitialized memory, but that requires `unsafe`.
    let mut buffer = reuse.into_vec();
    buffer.clear();
    buffer.resize(256, 0u8);

    loop {
        let nread = crate::linux_raw::readlinkat(dirfd, path, &mut buffer)?;

        let nread = nread as usize;
        assert!(nread <= buffer.len());
        if nread < buffer.len() {
            buffer.resize(nread, 0u8);
            return Ok(OsString::from_vec(buffer));
        }
        buffer.resize(buffer.len() * 2, 0u8);
    }
}

/// `mkdirat(fd, path, mode)`
#[inline]
pub fn mkdirat<P: path::Arg, Fd: AsFd>(dirfd: &Fd, path: P, mode: Mode) -> io::Result<()> {
    let dirfd = dirfd.as_fd();
    path.into_with_c_str(|path| _mkdirat(dirfd, &path, mode))
}

#[cfg(libc)]
fn _mkdirat(dirfd: BorrowedFd<'_>, path: &CStr, mode: Mode) -> io::Result<()> {
    unsafe { ret(libc::mkdirat(borrowed_fd(dirfd), c_str(path), mode.bits())) }
}

#[cfg(linux_raw)]
#[inline]
fn _mkdirat(dirfd: BorrowedFd<'_>, path: &CStr, mode: Mode) -> io::Result<()> {
    crate::linux_raw::mkdirat(dirfd, path, mode.bits() as u16)
}

/// `linkat(old_dirfd, old_path, new_dirfd, new_path, flags)`
#[inline]
pub fn linkat<P: path::Arg, Q: path::Arg, PFd: AsFd, QFd: AsFd>(
    old_dirfd: &PFd,
    old_path: P,
    new_dirfd: &QFd,
    new_path: Q,
    flags: AtFlags,
) -> io::Result<()> {
    let old_dirfd = old_dirfd.as_fd();
    let new_dirfd = new_dirfd.as_fd();
    old_path.into_with_c_str(|old_path| {
        new_path
            .into_with_c_str(|new_path| _linkat(old_dirfd, &old_path, new_dirfd, &new_path, flags))
    })
}

#[cfg(libc)]
fn _linkat(
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

#[cfg(linux_raw)]
#[inline]
fn _linkat(
    old_dirfd: BorrowedFd<'_>,
    old_path: &CStr,
    new_dirfd: BorrowedFd<'_>,
    new_path: &CStr,
    flags: AtFlags,
) -> io::Result<()> {
    crate::linux_raw::linkat(old_dirfd, old_path, new_dirfd, new_path, flags.bits())
}

/// `unlinkat(fd, path, flags)`
#[inline]
pub fn unlinkat<P: path::Arg, Fd: AsFd>(dirfd: &Fd, path: P, flags: AtFlags) -> io::Result<()> {
    let dirfd = dirfd.as_fd();
    path.into_with_c_str(|path| _unlinkat(dirfd, &path, flags))
}

#[cfg(libc)]
fn _unlinkat(dirfd: BorrowedFd<'_>, path: &CStr, flags: AtFlags) -> io::Result<()> {
    unsafe {
        ret(libc::unlinkat(
            borrowed_fd(dirfd),
            c_str(path),
            flags.bits(),
        ))
    }
}

#[cfg(linux_raw)]
#[inline]
fn _unlinkat(dirfd: BorrowedFd<'_>, path: &CStr, flags: AtFlags) -> io::Result<()> {
    crate::linux_raw::unlinkat(dirfd, path, flags.bits())
}

/// `renameat(old_dirfd, old_path, new_dirfd, new_path)`
#[inline]
pub fn renameat<P: path::Arg, Q: path::Arg, PFd: AsFd, QFd: AsFd>(
    old_dirfd: &PFd,
    old_path: P,
    new_dirfd: &QFd,
    new_path: Q,
) -> io::Result<()> {
    let old_dirfd = old_dirfd.as_fd();
    let new_dirfd = new_dirfd.as_fd();
    old_path.into_with_c_str(|old_path| {
        new_path.into_with_c_str(|new_path| _renameat(old_dirfd, &old_path, new_dirfd, &new_path))
    })
}

#[cfg(libc)]
fn _renameat(
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

#[cfg(linux_raw)]
#[inline]
fn _renameat(
    old_dirfd: BorrowedFd<'_>,
    old_path: &CStr,
    new_dirfd: BorrowedFd<'_>,
    new_path: &CStr,
) -> io::Result<()> {
    crate::linux_raw::renameat(old_dirfd, old_path, new_dirfd, new_path)
}

/// `symlinkat(old_dirfd, old_path, new_dirfd, new_path)`
#[inline]
pub fn symlinkat<P: path::Arg, Q: path::Arg, Fd: AsFd>(
    old_path: P,
    new_dirfd: &Fd,
    new_path: Q,
) -> io::Result<()> {
    let new_dirfd = new_dirfd.as_fd();
    old_path.into_with_c_str(|old_path| {
        new_path.into_with_c_str(|new_path| _symlinkat(&old_path, new_dirfd, &new_path))
    })
}

#[cfg(libc)]
fn _symlinkat(old_path: &CStr, new_dirfd: BorrowedFd<'_>, new_path: &CStr) -> io::Result<()> {
    unsafe {
        ret(libc::symlinkat(
            c_str(old_path),
            borrowed_fd(new_dirfd),
            c_str(new_path),
        ))
    }
}

#[cfg(linux_raw)]
#[inline]
fn _symlinkat(old_path: &CStr, new_dirfd: BorrowedFd<'_>, new_path: &CStr) -> io::Result<()> {
    crate::linux_raw::symlinkat(old_path, new_dirfd, new_path)
}

/// `fstatat(dirfd, path, flags)`
#[inline]
#[doc(alias = "fstatat")]
pub fn statat<P: path::Arg, Fd: AsFd>(dirfd: &Fd, path: P, flags: AtFlags) -> io::Result<Stat> {
    let dirfd = dirfd.as_fd();
    path.into_with_c_str(|path| _statat(dirfd, &path, flags))
}

#[cfg(libc)]
fn _statat(dirfd: BorrowedFd<'_>, path: &CStr, flags: AtFlags) -> io::Result<Stat> {
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

#[cfg(linux_raw)]
#[inline]
fn _statat(dirfd: BorrowedFd<'_>, path: &CStr, flags: AtFlags) -> io::Result<Stat> {
    crate::linux_raw::fstatat(dirfd, path, flags.bits())
}

/// `faccessat(dirfd, path, access, flags)`
#[inline]
#[doc(alias = "faccessat")]
pub fn accessat<P: path::Arg, Fd: AsFd>(
    dirfd: &Fd,
    path: P,
    access: Access,
    flags: AtFlags,
) -> io::Result<()> {
    let dirfd = dirfd.as_fd();
    path.into_with_c_str(|path| _accessat(dirfd, &path, access, flags))
}

#[cfg(all(libc, not(target_os = "emscripten")))]
fn _accessat(dirfd: BorrowedFd<'_>, path: &CStr, access: Access, flags: AtFlags) -> io::Result<()> {
    unsafe {
        ret(libc::faccessat(
            borrowed_fd(dirfd),
            c_str(path),
            access.bits(),
            flags.bits(),
        ))
    }
}

#[cfg(all(libc, target_os = "emscripten"))]
fn _accessat(
    _dirfd: BorrowedFd<'_>,
    _path: &CStr,
    _access: Access,
    _flags: AtFlags,
) -> io::Result<()> {
    Ok(())
}

#[cfg(linux_raw)]
#[inline]
fn _accessat(dirfd: BorrowedFd<'_>, path: &CStr, access: Access, flags: AtFlags) -> io::Result<()> {
    if flags.is_empty()
        || (flags.bits() == linux_raw_sys::v5_11::general::AT_EACCESS
            && crate::linux_raw::getuid() == crate::linux_raw::geteuid()
            && crate::linux_raw::getgid() == crate::linux_raw::getegid())
    {
        return crate::linux_raw::faccessat(dirfd, path, access.bits());
    }

    if flags.bits() != linux_raw_sys::v5_11::general::AT_EACCESS {
        return Err(io::Error::INVAL);
    }

    // TODO: Use faccessat2 in newer Linux versions.
    Err(io::Error::NOSYS)
}

/// `utimensat(dirfd, path, times, flags)`
#[inline]
pub fn utimensat<P: path::Arg, Fd: AsFd>(
    dirfd: &Fd,
    path: P,
    times: &[Timespec; 2],
    flags: AtFlags,
) -> io::Result<()> {
    let dirfd = dirfd.as_fd();
    path.into_with_c_str(|path| _utimensat(dirfd, &path, times, flags))
}

#[cfg(libc)]
fn _utimensat(
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

#[cfg(linux_raw)]
#[inline]
fn _utimensat(
    dirfd: BorrowedFd<'_>,
    path: &CStr,
    times: &[Timespec; 2],
    flags: AtFlags,
) -> io::Result<()> {
    crate::linux_raw::utimensat(dirfd, Some(path), times, flags.bits())
}

/// `fchmodat(dirfd, path, mode, 0)`
///
/// The flags argument is fixed to 0, so `AT_SYMLINK_NOFOLLOW` is not
/// supported. <details>
/// Platform support for this flag varies widely.
/// </details>
///
/// Note that this implementation does not support `O_PATH` file descriptors,
/// even on platforms where the host libc emulates it.
#[cfg(not(target_os = "wasi"))]
#[inline]
#[doc(alias = "fchmodat")]
pub fn chmodat<P: path::Arg, Fd: AsFd>(dirfd: &Fd, path: P, mode: Mode) -> io::Result<()> {
    let dirfd = dirfd.as_fd();
    path.into_with_c_str(|path| _chmodat(dirfd, &path, mode))
}

#[cfg(all(
    libc,
    not(any(target_os = "android", target_os = "linux", target_os = "wasi"))
))]
fn _chmodat(dirfd: BorrowedFd<'_>, path: &CStr, mode: Mode) -> io::Result<()> {
    unsafe {
        ret(libc::fchmodat(
            borrowed_fd(dirfd),
            c_str(path),
            mode.bits(),
            0,
        ))
    }
}

#[cfg(all(libc, any(target_os = "android", target_os = "linux")))]
fn _chmodat(dirfd: BorrowedFd<'_>, path: &CStr, mode: Mode) -> io::Result<()> {
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

#[cfg(linux_raw)]
#[inline]
fn _chmodat(dirfd: BorrowedFd<'_>, path: &CStr, mode: Mode) -> io::Result<()> {
    // Note that Linux's `fchmodat` does not have a flags argument.
    crate::linux_raw::fchmodat(dirfd, path, mode.bits() as u16)
}

/// `fclonefileat(src, dst_dir, dst, flags)`
#[cfg(any(target_os = "macos", target_os = "ios"))]
#[inline]
pub fn fclonefileat<Fd: AsFd, DstFd: AsFd, P: path::Arg>(
    src: &Fd,
    dst_dir: &DstFd,
    dst: P,
    flags: CloneFlags,
) -> io::Result<()> {
    let srcfd = src.as_fd();
    let dst_dirfd = dst_dir.as_fd();
    dst.into_with_c_str(|dst| _fclonefileat(srcfd.as_fd(), dst_dirfd.as_fd(), &dst, flags))
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn _fclonefileat(
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

/// `mknodat(dirfd, path, mode, dev)`
#[cfg(not(any(
    target_os = "redox",
    target_os = "wasi",
    target_os = "macos",
    target_os = "ios"
)))]
#[inline]
pub fn mknodat<P: path::Arg, Fd: AsFd>(
    dirfd: &Fd,
    path: P,
    mode: Mode,
    dev: Dev,
) -> io::Result<()> {
    let dirfd = dirfd.as_fd();
    path.into_with_c_str(|path| _mknodat(dirfd, &path, mode, dev))
}

#[cfg(all(
    libc,
    not(any(
        target_os = "redox",
        target_os = "wasi",
        target_os = "macos",
        target_os = "ios"
    ))
))]
fn _mknodat(dirfd: BorrowedFd<'_>, path: &CStr, mode: Mode, dev: Dev) -> io::Result<()> {
    unsafe {
        ret(libc::mknodat(
            borrowed_fd(dirfd),
            c_str(path),
            mode.bits(),
            dev,
        ))
    }
}

#[cfg(linux_raw)]
#[inline]
fn _mknodat(dirfd: BorrowedFd<'_>, path: &CStr, mode: Mode, dev: Dev) -> io::Result<()> {
    crate::linux_raw::mknodat(dirfd, path, mode.bits() as u16, dev)
}
