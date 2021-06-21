//! Network-related operations.

mod addr;
mod send_recv;
mod socket;
#[cfg(not(target_os = "wasi"))]
mod socketpair;

pub use addr::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrUnix, SocketAddrV4, SocketAddrV6};
pub use send_recv::{
    recv, recvfrom, send, sendto_unix, sendto_v4, sendto_v6, RecvFlags, SendFlags,
};
pub use socket::{
    accept, bind_unix, bind_v4, bind_v6, connect_unix, connect_v4, connect_v6, getpeername,
    getsockname, listen, shutdown, socket, socket_type, AcceptFlags, AddressFamily, Protocol,
    SocketType,
};
#[cfg(not(target_os = "wasi"))]
pub use socketpair::socketpair;
