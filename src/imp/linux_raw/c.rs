//! Adapt the Linux API to resemble a POSIX-style libc API.
//!
//! The linux_raw backend doesn't use actual libc; this just
//! defines certain types that are convenient to have defined.

#![allow(unused_imports, dead_code)]
#![allow(unsafe_code)]
#![allow(missing_docs)]

pub(crate) use linux_raw_sys::general::{
    __kernel_sa_family_t as sa_family_t, cmsghdr, in6_addr, in6_pktinfo, in_addr, in_pktinfo,
    iovec, msghdr, size_t, sockaddr, sockaddr_in, sockaddr_in6, sockaddr_un, ucred, AF_INET,
    AF_INET6, IPV6_PKTINFO, IPV6_RECVERR, IP_PKTINFO, IP_RECVERR, SCM_CREDENTIALS, SCM_RIGHTS,
    SOL_SOCKET,
};

pub(crate) use linux_raw_sys::ctypes::*;

use core::{mem, ptr};

/// The type of constants like `IPPROTO_IP`.
pub type IpConstantType = u32;

/// Given a length, returns it including the required alignment.
///
/// https://man7.org/linux/man-pages/man3/cmsg_align.3.html
#[allow(non_snake_case)]
pub const fn CMSG_ALIGN(len: size_t) -> size_t {
    len + mem::size_of::<usize>() as size_t - 1 & !(mem::size_of::<usize>() as size_t - 1)
}

/// Returns a pointer to the first `cmsghdr` in the ancillary data buffer associated with the passed `msghdr`.
///
/// https://man7.org/linux/man-pages/man3/cmsg.3.html
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
/// https://man7.org/linux/man-pages/man3/cmsg.3.html
#[allow(non_snake_case)]
pub unsafe fn CMSG_DATA(cmsg: *const cmsghdr) -> *mut c_uchar {
    cmsg.offset(1) as *mut _
}

/// Returns the number of bytes an ancillary element with payload of the passed
/// data length occupies.
///
/// https://man7.org/linux/man-pages/man3/cmsg.3.html
#[allow(non_snake_case)]
pub const fn CMSG_SPACE(length: size_t) -> size_t {
    CMSG_ALIGN(length) + CMSG_ALIGN(mem::size_of::<cmsghdr>() as size_t)
}

/// Given a length, returns it including the required alignment.
///
/// https://man7.org/linux/man-pages/man3/cmsg.3.html
#[allow(non_snake_case)]
pub const fn CMSG_LEN(length: size_t) -> size_t {
    CMSG_ALIGN(mem::size_of::<cmsghdr>() as size_t) + length
}

/// Returns the next valid `cmsghdr` after the passed `cmsghdr`. It returns NULL when there
/// isn't enough space left in the buffer.
///
/// https://man7.org/linux/man-pages/man3/cmsg.3.html
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

// TODO: move back to linux-raw-sys
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone)]
pub struct sock_extended_err {
    pub ee_errno: u32,
    pub ee_origin: u8,
    pub ee_type: u8,
    pub ee_code: u8,
    pub ee_pad: u8,
    pub ee_info: u32,
    pub ee_daa: u32,
}

pub const SO_EE_ORIGIN_NONE: u8 = 0;
pub const SO_EE_ORIGIN_LOCAL: u8 = 1;
pub const SO_EE_ORIGIN_ICMP: u8 = 2;
pub const SO_EE_ORIGIN_ICMP6: u8 = 3;
pub const SO_EE_ORIGIN_TIMESTAMPING: u8 = 4;

#[allow(non_snake_case)]
pub unsafe fn SO_EE_OFFENDER(ee: *const sock_extended_err) -> *mut sockaddr {
    ee.offset(1) as *mut sockaddr
}

// avoid enum bindgen type
pub const IPPROTO_IP: IpConstantType = linux_raw_sys::general::IPPROTO_IP as IpConstantType;
pub const IPPROTO_IPV6: IpConstantType = linux_raw_sys::general::IPPROTO_IPV6 as IpConstantType;

// TODO: move these constants to linux-raw-sys

#[cfg(target_os = "linux")]
pub const SOL_UDP: IpConstantType = 17;
// Only available on kernel >= 5.0
#[cfg(target_os = "linux")]
pub const UDP_GRO: IpConstantType = 104;
// only available on kernel >= 4.18
#[cfg(target_os = "linux")]
pub const UDP_SEGMENT: IpConstantType = 103;
#[cfg(any(
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
))]
pub const IP_RECVDSTADDR: IpConstantType = 7;
#[cfg(any(target_os = "android", target_os = "fuchsia", target_os = "linux"))]
pub const SO_RXQ_OVFL: IpConstantType = 40;
#[cfg(any(
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
))]
pub const IP_RECVIF: IpConstantType = 20;
