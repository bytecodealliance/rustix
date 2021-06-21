use io_lifetimes::BorrowedFd;
use unsafe_io::os::posish::RawFd;

/// Return a "file" which holds a handle which refers to the process current
/// directory (`AT_FDCWD`).
#[inline]
pub fn cwd() -> BorrowedFd<'static> {
    #[cfg(libc)]
    let at_fdcwd = libc::AT_FDCWD as RawFd;

    #[cfg(linux_raw)]
    let at_fdcwd = linux_raw_sys::general::AT_FDCWD as RawFd;

    // # Safety
    //
    // `AT_FDCWD` is a reserved value that is never dynamically allocated, so
    // it'll remain valid for the duration of 'static.
    #[allow(unsafe_code)]
    unsafe {
        BorrowedFd::<'static>::borrow_raw_fd(at_fdcwd)
    }
}
