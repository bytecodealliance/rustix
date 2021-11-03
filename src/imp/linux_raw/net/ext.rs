#![allow(unsafe_code)]
#![allow(dead_code)]

use super::super::libc;

#[inline]
pub(crate) const fn in_addr_s_addr(addr: libc::in_addr) -> u32 {
    addr.s_addr
}

#[inline]
pub(crate) const fn in_addr_new(s_addr: u32) -> libc::in_addr {
    libc::in_addr { s_addr }
}

#[inline]
pub(crate) const fn in6_addr_s6_addr(addr: libc::in6_addr) -> [u8; 16] {
    unsafe { addr.in6_u.u6_addr8 }
}

#[inline]
pub(crate) const fn in6_addr_new(s6_addr: [u8; 16]) -> libc::in6_addr {
    libc::in6_addr {
        in6_u: linux_raw_sys::general::in6_addr__bindgen_ty_1 { u6_addr8: s6_addr },
    }
}

#[inline]
pub(crate) const fn sockaddr_in6_sin6_scope_id(addr: &libc::sockaddr_in6) -> u32 {
    addr.sin6_scope_id
}

#[inline]
pub(crate) const fn sockaddr_in6_new(
    #[cfg(any(
        target_os = "netbsd",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd"
    ))]
    sin6_len: u8,
    sin6_family: linux_raw_sys::general::__kernel_sa_family_t,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: linux_raw_sys::general::in6_addr,
    sin6_scope_id: u32,
) -> libc::sockaddr_in6 {
    libc::sockaddr_in6 {
        #[cfg(any(
            target_os = "netbsd",
            target_os = "macos",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "openbsd"
        ))]
        sin6_len,
        sin6_family,
        sin6_port,
        sin6_flowinfo,
        sin6_addr,
        sin6_scope_id,
    }
}
