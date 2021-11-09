#[cfg(not(windows))]
use super::c;

mod addr;
mod read_sockaddr;
mod send_recv;
mod types;
mod write_sockaddr;

pub(crate) mod ext;
pub(crate) mod syscalls;
pub use addr::SocketAddrStorage;
#[cfg(not(windows))]
pub use addr::SocketAddrUnix;
pub(crate) use read_sockaddr::{read_sockaddr, read_sockaddr_os};
pub use send_recv::{RecvFlags, SendFlags};
pub use types::{AcceptFlags, AddressFamily, Protocol, Shutdown, SocketFlags, SocketType, Timeout};
#[cfg(not(windows))]
pub(crate) use write_sockaddr::encode_sockaddr_unix;
pub(crate) use write_sockaddr::{encode_sockaddr_v4, encode_sockaddr_v6, write_sockaddr};

/// Return the offset of the `sun_path` field of `sockaddr_un`.
#[cfg(not(windows))]
#[inline]
pub(crate) fn offsetof_sun_path() -> usize {
    let z = c::sockaddr_un {
        #[cfg(any(
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        sun_len: 0_u8,
        #[cfg(any(
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        sun_family: 0_u8,
        #[cfg(not(any(
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd"
        )))]
        sun_family: 0_u16,
        #[cfg(any(
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        sun_path: [0; 104],
        #[cfg(not(any(
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd"
        )))]
        sun_path: [0; 108],
    };
    (crate::as_ptr(&z.sun_path) as usize) - (crate::as_ptr(&z) as usize)
}
