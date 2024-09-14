use crate::{backend, io};

pub use crate::timespec::Timespec;

/// Bitfield array element type for use with [`select`].
#[cfg(all(
    target_pointer_width = "64",
    any(target_os = "freebsd", target_os = "dragonfly")
))]
pub type FdSetElement = i64;

/// Bitfield array element type for use with [`select`].
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
/// where a fd `fd` is in the set if the element at index
/// `fd / (size_of::<FdSetElement>() * 8)` has the bit
/// `1 << (fd % (size_of::<FdSetElement>() * 8))` set.
///
/// In particular, on Apple platforms, it behaves as if
/// `_DARWIN_UNLIMITED_SELECT` were predefined. And on Linux platforms, it is
/// not defined because Linux's `select` always has an `FD_SETSIZE` limitation.
/// On Linux, it is recommended to use [`poll`] instead.
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
