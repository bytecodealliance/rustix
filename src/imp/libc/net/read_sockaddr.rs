use super::{SocketAddr, SocketAddrUnix};
use crate::{as_ptr, io};
use libc::sockaddr_storage;
use std::ffi::CStr;
use std::mem::size_of;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

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

pub(crate) unsafe fn read_sockaddr(
    storage: *const sockaddr_storage,
    len: usize,
) -> io::Result<SocketAddr> {
    let offsetof_sun_path = super::offsetof_sun_path();

    if len < size_of::<libc::sa_family_t>() {
        return Err(io::Error::INVAL);
    }
    match read_ss_family(storage).into() {
        libc::AF_INET => {
            if len < size_of::<libc::sockaddr_in>() {
                return Err(io::Error::INVAL);
            }
            let decode = *storage.cast::<libc::sockaddr_in>();
            Ok(SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr::from(u32::from_be(decode.sin_addr.s_addr)),
                u16::from_be(decode.sin_port),
            )))
        }
        libc::AF_INET6 => {
            if len < size_of::<libc::sockaddr_in6>() {
                return Err(io::Error::INVAL);
            }
            let decode = *storage.cast::<libc::sockaddr_in6>();
            Ok(SocketAddr::V6(SocketAddrV6::new(
                Ipv6Addr::from(decode.sin6_addr.s6_addr),
                u16::from_be(decode.sin6_port),
                u32::from_be(decode.sin6_flowinfo),
                decode.sin6_scope_id,
            )))
        }
        libc::AF_UNIX => {
            if len < offsetof_sun_path {
                return Err(io::Error::INVAL);
            }
            if len == offsetof_sun_path {
                Ok(SocketAddr::Unix(SocketAddrUnix::new(&[][..]).unwrap()))
            } else {
                let decode = *storage.cast::<libc::sockaddr_un>();

                // Trim off unused bytes from the end of `path_bytes`.
                let path_bytes = if cfg!(target_os = "freebsd") {
                    // FreeBSD sometimes sets the length to longer than the length
                    // of the NUL-terminated string. Find the NUL and truncate the
                    // string accordingly.
                    &decode.sun_path[..decode.sun_path.iter().position(|b| *b == 0).unwrap()]
                } else {
                    // Otherwise, use the provided length.
                    let provided_len = len - 1 - offsetof_sun_path;
                    if decode.sun_path[provided_len] != b'\0' as libc::c_char {
                        return Err(io::Error::INVAL);
                    }
                    debug_assert_eq!(
                        CStr::from_ptr(decode.sun_path.as_ptr()).to_bytes().len(),
                        provided_len
                    );
                    &decode.sun_path[..provided_len]
                };

                Ok(SocketAddr::Unix(
                    SocketAddrUnix::new(path_bytes.iter().map(|c| *c as u8).collect::<Vec<u8>>())
                        .unwrap(),
                ))
            }
        }
        _ => Err(io::Error::INVAL),
    }
}

pub(crate) unsafe fn read_sockaddr_os(storage: *const sockaddr_storage, len: usize) -> SocketAddr {
    let z = libc::sockaddr_un {
        #[cfg(any(
            target_os = "netbsd",
            target_os = "macos",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "openbsd"
        ))]
        sun_len: 0_u8,
        #[cfg(any(
            target_os = "netbsd",
            target_os = "macos",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "openbsd"
        ))]
        sun_family: 0_u8,
        #[cfg(not(any(
            target_os = "netbsd",
            target_os = "macos",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "openbsd"
        )))]
        sun_family: 0_u16,
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
    let offsetof_sun_path = (as_ptr(&z.sun_path) as usize) - (as_ptr(&z) as usize);

    assert!(len >= size_of::<libc::sa_family_t>());
    match read_ss_family(storage).into() {
        libc::AF_INET => {
            assert!(len >= size_of::<libc::sockaddr_in>());
            let decode = *storage.cast::<libc::sockaddr_in>();
            SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr::from(u32::from_be(decode.sin_addr.s_addr)),
                u16::from_be(decode.sin_port),
            ))
        }
        libc::AF_INET6 => {
            assert!(len >= size_of::<libc::sockaddr_in6>());
            let decode = *storage.cast::<libc::sockaddr_in6>();
            SocketAddr::V6(SocketAddrV6::new(
                Ipv6Addr::from(decode.sin6_addr.s6_addr),
                u16::from_be(decode.sin6_port),
                u32::from_be(decode.sin6_flowinfo),
                decode.sin6_scope_id,
            ))
        }
        libc::AF_UNIX => {
            assert!(len >= offsetof_sun_path);
            if len == offsetof_sun_path {
                SocketAddr::Unix(SocketAddrUnix::new(&[][..]).unwrap())
            } else {
                let decode = *storage.cast::<libc::sockaddr_un>();
                assert_eq!(
                    decode.sun_path[len - 1 - offsetof_sun_path],
                    b'\0' as libc::c_char
                );
                let path_bytes = &decode.sun_path[..len - 1 - offsetof_sun_path];

                // FreeBSD sometimes sets the length to longer than the length
                // of the NUL-terminated string. Find the NUL and truncate the
                // string accordingly.
                #[cfg(target_os = "freebsd")]
                let path_bytes = &path_bytes[..path_bytes.iter().position(|b| *b == 0).unwrap()];

                SocketAddr::Unix(
                    SocketAddrUnix::new(path_bytes.iter().map(|c| *c as u8).collect::<Vec<u8>>())
                        .unwrap(),
                )
            }
        }
        other => unimplemented!("{:?}", other),
    }
}
