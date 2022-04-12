//! Functions which operate on file descriptors which might be terminals.

#[cfg(not(any(target_os = "fuchsia", target_os = "wasi")))]
#[cfg(feature = "procfs")]
use crate::io;
#[cfg(not(any(target_os = "fuchsia", target_os = "wasi")))]
#[cfg(feature = "procfs")]
use {
    crate::ffi::ZString, crate::imp, crate::path::SMALL_PATH_BUFFER_SIZE, alloc::vec::Vec,
    imp::fd::AsFd, imp::fd::BorrowedFd,
};

/// `ttyname_r(fd)`
///
/// If `reuse` is non-empty, reuse its buffer to store the result if possible.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/ttyname.html
/// [Linux]: https://man7.org/linux/man-pages/man3/ttyname.3.html
#[cfg(not(any(target_os = "fuchsia", target_os = "wasi")))]
#[cfg(feature = "procfs")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "procfs")))]
#[inline]
pub fn ttyname<Fd: AsFd, B: Into<Vec<u8>>>(dirfd: Fd, reuse: B) -> io::Result<ZString> {
    _ttyname(dirfd.as_fd(), reuse.into())
}

#[cfg(not(any(target_os = "fuchsia", target_os = "wasi")))]
#[cfg(feature = "procfs")]
fn _ttyname(dirfd: BorrowedFd<'_>, mut buffer: Vec<u8>) -> io::Result<ZString> {
    // This code would benefit from having a better way to read into
    // uninitialized memory, but that requires `unsafe`.
    buffer.clear();
    buffer.reserve(SMALL_PATH_BUFFER_SIZE);
    buffer.resize(buffer.capacity(), 0_u8);

    loop {
        match imp::io::syscalls::ttyname(dirfd, &mut buffer) {
            Err(imp::io::Error::RANGE) => {
                buffer.reserve(1); // use `Vec` reallocation strategy to grow capacity exponentially
                buffer.resize(buffer.capacity(), 0_u8);
            }
            Ok(len) => {
                buffer.resize(len, 0_u8);
                return Ok(ZString::new(buffer).unwrap());
            }
            Err(errno) => return Err(errno),
        }
    }
}
