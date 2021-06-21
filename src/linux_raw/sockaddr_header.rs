//! The BSD sockets API requires us to read the `ss_family` field before
//! we can interpret the rest of a `sockaddr` produced by the kernel.
#![allow(unsafe_code)]

use super::sockaddr::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrUnix, SocketAddrV4, SocketAddrV6};
use linux_raw_sys::general::{__kernel_sockaddr_storage, sockaddr};

// This must match the header of `sockaddr`.
#[repr(C)]
struct sockaddr_header {
    ss_family: u16,
}

#[inline]
unsafe fn read_ss_family(storage: *const sockaddr) -> u16 {
    // Assert that we know the layout of `sockaddr`.
    let _ = sockaddr {
        __storage: __kernel_sockaddr_storage {
            ss_family: 0_u16,
            __data: [0; 126_usize],
        },
    };

    (*storage.cast::<sockaddr_header>()).ss_family
}

pub(crate) unsafe fn decode_sockaddr(storage: *const sockaddr) -> SocketAddr {
    match read_ss_family(storage).into() {
        linux_raw_sys::general::AF_INET => {
            let decode = *storage.cast::<linux_raw_sys::general::sockaddr_in>();
            SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr(decode.sin_addr),
                u16::from_be(decode.sin_port),
            ))
        }
        linux_raw_sys::general::AF_INET6 => {
            let decode = *storage.cast::<linux_raw_sys::general::sockaddr_in6>();
            SocketAddr::V6(SocketAddrV6::new(
                Ipv6Addr(decode.sin6_addr),
                u16::from_be(decode.sin6_port),
                decode.sin6_flowinfo,
                decode.sin6_scope_id,
            ))
        }
        linux_raw_sys::general::AF_LOCAL => {
            let decode = *storage.cast::<linux_raw_sys::general::sockaddr_un>();
            SocketAddr::Unix(
                SocketAddrUnix::new(
                    decode
                        .sun_path
                        .iter()
                        .map(|c| *c as u8)
                        .collect::<Vec<u8>>(),
                )
                .unwrap(),
            )
        }
        other => unimplemented!("{:?}", other),
    }
}
