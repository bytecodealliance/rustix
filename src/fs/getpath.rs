use crate::zero_ok;
use std::{
    io,
    os::unix::io::{AsRawFd, RawFd},
};

/// `fcntl(fd, F_GETPATH)`
pub fn getpath<Fd: AsRawFd>(fd: &Fd, buf: &mut [u8]) -> io::Result<()> {
    let fd = fd.as_raw_fd();
    unsafe { _getpath(fd, buf) }
}

unsafe fn _getpath(fd: RawFd, buf: &mut [u8]) -> io::Result<()> {
    // From the macOS `fcntl` man page:
    // `F_GETPATH` - Get the path of the file descriptor `Fildes`. The argument
    //               must be a buffer of size `MAXPATHLEN` or greater.
    //
    // https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fcntl.2.html
    assert!(buf.len() >= libc::PATH_MAX as usize);
    zero_ok(libc::fcntl(fd, libc::F_GETPATH, buf.as_mut_ptr()))
}
