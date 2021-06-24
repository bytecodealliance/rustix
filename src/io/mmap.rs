//! The `mmap` API.
//!
//! # Safety
//!
//! `mmap` manipulates raw pointers and has special semantics and is
//! wildly unsafe.
#![allow(unsafe_code)]

use crate::io;
use bitflags::bitflags;
use io_lifetimes::{AsFd, BorrowedFd};
#[cfg(all(
    libc,
    not(any(
        target_os = "android",
        target_os = "linux",
        target_os = "emscripten",
        target_os = "l4re",
    ))
))]
use libc::mmap as libc_mmap;
#[cfg(all(
    libc,
    any(
        target_os = "android",
        target_os = "linux",
        target_os = "emscripten",
        target_os = "l4re"
    )
))]
use libc::mmap64 as libc_mmap;
use std::os::raw::c_void;
#[cfg(libc)]
use {
    crate::libc::conv::{borrowed_fd, ret},
    std::os::raw::c_int,
};

#[cfg(libc)]
bitflags! {
    /// `PROT_*` flags for use with `mmap`.
    pub struct ProtFlags: c_int {
        /// `PROT_READ`
        const READ = libc::PROT_READ;
        /// `PROT_WRITE`
        const WRITE = libc::PROT_WRITE;
        /// `PROT_EXEC`
        const EXEC = libc::PROT_EXEC;
        /// `PROT_NONE`
        const NONE = libc::PROT_NONE;
    }
}

#[cfg(linux_raw)]
bitflags! {
    /// `PROT_*` flags for use with `mmap`.
    pub struct ProtFlags: u32 {
        /// `PROT_READ`
        const READ = linux_raw_sys::general::PROT_READ;
        /// `PROT_WRITE`
        const WRITE = linux_raw_sys::general::PROT_WRITE;
        /// `PROT_EXEC`
        const EXEC = linux_raw_sys::general::PROT_EXEC;
        /// `PROT_NONE`
        const NONE = linux_raw_sys::general::PROT_NONE;
    }
}

#[cfg(libc)]
bitflags! {
    /// `MAP_*` flags for use with `mmap`.
    pub struct MapFlags: c_int {
        /// `MAP_SHARED`
        const SHARED = libc::MAP_SHARED;
        /// `MAP_SHARED_VALIDATE`
        #[cfg(not(any(target_os = "android", target_os = "netbsd", target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "emscripten", target_os = "fuchsia", target_os = "redox")))]
        const SHARED_VALIDATE = libc::MAP_SHARED_VALIDATE;
        /// `MAP_PRIVATE`
        const PRIVATE = libc::MAP_PRIVATE;
        /// `MAP_ANONYMOUS`, aka `MAP_ANON`
        const ANONYMOUS = libc::MAP_ANONYMOUS;
        /// `MAP_DENYWRITE`
        #[cfg(not(any(target_os = "netbsd", target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "redox")))]
        const DENYWRITE = libc::MAP_DENYWRITE;
        /// `MAP_FIXED`
        #[cfg(not(any(target_os = "android", target_os = "netbsd", target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "emscripten", target_os = "fuchsia", target_os = "redox")))]
        const FIXED_NOREPLACE = libc::MAP_FIXED_NOREPLACE;
        /// `MAP_GROWSDOWN`
        #[cfg(not(any(target_os = "netbsd", target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "redox")))]
        const GROWSDOWN = libc::MAP_GROWSDOWN;
        /// `MAP_HUGETLB`
        #[cfg(not(any(target_os = "netbsd", target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "redox")))]
        const HUGETLB = libc::MAP_HUGETLB;
        /// `MAP_HUGE_2MB`
        #[cfg(not(any(target_os = "android", target_os = "netbsd", target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "emscripten", target_os = "fuchsia", target_os = "redox")))]
        const HUGE_2MB = libc::MAP_HUGE_2MB;
        /// `MAP_HUGE_1GB`
        #[cfg(not(any(target_os = "android", target_os = "netbsd", target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "emscripten", target_os = "fuchsia", target_os = "redox")))]
        const HUGE_1GB = libc::MAP_HUGE_1GB;
        /// `MAP_LOCKED`
        #[cfg(not(any(target_os = "netbsd", target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "redox")))]
        const LOCKED = libc::MAP_LOCKED;
        /// `MAP_NORESERVE`
        #[cfg(not(any(target_os = "freebsd", target_os = "redox")))]
        const NORESERVE = libc::MAP_NORESERVE;
        /// `MAP_POPULATE`
        #[cfg(not(any(target_os = "netbsd", target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "redox")))]
        const POPULATE = libc::MAP_POPULATE;
        /// `MAP_STACK`
        #[cfg(not(any(target_os = "netbsd", target_os = "macos", target_os = "ios", target_os = "redox")))]
        const STACK = libc::MAP_STACK;
        /// `MAP_SYNC`
        #[cfg(not(any(target_os = "android", target_os = "netbsd", target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "emscripten", target_os = "fuchsia", target_os = "redox")))]
        const SYNC = libc::MAP_SYNC;
        /// `MAP_UNINITIALIZED`
        #[cfg(any())]
        const UNINITIALIZED = libc::MAP_UNINITIALIZED;
    }
}

#[cfg(linux_raw)]
bitflags! {
    /// `MAP_*` flags for use with `mmap`.
    pub struct MapFlags: u32 {
        /// `MAP_SHARED`
        const SHARED = linux_raw_sys::general::MAP_SHARED;
        /// `MAP_SHARED_VALIDATE`
        const SHARED_VALIDATE = linux_raw_sys::v5_4::general::MAP_SHARED_VALIDATE;
        /// `MAP_PRIVATE`
        const PRIVATE = linux_raw_sys::general::MAP_PRIVATE;
        /// `MAP_ANONYMOUS`, aka `MAP_ANON`
        const ANONYMOUS = linux_raw_sys::general::MAP_ANONYMOUS;
        /// `MAP_DENYWRITE`, aka `MAP_DENYWRITE`
        const DENYWRITE = linux_raw_sys::general::MAP_DENYWRITE;
        /// `MAP_FIXED`, aka `MAP_FIXED`
        const FIXED_NOREPLACE = linux_raw_sys::v5_4::general::MAP_FIXED_NOREPLACE;
        /// `MAP_GROWSDOWN`
        const GROWSDOWN = linux_raw_sys::general::MAP_GROWSDOWN;
        /// `MAP_HUGETLB`
        const HUGETLB = linux_raw_sys::general::MAP_HUGETLB;
        /// `MAP_HUGE_2MB`
        const HUGE_2MB = linux_raw_sys::v5_4::general::MAP_HUGE_2MB;
        /// `MAP_HUGE_1GB`
        const HUGE_1GB = linux_raw_sys::v5_4::general::MAP_HUGE_1GB;
        /// `MAP_LOCKED`
        const LOCKED = linux_raw_sys::general::MAP_LOCKED;
        /// `MAP_NORESERVE`
        const NORESERVE = linux_raw_sys::general::MAP_NORESERVE;
        /// `MAP_POPULATE`
        const POPULATE = linux_raw_sys::general::MAP_POPULATE;
        /// `MAP_STACK`
        const STACK = linux_raw_sys::general::MAP_STACK;
        /// `MAP_SYNC`
        const SYNC = linux_raw_sys::v5_4::general::MAP_SYNC;
        /// `MAP_UNINITIALIZED`
        const UNINITIALIZED = linux_raw_sys::v5_4::general::MAP_UNINITIALIZED;
    }
}

/// `mmap(fd, len, prot, flags, fd, offset)`
///
/// # Safety
///
/// Raw pointers and lots of special semantics.
#[inline]
pub unsafe fn mmap<Fd: AsFd>(
    ptr: *mut c_void,
    len: usize,
    prot: ProtFlags,
    flags: MapFlags,
    fd: &Fd,
    offset: u64,
) -> io::Result<*mut c_void> {
    let fd = fd.as_fd();
    _mmap(ptr, len, prot, flags, fd, offset)
}

#[cfg(libc)]
unsafe fn _mmap(
    ptr: *mut c_void,
    len: usize,
    prot: ProtFlags,
    flags: MapFlags,
    fd: BorrowedFd<'_>,
    offset: u64,
) -> io::Result<*mut c_void> {
    let res = libc_mmap(
        ptr,
        len,
        prot.bits(),
        flags.bits(),
        borrowed_fd(fd),
        offset as i64,
    );
    if res == libc::MAP_FAILED {
        Err(io::Error::last_os_error())
    } else {
        Ok(res)
    }
}

#[cfg(linux_raw)]
unsafe fn _mmap(
    ptr: *mut c_void,
    len: usize,
    prot: ProtFlags,
    flags: MapFlags,
    fd: BorrowedFd<'_>,
    offset: u64,
) -> io::Result<*mut c_void> {
    crate::linux_raw::mmap(ptr, len, prot.bits(), flags.bits(), fd, offset)
}

/// `munmap(ptr, len)`
///
/// # Safety
///
/// Raw pointers and lots of special semantics.
#[inline]
pub unsafe fn munmap(ptr: *mut c_void, len: usize) -> io::Result<()> {
    _munmap(ptr, len)
}

#[cfg(libc)]
unsafe fn _munmap(ptr: *mut c_void, len: usize) -> io::Result<()> {
    ret(libc::munmap(ptr, len))
}

#[cfg(linux_raw)]
#[inline]
unsafe fn _munmap(ptr: *mut c_void, len: usize) -> io::Result<()> {
    crate::linux_raw::munmap(ptr, len)
}
