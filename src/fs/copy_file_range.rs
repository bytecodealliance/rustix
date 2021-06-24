use crate::io;
use io_lifetimes::{AsFd, BorrowedFd};
use std::convert::TryInto;
#[cfg(libc)]
use {
    crate::libc::conv::borrowed_fd,
    crate::negone_err,
    std::{mem::size_of, ptr::null_mut},
};

/// `copy_file_range(fd_in, off_in, fd_out, off_out, len, 0)`
#[inline]
pub fn copy_file_range<InFd: AsFd, OutFd: AsFd>(
    fd_in: &InFd,
    off_in: Option<&mut u64>,
    fd_out: &OutFd,
    off_out: Option<&mut u64>,
    len: u64,
) -> io::Result<u64> {
    let fd_in = fd_in.as_fd();
    let fd_out = fd_out.as_fd();
    _copy_file_range(fd_in, off_in, fd_out, off_out, len)
}

#[cfg(libc)]
fn _copy_file_range(
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
        negone_err(libc::syscall(
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

#[cfg(linux_raw)]
#[inline]
fn _copy_file_range(
    fd_in: BorrowedFd<'_>,
    off_in: Option<&mut u64>,
    fd_out: BorrowedFd<'_>,
    off_out: Option<&mut u64>,
    len: u64,
) -> io::Result<u64> {
    let len: usize = len.try_into().unwrap_or(usize::MAX);
    crate::linux_raw::copy_file_range(fd_in, off_in, fd_out, off_out, len, 0)
        .map(|result| result as u64)
}
