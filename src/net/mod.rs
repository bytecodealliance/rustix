//! Network-related operations.

mod send_recv;
mod socket;

pub use send_recv::{recv, send};
pub use socket::{
    bind_in, bind_in6, bind_un, connect_in, connect_in6, connect_un, listen, shutdown, socket,
    socket_type, AddressFamily, Protocol, SocketType,
};

#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use socket::accept;
#[cfg(not(any(target_os = "ios", target_os = "macos")))]
pub use socket::{accept4, AcceptFlags};

#[cfg(libc)]
pub use libc::sockaddr_un as SocketAddrUnix;

#[cfg(linux_raw)]
pub use linux_raw_sys::general::sockaddr_un as SocketAddrUnix;

#[cfg(libc)]
pub use libc::sockaddr_in as SocketAddrV4;

#[cfg(linux_raw)]
pub use linux_raw_sys::general::sockaddr_in as SocketAddrV4;

#[cfg(libc)]
pub use libc::sockaddr_in6 as SocketAddrV6;

#[cfg(linux_raw)]
pub use linux_raw_sys::general::sockaddr_in6 as SocketAddrV6;

/// `accept` can dynamically accept any kind of address.
pub enum SocketAddr {
    /// An IPv4 socket address.
    V4(SocketAddrV4),
    /// An IPv6 socket address.
    V6(SocketAddrV6),
    /// A Unix-domain socket address.
    Unix(SocketAddrUnix),
}
