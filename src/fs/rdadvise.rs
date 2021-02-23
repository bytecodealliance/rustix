use crate::zero_ok;
use std::{convert::TryInto, io};
use unsafe_io::{os::posish::AsRawFd, AsUnsafeHandle, UnsafeHandle};

/// `fcntl(fd, F_RDADVISE, radvisory { offset, len })`
pub fn rdadvise<Fd: AsUnsafeHandle>(fd: &Fd, offset: u64, len: u64) -> io::Result<()> {
    let fd = fd.as_unsafe_handle();
    unsafe { _rdadvise(fd, offset, len) }
}

unsafe fn _rdadvise(fd: UnsafeHandle, offset: u64, len: u64) -> io::Result<()> {
    // From the macOS `fcntl` man page:
    // `F_RDADVISE` - Issue an advisory read async with no copy to user.
    //
    // The `F_RDADVISE` command operates on the following structure which holds information passed
    // from the user to the system:
    //
    // ```
    // struct radvisory {
    //      off_t   ra_offset;  /* offset into the file */
    //      int     ra_count;   /* size of the read     */
    // };
    // ```
    //
    // https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fcntl.2.html
    let ra_offset = match offset.try_into() {
        Ok(len) => len,
        // If this conversion fails, the user is providing an offset outside
        // any possible file extent, so just ignore it.
        Err(_) => return Ok(()),
    };
    let ra_count = match len.try_into() {
        Ok(len) => len,
        // If this conversion fails, the user is providing a dubiously large
        // hint which is unlikely to improve performance.
        Err(_) => return Ok(()),
    };
    let radvisory = libc::radvisory {
        ra_offset,
        ra_count,
    };
    zero_ok(libc::fcntl(fd.as_raw_fd(), libc::F_RDADVISE, &radvisory))
}
