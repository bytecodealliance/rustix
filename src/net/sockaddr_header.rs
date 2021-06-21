use crate::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrUnix, SocketAddrV4, SocketAddrV6};
use libc::sockaddr_storage;

// This must match the header of `sockaddr`.
#[repr(C)]
struct sockaddr_header {
    #[cfg(any(
        target_os = "netbsd",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd"
    ))]
    sa_len: u8,
    #[cfg(any(
        target_os = "netbsd",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd"
    ))]
    ss_family: u8,
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd"
    )))]
    ss_family: u16,
}

#[inline]
unsafe fn read_ss_family(storage: *const sockaddr_storage) -> u16 {
    // Assert that we know the layout of `sockaddr`.
    let _ = libc::sockaddr {
        #[cfg(any(
            target_os = "netbsd",
            target_os = "macos",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "openbsd"
        ))]
        sa_len: 0_u8,
        #[cfg(any(
            target_os = "netbsd",
            target_os = "macos",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "openbsd"
        ))]
        sa_family: 0_u8,
        #[cfg(not(any(
            target_os = "netbsd",
            target_os = "macos",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "openbsd"
        )))]
        sa_family: 0_u16,
        sa_data: [0; 14],
    };

    (*storage.cast::<sockaddr_header>()).ss_family.into()
}

pub(crate) unsafe fn decode_sockaddr(storage: *const sockaddr_storage) -> SocketAddr {
    match read_ss_family(storage).into() {
        libc::AF_INET => {
            let decode = *storage.cast::<libc::sockaddr_in>();
            SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr(decode.sin_addr),
                u16::from_be(decode.sin_port),
            ))
        }
        libc::AF_INET6 => {
            let decode = *storage.cast::<libc::sockaddr_in6>();
            SocketAddr::V6(SocketAddrV6::new(
                Ipv6Addr(decode.sin6_addr),
                u16::from_be(decode.sin6_port),
                decode.sin6_flowinfo,
                decode.sin6_scope_id,
            ))
        }
        libc::AF_UNIX => {
            let decode = *storage.cast::<libc::sockaddr_un>();
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
