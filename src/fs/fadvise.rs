use crate::{imp, io};
use io_lifetimes::AsFd;

pub use imp::fs::Advice;

/// `posix_fadvise(fd, offset, len, advice)`
#[inline]
#[doc(alias = "posix_fadvise")]
pub fn fadvise<Fd: AsFd>(fd: &Fd, offset: u64, len: u64, advice: Advice) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::fadvise(fd, offset, len, advice)
}
