//! The `select` function.
//!
//! # Safety
//!
//! `select` is unsafe due to I/O safety.
#![allow(unsafe_code)]

use crate::{backend, io};

pub use crate::timespec::{Nsecs, Secs, Timespec};

/// Bitvector element type for use with [`select`].
#[cfg(all(
    target_pointer_width = "64",
    any(target_os = "freebsd", target_os = "dragonfly")
))]
pub type FdSetElement = u64;

/// Bitvector element type for use with [`select`].
#[cfg(not(all(
    target_pointer_width = "64",
    any(target_os = "freebsd", target_os = "dragonfly")
)))]
pub type FdSetElement = u32;

/// `select(nfds, readfds, writefds, exceptfds, timeout)`—Wait for events on
/// sets of file descriptors.
///
/// `readfds`, `writefds`, `exceptfds` must point to arrays of `FdSetElement`
/// containing at least `nfds.div_ceil(size_of::<FdSetElement>())` elements.
///
/// This `select` wrapper differs from POSIX in that `nfds` is not limited to
/// `FD_SETSIZE`. Instead of using the opaque fixed-sized `fd_set` type, this
/// function takes raw pointers to arrays of
/// `nfds.div_ceil(size_of::<FdSetElement>())` elements of type `FdSetElement`,
/// representing bitvectors where a fd `fd` is set if the element at index
/// `fd / (size_of::<FdSetElement>() * 8)` has the bit
/// `1 << (fd % (size_of::<FdSetElement>() * 8))` set. Convenience functions
/// [`fd_set_insert`], [`fd_set_remove`], [`fd_set_contains`],
/// [`fd_set_num_elements`], and [`FdSetIter`] are provided for setting,
/// clearing, testing, sizing, and iterating through bitvectors.
///
/// In particular, on Apple platforms, this function behaves as if
/// `_DARWIN_UNLIMITED_SELECT` were predefined.
///
/// On illumos, this function is not defined because the `select` function on
/// this platform always has an `FD_SETSIZE` limitation, following POSIX. This
/// platform's documentation recommends using [`poll`] instead.
///
/// [`poll`]: crate::event::poll()
///
/// # Safety
///
/// All set bits in all the sets must correspond to open file descriptors.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Apple]
///  - [FreeBSD]
///  - [NetBSD]
///  - [OpenBSD]
///  - [DragonFly BSD]
///  - [Winsock]
///  - [glibc]
///
///  [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/select.html
///  [Linux]: https://man7.org/linux/man-pages/man2/select.2.html
///  [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/select.2.html
///  [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=select&sektion=2
///  [NetBSD]: https://man.netbsd.org/select.2
///  [OpenBSD]: https://man.openbsd.org/select.2
///  [DragonFly BSD]: https://man.dragonflybsd.org/?command=select&section=2
///  [Winsock]: https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-select
///  [glibc]: https://sourceware.org/glibc/manual/latest/html_node/Waiting-for-I_002fO.html#index-select
pub unsafe fn select(
    nfds: i32,
    readfds: Option<&mut [FdSetElement]>,
    writefds: Option<&mut [FdSetElement]>,
    exceptfds: Option<&mut [FdSetElement]>,
    timeout: Option<&Timespec>,
) -> io::Result<i32> {
    backend::event::syscalls::select(nfds, readfds, writefds, exceptfds, timeout)
}

const BITS: usize = core::mem::size_of::<FdSetElement>() * 8;
use crate::fd::RawFd;

/// Set `fd` in the bitvector pointed to by `fds`.
#[doc(alias = "FD_SET")]
#[inline]
pub fn fd_set_insert(fds: &mut [FdSetElement], fd: RawFd) {
    let fd = fd as usize;
    fds[fd / BITS] |= 1 << (fd % BITS);
}

/// Clear `fd` in the bitvector pointed to by `fds`.
#[doc(alias = "FD_CLR")]
#[inline]
pub fn fd_set_remove(fds: &mut [FdSetElement], fd: RawFd) {
    let fd = fd as usize;
    fds[fd / BITS] &= !(1 << (fd % BITS));
}

/// Test whether `fd` is set in the bitvector pointed to by `fds`.
#[doc(alias = "FD_ISSET")]
#[inline]
pub fn fd_set_contains(fds: &[FdSetElement], fd: RawFd) -> bool {
    let fd = fd as usize;
    (fds[fd / BITS] & (1 << (fd % BITS))) != 0
}

/// Compute the minimum `nfds` value needed for the bitvector pointed to by
/// `fds`.
#[inline]
pub fn fd_set_bound(fds: &[FdSetElement]) -> RawFd {
    if let Some(position) = fds.iter().rposition(|element| *element != 0) {
        let element = fds[position];
        (position * BITS + (BITS - element.leading_zeros() as usize)) as RawFd
    } else {
        0
    }
}

/// Compute the number of `FdSetElement`s needed to hold a bitvector which can
/// contain file descriptors less than `nfds`.
#[inline]
pub fn fd_set_num_elements(nfds: RawFd) -> usize {
    let nfds = nfds as usize;
    (nfds + (BITS - 1)) / BITS
}

/// An iterator over the set fds in a bitvector.
pub struct FdSetIter<'a> {
    current: RawFd,
    fds: &'a [FdSetElement],
}

impl<'a> FdSetIter<'a> {
    /// Construct a `FdSetIter` for the given bitvector.
    pub fn new(fds: &'a [FdSetElement]) -> Self {
        Self { current: 0, fds }
    }
}

impl<'a> Iterator for FdSetIter<'a> {
    type Item = RawFd;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(element) = self.fds.get(self.current as usize / BITS) {
            // Test whether the current element has more bits set.
            let shifted = element >> ((self.current as usize % BITS) as u32);
            if shifted != 0 {
                let fd = self.current + shifted.trailing_zeros() as RawFd;
                self.current = fd + 1;
                return Some(fd);
            }

            // Search through the array for the next element with bits set.
            if let Some(index) = self.fds[(self.current as usize / BITS) + 1..]
                .iter()
                .position(|element| *element != 0)
            {
                let index = index + (self.current as usize / BITS) + 1;
                let element = self.fds[index];
                let fd = (index * BITS) as RawFd + element.trailing_zeros() as RawFd;
                self.current = fd + 1;
                return Some(fd);
            }
        }
        None
    }
}
