mod addr;
mod read_sockaddr;
mod send_recv;
mod types;
mod write_sockaddr;

pub(crate) mod ext;
pub(crate) mod syscalls;
pub(crate) use read_sockaddr::{read_sockaddr, read_sockaddr_os};
pub(crate) use write_sockaddr::{
    encode_sockaddr_unix, encode_sockaddr_v4, encode_sockaddr_v6, write_sockaddr,
};

pub use addr::{SocketAddrStorage, SocketAddrUnix};
pub use send_recv::{RecvFlags, SendFlags};
pub use types::{AcceptFlags, AddressFamily, Protocol, Shutdown, SocketFlags, SocketType, Timeout};

/// Return the offset of the `sun_path` field of `sockaddr_un`.
#[inline]
pub(crate) fn offsetof_sun_path() -> usize {
    let z = linux_raw_sys::general::sockaddr_un {
        sun_family: 0_u16,
        sun_path: [0; 108],
    };
    (crate::as_ptr(&z.sun_path) as usize) - (crate::as_ptr(&z) as usize)
}
