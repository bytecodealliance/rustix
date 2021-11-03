#[cfg(windows)]
use super::libc;

#[cfg(not(windows))]
#[inline]
pub(crate) const fn in_addr_s_addr(addr: libc::in_addr) -> u32 {
    addr.s_addr
}

#[cfg(windows)]
#[inline]
pub(crate) const fn in_addr_s_addr(addr: libc::in_addr) -> u32 {
    unsafe { *addr.S_un.S_addr() }
}

#[cfg(not(windows))]
#[inline]
pub(crate) const fn in_addr_new(s_addr: u32) -> libc::in_addr {
    libc::in_addr { s_addr }
}

#[cfg(windows)]
#[inline]
pub(crate) const fn in_addr_new(s_addr: u32) -> libc::in_addr {
    let mut me = libc::in_addr::default();
    unsafe {
        *me.S_un.S_addr_mut() = s_addr;
    }
    me
}

#[cfg(not(windows))]
#[inline]
pub(crate) const fn in6_addr_s6_addr(addr: libc::in6_addr) -> [u8; 16] {
    addr.s6_addr
}

#[cfg(windows)]
#[inline]
pub(crate) const fn in6_addr_s6_addr(addr: libc::in6_addr) -> [u8; 16] {
    unsafe { *addr.u.Byte() }
}

#[cfg(not(windows))]
#[inline]
pub(crate) const fn in6_addr_new(s6_addr: [u8; 16]) -> libc::in6_addr {
    libc::in6_addr { s6_addr }
}

#[cfg(windows)]
#[inline]
pub(crate) const fn in6_addr_new(s6_addr: [u8; 16]) -> libc::in6_addr {
    let mut me = libc::in6_addr::default();
    unsafe {
        *me.u.Byte_mut() = s6_addr;
    }
    me
}

#[cfg(not(windows))]
#[inline]
pub(crate) const fn sockaddr_in6_sin6_scope_id(addr: libc::sockaddr_in6) -> u32 {
    addr.sin6_scope_id
}

#[cfg(windows)]
#[inline]
pub(crate) const fn sockaddr_in6_sin6_scope_id(addr: libc::sockaddr_in6) -> u32 {
    unsafe { *addr.u.sin6_scope_id() }
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
    let mut me = libc::sockaddr_in6::default();
    me.sin6_family = sin6_family;
    me.sin6_port = sin6_port;
    me.sin6_flowinfo = sin6_flowinfo;
    me.sin6_addr = sin6_addr;
    unsafe {
        *me.u.sin6_scope_id_mut() = sin6_scope_id;
    }
    me
}
