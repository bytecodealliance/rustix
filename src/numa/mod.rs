//! The `numa` API.
//!
//! # Safety
//!
//! `mbind` and related functions manipulate raw pointers and have special
//! semantics and are wildly unsafe.
#![allow(unsafe_code)]

use crate::{backend, io};
use core::ffi::c_void;

pub use backend::numa::types::{Mode, ModeFlags};

/// `mbind(addr, len, mode, nodemask)`-Set memory policy for a memory range.
///
/// # Safety
///
/// This function operates on raw pointers, but it should only be used
/// on memory which the caller owns.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/mbind.2.html
#[cfg(linux_kernel)]
#[inline]
pub unsafe fn mbind(
    addr: *mut c_void,
    len: usize,
    mode: Mode,
    nodemask: &[u64],
    flags: ModeFlags,
) -> io::Result<()> {
    backend::numa::syscalls::mbind(addr, len, mode, nodemask, flags)
}

/// `set_mempolicy(mode, nodemask)`-Set default NUMA memory policy for
/// a thread and its children.
///
/// # Safety
///
/// This function operates on raw pointers, but it should only be used
/// on memory which the caller owns.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/set_mempolicy.2.html
#[cfg(linux_kernel)]
#[inline]
pub unsafe fn set_mempolicy(mode: Mode, nodemask: &[u64]) -> io::Result<()> {
    backend::numa::syscalls::set_mempolicy(mode, nodemask)
}

/// `get_mempolicy_node(addr)`-Return the node ID of the node on which
/// the address addr is allocated.
///
/// If flags specifies both MPOL_F_NODE and MPOL_F_ADDR,
/// get_mempolicy() will return the node ID of the node on which the
/// address addr is allocated into the location pointed to by mode.
/// If no page has yet been allocated for the specified address,
/// get_mempolicy() will allocate a page as if the thread had
/// performed a read (load) access to that address, and return the ID
/// of the node where that page was allocated.
///
/// # Safety
///
/// This function operates on raw pointers, but it should only be used
/// on memory which the caller owns.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/get_mempolicy.2.html
#[cfg(linux_kernel)]
#[inline]
pub unsafe fn get_mempolicy_node(addr: *mut c_void) -> io::Result<usize> {
    backend::numa::syscalls::get_mempolicy_node(addr)
}

/// `get_mempolicy_next_node(addr)`-Return node ID of the next node
/// that will be used for interleaving of internal kernel pages
/// allocated on behalf of the thread.
///
/// If flags specifies MPOL_F_NODE, but not MPOL_F_ADDR, and the
/// thread's current policy is MPOL_INTERLEAVE, then get_mempolicy()
/// will return in the location pointed to by a non-NULL mode
/// argument, the node ID of the next node that will be used for
/// interleaving of internal kernel pages allocated on behalf of the
/// thread.  These allocations include pages for memory-mapped files
/// in process memory ranges mapped using the mmap(2) call with the
/// MAP_PRIVATE flag for read accesses, and in memory ranges mapped
/// with the MAP_SHARED flag for all accesses.
///
/// # Safety
///
/// This function operates on raw pointers, but it should only be used
/// on memory which the caller owns.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/get_mempolicy.2.html
#[cfg(linux_kernel)]
#[inline]
pub unsafe fn get_mempolicy_next_node() -> io::Result<usize> {
    backend::numa::syscalls::get_mempolicy_next_node()
}
