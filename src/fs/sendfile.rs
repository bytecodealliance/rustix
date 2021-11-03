use crate::{imp, io};
use imp::fd::AsFd;

/// `sendfile(out_fd, in_fd, offset, count)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/sendfile.2.html
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
#[inline]
pub fn sendfile<OutFd: AsFd, InFd: AsFd>(
    out_fd: &OutFd,
    in_fd: &InFd,
    offset: Option<&mut u64>,
    count: usize,
) -> io::Result<usize> {
    let out_fd = out_fd.as_fd();
    let in_fd = in_fd.as_fd();
    imp::syscalls::sendfile(out_fd, in_fd, offset, count)
}
