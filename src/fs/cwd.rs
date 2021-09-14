use std::ffi::OsString;

use crate::imp;
use crate::io::{self, RawFd};
use io_lifetimes::BorrowedFd;

/// `AT_FDCWD`—Returns a handle representing the current working directory.
///
/// This returns a file descriptor which refers to the process current
/// directory which can be used as the directory argument in `*at`
/// functions such as [`openat`].
///
/// # References
///  - [POSIX]
///
/// [`openat`]: crate::fs::openat
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/fcntl.h.html
#[inline]
#[doc(alias = "AT_FDCWD")]
pub fn cwd() -> BorrowedFd<'static> {
    let at_fdcwd = imp::io::AT_FDCWD as RawFd;

    // # Safety
    //
    // `AT_FDCWD` is a reserved value that is never dynamically allocated, so
    // it'll remain valid for the duration of 'static.
    #[allow(unsafe_code)]
    unsafe {
        BorrowedFd::<'static>::borrow_raw_fd(at_fdcwd)
    }
}

/// `getcwd()`
///
/// If `reuse` is non-empty, reuse its buffer to store the result if possible.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getcwd.html
/// [Linux]: https://man7.org/linux/man-pages/man3/getcwd.3.html
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn getcwd(reuse: OsString) -> io::Result<OsString> {
    use std::os::unix::prelude::OsStringExt;

    // This code would benefit from having a better way to read into
    // uninitialized memory, but that requires `unsafe`.
    let mut buffer = reuse.into_vec();
    buffer.clear();
    buffer.resize(256, 0_u8);

    loop {
        match imp::syscalls::getcwd(&mut buffer) {
            Err(imp::io::Error::RANGE) => buffer.resize(buffer.len() * 2, 0_u8),
            Ok(_) => {
                let len = buffer.iter().position(|x| *x == b'\0').unwrap();
                buffer.resize(len, 0_u8);
                return Ok(OsString::from_vec(buffer));
            }
            Err(errno) => return Err(errno),
        }
    }
}
