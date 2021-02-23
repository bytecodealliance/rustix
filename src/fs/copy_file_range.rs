use crate::negone_err;
use std::{convert::TryInto, io, mem, ptr};
use unsafe_io::{os::posish::AsRawFd, AsUnsafeHandle, UnsafeHandle};

/// `copy_file_range(fd_in, off_in, fd_out, off_out, len, 0)`
#[inline]
pub fn copy_file_range<InUnsafeHandle: AsUnsafeHandle, OutUnsafeHandle: AsUnsafeHandle>(
    fd_in: &InUnsafeHandle,
    off_in: Option<&mut u64>,
    fd_out: &OutUnsafeHandle,
    off_out: Option<&mut u64>,
    len: u64,
) -> io::Result<u64> {
    let fd_in = fd_in.as_unsafe_handle();
    let fd_out = fd_out.as_unsafe_handle();
    unsafe { _copy_file_range(fd_in, off_in, fd_out, off_out, len) }
}

unsafe fn _copy_file_range(
    fd_in: UnsafeHandle,
    off_in: Option<&mut u64>,
    fd_out: UnsafeHandle,
    off_out: Option<&mut u64>,
    len: u64,
) -> io::Result<u64> {
    assert_eq!(mem::size_of::<libc::loff_t>(), mem::size_of::<u64>());

    let mut off_in_val: libc::loff_t = 0;
    let mut off_out_val: libc::loff_t = 0;
    let off_in_ptr = if let Some(off_in) = &off_in {
        off_in_val = (**off_in)
            .try_into()
            .map_err(|_overflow_err| io::Error::from_raw_os_error(libc::EOVERFLOW))?;
        &mut off_in_val
    } else {
        ptr::null_mut()
    };
    let off_out_ptr = if let Some(off_out) = &off_out {
        off_out_val = (**off_out)
            .try_into()
            .map_err(|_overflow_err| io::Error::from_raw_os_error(libc::EOVERFLOW))?;
        &mut off_out_val
    } else {
        ptr::null_mut()
    };
    let len: usize = len.try_into().unwrap_or(usize::MAX);
    let copied = negone_err(libc::syscall(
        libc::SYS_copy_file_range,
        fd_in.as_raw_fd(),
        off_in_ptr,
        fd_out.as_raw_fd(),
        off_out_ptr,
        len,
        0, // no flags are defined yet
    ))?;
    if let Some(off_in) = off_in {
        *off_in = off_in_val as u64;
    }
    if let Some(off_out) = off_out {
        *off_out = off_out_val as u64;
    }
    Ok(copied as u64)
}
