use crate::{imp, io};
use io_lifetimes::AsFd;
use std::path::PathBuf;

/// `fcntl(fd, F_GETPATH)`
#[inline]
pub fn getpath<Fd: AsFd>(fd: &Fd) -> io::Result<PathBuf> {
    let fd = fd.as_fd();
    imp::syscalls::getpath(fd)
}
