use crate::{imp, io};
use io_lifetimes::AsFd;

/// `fcntl(fd, F_RDADVISE, radvisory { offset, len })`
#[inline]
pub fn fcntl_rdadvise<Fd: AsFd>(fd: &Fd, offset: u64, len: u64) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::fcntl_rdadvise(fd, offset, len)
}
