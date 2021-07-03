use crate::{imp, io};
use io_lifetimes::AsFd;

/// `copy_file_range(fd_in, off_in, fd_out, off_out, len, 0)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/copy_file_range.2.html
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
    imp::syscalls::copy_file_range(fd_in, off_in, fd_out, off_out, len)
}
