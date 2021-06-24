use crate::io;
use io_lifetimes::{AsFd, BorrowedFd};
#[cfg(libc)]
use {crate::negone_err, std::mem::transmute, unsafe_io::os::posish::AsRawFd};

/// `sendfile(out_fd, in_fd, offset, count)`
#[cfg(any(linux_raw, target_os = "linux"))]
#[inline]
pub fn sendfile<OutFd: AsFd, InFd: AsFd>(
    out_fd: &OutFd,
    in_fd: &InFd,
    offset: Option<&mut u64>,
    count: usize,
) -> io::Result<usize> {
    let out_fd = out_fd.as_fd();
    let in_fd = in_fd.as_fd();
    _sendfile(out_fd, in_fd, offset, count)
}

#[cfg(all(libc, any(target_os = "linux")))]
fn _sendfile(
    out_fd: BorrowedFd<'_>,
    in_fd: BorrowedFd<'_>,
    offset: Option<&mut u64>,
    count: usize,
) -> io::Result<usize> {
    unsafe {
        let nsent = negone_err(libc::sendfile64(
            out_fd.as_raw_fd(),
            in_fd.as_raw_fd(),
            transmute(offset),
            count,
        ))?;
        Ok(nsent as usize)
    }
}

#[cfg(linux_raw)]
#[inline]
fn _sendfile(
    out_fd: BorrowedFd<'_>,
    in_fd: BorrowedFd<'_>,
    offset: Option<&mut u64>,
    count: usize,
) -> io::Result<usize> {
    crate::linux_raw::sendfile(out_fd, in_fd, offset, count)
}
