//! Network-related operations.

use crate::imp;

#[cfg(not(feature = "std"))]
mod addr;
#[cfg(not(feature = "std"))]
mod ip;
mod send_recv;
mod socket;
mod socket_addr_any;
#[cfg(not(windows))]
mod socket_ancillary;
#[cfg(not(any(windows, target_os = "wasi")))]
mod socketpair;
#[cfg(windows)]
mod wsa;

pub mod sockopt;

pub use send_recv::{
    recv, recvfrom, recvmsg_v4, recvmsg_v6, send, sendmsg_v4, sendmsg_v6, sendto_v4, sendto_v6,
    RecvFlags, RecvMsgV4, RecvMsgV6, SendFlags,
};
#[cfg(not(windows))]
pub use send_recv::{
    recvmsg_unix, recvmsg_unix_with_ancillary, recvmsg_v4_with_ancillary,
    recvmsg_v6_with_ancillary, sendmsg_unix, sendmsg_unix_with_ancillary,
    sendmsg_v4_with_ancillary, sendmsg_v6_with_ancillary, sendto_unix, RecvMsgUnix,
};
pub use socket::{
    accept, accept_with, acceptfrom, acceptfrom_with, bind_v4, bind_v6, connect_v4, connect_v6,
    getpeername, getsockname, listen, shutdown, socket, socket_with, AcceptFlags, AddressFamily,
    Protocol, Shutdown, SocketFlags, SocketType,
};
#[cfg(not(windows))]
pub use socket::{bind_unix, connect_unix};
pub use socket_addr_any::SocketAddrAny;
#[cfg(not(windows))]
pub use socket_ancillary::*;
#[cfg(not(any(windows, target_os = "wasi")))]
pub use socketpair::socketpair;
#[cfg(windows)]
pub use wsa::{wsa_cleanup, wsa_startup};

pub use imp::net::SocketAddrStorage;
#[cfg(not(windows))]
pub use imp::net::SocketAddrUnix;

// Declare the `Ip` and `Socket` address types.
#[cfg(not(feature = "std"))]
pub use addr::{SocketAddr, SocketAddrV4, SocketAddrV6};
#[cfg(not(feature = "std"))]
pub use ip::{IpAddr, Ipv4Addr, Ipv6Addr, Ipv6MulticastScope};
#[cfg(feature = "std")]
pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

#[cfg(not(windows))]
pub(crate) use send_recv::{encode_msghdr_unix_send, encode_socketaddr_unix_opt};
pub(crate) use send_recv::{
    encode_msghdr_v4_send, encode_msghdr_v6_send, encode_socketaddr_v4_opt,
    encode_socketaddr_v6_opt,
};
