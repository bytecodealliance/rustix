use crate::imp;
use io_lifetimes::BorrowedFd;
use unsafe_io::os::posish::RawFd;

/// `AT_FDCWD`
///
/// This returns a file descriptor which refers to the process current
/// directory which can be used as the directory argument in `*at`
/// functions such as [`openat`].
///
/// [`openat`]: crate::fs::openat
#[inline]
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
