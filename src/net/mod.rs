//! Network-related operations.

use crate::imp;

mod send_recv;
mod socket;
#[cfg(not(target_os = "wasi"))]
mod socketpair;

pub use send_recv::{
    recv, recvfrom, send, sendto_unix, sendto_v4, sendto_v6, RecvFlags, SendFlags,
};
pub use socket::{
    accept, accept_with, acceptfrom, acceptfrom_with, bind_unix, bind_v4, bind_v6, connect_unix,
    connect_v4, connect_v6, getpeername, getsockname, getsockopt_socket_type, listen, shutdown,
    socket, socket_with, AcceptFlags, AddressFamily, Protocol, Shutdown, SocketFlags, SocketType,
};
#[cfg(not(target_os = "wasi"))]
pub use socketpair::socketpair;

pub use imp::net::{SocketAddr, SocketAddrStorage, SocketAddrUnix};
pub use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};
