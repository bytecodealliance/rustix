use super::super::fd::{
    AsFd, AsFilelike, AsRawFd, BorrowedFd, FromFd, FromRawFd, IntoFd, IntoRawFd, RawFd,
};
use crate::fs::Timestamps;
use super::super::time::Timespec;
use super::super::wasi_filesystem;
use super::{Access, Advice as FsAdvice, AtFlags, FallocateFlags, FdFlags, Mode, OFlags, Stat};
use crate::ffi::CStr;
use crate::io::{self, OwnedFd, SeekFrom};
use core::convert::TryInto;
use core::mem::{ManuallyDrop, MaybeUninit};

#[inline]
pub(crate) fn openat(
    dirfd: BorrowedFd<'_>,
    filename: &CStr,
    flags: OFlags,
    mode: Mode,
) -> io::Result<OwnedFd> {
    let all_read = Mode::IRUSR | Mode::IRGRP | Mode::IROTH;
    let all_write = Mode::IWUSR | Mode::IWGRP | Mode::IWOTH;
    let all_exe = Mode::IXUSR | Mode::IXGRP | Mode::IXOTH;

    let dir_desc = dirfd.as_filelike_view::<wasi_filesystem::Descriptor>();

    let fdflags = wasi_filesystem::Flags::from_bits_preserve((flags.bits() >> 8) as u8);
    let oflags = wasi_filesystem::OFlags::from_bits_preserve((flags.bits() & 0xff) as u8);

    let atflags = if flags.contains(OFlags::NOFOLLOW) {
        wasi_filesystem::AtFlags::empty()
    } else {
        wasi_filesystem::AtFlags::SYMLINK_FOLLOW
    };

    let mut open_mode = wasi_filesystem::Mode::empty();
    if (mode & all_read) == all_read {
        open_mode |= wasi_filesystem::Mode::READABLE;
    }
    if (mode & all_write) == all_write {
        open_mode |= wasi_filesystem::Mode::WRITEABLE;
    }
    if (mode & all_exe) == all_exe {
        open_mode |= wasi_filesystem::Mode::EXECUTABLE;
    }

    let desc = dir_desc.open_at(
        atflags,
        filename.to_str().expect("FIXME: handle invalid bytes"),
        oflags,
        fdflags,
        open_mode,
    )?;
    Ok(OwnedFd::from_fd(desc.into_fd()))
}

#[inline]
pub(crate) fn chmodat(dirfd: BorrowedFd<'_>, filename: &CStr, mode: Mode) -> io::Result<()> {
    todo!("chmodat")
}

#[inline]
pub(crate) fn fchmod(fd: BorrowedFd<'_>, mode: Mode) -> io::Result<()> {
    todo!("fchmod")
}

#[inline]
pub(crate) fn seek(fd: BorrowedFd<'_>, pos: SeekFrom) -> io::Result<u64> {
    todo!("seek")
}

#[inline]
pub(crate) fn tell(fd: BorrowedFd<'_>) -> io::Result<u64> {
    todo!("tell")
}

#[inline]
pub(crate) fn ftruncate(fd: BorrowedFd<'_>, length: u64) -> io::Result<()> {
    todo!("ftruncate")
}

#[inline]
pub(crate) fn fallocate(
    fd: BorrowedFd<'_>,
    mode: FallocateFlags,
    offset: u64,
    len: u64,
) -> io::Result<()> {
    todo!("fallocate")
}

#[inline]
pub(crate) fn fadvise(fd: BorrowedFd<'_>, pos: u64, len: u64, advice: FsAdvice) -> io::Result<()> {
    todo!("fadvise")
}

#[inline]
pub(crate) fn fsync(fd: BorrowedFd<'_>) -> io::Result<()> {
    fd.as_filelike_view::<wasi_filesystem::Descriptor>()
        .sync()?;
    Ok(())
}

#[inline]
pub(crate) fn fdatasync(fd: BorrowedFd<'_>) -> io::Result<()> {
    fd.as_filelike_view::<wasi_filesystem::Descriptor>()
        .datasync()?;
    Ok(())
}

#[inline]
pub(crate) fn fstat(fd: BorrowedFd<'_>) -> io::Result<Stat> {
    todo!("fstat")
}

#[inline]
pub(crate) fn statat(dirfd: BorrowedFd<'_>, filename: &CStr, flags: AtFlags) -> io::Result<Stat> {
    todo!("statat")
}

#[inline]
pub(crate) fn readlinkat(dirfd: BorrowedFd<'_>, path: &CStr, buf: &mut [u8]) -> io::Result<usize> {
    todo!("readlinkat")
}

#[inline]
pub(crate) fn fcntl_getfd(fd: BorrowedFd<'_>) -> io::Result<FdFlags> {
    todo!("fcntl_getfd")
}

#[inline]
pub(crate) fn fcntl_setfd(fd: BorrowedFd<'_>, flags: FdFlags) -> io::Result<()> {
    todo!("fcntl_setfd")
}

#[inline]
pub(crate) fn fcntl_getfl(fd: BorrowedFd<'_>) -> io::Result<OFlags> {
    todo!("fcntl_getfl")
}

#[inline]
pub(crate) fn fcntl_setfl(fd: BorrowedFd<'_>, flags: OFlags) -> io::Result<()> {
    todo!("fcntl_setfl")
}

#[inline]
pub(crate) fn renameat(
    old_dirfd: BorrowedFd<'_>,
    oldname: &CStr,
    new_dirfd: BorrowedFd<'_>,
    newname: &CStr,
) -> io::Result<()> {
    todo!("renameat")
}

#[inline]
pub(crate) fn unlinkat(dirfd: BorrowedFd<'_>, pathname: &CStr, flags: AtFlags) -> io::Result<()> {
    todo!("unlinkat")
}

#[inline]
pub(crate) fn linkat(
    old_dirfd: BorrowedFd<'_>,
    oldname: &CStr,
    new_dirfd: BorrowedFd<'_>,
    newname: &CStr,
    flags: AtFlags,
) -> io::Result<()> {
    todo!("linkat")
}

#[inline]
pub(crate) fn symlinkat(oldname: &CStr, dirfd: BorrowedFd<'_>, newname: &CStr) -> io::Result<()> {
    todo!("symlinkat")
}

#[inline]
pub(crate) fn mkdirat(dirfd: BorrowedFd<'_>, pathname: &CStr, mode: Mode) -> io::Result<()> {
    todo!("mkdirat")
}

#[inline]
pub(crate) fn getdents(fd: BorrowedFd<'_>, dirent: &mut [u8]) -> io::Result<usize> {
    todo!("getdents")
}

#[inline]
pub(crate) fn utimensat(
    dirfd: BorrowedFd<'_>,
    pathname: &CStr,
    utimes: &Timestamps,
    flags: AtFlags,
) -> io::Result<()> {
    todo!("utimensat")
}

#[inline]
pub(crate) fn futimens(fd: BorrowedFd<'_>, times: &Timestamps) -> io::Result<()> {
    todo!("futimens")
}

pub(crate) fn accessat(
    dirfd: BorrowedFd<'_>,
    path: &CStr,
    access: Access,
    flags: AtFlags,
) -> io::Result<()> {
    todo!("accessat")
}
