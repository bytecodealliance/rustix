//! Network-related operations.

mod send_recv;
#[cfg(libc)]
mod sockaddr;
#[cfg(libc)]
mod sockaddr_header;
mod socket;
#[cfg(not(target_os = "wasi"))]
mod socketpair;

pub use send_recv::{
    recv, recvfrom, send, sendto_unix, sendto_v4, sendto_v6, RecvFlags, SendFlags,
};
pub use socket::{
    accept, bind_unix, bind_v4, bind_v6, connect_unix, connect_v4, connect_v6, getpeername,
    getsockname, getsockopt_socket_type, listen, shutdown, socket, AcceptFlags, AddressFamily,
    Protocol, SocketType,
};
#[cfg(not(target_os = "wasi"))]
pub use socketpair::socketpair;

#[cfg(linux_raw)]
pub use crate::linux_raw::{
    Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrUnix, SocketAddrV4, SocketAddrV6,
};
#[cfg(libc)]
pub use sockaddr::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrUnix, SocketAddrV4, SocketAddrV6};
