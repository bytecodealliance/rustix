use crate::io;
use io_lifetimes::{AsFd, BorrowedFd};
use std::{convert::TryInto, mem};
#[cfg(libc)]
use {crate::negone_err, std::ptr, unsafe_io::os::posish::AsRawFd};

/// `copy_file_range(fd_in, off_in, fd_out, off_out, len, 0)`
#[inline]
pub fn copy_file_range<'in_f, 'out_f, InFd: AsFd<'in_f>, OutFd: AsFd<'out_f>>(
    fd_in: InFd,
    off_in: Option<&mut u64>,
    fd_out: OutFd,
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
    assert_eq!(mem::size_of::<libc::loff_t>(), mem::size_of::<u64>());

    let mut off_in_val: libc::loff_t = 0;
    let mut off_out_val: libc::loff_t = 0;
    let off_in_ptr = if let Some(off_in) = &off_in {
        off_in_val = (**off_in)
            .try_into()
            .map_err(|_overflow_err| io::Error::OVERFLOW)?;
        &mut off_in_val
    } else {
        ptr::null_mut()
    };
    let off_out_ptr = if let Some(off_out) = &off_out {
        off_out_val = (**off_out)
            .try_into()
            .map_err(|_overflow_err| io::Error::OVERFLOW)?;
        &mut off_out_val
    } else {
        ptr::null_mut()
    };
    let len: usize = len.try_into().unwrap_or(usize::MAX);
    let copied = unsafe {
        negone_err(libc::syscall(
            libc::SYS_copy_file_range,
            fd_in.as_raw_fd(),
            off_in_ptr,
            fd_out.as_raw_fd(),
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
    let (off_in, off_out) = unsafe { (mem::transmute(off_in), mem::transmute(off_out)) };
    crate::linux_raw::copy_file_range(fd_in, off_in, fd_out, off_out, len, 0)
        .map(|result| result as u64)
}
