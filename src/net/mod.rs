//! Network-related operations.

use crate::imp;

mod send_recv;
mod socket;
#[cfg(not(any(windows, target_os = "wasi")))]
mod socketpair;
#[cfg(windows)]
mod wsa;

pub mod sockopt;

#[cfg(not(windows))]
pub use send_recv::sendto_unix;
pub use send_recv::{recv, recvfrom, send, sendto_v4, sendto_v6, RecvFlags, SendFlags};
pub use socket::{
    accept, accept_with, acceptfrom, acceptfrom_with, bind_v4, bind_v6, connect_v4, connect_v6,
    getpeername, getsockname, listen, shutdown, socket, socket_with, AcceptFlags, AddressFamily,
    Protocol, Shutdown, SocketFlags, SocketType,
};
#[cfg(not(windows))]
pub use socket::{bind_unix, connect_unix};
#[cfg(not(any(windows, target_os = "wasi")))]
pub use socketpair::socketpair;
#[cfg(windows)]
pub use wsa::{wsa_cleanup, wsa_startup};

#[cfg(not(windows))]
pub use imp::net::SocketAddrUnix;
pub use imp::net::{SocketAddr, SocketAddrStorage};
pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};
