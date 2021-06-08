use crate::zero_ok;
use io_lifetimes::{AsFd, BorrowedFd};
use std::{io, os::unix::ffi::OsStringExt, path::PathBuf};
use unsafe_io::os::posish::AsRawFd;

/// `fcntl(fd, F_GETPATH)`
pub fn getpath<Fd: AsFd>(fd: &Fd) -> io::Result<PathBuf> {
    let fd = fd.as_fd();
    _getpath(fd)
}

fn _getpath(fd: BorrowedFd<'_>) -> io::Result<PathBuf> {
    // The use of PATH_MAX is generally not encouraged, but it
    // is inevitable in this case because macOS defines `fcntl` with
    // `F_GETPATH` in terms of `MAXPATHLEN`, and there are no
    // alternatives. If a better method is invented, it should be used
    // instead.
    let mut buf = vec![0; libc::PATH_MAX as usize];

    // From the macOS `fcntl` man page:
    // `F_GETPATH` - Get the path of the file descriptor `Fildes`. The argument
    //               must be a buffer of size `MAXPATHLEN` or greater.
    //
    // https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fcntl.2.html
    unsafe {
        zero_ok(libc::fcntl(
            fd.as_raw_fd(),
            libc::F_GETPATH,
            buf.as_mut_ptr(),
        ))?;
    }

    let l = buf.iter().position(|&c| c == 0).unwrap();
    buf.truncate(l as usize);
    buf.shrink_to_fit();
    Ok(PathBuf::from(std::ffi::OsString::from_vec(buf)))
}
