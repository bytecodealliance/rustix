//! The BSD sockets API requires us to read the `ss_family` field before
//! we can interpret the rest of a `sockaddr` produced by the kernel.
#![allow(unsafe_code)]

use super::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrUnix, SocketAddrV4, SocketAddrV6};
use crate::as_ptr;
use linux_raw_sys::general::{__kernel_sockaddr_storage, sockaddr};
use std::mem::size_of;

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

pub(crate) unsafe fn decode_sockaddr(storage: *const sockaddr, len: u32) -> SocketAddr {
    let z = linux_raw_sys::general::sockaddr_un {
        sun_family: 0_u16,
        sun_path: [0; 108],
    };
    let offsetof_sun_path = (as_ptr(&z.sun_path) as usize) - (as_ptr(&z) as usize);

    assert!(len as usize >= size_of::<linux_raw_sys::general::__kernel_sa_family_t>());
    match read_ss_family(storage).into() {
        linux_raw_sys::general::AF_INET => {
            assert!(len as usize >= size_of::<linux_raw_sys::general::sockaddr_in>());
            let decode = *storage.cast::<linux_raw_sys::general::sockaddr_in>();
            SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr(decode.sin_addr),
                u16::from_be(decode.sin_port),
            ))
        }
        linux_raw_sys::general::AF_INET6 => {
            assert!(len as usize >= size_of::<linux_raw_sys::general::sockaddr_in6>());
            let decode = *storage.cast::<linux_raw_sys::general::sockaddr_in6>();
            SocketAddr::V6(SocketAddrV6::new(
                Ipv6Addr(decode.sin6_addr),
                u16::from_be(decode.sin6_port),
                decode.sin6_flowinfo,
                decode.sin6_scope_id,
            ))
        }
        linux_raw_sys::general::AF_UNIX => {
            assert!(len as usize >= offsetof_sun_path);
            if len as usize == offsetof_sun_path {
                SocketAddr::Unix(SocketAddrUnix::new(&[][..]).unwrap())
            } else {
                let decode = *storage.cast::<linux_raw_sys::general::sockaddr_un>();
                assert_eq!(
                    decode.sun_path[len as usize - 1 - offsetof_sun_path],
                    b'\0' as std::os::raw::c_char
                );
                SocketAddr::Unix(
                    SocketAddrUnix::new(
                        decode.sun_path[..len as usize - 1 - offsetof_sun_path]
                            .iter()
                            .map(|c| *c as u8)
                            .collect::<Vec<u8>>(),
                    )
                    .unwrap(),
                )
            }
        }
        other => unimplemented!("{:?}", other),
    }
}
