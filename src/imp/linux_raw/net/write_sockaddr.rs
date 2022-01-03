//! The BSD sockets API requires us to read the `ss_family` field before
//! we can interpret the rest of a `sockaddr` produced by the kernel.
#![allow(unsafe_code)]

use super::super::c;
use crate::net::{SocketAddrAny, SocketAddrStorage, SocketAddrUnix, SocketAddrV4, SocketAddrV6};
use core::mem::size_of;

pub(crate) unsafe fn write_sockaddr(
    addr: &SocketAddrAny,
    storage: *mut SocketAddrStorage,
) -> usize {
    match addr {
        SocketAddrAny::V4(v4) => write_sockaddr_v4(v4, storage),
        SocketAddrAny::V6(v6) => write_sockaddr_v6(v6, storage),
        SocketAddrAny::Unix(unix) => write_sockaddr_unix(unix, storage),
    }
}

pub(crate) unsafe fn encode_sockaddr_v4(v4: &SocketAddrV4) -> linux_raw_sys::general::sockaddr_in {
    linux_raw_sys::general::sockaddr_in {
        sin_family: linux_raw_sys::general::AF_INET as _,
        sin_port: u16::to_be(v4.port()),
        sin_addr: linux_raw_sys::general::in_addr {
            s_addr: u32::from_ne_bytes(v4.ip().octets()),
        },
        __pad: [0; 8_usize],
    }
}

unsafe fn write_sockaddr_v4(v4: &SocketAddrV4, storage: *mut SocketAddrStorage) -> usize {
    let encoded = encode_sockaddr_v4(v4);
    core::ptr::write(storage.cast(), encoded);
    size_of::<linux_raw_sys::general::sockaddr_in>()
}

pub(crate) unsafe fn encode_sockaddr_v6(v6: &SocketAddrV6) -> linux_raw_sys::general::sockaddr_in6 {
    linux_raw_sys::general::sockaddr_in6 {
        sin6_family: linux_raw_sys::general::AF_INET6 as _,
        sin6_port: u16::to_be(v6.port()),
        sin6_flowinfo: u32::to_be(v6.flowinfo()),
        sin6_addr: linux_raw_sys::general::in6_addr {
            in6_u: linux_raw_sys::general::in6_addr__bindgen_ty_1 {
                u6_addr8: v6.ip().octets(),
            },
        },
        sin6_scope_id: v6.scope_id(),
    }
}

unsafe fn write_sockaddr_v6(v6: &SocketAddrV6, storage: *mut SocketAddrStorage) -> usize {
    let encoded = encode_sockaddr_v6(v6);
    core::ptr::write(storage.cast(), encoded);
    size_of::<linux_raw_sys::general::sockaddr_in6>()
}

pub(crate) unsafe fn encode_sockaddr_unix(
    unix: &SocketAddrUnix,
) -> linux_raw_sys::general::sockaddr_un {
    let mut encoded = linux_raw_sys::general::sockaddr_un {
        sun_family: linux_raw_sys::general::AF_UNIX as _,
        sun_path: [0; 108_usize],
    };
    let bytes = unix.path().to_bytes();
    for (i, b) in bytes.iter().enumerate() {
        encoded.sun_path[i] = *b as c::c_char;
    }
    encoded.sun_path[bytes.len()] = b'\0' as c::c_char;
    encoded
}

unsafe fn write_sockaddr_unix(unix: &SocketAddrUnix, storage: *mut SocketAddrStorage) -> usize {
    let encoded = encode_sockaddr_unix(unix);
    core::ptr::write(storage.cast(), encoded);
    super::offsetof_sun_path() + unix.path().to_bytes().len() + 1
}
