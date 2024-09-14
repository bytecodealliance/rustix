use crate::{backend, io};

pub use crate::timespec::Timespec;

/// Bitvector element type for use with [`select`].
#[cfg(all(
    target_pointer_width = "64",
    any(target_os = "freebsd", target_os = "dragonfly")
))]
pub type FdSetElement = i64;

/// Bitvector element type for use with [`select`].
#[cfg(not(all(
    target_pointer_width = "64",
    any(target_os = "freebsd", target_os = "dragonfly")
)))]
pub type FdSetElement = i32;

/// `select(nfds, readfds, writefds, exceptfds, timeout)`—Wait for events on
/// sets of file descriptors.
///
/// This `select` wrapper differs from POSIX in that `nfds` is not limited to
/// `FD_SETSIZE`. Instead of using the opaque fixed-sized `fd_set` type, this
/// function takes raw pointers to arrays of
/// `nfds.div_ceil(size_of::<FdSetElement>())` elements of type `FdSetElement`,
/// representing bitvectors where a fd `fd` is set if the element at index
/// `fd / (size_of::<FdSetElement>() * 8)` has the bit
/// `1 << (fd % (size_of::<FdSetElement>() * 8))` set. Convenience functions
/// [`fd_set`], [`fd_clr`], [`fd_isset`], and [`fd_bitvector_len`] are provided
/// for setting, clearing, testing, and sizing bitvectors.
///
/// In particular, on Apple platforms, this function behaves as if
/// `_DARWIN_UNLIMITED_SELECT` were predefined.
///
/// On Linux, illumos, and OpenBSD, this function is not defined because the
/// `select` functions on these platforms always has an `FD_SETSIZE`
/// limitation, following POSIX. These platforms' documentation recommend using
/// [`poll`] instead.
///
/// [`poll`]: crate::event::poll
///
/// # Safety
///
/// `readfds`, `writefds`, `exceptfds` must point to arrays of `FdSetElement`
/// containing at least `nfds.div_ceil(size_of::<FdSetElement>())` elements.
///
/// # References
///  - [POSIX]
///  - [Apple]
///  - [FreeBSD]
///  - [NetBSD]
///  - [DragonFly BSD]
///
///  [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/select.html
///  [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/select.2.html
///  [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=select&sektion=2
///  [NetBSD]: https://man.netbsd.org/select.2
///  [DragonFly BSD]: https://man.dragonflybsd.org/?command=select&section=2
pub unsafe fn select(
    nfds: i32,
    readfds: *mut FdSetElement,
    writefds: *mut FdSetElement,
    exceptfds: *mut FdSetElement,
    timeout: Option<&Timespec>,
) -> io::Result<i32> {
    backend::event::syscalls::select(nfds, readfds, writefds, exceptfds, timeout)
}

const BITS: usize = size_of::<FdSetElement>() * 8;
use crate::fd::RawFd;

/// Set `fd` in the bitvector pointed to by `fds`.
#[doc(alias = "FD_SET")]
#[inline]
pub fn fd_set(fd: RawFd, fds: &mut [FdSetElement]) {
    let fd = fd as usize;
    fds[fd / BITS] |= 1 << (fd % BITS);
}

/// Clear `fd` in the bitvector pointed to by `fds`.
#[doc(alias = "FD_CLR")]
#[inline]
pub fn fd_clr(fd: RawFd, fds: &mut [FdSetElement]) {
    let fd = fd as usize;
    fds[fd / BITS] &= !(1 << (fd % BITS));
}

/// Test whether `fd` is set in the bitvector pointed to by `fds`.
#[doc(alias = "FD_ISSET")]
#[inline]
pub fn fd_isset(fd: RawFd, fds: &[FdSetElement]) -> bool {
    let fd = fd as usize;
    (fds[fd / BITS] & (1 << (fd % BITS))) != 0
}

/// Compute the number of `FdSetElement`s needed to hold a bitvector which can
/// contain file descriptors less than `nfds`.
#[inline]
pub fn fd_bitvector_len(nfds: RawFd) -> usize {
    let nfds = nfds as usize;
    (nfds + (BITS - 1)) / BITS
}
