//! POSIX-style `*at` functions.

#[cfg(any(target_os = "macos", target_os = "ios"))]
use crate::fs::CloneFlags;
use crate::{
    fs::{Access, AtFlags, Mode, OFlags, Stat},
    negone_err, path,
    time::Timespec,
    zero_ok,
};
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
#[cfg(not(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re"
)))]
use libc::{fstatat as libc_fstatat, openat as libc_openat};
#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re"
))]
use libc::{fstatat64 as libc_fstatat, openat64 as libc_openat};
#[cfg(unix)]
use std::os::unix::ffi::OsStringExt;
#[cfg(target_os = "wasi")]
use std::os::wasi::ffi::OsStringExt;
use std::{
    convert::TryInto,
    ffi::{CStr, OsString},
    io,
    mem::{ManuallyDrop, MaybeUninit},
};
use unsafe_io::os::posish::{AsRawFd, FromRawFd, RawFd};

/// Return a "file" which holds a handle which refers to the process current
/// directory (`AT_FDCWD`). It is a `ManuallyDrop`, however the caller should
/// not drop it explicitly, as it refers to an ambient authority rather than
/// an open resource.
#[inline]
pub fn cwd() -> ManuallyDrop<OwnedFd> {
    ManuallyDrop::new(unsafe { OwnedFd::from_raw_fd(libc::AT_FDCWD as RawFd) })
}

/// `openat(dirfd, path, oflags, mode)`
#[inline]
pub fn openat<P: path::Arg, Fd: AsFd>(
    dirfd: &Fd,
    path: P,
    oflags: OFlags,
    mode: Mode,
) -> io::Result<OwnedFd> {
    let dirfd = dirfd.as_fd();
    let path = path.as_c_str()?;
    unsafe { _openat(dirfd, &path, oflags, mode) }
}

unsafe fn _openat(
    dirfd: BorrowedFd<'_>,
    path: &CStr,
    oflags: OFlags,
    mode: Mode,
) -> io::Result<OwnedFd> {
    #[allow(clippy::useless_conversion)]
    let fd = negone_err(libc_openat(
        dirfd.as_raw_fd() as libc::c_int,
        path.as_ptr(),
        oflags.bits(),
        libc::c_uint::from(mode.bits()),
    ))?;

    Ok(OwnedFd::from_raw_fd(fd as RawFd))
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
    let path = path.as_c_str()?;
    unsafe { _readlinkat(dirfd, &path, reuse) }
}

unsafe fn _readlinkat(dirfd: BorrowedFd<'_>, path: &CStr, reuse: OsString) -> io::Result<OsString> {
    let mut buffer = reuse.into_vec();

    // Start with a buffer big enough for the vast majority of paths.
    // This and the `reserve` below would be a good candidate for `try_reserve`.
    // https://github.com/rust-lang/rust/issues/48043
    buffer.clear();
    buffer.reserve(256);

    loop {
        let nread = negone_err(libc::readlinkat(
            dirfd.as_raw_fd() as libc::c_int,
            path.as_ptr(),
            buffer.as_mut_ptr().cast::<libc::c_char>(),
            buffer.capacity(),
        ))?;

        let nread = nread.try_into().unwrap();
        assert!(nread <= buffer.capacity());
        buffer.set_len(nread);
        if nread < buffer.capacity() {
            return Ok(OsString::from_vec(buffer));
        }

        // Use `Vec`'s builtin capacity-doubling strategy.
        buffer.reserve(1);
    }
}

/// `mkdirat(fd, path, mode)`
#[inline]
pub fn mkdirat<P: path::Arg, Fd: AsFd>(dirfd: &Fd, path: P, mode: Mode) -> io::Result<()> {
    let dirfd = dirfd.as_fd();
    let path = path.as_c_str()?;
    unsafe { _mkdirat(dirfd, &path, mode) }
}

unsafe fn _mkdirat(dirfd: BorrowedFd<'_>, path: &CStr, mode: Mode) -> io::Result<()> {
    zero_ok(libc::mkdirat(
        dirfd.as_raw_fd() as libc::c_int,
        path.as_ptr(),
        mode.bits(),
    ))
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
    let old_path = old_path.as_c_str()?;
    let new_path = new_path.as_c_str()?;
    unsafe { _linkat(old_dirfd, &old_path, new_dirfd, &new_path, flags) }
}

unsafe fn _linkat(
    old_dirfd: BorrowedFd<'_>,
    old_path: &CStr,
    new_dirfd: BorrowedFd<'_>,
    new_path: &CStr,
    flags: AtFlags,
) -> io::Result<()> {
    zero_ok(libc::linkat(
        old_dirfd.as_raw_fd() as libc::c_int,
        old_path.as_ptr(),
        new_dirfd.as_raw_fd() as libc::c_int,
        new_path.as_ptr(),
        flags.bits(),
    ))
}

/// `unlinkat(fd, path, flags)`
#[inline]
pub fn unlinkat<P: path::Arg, Fd: AsFd>(dirfd: &Fd, path: P, flags: AtFlags) -> io::Result<()> {
    let dirfd = dirfd.as_fd();
    let path = path.as_c_str()?;
    unsafe { _unlinkat(dirfd, &path, flags) }
}

unsafe fn _unlinkat(dirfd: BorrowedFd<'_>, path: &CStr, flags: AtFlags) -> io::Result<()> {
    zero_ok(libc::unlinkat(
        dirfd.as_raw_fd() as libc::c_int,
        path.as_ptr(),
        flags.bits(),
    ))
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
    let old_path = old_path.as_c_str()?;
    let new_path = new_path.as_c_str()?;
    unsafe { _renameat(old_dirfd, &old_path, new_dirfd, &new_path) }
}

unsafe fn _renameat(
    old_dirfd: BorrowedFd<'_>,
    old_path: &CStr,
    new_dirfd: BorrowedFd<'_>,
    new_path: &CStr,
) -> io::Result<()> {
    zero_ok(libc::renameat(
        old_dirfd.as_raw_fd() as libc::c_int,
        old_path.as_ptr(),
        new_dirfd.as_raw_fd() as libc::c_int,
        new_path.as_ptr(),
    ))
}

/// `symlinkat(old_dirfd, old_path, new_dirfd, new_path)`
#[inline]
pub fn symlinkat<P: path::Arg, Q: path::Arg, Fd: AsFd>(
    old_path: P,
    new_dirfd: &Fd,
    new_path: Q,
) -> io::Result<()> {
    let new_dirfd = new_dirfd.as_fd();
    let old_path = old_path.as_c_str()?;
    let new_path = new_path.as_c_str()?;
    unsafe { _symlinkat(&old_path, new_dirfd, &new_path) }
}

unsafe fn _symlinkat(
    old_path: &CStr,
    new_dirfd: BorrowedFd<'_>,
    new_path: &CStr,
) -> io::Result<()> {
    zero_ok(libc::symlinkat(
        old_path.as_ptr(),
        new_dirfd.as_raw_fd() as libc::c_int,
        new_path.as_ptr(),
    ))
}

/// `fstatat(dirfd, path, flags)`
#[inline]
pub fn statat<P: path::Arg, Fd: AsFd>(dirfd: &Fd, path: P, flags: AtFlags) -> io::Result<Stat> {
    let dirfd = dirfd.as_fd();
    let path = path.as_c_str()?;
    unsafe { _statat(dirfd, &path, flags) }
}

unsafe fn _statat(dirfd: BorrowedFd<'_>, path: &CStr, flags: AtFlags) -> io::Result<Stat> {
    let mut stat = MaybeUninit::<Stat>::uninit();
    zero_ok(libc_fstatat(
        dirfd.as_raw_fd() as libc::c_int,
        path.as_ptr(),
        stat.as_mut_ptr(),
        flags.bits(),
    ))?;
    Ok(stat.assume_init())
}

/// `faccessat(dirfd, path, access, flags)`
#[inline]
pub fn accessat<P: path::Arg, Fd: AsFd>(
    dirfd: &Fd,
    path: P,
    access: Access,
    flags: AtFlags,
) -> io::Result<()> {
    let dirfd = dirfd.as_fd();
    let path = path.as_c_str()?;
    unsafe { _accessat(dirfd, &path, access, flags) }
}

#[cfg(not(target_os = "emscripten"))]
unsafe fn _accessat(
    dirfd: BorrowedFd<'_>,
    path: &CStr,
    access: Access,
    flags: AtFlags,
) -> io::Result<()> {
    zero_ok(libc::faccessat(
        dirfd.as_raw_fd() as libc::c_int,
        path.as_ptr(),
        access.bits(),
        flags.bits(),
    ))
}

// Temporarily disable on Emscripten until https://github.com/rust-lang/libc/pull/1836
// is available.
#[cfg(target_os = "emscripten")]
unsafe fn _accessat(
    _dirfd: BorrowedFd<'_>,
    _path: &CStr,
    _access: Access,
    _flags: AtFlags,
) -> io::Result<()> {
    Ok(())
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
    let path = path.as_c_str()?;
    unsafe { _utimensat(dirfd, &path, times, flags) }
}

unsafe fn _utimensat(
    dirfd: BorrowedFd<'_>,
    path: &CStr,
    times: &[Timespec; 2],
    flags: AtFlags,
) -> io::Result<()> {
    zero_ok(libc::utimensat(
        dirfd.as_raw_fd() as libc::c_int,
        path.as_ptr(),
        times.as_ptr(),
        flags.bits(),
    ))
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
pub fn chmodat<P: path::Arg, Fd: AsFd>(dirfd: &Fd, path: P, mode: Mode) -> io::Result<()> {
    let dirfd = dirfd.as_fd();
    let path = path.as_c_str()?;
    unsafe { _chmodat(dirfd, &path, mode) }
}

#[cfg(not(any(target_os = "android", target_os = "linux", target_os = "wasi")))]
unsafe fn _chmodat(dirfd: BorrowedFd<'_>, path: &CStr, mode: Mode) -> io::Result<()> {
    zero_ok(libc::fchmodat(
        dirfd.as_raw_fd() as libc::c_int,
        path.as_ptr(),
        mode.bits(),
        0,
    ))
}

#[cfg(any(target_os = "android", target_os = "linux"))]
unsafe fn _chmodat(dirfd: BorrowedFd<'_>, path: &CStr, mode: Mode) -> io::Result<()> {
    // Note that Linux's `fchmodat` does not have a flags argument.
    zero_ok(libc::syscall(
        libc::SYS_fchmodat,
        dirfd.as_raw_fd() as libc::c_int,
        path.as_ptr(),
        mode.bits(),
    ))
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
    let dst = dst.as_c_str()?;
    unsafe { _fclonefileat(srcfd.as_raw_fd(), dst_dirfd.as_raw_fd(), &dst, flags) }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
unsafe fn _fclonefileat(
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

    zero_ok(fclonefileat(srcfd, dst_dirfd, dst.as_ptr(), flags.bits()))
}
