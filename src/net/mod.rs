//! Network-related operations.

mod send_recv;
mod socket;

pub use send_recv::{recv, send};
pub use socket::{socket_type, AddressFamily, SocketType};
