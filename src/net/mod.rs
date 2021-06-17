//! Network-related operations.

mod send_recv;
mod socket;

pub use send_recv::{recv, send};
pub use socket::{
    bind_in, bind_in6, bind_un, connect_in, connect_in6, connect_un, listen, shutdown, socket,
    socket_type, AddressFamily, Protocol, SocketType,
};

#[cfg(libc)]
pub use libc::sockaddr_un as SockaddrUn;

#[cfg(linux_raw)]
pub use linux_raw_sys::general::sockaddr_un as SockaddrUn;

#[cfg(libc)]
pub use libc::sockaddr_in as SockaddrIn;

#[cfg(linux_raw)]
pub use linux_raw_sys::general::sockaddr_in as SockaddrIn;

#[cfg(libc)]
pub use libc::sockaddr_in6 as SockaddrIn6;

#[cfg(linux_raw)]
pub use linux_raw_sys::general::sockaddr_in6 as SockaddrIn6;
