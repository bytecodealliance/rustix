use crate::{imp, io};
use io_lifetimes::AsFd;

/// `fcntl(fd, F_RDADVISE, radvisory { offset, len })`
///
/// # References
///  - [Apple]
///
/// [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fcntl.2.html
#[inline]
pub fn fcntl_rdadvise<Fd: AsFd>(fd: &Fd, offset: u64, len: u64) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::fcntl_rdadvise(fd, offset, len)
}
