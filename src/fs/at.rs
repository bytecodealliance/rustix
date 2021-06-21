//! POSIX-style `*at` functions.

#[cfg(any(target_os = "macos", target_os = "ios"))]
use crate::fs::CloneFlags;
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
    crate::{negone_err, zero_ok},
    std::mem::MaybeUninit,
    unsafe_io::os::posish::{AsRawFd, FromRawFd, RawFd},
};

/// `openat(dirfd, path, oflags, mode)`
#[inline]
pub fn openat<'f, P: path::Arg, Fd: AsFd<'f>>(
    dirfd: Fd,
    path: P,
    oflags: OFlags,
    mode: Mode,
) -> io::Result<OwnedFd> {
    let dirfd = dirfd.as_fd();
    let path = path.into_c_str()?;
    _openat(dirfd, &path, oflags, mode)
}

#[cfg(libc)]
fn _openat(dirfd: BorrowedFd<'_>, path: &CStr, oflags: OFlags, mode: Mode) -> io::Result<OwnedFd> {
    unsafe {
        #[allow(clippy::useless_conversion)]
        let fd = negone_err(libc_openat(
            dirfd.as_raw_fd() as libc::c_int,
            path.as_ptr(),
            oflags.bits(),
            libc::c_uint::from(mode.bits()),
        ))?;

        Ok(OwnedFd::from_raw_fd(fd as RawFd))
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
pub fn readlinkat<'f, P: path::Arg, Fd: AsFd<'f>>(
    dirfd: Fd,
    path: P,
    reuse: OsString,
) -> io::Result<OsString> {
    let dirfd = dirfd.as_fd();
    let path = path.into_c_str()?;
    _readlinkat(dirfd, &path, reuse)
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
            negone_err(libc::readlinkat(
                dirfd.as_raw_fd() as libc::c_int,
                path.as_ptr(),
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
pub fn mkdirat<'f, P: path::Arg, Fd: AsFd<'f>>(dirfd: Fd, path: P, mode: Mode) -> io::Result<()> {
    let dirfd = dirfd.as_fd();
    let path = path.into_c_str()?;
    _mkdirat(dirfd, &path, mode)
}

#[cfg(libc)]
fn _mkdirat(dirfd: BorrowedFd<'_>, path: &CStr, mode: Mode) -> io::Result<()> {
    unsafe {
        zero_ok(libc::mkdirat(
            dirfd.as_raw_fd() as libc::c_int,
            path.as_ptr(),
            mode.bits(),
        ))
    }
}

#[cfg(linux_raw)]
#[inline]
fn _mkdirat(dirfd: BorrowedFd<'_>, path: &CStr, mode: Mode) -> io::Result<()> {
    crate::linux_raw::mkdirat(dirfd, path, mode.bits() as u16)
}

/// `linkat(old_dirfd, old_path, new_dirfd, new_path, flags)`
#[inline]
pub fn linkat<'pf, 'qf, P: path::Arg, Q: path::Arg, PFd: AsFd<'pf>, QFd: AsFd<'qf>>(
    old_dirfd: PFd,
    old_path: P,
    new_dirfd: QFd,
    new_path: Q,
    flags: AtFlags,
) -> io::Result<()> {
    let old_dirfd = old_dirfd.as_fd();
    let new_dirfd = new_dirfd.as_fd();
    let old_path = old_path.into_c_str()?;
    let new_path = new_path.into_c_str()?;
    _linkat(old_dirfd, &old_path, new_dirfd, &new_path, flags)
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
        zero_ok(libc::linkat(
            old_dirfd.as_raw_fd() as libc::c_int,
            old_path.as_ptr(),
            new_dirfd.as_raw_fd() as libc::c_int,
            new_path.as_ptr(),
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
pub fn unlinkat<'f, P: path::Arg, Fd: AsFd<'f>>(
    dirfd: Fd,
    path: P,
    flags: AtFlags,
) -> io::Result<()> {
    let dirfd = dirfd.as_fd();
    let path = path.into_c_str()?;
    _unlinkat(dirfd, &path, flags)
}

#[cfg(libc)]
fn _unlinkat(dirfd: BorrowedFd<'_>, path: &CStr, flags: AtFlags) -> io::Result<()> {
    unsafe {
        zero_ok(libc::unlinkat(
            dirfd.as_raw_fd() as libc::c_int,
            path.as_ptr(),
            flags.bits(),
        ))
    }
}

#[cfg(linux_raw)]
fn _unlinkat(dirfd: BorrowedFd<'_>, path: &CStr, flags: AtFlags) -> io::Result<()> {
    crate::linux_raw::unlinkat(dirfd, path, flags.bits())
}

/// `renameat(old_dirfd, old_path, new_dirfd, new_path)`
#[inline]
pub fn renameat<'pf, 'qf, P: path::Arg, Q: path::Arg, PFd: AsFd<'pf>, QFd: AsFd<'qf>>(
    old_dirfd: PFd,
    old_path: P,
    new_dirfd: QFd,
    new_path: Q,
) -> io::Result<()> {
    let old_dirfd = old_dirfd.as_fd();
    let new_dirfd = new_dirfd.as_fd();
    let old_path = old_path.into_c_str()?;
    let new_path = new_path.into_c_str()?;
    _renameat(old_dirfd, &old_path, new_dirfd, &new_path)
}

#[cfg(libc)]
fn _renameat(
    old_dirfd: BorrowedFd<'_>,
    old_path: &CStr,
    new_dirfd: BorrowedFd<'_>,
    new_path: &CStr,
) -> io::Result<()> {
    unsafe {
        zero_ok(libc::renameat(
            old_dirfd.as_raw_fd() as libc::c_int,
            old_path.as_ptr(),
            new_dirfd.as_raw_fd() as libc::c_int,
            new_path.as_ptr(),
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
pub fn symlinkat<'f, P: path::Arg, Q: path::Arg, Fd: AsFd<'f>>(
    old_path: P,
    new_dirfd: Fd,
    new_path: Q,
) -> io::Result<()> {
    let new_dirfd = new_dirfd.as_fd();
    let old_path = old_path.into_c_str()?;
    let new_path = new_path.into_c_str()?;
    _symlinkat(&old_path, new_dirfd, &new_path)
}

#[cfg(libc)]
fn _symlinkat(old_path: &CStr, new_dirfd: BorrowedFd<'_>, new_path: &CStr) -> io::Result<()> {
    unsafe {
        zero_ok(libc::symlinkat(
            old_path.as_ptr(),
            new_dirfd.as_raw_fd() as libc::c_int,
            new_path.as_ptr(),
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
pub fn statat<'f, P: path::Arg, Fd: AsFd<'f>>(
    dirfd: Fd,
    path: P,
    flags: AtFlags,
) -> io::Result<Stat> {
    let dirfd = dirfd.as_fd();
    let path = path.into_c_str()?;
    _statat(dirfd, &path, flags)
}

#[cfg(libc)]
fn _statat(dirfd: BorrowedFd<'_>, path: &CStr, flags: AtFlags) -> io::Result<Stat> {
    let mut stat = MaybeUninit::<Stat>::uninit();
    unsafe {
        zero_ok(libc_fstatat(
            dirfd.as_raw_fd() as libc::c_int,
            path.as_ptr(),
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
pub fn accessat<'f, P: path::Arg, Fd: AsFd<'f>>(
    dirfd: Fd,
    path: P,
    access: Access,
    flags: AtFlags,
) -> io::Result<()> {
    let dirfd = dirfd.as_fd();
    let path = path.into_c_str()?;
    _accessat(dirfd, &path, access, flags)
}

#[cfg(all(libc, not(target_os = "emscripten")))]
fn _accessat(dirfd: BorrowedFd<'_>, path: &CStr, access: Access, flags: AtFlags) -> io::Result<()> {
    unsafe {
        zero_ok(libc::faccessat(
            dirfd.as_raw_fd() as libc::c_int,
            path.as_ptr(),
            access.bits(),
            flags.bits(),
        ))
    }
}

// Temporarily disable on Emscripten until https://github.com/rust-lang/libc/pull/1836
// is available.
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
pub fn utimensat<'f, P: path::Arg, Fd: AsFd<'f>>(
    dirfd: Fd,
    path: P,
    times: &[Timespec; 2],
    flags: AtFlags,
) -> io::Result<()> {
    let dirfd = dirfd.as_fd();
    let path = path.into_c_str()?;
    _utimensat(dirfd, &path, times, flags)
}

#[cfg(libc)]
fn _utimensat(
    dirfd: BorrowedFd<'_>,
    path: &CStr,
    times: &[Timespec; 2],
    flags: AtFlags,
) -> io::Result<()> {
    unsafe {
        zero_ok(libc::utimensat(
            dirfd.as_raw_fd() as libc::c_int,
            path.as_ptr(),
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
    crate::linux_raw::utimensat(dirfd, Some(path), &times, flags.bits())
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
pub fn chmodat<'f, P: path::Arg, Fd: AsFd<'f>>(dirfd: Fd, path: P, mode: Mode) -> io::Result<()> {
    let dirfd = dirfd.as_fd();
    let path = path.into_c_str()?;
    _chmodat(dirfd, &path, mode)
}

#[cfg(all(
    libc,
    not(any(target_os = "android", target_os = "linux", target_os = "wasi"))
))]
fn _chmodat(dirfd: BorrowedFd<'_>, path: &CStr, mode: Mode) -> io::Result<()> {
    unsafe {
        zero_ok(libc::fchmodat(
            dirfd.as_raw_fd() as libc::c_int,
            path.as_ptr(),
            mode.bits(),
            0,
        ))
    }
}

#[cfg(all(libc, any(target_os = "android", target_os = "linux")))]
fn _chmodat(dirfd: BorrowedFd<'_>, path: &CStr, mode: Mode) -> io::Result<()> {
    // Note that Linux's `fchmodat` does not have a flags argument.
    unsafe {
        zero_ok(libc::syscall(
            libc::SYS_fchmodat,
            dirfd.as_raw_fd() as libc::c_int,
            path.as_ptr(),
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
pub fn fclonefileat<'f, 'dst_f, Fd: AsFd<'f>, DstFd: AsFd<'dst_f>, P: path::Arg>(
    src: Fd,
    dst_dir: DstFd,
    dst: P,
    flags: CloneFlags,
) -> io::Result<()> {
    let srcfd = src.as_fd();
    let dst_dirfd = dst_dir.as_fd();
    let dst = dst.into_c_str()?;
    _fclonefileat(srcfd.as_fd(), dst_dirfd.as_fd(), &dst, flags)
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

    unsafe { zero_ok(fclonefileat(srcfd, dst_dirfd, dst.as_ptr(), flags.bits())) }
}
