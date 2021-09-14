//! The BSD sockets API requires us to read the `ss_family` field before
//! we can interpret the rest of a `sockaddr` produced by the kernel.
#![allow(unsafe_code)]

use super::{SocketAddr, SocketAddrStorage, SocketAddrUnix};
use std::mem::size_of;
use std::net::{SocketAddrV4, SocketAddrV6};

pub(crate) unsafe fn write_sockaddr(addr: &SocketAddr, storage: *mut SocketAddrStorage) -> usize {
    match addr {
        SocketAddr::V4(v4) => write_sockaddr_v4(v4, storage),
        SocketAddr::V6(v6) => write_sockaddr_v6(v6, storage),
        SocketAddr::Unix(unix) => write_sockaddr_unix(unix, storage),
    }
}

pub(crate) unsafe fn encode_sockaddr_v4(v4: &SocketAddrV4) -> libc::sockaddr_in {
    libc::sockaddr_in {
        sin_family: libc::AF_INET as _,
        sin_port: u16::to_be(v4.port()),
        sin_addr: libc::in_addr {
            s_addr: u32::from_ne_bytes(v4.ip().octets()),
        },
        sin_zero: [0; 8usize],
    }
}

unsafe fn write_sockaddr_v4(v4: &SocketAddrV4, storage: *mut SocketAddrStorage) -> usize {
    let encoded = encode_sockaddr_v4(v4);
    std::ptr::write(storage.cast::<_>(), encoded);
    size_of::<libc::sockaddr_in>()
}

pub(crate) unsafe fn encode_sockaddr_v6(v6: &SocketAddrV6) -> libc::sockaddr_in6 {
    libc::sockaddr_in6 {
        sin6_family: libc::AF_INET6 as _,
        sin6_port: u16::to_be(v6.port()),
        sin6_flowinfo: u32::to_be(v6.flowinfo()),
        sin6_addr: libc::in6_addr {
            s6_addr: v6.ip().octets(),
        },
        sin6_scope_id: v6.scope_id(),
    }
}

unsafe fn write_sockaddr_v6(v6: &SocketAddrV6, storage: *mut SocketAddrStorage) -> usize {
    let encoded = encode_sockaddr_v6(v6);
    std::ptr::write(storage.cast::<_>(), encoded);
    size_of::<libc::sockaddr_in6>()
}

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

unsafe fn write_sockaddr_unix(unix: &SocketAddrUnix, storage: *mut SocketAddrStorage) -> usize {
    let encoded = encode_sockaddr_unix(unix);
    std::ptr::write(storage.cast::<_>(), encoded);
    super::offsetof_sun_path() + unix.path().to_bytes().len() + 1
}
