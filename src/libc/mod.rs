pub(crate) mod conv;
pub(crate) mod sockaddr;
pub(crate) mod sockaddr_header;

#[cfg(all(
    libc,
    any(
        target_os = "android",
        target_os = "linux",
        target_os = "emscripten",
        target_os = "l4re",
    )
))]
pub(crate) use libc::off64_t as libc_off_t;
#[cfg(all(
    libc,
    not(any(
        target_os = "android",
        target_os = "linux",
        target_os = "emscripten",
        target_os = "l4re",
    ))
))]
pub(crate) use libc::off_t as libc_off_t;

pub use sockaddr::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrUnix, SocketAddrV4, SocketAddrV6};
