use crate::{imp, io, path};
use io_lifetimes::OwnedFd;

pub use imp::fs::MemfdFlags;

/// `memfd_create(path, flags)`
#[inline]
pub fn memfd_create<P: path::Arg>(path: P, flags: MemfdFlags) -> io::Result<OwnedFd> {
    path.into_with_c_str(|path| imp::syscalls::memfd_create(&path, flags))
}
