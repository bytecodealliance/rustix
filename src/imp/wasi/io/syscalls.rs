use super::super::fd::{AsFilelike, BorrowedFd, FromRawFd, OwnedFd, RawFd};
use super::super::wasi_filesystem;
use super::PollFd;
use crate::io::{self, IoSlice, IoSliceMut};

#[inline]
pub(crate) fn read(fd: BorrowedFd<'_>, buf: &mut [u8]) -> io::Result<usize> {
    let nread = fd
        .as_filelike_view::<wasi_filesystem::Descriptor>()
        .read(buf)?;
    Ok(nread as usize)
}

#[inline]
pub(crate) fn write(fd: BorrowedFd<'_>, buf: &[u8]) -> io::Result<usize> {
    let nwritten = fd
        .as_filelike_view::<wasi_filesystem::Descriptor>()
        .write(buf)?;
    Ok(nwritten as usize)
}

#[inline]
pub(crate) fn pread(fd: BorrowedFd<'_>, buf: &mut [u8], offset: u64) -> io::Result<usize> {
    let nread = fd
        .as_filelike_view::<wasi_filesystem::Descriptor>()
        .pread(buf, offset)?;
    Ok(nread as usize)
}

#[inline]
pub(crate) fn pwrite(fd: BorrowedFd<'_>, buf: &[u8], offset: u64) -> io::Result<usize> {
    let nwritten = fd
        .as_filelike_view::<wasi_filesystem::Descriptor>()
        .pwrite(buf, offset)?;
    Ok(nwritten as usize)
}

pub(crate) fn readv(fd: BorrowedFd<'_>, bufs: &[IoSliceMut]) -> io::Result<usize> {
    todo!("readv")
}

pub(crate) fn writev(fd: BorrowedFd<'_>, bufs: &[IoSlice]) -> io::Result<usize> {
    todo!("writev")
}

pub(crate) fn preadv(fd: BorrowedFd<'_>, bufs: &[IoSliceMut], offset: u64) -> io::Result<usize> {
    todo!("preadv")
}

pub(crate) fn pwritev(fd: BorrowedFd<'_>, bufs: &[IoSlice], offset: u64) -> io::Result<usize> {
    todo!("pwritev")
}

pub(crate) fn isatty(fd: BorrowedFd<'_>) -> bool {
    todo!("isatty")
}

pub(crate) fn is_read_write(fd: BorrowedFd<'_>) -> io::Result<(bool, bool)> {
    todo!("is_read_write")
}

pub(crate) fn poll(fds: &mut [PollFd<'_>], timeout: i32) -> io::Result<usize> {
    todo!("poll")
}

pub(crate) fn ioctl_fionread(fd: BorrowedFd<'_>) -> io::Result<u64> {
    todo!("ioctl_fionread")
}

pub(crate) fn ioctl_fionbio(fd: BorrowedFd<'_>, value: bool) -> io::Result<()> {
    todo!("ioctl_fionbio")
}

#[inline]
pub(crate) unsafe fn close(raw_fd: RawFd) {
    let _ = wasi_filesystem::Descriptor::from_raw_fd(raw_fd);
}
