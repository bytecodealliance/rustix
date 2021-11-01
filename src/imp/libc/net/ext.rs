#[cfg(windows)]
use super::libc;

pub(crate) trait Ipv4AddrExt {
    fn s_addr(&self) -> u32;
    fn new(s_addr: u32) -> Self;
}

impl Ipv4AddrExt for libc::in_addr {
    #[cfg(not(windows))]
    #[inline]
    fn s_addr(&self) -> u32 {
        self.s_addr
    }

    #[cfg(windows)]
    #[inline]
    fn s_addr(&self) -> u32 {
        unsafe { *self.S_un.S_addr() }
    }

    #[cfg(not(windows))]
    #[inline]
    fn new(s_addr: u32) -> Self {
        Self { s_addr }
    }

    #[cfg(windows)]
    #[inline]
    fn new(s_addr: u32) -> Self {
        let mut me = Self::default();
        unsafe {
            *me.S_un.S_addr_mut() = s_addr;
        }
        me
    }
}

pub(crate) trait Ipv6AddrExt {
    fn s6_addr(&self) -> [u8; 16];
    fn new(s6_addr: [u8; 16]) -> Self;
}

impl Ipv6AddrExt for libc::in6_addr {
    #[cfg(not(windows))]
    #[inline]
    fn s6_addr(&self) -> [u8; 16] {
        self.s6_addr
    }

    #[cfg(windows)]
    #[inline]
    fn s6_addr(&self) -> [u8; 16] {
        unsafe { *self.u.Byte() }
    }

    #[cfg(not(windows))]
    #[inline]
    fn new(s6_addr: [u8; 16]) -> Self {
        Self { s6_addr }
    }

    #[cfg(windows)]
    #[inline]
    fn new(s6_addr: [u8; 16]) -> Self {
        let mut me = Self::default();
        unsafe {
            *me.u.Byte_mut() = s6_addr;
        }
        me
    }
}

pub(crate) trait SocketAddrV6Ext {
    fn sin6_scope_id(&self) -> u32;
    fn new(
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
    ) -> Self;
}

impl SocketAddrV6Ext for libc::sockaddr_in6 {
    #[cfg(not(windows))]
    #[inline]
    fn sin6_scope_id(&self) -> u32 {
        self.sin6_scope_id
    }

    #[cfg(windows)]
    #[inline]
    fn sin6_scope_id(&self) -> u32 {
        unsafe { *self.u.sin6_scope_id() }
    }

    #[cfg(not(windows))]
    #[inline]
    fn new(
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
    ) -> Self {
        Self {
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
    fn new(
        sin6_family: u16,
        sin6_port: u16,
        sin6_flowinfo: u32,
        sin6_addr: libc::in6_addr,
        sin6_scope_id: u32,
    ) -> Self {
        let mut me = Self::default();
        me.sin6_family = sin6_family;
        me.sin6_port = sin6_port;
        me.sin6_flowinfo = sin6_flowinfo;
        me.sin6_addr = sin6_addr;
        unsafe {
            *me.u.sin6_scope_id_mut() = sin6_scope_id;
        }
        me
    }
}
