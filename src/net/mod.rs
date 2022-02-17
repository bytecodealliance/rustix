//! Network-related operations.
//!
//! On Windows, one must call [`wsa_startup`] in the process before calling any
//! of these APIs. [`wsa_cleanup`] may be used in the process if these APIs are
//! no longer needed.
//!
//! [`wsa_startup`]: https://docs.rs/rustix/latest/x86_64-pc-windows-msvc/rustix/net/fn.wsa_startup.html
//! [`wsa_cleanup`]: https://docs.rs/rustix/latest/x86_64-pc-windows-msvc/rustix/net/fn.wsa_cleanup.html

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
    recv, recvfrom, recvmsg, recvmsg_v4, recvmsg_v6, send, sendmsg_v4, sendmsg_v6, sendto,
    sendto_any, sendto_v4, sendto_v6, RecvFlags, RecvMsgAny, RecvMsgV4, RecvMsgV6, SendFlags,
};
#[cfg(unix)]
pub use send_recv::{
    recvmsg_unix, recvmsg_unix_with_ancillary, recvmsg_v4_with_ancillary,
    recvmsg_v6_with_ancillary, recvmsg_with_ancillary, sendmsg_unix, sendmsg_unix_with_ancillary,
    sendmsg_v4_with_ancillary, sendmsg_v6_with_ancillary, sendto_unix, RecvMsgUnix,
};
pub use socket::{
    accept, accept_with, acceptfrom, acceptfrom_with, bind, bind_any, bind_v4, bind_v6, connect,
    connect_any, connect_v4, connect_v6, getpeername, getsockname, listen, shutdown, socket,
    socket_with, AcceptFlags, AddressFamily, Protocol, Shutdown, SocketFlags, SocketType,
};
#[cfg(unix)]
pub use socket::{bind_unix, connect_unix, SocketAddrUnix};
pub use socket_addr_any::{SocketAddrAny, SocketAddrStorage};
#[cfg(not(windows))]
pub use socket_ancillary::*;
#[cfg(not(any(windows, target_os = "wasi")))]
pub use socketpair::socketpair;
#[cfg(windows)]
pub use wsa::{wsa_cleanup, wsa_startup};

// Declare the `Ip` and `Socket` address types.
#[cfg(not(feature = "std"))]
pub use addr::{SocketAddr, SocketAddrV4, SocketAddrV6};
#[cfg(not(feature = "std"))]
pub use ip::{IpAddr, Ipv4Addr, Ipv6Addr, Ipv6MulticastScope};
#[cfg(feature = "std")]
pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

#[cfg(not(windows))]
pub(crate) use send_recv::{
    encode_msghdr_any_recv, encode_msghdr_unix_recv, encode_msghdr_unix_send,
    encode_msghdr_v4_recv, encode_msghdr_v4_send, encode_msghdr_v6_recv, encode_msghdr_v6_send,
    encode_socketaddr_unix_opt,
};
pub(crate) use send_recv::{encode_socketaddr_v4_opt, encode_socketaddr_v6_opt};
