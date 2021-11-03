#[cfg(windows)]
use super::super::libc;

/// The windows `sockaddr_in6` type is a union with accessor functions which
/// are not `const fn`. Define our own layout-compatible version so that we
/// can transmute in and out of it.
#[cfg(windows)]
#[repr(C)]
struct sockaddr_in6 {
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: libc::in6_addr,
    sin6_scope_id: u32,
}

#[cfg(not(windows))]
#[inline]
pub(crate) const fn in_addr_s_addr(addr: libc::in_addr) -> u32 {
    addr.s_addr
}

#[cfg(windows)]
#[inline]
pub(crate) const fn in_addr_s_addr(addr: libc::in_addr) -> u32 {
    // This should be `*addr.S_un.S_addr()`, except that isn't a `const fn`.
    unsafe { std::mem::transmute(addr) }
}

#[cfg(not(windows))]
#[inline]
pub(crate) const fn in_addr_new(s_addr: u32) -> libc::in_addr {
    libc::in_addr { s_addr }
}

#[cfg(windows)]
#[inline]
pub(crate) const fn in_addr_new(s_addr: u32) -> libc::in_addr {
    unsafe { std::mem::transmute(s_addr) }
}

#[cfg(not(windows))]
#[inline]
pub(crate) const fn in6_addr_s6_addr(addr: libc::in6_addr) -> [u8; 16] {
    addr.s6_addr
}

#[cfg(windows)]
#[inline]
pub(crate) const fn in6_addr_s6_addr(addr: libc::in6_addr) -> [u8; 16] {
    unsafe { std::mem::transmute(addr) }
}

#[cfg(not(windows))]
#[inline]
pub(crate) const fn in6_addr_new(s6_addr: [u8; 16]) -> libc::in6_addr {
    libc::in6_addr { s6_addr }
}

#[cfg(windows)]
#[inline]
pub(crate) const fn in6_addr_new(s6_addr: [u8; 16]) -> libc::in6_addr {
    unsafe { std::mem::transmute(s6_addr) }
}

#[cfg(not(windows))]
#[inline]
pub(crate) const fn sockaddr_in6_sin6_scope_id(addr: libc::sockaddr_in6) -> u32 {
    addr.sin6_scope_id
}

#[cfg(windows)]
#[inline]
pub(crate) const fn sockaddr_in6_sin6_scope_id(addr: libc::sockaddr_in6) -> u32 {
    let addr: sockaddr_in6 = unsafe { std::mem::transmute(addr) };
    addr.sin6_scope_id
}

#[cfg(not(windows))]
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
    sin6_family: libc::sa_family_t,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: libc::in6_addr,
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

#[cfg(windows)]
#[inline]
pub(crate) const fn sockaddr_in6_new(
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: libc::in6_addr,
    sin6_scope_id: u32,
) -> libc::sockaddr_in6 {
    let addr = sockaddr_in6 {
        sin6_family,
        sin6_port,
        sin6_flowinfo,
        sin6_addr,
        sin6_scope_id,
    };
    unsafe { std::mem::transmute(addr) }
}
