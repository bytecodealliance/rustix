//! Adapt the Linux API to resemble a POSIX-style libc API.
//!
//! The linux_raw backend doesn't use actual libc; this just
//! defines certain types that are convenient to have defined.

#![allow(unused_imports)]
#![allow(unsafe_code)]

pub(crate) use linux_raw_sys::general::{
    __kernel_sa_family_t as sa_family_t, cmsghdr, in6_addr, in6_pktinfo, in_addr, in_pktinfo,
    iovec, msghdr, size_t, sockaddr_in, sockaddr_in6, sockaddr_un, ucred, AF_INET, AF_INET6,
    SCM_CREDENTIALS, SCM_RIGHTS, SOL_SOCKET,
};

pub(crate) use linux_raw_sys::ctypes::*;

use core::{mem, ptr};

/// Given a length, returns it including the required alignment.
///
/// https://linux.die.net/man/3/cmsg_align
#[allow(non_snake_case)]
pub const fn CMSG_ALIGN(len: size_t) -> size_t {
    len + mem::size_of::<usize>() as size_t - 1 & !(mem::size_of::<usize>() as size_t - 1)
}

/// Returns a pointer to the first `cmsghdr` in the ancillary data buffer associated with the passed `msghdr`.
///
/// https://linux.die.net/man/3/cmsg_align
/// Safety: `mhdr` must point to an initialized `msghdr`.
#[allow(non_snake_case)]
pub unsafe fn CMSG_FIRSTHDR(mhdr: *const msghdr) -> *mut cmsghdr {
    if (*mhdr).msg_controllen as usize >= mem::size_of::<cmsghdr>() {
        (*mhdr).msg_control as *mut cmsghdr
    } else {
        ptr::null_mut()
    }
}

/// Returns a pointer to the data portion of a cmsghdr.
///
/// https://linux.die.net/man/3/cmsg_align
#[allow(non_snake_case)]
pub unsafe fn CMSG_DATA(cmsg: *const cmsghdr) -> *mut c_uchar {
    cmsg.offset(1) as *mut _
}

/// Returns the number of bytes an ancillary element with payload of the passed
/// data length occupies.
///
/// https://linux.die.net/man/3/cmsg_align
#[allow(non_snake_case)]
pub const fn CMSG_SPACE(length: size_t) -> size_t {
    CMSG_ALIGN(length) + CMSG_ALIGN(mem::size_of::<cmsghdr>() as size_t)
}

/// Given a length, returns it including the required alignment.
///
/// https://linux.die.net/man/3/cmsg_align
#[allow(non_snake_case)]
pub const fn CMSG_LEN(length: size_t) -> size_t {
    CMSG_ALIGN(mem::size_of::<cmsghdr>() as size_t) + length
}

/// Returns the next valid `cmsghdr` after the passed `cmsghdr`. It returns NULL when there
/// isn't enough space left in the buffer.
///
/// https://linux.die.net/man/3/cmsg_align
#[allow(non_snake_case)]
pub unsafe fn CMSG_NXTHDR(mhdr: *const msghdr, cmsg: *const cmsghdr) -> *mut cmsghdr {
    if ((*cmsg).cmsg_len as usize) < mem::size_of::<cmsghdr>() {
        return ptr::null_mut();
    };
    let next = (cmsg as usize + CMSG_ALIGN((*cmsg).cmsg_len) as usize) as *mut cmsghdr;
    let max = (*mhdr).msg_control as usize + (*mhdr).msg_controllen as usize;
    if (next.offset(1)) as usize > max
        || next as usize + CMSG_ALIGN((*next).cmsg_len) as usize > max
    {
        0 as *mut cmsghdr
    } else {
        next as *mut cmsghdr
    }
}
