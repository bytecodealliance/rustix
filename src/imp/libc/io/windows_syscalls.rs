//! Windows system calls in the `io` module.

use super::super::conv::{borrowed_fd, ret};
use super::super::fd::LibcFd;
use super::c;
use crate::fd::{BorrowedFd, RawFd};
use crate::io;

pub(crate) unsafe fn close(raw_fd: RawFd) {
    let _ = c::close(raw_fd as LibcFd);
}

pub(crate) fn ioctl_fionbio(fd: BorrowedFd<'_>, value: bool) -> io::Result<()> {
    unsafe {
        let mut data = value as c::c_uint;
        ret(c::ioctl(borrowed_fd(fd), c::FIONBIO, &mut data))
    }
}
