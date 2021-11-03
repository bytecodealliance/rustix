//! The BSD sockets API requires us to read the `ss_family` field before
//! we can interpret the rest of a `sockaddr` produced by the kernel.
#![allow(unsafe_code)]

#[cfg(windows)]
use super::super::libc;
use super::ext::{in6_addr_new, in_addr_new, sockaddr_in6_new};
use super::SocketAddrStorage;
#[cfg(not(windows))]
use super::SocketAddrUnix;
use crate::net::{SocketAddrAny, SocketAddrV4, SocketAddrV6};
use std::mem::size_of;

pub(crate) unsafe fn write_sockaddr(
    addr: &SocketAddrAny,
    storage: *mut SocketAddrStorage,
) -> usize {
    match addr {
        SocketAddrAny::V4(v4) => write_sockaddr_v4(v4, storage),
        SocketAddrAny::V6(v6) => write_sockaddr_v6(v6, storage),
        #[cfg(not(windows))]
        SocketAddrAny::Unix(unix) => write_sockaddr_unix(unix, storage),
    }
}

pub(crate) unsafe fn encode_sockaddr_v4(v4: &SocketAddrV4) -> libc::sockaddr_in {
    libc::sockaddr_in {
        #[cfg(any(
            target_os = "netbsd",
            target_os = "macos",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "openbsd"
        ))]
        sin_len: size_of::<libc::sockaddr_in>() as _,
        sin_family: libc::AF_INET as _,
        sin_port: u16::to_be(v4.port()),
        sin_addr: in_addr_new(u32::from_ne_bytes(v4.ip().octets())),
        sin_zero: [0; 8usize],
    }
}

unsafe fn write_sockaddr_v4(v4: &SocketAddrV4, storage: *mut SocketAddrStorage) -> usize {
    let encoded = encode_sockaddr_v4(v4);
    std::ptr::write(storage.cast::<_>(), encoded);
    size_of::<libc::sockaddr_in>()
}

pub(crate) unsafe fn encode_sockaddr_v6(v6: &SocketAddrV6) -> libc::sockaddr_in6 {
    #[cfg(any(
        target_os = "netbsd",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd"
    ))]
    {
        sockaddr_in6_new(
            size_of::<libc::sockaddr_in6>() as _,
            libc::AF_INET6 as _,
            u16::to_be(v6.port()),
            u32::to_be(v6.flowinfo()),
            in6_addr_new(v6.ip().octets()),
            v6.scope_id(),
        )
    }
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd"
    )))]
    {
        sockaddr_in6_new(
            libc::AF_INET6 as _,
            u16::to_be(v6.port()),
            u32::to_be(v6.flowinfo()),
            in6_addr_new(v6.ip().octets()),
            v6.scope_id(),
        )
    }
}

unsafe fn write_sockaddr_v6(v6: &SocketAddrV6, storage: *mut SocketAddrStorage) -> usize {
    let encoded = encode_sockaddr_v6(v6);
    std::ptr::write(storage.cast::<_>(), encoded);
    size_of::<libc::sockaddr_in6>()
}

#[cfg(not(windows))]
pub(crate) unsafe fn encode_sockaddr_unix(unix: &SocketAddrUnix) -> libc::sockaddr_un {
    let mut encoded = libc::sockaddr_un {
        #[cfg(any(
            target_os = "netbsd",
            target_os = "macos",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "openbsd"
        ))]
        sun_len: size_of::<libc::sockaddr_un>() as _,
        sun_family: libc::AF_UNIX as _,
        #[cfg(any(
            target_os = "netbsd",
            target_os = "macos",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "openbsd"
        ))]
        sun_path: [0; 104],
        #[cfg(not(any(
            target_os = "netbsd",
            target_os = "macos",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "openbsd"
        )))]
        sun_path: [0; 108],
    };
    let bytes = unix.path().to_bytes();
    for (i, b) in bytes.iter().enumerate() {
        encoded.sun_path[i] = *b as libc::c_char;
    }
    encoded.sun_path[bytes.len()] = b'\0' as libc::c_char;
    encoded
}

#[cfg(not(windows))]
unsafe fn write_sockaddr_unix(unix: &SocketAddrUnix, storage: *mut SocketAddrStorage) -> usize {
    let encoded = encode_sockaddr_unix(unix);
    std::ptr::write(storage.cast::<_>(), encoded);
    super::offsetof_sun_path() + unix.path().to_bytes().len() + 1
}
