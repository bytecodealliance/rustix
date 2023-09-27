//! `getsockopt` and `setsockopt` functions.
//!
//! In the rustix API, there is a separate function for each option, so that
//! it can be given an option-specific type signature.
//!
//! # References for all `get_*` functions:
//!
//!  - [POSIX `getsockopt`]
//!  - [Linux `getsockopt`]
//!  - [Winsock2 `getsockopt`]
//!  - [Apple `getsockopt`]
//!  - [FreeBSD `getsockopt`]
//!  - [NetBSD `getsockopt`]
//!  - [OpenBSD `getsockopt`]
//!  - [DragonFly BSD `getsockopt`]
//!  - [illumos `getsockopt`]
//!  - [glibc `getsockopt`]
//!
//! [POSIX `getsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockopt.html
//! [Linux `getsockopt`]: https://man7.org/linux/man-pages/man2/getsockopt.2.html
//! [Winsock2 `getsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-getsockopt
//! [Apple `getsockopt`]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getsockopt.2.html
//! [FreeBSD `getsockopt`]: https://man.freebsd.org/cgi/man.cgi?query=getsockopt&sektion=2
//! [NetBSD `getsockopt`]: https://man.netbsd.org/getsockopt.2
//! [OpenBSD `getsockopt`]: https://man.openbsd.org/getsockopt.2
//! [DragonFly BSD `getsockopt`]: https://man.dragonflybsd.org/?command=getsockopt&section=2
//! [illumos `getsockopt`]: https://illumos.org/man/3SOCKET/getsockopt
//! [glibc `getsockopt`]: https://www.gnu.org/software/libc/manual/html_node/Socket-Option-Functions.html
//!
//! # References for all `set_*` functions:
//!
//!  - [POSIX `setsockopt`]
//!  - [Linux `setsockopt`]
//!  - [Winsock2 `setsockopt`]
//!  - [Apple `setsockopt`]
//!  - [FreeBSD `setsockopt`]
//!  - [NetBSD `setsockopt`]
//!  - [OpenBSD `setsockopt`]
//!  - [DragonFly BSD `setsockopt`]
//!  - [illumos `setsockopt`]
//!  - [glibc `setsockopt`]
//!
//! [POSIX `setsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setsockopt.html
//! [Linux `setsockopt`]: https://man7.org/linux/man-pages/man2/setsockopt.2.html
//! [Winsock2 `setsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-setsockopt
//! [Apple `setsockopt`]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setsockopt.2.html
//! [FreeBSD `setsockopt`]: https://man.freebsd.org/cgi/man.cgi?query=setsockopt&sektion=2
//! [NetBSD `setsockopt`]: https://man.netbsd.org/setsockopt.2
//! [OpenBSD `setsockopt`]: https://man.openbsd.org/setsockopt.2
//! [DragonFly BSD `setsockopt`]: https://man.dragonflybsd.org/?command=setsockopt&section=2
//! [illumos `setsockopt`]: https://illumos.org/man/3SOCKET/setsockopt
//! [glibc `setsockopt`]: https://www.gnu.org/software/libc/manual/html_node/Socket-Option-Functions.html
//!
//! # References for `get_socket_*` and `set_socket_*` functions:
//!
//!  - [References for all `get_*` functions]
//!  - [References for all `set_*` functions]
//!  - [POSIX `sys/socket.h`]
//!  - [Linux `socket`]
//!  - [Winsock2 `SOL_SOCKET` options]
//!  - [glibc `SOL_SOCKET` Options]
//!
//! [POSIX `sys/socket.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/sys_socket.h.html
//! [Linux `socket`]: https://man7.org/linux/man-pages/man7/socket.7.html
//! [Winsock2 `SOL_SOCKET` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/sol-socket-socket-options
//! [glibc `SOL_SOCKET` options]: https://www.gnu.org/software/libc/manual/html_node/Socket_002dLevel-Options.html
//!
//! # References for `get_ip_*` and `set_ip_*` functions:
//!
//!  - [References for all `get_*` functions]
//!  - [References for all `set_*` functions]
//!  - [POSIX `netinet/in.h`]
//!  - [Linux `ip`]
//!  - [Winsock2 `IPPROTO_IP` options]
//!  - [Apple `ip`]
//!  - [FreeBSD `ip`]
//!  - [NetBSD `ip`]
//!  - [OpenBSD `ip`]
//!  - [DragonFly BSD `ip`]
//!  - [illumos `ip`]
//!
//! [POSIX `netinet/in.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/netinet_in.h.html
//! [Linux `ip`]: https://man7.org/linux/man-pages/man7/ip.7.html
//! [Winsock2 `IPPROTO_IP` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/ipproto-ip-socket-options
//! [Apple `ip`]: https://opensource.apple.com/source/xnu/xnu-7195.81.3/bsd/man/man4/ip.4.auto.html
//! [FreeBSD `ip`]: https://man.freebsd.org/cgi/man.cgi?query=ip&sektion=4
//! [NetBSD `ip`]: https://man.netbsd.org/ip.4
//! [OpenBSD `ip`]: https://man.openbsd.org/ip.4
//! [DragonFly BSD `ip`]: https://man.dragonflybsd.org/?command=ip&section=4
//! [illumos `ip`]: https://illumos.org/man/4P/ip
//!
//! # References for `get_ipv6_*` and `set_ipv6_*` functions:
//!
//!  - [References for all `get_*` functions]
//!  - [References for all `set_*` functions]
//!  - [POSIX `netinet/in.h`]
//!  - [Linux `ipv6`]
//!  - [Winsock2 `IPPROTO_IPV6` options]
//!  - [Apple `ip6`]
//!  - [FreeBSD `ip6`]
//!  - [NetBSD `ip6`]
//!  - [OpenBSD `ip6`]
//!  - [DragonFly BSD `ip6`]
//!  - [illumos `ip6`]
//!
//! [POSIX `netinet/in.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/netinet_in.h.html
//! [Linux `ipv6`]: https://man7.org/linux/man-pages/man7/ipv6.7.html
//! [Winsock2 `IPPROTO_IPV6` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/ipproto-ipv6-socket-options
//! [Apple `ip6`]: https://opensource.apple.com/source/xnu/xnu-7195.81.3/bsd/man/man4/ip6.4.auto.html
//! [FreeBSD `ip6`]: https://man.freebsd.org/cgi/man.cgi?query=ip6&sektion=4
//! [NetBSD `ip6`]: https://man.netbsd.org/ip6.4
//! [OpenBSD `ip6`]: https://man.openbsd.org/ip6.4
//! [DragonFly BSD `ip6`]: https://man.dragonflybsd.org/?command=ip6&section=4
//! [illumos `ip6`]: https://illumos.org/man/4P/ip6
//!
//! # References for `get_tcp_*` and `set_tcp_*` functions:
//!
//!  - [References for all `get_*` functions]
//!  - [References for all `set_*` functions]
//!  - [POSIX `netinet/tcp.h`]
//!  - [Linux `tcp`]
//!  - [Winsock2 `IPPROTO_TCP` options]
//!  - [Apple `tcp`]
//!  - [FreeBSD `tcp`]
//!  - [NetBSD `tcp`]
//!  - [OpenBSD `tcp`]
//!  - [DragonFly BSD `tcp`]
//!  - [illumos `tcp`]
//!
//! [POSIX `netinet/tcp.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/netinet_tcp.h.html
//! [Linux `tcp`]: https://man7.org/linux/man-pages/man7/tcp.7.html
//! [Winsock2 `IPPROTO_TCP` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/ipproto-tcp-socket-options
//! [Apple `tcp`]: https://opensource.apple.com/source/xnu/xnu-7195.81.3/bsd/man/man4/tcp.4.auto.html
//! [FreeBSD `tcp`]: https://man.freebsd.org/cgi/man.cgi?query=tcp&sektion=4
//! [NetBSD `tcp`]: https://man.netbsd.org/tcp.4
//! [OpenBSD `tcp`]: https://man.openbsd.org/tcp.4
//! [DragonFly BSD `tcp`]: https://man.dragonflybsd.org/?command=tcp&section=4
//! [illumos `tcp`]: https://illumos.org/man/4P/tcp
//!
//! [References for all `get_*` functions]: #references-for-all-get_-functions
//! [References for all `set_*` functions]: #references-for-all-set_-functions

#![doc(alias = "getsockopt")]
#![doc(alias = "setsockopt")]

#[cfg(not(any(
    apple,
    windows,
    target_os = "aix",
    target_os = "dragonfly",
    target_os = "emscripten",
    target_os = "espidf",
    target_os = "haiku",
    target_os = "netbsd",
    target_os = "nto",
)))]
use crate::net::AddressFamily;
use crate::net::{Ipv4Addr, Ipv6Addr, SocketType};
use crate::{backend, io};
use backend::c;
use backend::fd::AsFd;
use core::time::Duration;

/// Timeout identifier for use with [`set_socket_timeout`] and
/// [`get_socket_timeout`].
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum Timeout {
    /// `SO_RCVTIMEO`—Timeout for receiving.
    Recv = c::SO_RCVTIMEO as _,

    /// `SO_SNDTIMEO`—Timeout for sending.
    Send = c::SO_SNDTIMEO as _,
}

/// `getsockopt(fd, SOL_SOCKET, SO_TYPE)`—Returns the type of a socket.
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[inline]
#[doc(alias = "SO_TYPE")]
pub fn get_socket_type<Fd: AsFd>(fd: Fd) -> io::Result<SocketType> {
    backend::net::sockopt::get_socket_type(fd.as_fd())
}

/// `setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, value)`—Set whether local
/// addresses may be reused in `bind`.
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[inline]
#[doc(alias = "SO_REUSEADDR")]
pub fn set_socket_reuseaddr<Fd: AsFd>(fd: Fd, value: bool) -> io::Result<()> {
    backend::net::sockopt::set_socket_reuseaddr(fd.as_fd(), value)
}

/// `getsockopt(fd, SOL_SOCKET, SO_REUSEADDR)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[inline]
#[doc(alias = "SO_REUSEADDR")]
pub fn get_socket_reuseaddr<Fd: AsFd>(fd: Fd) -> io::Result<bool> {
    backend::net::sockopt::get_socket_reuseaddr(fd.as_fd())
}

/// `setsockopt(fd, SOL_SOCKET, SO_BROADCAST, broadcast)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[inline]
#[doc(alias = "SO_BROADCAST")]
pub fn set_socket_broadcast<Fd: AsFd>(fd: Fd, value: bool) -> io::Result<()> {
    backend::net::sockopt::set_socket_broadcast(fd.as_fd(), value)
}

/// `getsockopt(fd, SOL_SOCKET, SO_BROADCAST)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[inline]
#[doc(alias = "SO_BROADCAST")]
pub fn get_socket_broadcast<Fd: AsFd>(fd: Fd) -> io::Result<bool> {
    backend::net::sockopt::get_socket_broadcast(fd.as_fd())
}

/// `setsockopt(fd, SOL_SOCKET, SO_LINGER, value)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[inline]
#[doc(alias = "SO_LINGER")]
pub fn set_socket_linger<Fd: AsFd>(fd: Fd, value: Option<Duration>) -> io::Result<()> {
    backend::net::sockopt::set_socket_linger(fd.as_fd(), value)
}

/// `getsockopt(fd, SOL_SOCKET, SO_LINGER)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[inline]
#[doc(alias = "SO_LINGER")]
pub fn get_socket_linger<Fd: AsFd>(fd: Fd) -> io::Result<Option<Duration>> {
    backend::net::sockopt::get_socket_linger(fd.as_fd())
}

/// `setsockopt(fd, SOL_SOCKET, SO_PASSCRED, value)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[cfg(linux_kernel)]
#[inline]
#[doc(alias = "SO_PASSCRED")]
pub fn set_socket_passcred<Fd: AsFd>(fd: Fd, value: bool) -> io::Result<()> {
    backend::net::sockopt::set_socket_passcred(fd.as_fd(), value)
}

/// `getsockopt(fd, SOL_SOCKET, SO_PASSCRED)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[cfg(linux_kernel)]
#[inline]
#[doc(alias = "SO_PASSCRED")]
pub fn get_socket_passcred<Fd: AsFd>(fd: Fd) -> io::Result<bool> {
    backend::net::sockopt::get_socket_passcred(fd.as_fd())
}

/// `setsockopt(fd, SOL_SOCKET, id, timeout)`—Set the sending or receiving
/// timeout.
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[inline]
#[doc(alias = "SO_RCVTIMEO")]
#[doc(alias = "SO_SNDTIMEO")]
pub fn set_socket_timeout<Fd: AsFd>(
    fd: Fd,
    id: Timeout,
    timeout: Option<Duration>,
) -> io::Result<()> {
    backend::net::sockopt::set_socket_timeout(fd.as_fd(), id, timeout)
}

/// `getsockopt(fd, SOL_SOCKET, id)`—Get the sending or receiving timeout.
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[inline]
#[doc(alias = "SO_RCVTIMEO")]
#[doc(alias = "SO_SNDTIMEO")]
pub fn get_socket_timeout<Fd: AsFd>(fd: Fd, id: Timeout) -> io::Result<Option<Duration>> {
    backend::net::sockopt::get_socket_timeout(fd.as_fd(), id)
}

/// `getsockopt(fd, SOL_SOCKET, SO_ERROR)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[inline]
#[doc(alias = "SO_ERROR")]
pub fn get_socket_error<Fd: AsFd>(fd: Fd) -> io::Result<Result<(), io::Errno>> {
    backend::net::sockopt::get_socket_error(fd.as_fd())
}

/// `getsockopt(fd, SOL_SOCKET, SO_NOSIGPIPE)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[cfg(any(apple, target_os = "freebsd"))]
#[doc(alias = "SO_NOSIGPIPE")]
#[inline]
pub fn get_socket_nosigpipe<Fd: AsFd>(fd: Fd) -> io::Result<bool> {
    backend::net::sockopt::get_socket_nosigpipe(fd.as_fd())
}

/// `setsockopt(fd, SOL_SOCKET, SO_NOSIGPIPE, value)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[cfg(any(apple, target_os = "freebsd"))]
#[doc(alias = "SO_NOSIGPIPE")]
#[inline]
pub fn set_socket_nosigpipe<Fd: AsFd>(fd: Fd, value: bool) -> io::Result<()> {
    backend::net::sockopt::set_socket_nosigpipe(fd.as_fd(), value)
}

/// `setsockopt(fd, SOL_SOCKET, SO_KEEPALIVE, value)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[inline]
#[doc(alias = "SO_KEEPALIVE")]
pub fn set_socket_keepalive<Fd: AsFd>(fd: Fd, value: bool) -> io::Result<()> {
    backend::net::sockopt::set_socket_keepalive(fd.as_fd(), value)
}

/// `getsockopt(fd, SOL_SOCKET, SO_KEEPALIVE)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[inline]
#[doc(alias = "SO_KEEPALIVE")]
pub fn get_socket_keepalive<Fd: AsFd>(fd: Fd) -> io::Result<bool> {
    backend::net::sockopt::get_socket_keepalive(fd.as_fd())
}

/// `setsockopt(fd, SOL_SOCKET, SO_RCVBUF, value)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[inline]
#[doc(alias = "SO_RCVBUF")]
pub fn set_socket_recv_buffer_size<Fd: AsFd>(fd: Fd, value: usize) -> io::Result<()> {
    backend::net::sockopt::set_socket_recv_buffer_size(fd.as_fd(), value)
}

/// `getsockopt(fd, SOL_SOCKET, SO_RCVBUF)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[inline]
#[doc(alias = "SO_RCVBUF")]
pub fn get_socket_recv_buffer_size<Fd: AsFd>(fd: Fd) -> io::Result<usize> {
    backend::net::sockopt::get_socket_recv_buffer_size(fd.as_fd())
}

/// `setsockopt(fd, SOL_SOCKET, SO_SNDBUF, value)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[inline]
#[doc(alias = "SO_SNDBUF")]
pub fn set_socket_send_buffer_size<Fd: AsFd>(fd: Fd, value: usize) -> io::Result<()> {
    backend::net::sockopt::set_socket_send_buffer_size(fd.as_fd(), value)
}

/// `getsockopt(fd, SOL_SOCKET, SO_SNDBUF)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[inline]
#[doc(alias = "SO_SNDBUF")]
pub fn get_socket_send_buffer_size<Fd: AsFd>(fd: Fd) -> io::Result<usize> {
    backend::net::sockopt::get_socket_send_buffer_size(fd.as_fd())
}

/// `getsockopt(fd, SOL_SOCKET, SO_DOMAIN)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[cfg(not(any(
    apple,
    windows,
    target_os = "aix",
    target_os = "dragonfly",
    target_os = "emscripten",
    target_os = "espidf",
    target_os = "haiku",
    target_os = "netbsd",
    target_os = "nto",
)))]
#[inline]
#[doc(alias = "SO_DOMAIN")]
pub fn get_socket_domain<Fd: AsFd>(fd: Fd) -> io::Result<AddressFamily> {
    backend::net::sockopt::get_socket_domain(fd.as_fd())
}

/// `getsockopt(fd, SOL_SOCKET, SO_ACCEPTCONN)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[cfg(not(apple))] // Apple platforms declare the constant, but do not actually implement it.
#[inline]
#[doc(alias = "SO_ACCEPTCONN")]
pub fn get_socket_acceptconn<Fd: AsFd>(fd: Fd) -> io::Result<bool> {
    backend::net::sockopt::get_socket_acceptconn(fd.as_fd())
}

/// `setsockopt(fd, SOL_SOCKET, SO_OOBINLINE, value)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[inline]
#[doc(alias = "SO_OOBINLINE")]
pub fn set_socket_oobinline<Fd: AsFd>(fd: Fd, value: bool) -> io::Result<()> {
    backend::net::sockopt::set_socket_oobinline(fd.as_fd(), value)
}

/// `getsockopt(fd, SOL_SOCKET, SO_OOBINLINE)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[inline]
#[doc(alias = "SO_OOBINLINE")]
pub fn get_socket_oobinline<Fd: AsFd>(fd: Fd) -> io::Result<bool> {
    backend::net::sockopt::get_socket_oobinline(fd.as_fd())
}

/// `setsockopt(fd, IPPROTO_IP, IP_TTL, value)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions
#[inline]
#[doc(alias = "IP_TTL")]
pub fn set_ip_ttl<Fd: AsFd>(fd: Fd, value: u32) -> io::Result<()> {
    backend::net::sockopt::set_ip_ttl(fd.as_fd(), value)
}

/// `getsockopt(fd, IPPROTO_IP, IP_TTL)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions
#[inline]
#[doc(alias = "IP_TTL")]
pub fn get_ip_ttl<Fd: AsFd>(fd: Fd) -> io::Result<u32> {
    backend::net::sockopt::get_ip_ttl(fd.as_fd())
}

/// `setsockopt(fd, IPPROTO_IPV6, IPV6_V6ONLY, only_v6)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions
#[inline]
#[doc(alias = "IPV6_V6ONLY")]
pub fn set_ipv6_v6only<Fd: AsFd>(fd: Fd, value: bool) -> io::Result<()> {
    backend::net::sockopt::set_ipv6_v6only(fd.as_fd(), value)
}

/// `getsockopt(fd, IPPROTO_IPV6, IPV6_V6ONLY)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions
#[inline]
#[doc(alias = "IPV6_V6ONLY")]
pub fn get_ipv6_v6only<Fd: AsFd>(fd: Fd) -> io::Result<bool> {
    backend::net::sockopt::get_ipv6_v6only(fd.as_fd())
}

/// `setsockopt(fd, IPPROTO_IP, IP_MULTICAST_LOOP, value)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions
#[inline]
#[doc(alias = "IP_MULTICAST_LOOP")]
pub fn set_ip_multicast_loop<Fd: AsFd>(fd: Fd, value: bool) -> io::Result<()> {
    backend::net::sockopt::set_ip_multicast_loop(fd.as_fd(), value)
}

/// `getsockopt(fd, IPPROTO_IP, IP_MULTICAST_LOOP)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions
#[inline]
#[doc(alias = "IP_MULTICAST_LOOP")]
pub fn get_ip_multicast_loop<Fd: AsFd>(fd: Fd) -> io::Result<bool> {
    backend::net::sockopt::get_ip_multicast_loop(fd.as_fd())
}

/// `setsockopt(fd, IPPROTO_IP, IP_MULTICAST_TTL, value)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions
#[inline]
#[doc(alias = "IP_MULTICAST_TTL")]
pub fn set_ip_multicast_ttl<Fd: AsFd>(fd: Fd, value: u32) -> io::Result<()> {
    backend::net::sockopt::set_ip_multicast_ttl(fd.as_fd(), value)
}

/// `getsockopt(fd, IPPROTO_IP, IP_MULTICAST_TTL)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions
#[inline]
#[doc(alias = "IP_MULTICAST_TTL")]
pub fn get_ip_multicast_ttl<Fd: AsFd>(fd: Fd) -> io::Result<u32> {
    backend::net::sockopt::get_ip_multicast_ttl(fd.as_fd())
}

/// `setsockopt(fd, IPPROTO_IPV6, IPV6_MULTICAST_LOOP, value)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions
#[inline]
#[doc(alias = "IPV6_MULTICAST_LOOP")]
pub fn set_ipv6_multicast_loop<Fd: AsFd>(fd: Fd, value: bool) -> io::Result<()> {
    backend::net::sockopt::set_ipv6_multicast_loop(fd.as_fd(), value)
}

/// `getsockopt(fd, IPPROTO_IPV6, IPV6_MULTICAST_LOOP)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions
#[inline]
#[doc(alias = "IPV6_MULTICAST_LOOP")]
pub fn get_ipv6_multicast_loop<Fd: AsFd>(fd: Fd) -> io::Result<bool> {
    backend::net::sockopt::get_ipv6_multicast_loop(fd.as_fd())
}

/// `getsockopt(fd, IPPROTO_IPV6, IPV6_UNICAST_HOPS)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions
#[inline]
#[doc(alias = "IPV6_UNICAST_HOPS")]
pub fn get_ipv6_unicast_hops<Fd: AsFd>(fd: Fd) -> io::Result<u8> {
    backend::net::sockopt::get_ipv6_unicast_hops(fd.as_fd())
}

/// `setsockopt(fd, IPPROTO_IPV6, IPV6_UNICAST_HOPS, value)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions
#[inline]
#[doc(alias = "IPV6_UNICAST_HOPS")]
pub fn set_ipv6_unicast_hops<Fd: AsFd>(fd: Fd, value: Option<u8>) -> io::Result<()> {
    backend::net::sockopt::set_ipv6_unicast_hops(fd.as_fd(), value)
}

/// `setsockopt(fd, IPPROTO_IPV6, IPV6_MULTICAST_HOPS, value)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions
#[inline]
#[doc(alias = "IPV6_MULTICAST_HOPS")]
pub fn set_ipv6_multicast_hops<Fd: AsFd>(fd: Fd, value: u32) -> io::Result<()> {
    backend::net::sockopt::set_ipv6_multicast_hops(fd.as_fd(), value)
}

/// `getsockopt(fd, IPPROTO_IPV6, IPV6_MULTICAST_HOPS)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions
#[inline]
#[doc(alias = "IPV6_MULTICAST_HOPS")]
pub fn get_ipv6_multicast_hops<Fd: AsFd>(fd: Fd) -> io::Result<u32> {
    backend::net::sockopt::get_ipv6_multicast_hops(fd.as_fd())
}

/// `setsockopt(fd, IPPROTO_IP, IP_ADD_MEMBERSHIP, multiaddr, interface)`
///
/// This is similar to [`set_ip_add_membership`] but always sets `ifindex` value
/// to zero.
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions
#[inline]
#[doc(alias = "IP_ADD_MEMBERSHIP")]
pub fn set_ip_add_membership<Fd: AsFd>(
    fd: Fd,
    multiaddr: &Ipv4Addr,
    interface: &Ipv4Addr,
) -> io::Result<()> {
    backend::net::sockopt::set_ip_add_membership(fd.as_fd(), multiaddr, interface)
}

/// `setsockopt(fd, IPPROTO_IP, IP_ADD_MEMBERSHIP, multiaddr, address, ifindex)`
///
/// This is similar to [`set_ip_add_membership_with_ifindex`] but additionally
/// allows a `ifindex` value to be given.
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions
#[cfg(any(
    apple,
    freebsdlike,
    linux_like,
    target_os = "fuchsia",
    target_os = "openbsd"
))]
#[inline]
#[doc(alias = "IP_ADD_MEMBERSHIP")]
pub fn set_ip_add_membership_with_ifindex<Fd: AsFd>(
    fd: Fd,
    multiaddr: &Ipv4Addr,
    address: &Ipv4Addr,
    ifindex: i32,
) -> io::Result<()> {
    backend::net::sockopt::set_ip_add_membership_with_ifindex(
        fd.as_fd(),
        multiaddr,
        address,
        ifindex,
    )
}

/// `setsockopt(fd, IPPROTO_IP, IP_ADD_SOURCE_MEMBERSHIP)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions
#[cfg(any(apple, freebsdlike, linux_like, solarish, target_os = "aix"))]
#[inline]
#[doc(alias = "IP_ADD_SOURCE_MEMBERSHIP")]
pub fn set_ip_add_source_membership<Fd: AsFd>(
    fd: Fd,
    multiaddr: &Ipv4Addr,
    interface: &Ipv4Addr,
    sourceaddr: &Ipv4Addr,
) -> io::Result<()> {
    backend::net::sockopt::set_ip_add_source_membership(
        fd.as_fd(),
        multiaddr,
        interface,
        sourceaddr,
    )
}

/// `setsockopt(fd, IPPROTO_IP, IP_DROP_SOURCE_MEMBERSHIP)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions
#[cfg(any(apple, freebsdlike, linux_like, solarish, target_os = "aix"))]
#[inline]
#[doc(alias = "IP_DROP_SOURCE_MEMBERSHIP")]
pub fn set_ip_drop_source_membership<Fd: AsFd>(
    fd: Fd,
    multiaddr: &Ipv4Addr,
    interface: &Ipv4Addr,
    sourceaddr: &Ipv4Addr,
) -> io::Result<()> {
    backend::net::sockopt::set_ip_drop_source_membership(
        fd.as_fd(),
        multiaddr,
        interface,
        sourceaddr,
    )
}

/// `setsockopt(fd, IPPROTO_IPV6, IPV6_ADD_MEMBERSHIP, multiaddr, interface)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions
#[inline]
#[doc(alias = "IPV6_JOIN_GROUP")]
#[doc(alias = "IPV6_ADD_MEMBERSHIP")]
pub fn set_ipv6_add_membership<Fd: AsFd>(
    fd: Fd,
    multiaddr: &Ipv6Addr,
    interface: u32,
) -> io::Result<()> {
    backend::net::sockopt::set_ipv6_add_membership(fd.as_fd(), multiaddr, interface)
}

/// `setsockopt(fd, IPPROTO_IP, IP_DROP_MEMBERSHIP, multiaddr, interface)`
///
/// This is similar to [`set_ip_drop_membership`] but always sets `ifindex` value
/// to zero.
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions
#[inline]
#[doc(alias = "IP_DROP_MEMBERSHIP")]
pub fn set_ip_drop_membership<Fd: AsFd>(
    fd: Fd,
    multiaddr: &Ipv4Addr,
    interface: &Ipv4Addr,
) -> io::Result<()> {
    backend::net::sockopt::set_ip_drop_membership(fd.as_fd(), multiaddr, interface)
}

/// `setsockopt(fd, IPPROTO_IP, IP_DROP_MEMBERSHIP, multiaddr, interface)`
///
/// This is similar to [`set_ip_drop_membership_with_ifindex`] but additionally
/// allows a `ifindex` value to be given.
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions
#[cfg(any(
    apple,
    freebsdlike,
    linux_like,
    target_os = "fuchsia",
    target_os = "openbsd"
))]
#[inline]
#[doc(alias = "IP_DROP_MEMBERSHIP")]
pub fn set_ip_drop_membership_with_ifindex<Fd: AsFd>(
    fd: Fd,
    multiaddr: &Ipv4Addr,
    address: &Ipv4Addr,
    ifindex: i32,
) -> io::Result<()> {
    backend::net::sockopt::set_ip_drop_membership_with_ifindex(
        fd.as_fd(),
        multiaddr,
        address,
        ifindex,
    )
}

/// `setsockopt(fd, IPPROTO_IPV6, IPV6_DROP_MEMBERSHIP, multiaddr, interface)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions
#[inline]
#[doc(alias = "IPV6_LEAVE_GROUP")]
#[doc(alias = "IPV6_DROP_MEMBERSHIP")]
pub fn set_ipv6_drop_membership<Fd: AsFd>(
    fd: Fd,
    multiaddr: &Ipv6Addr,
    interface: u32,
) -> io::Result<()> {
    backend::net::sockopt::set_ipv6_drop_membership(fd.as_fd(), multiaddr, interface)
}

/// `setsockopt(fd, IPPROTO_IP, IP_TOS, value)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions
#[cfg(any(
    bsd,
    linux_like,
    target_os = "aix",
    target_os = "fuchsia",
    target_os = "haiku",
    target_os = "nto",
    target_env = "newlib"
))]
#[inline]
#[doc(alias = "IP_TOS")]
pub fn set_ip_tos<Fd: AsFd>(fd: Fd, value: u8) -> io::Result<()> {
    backend::net::sockopt::set_ip_tos(fd.as_fd(), value)
}

/// `getsockopt(fd, IPPROTO_IP, IP_TOS)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions
#[cfg(any(
    bsd,
    linux_like,
    target_os = "aix",
    target_os = "fuchsia",
    target_os = "haiku",
    target_os = "nto",
    target_env = "newlib"
))]
#[inline]
#[doc(alias = "IP_TOS")]
pub fn get_ip_tos<Fd: AsFd>(fd: Fd) -> io::Result<u8> {
    backend::net::sockopt::get_ip_tos(fd.as_fd())
}

/// `setsockopt(fd, IPPROTO_IP, IP_RECVTOS, value)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions
#[cfg(any(apple, linux_like, target_os = "freebsd", target_os = "fuchsia"))]
#[inline]
#[doc(alias = "IP_RECVTOS")]
pub fn set_ip_recvtos<Fd: AsFd>(fd: Fd, value: bool) -> io::Result<()> {
    backend::net::sockopt::set_ip_recvtos(fd.as_fd(), value)
}

/// `getsockopt(fd, IPPROTO_IP, IP_RECVTOS)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions
#[cfg(any(apple, linux_like, target_os = "freebsd", target_os = "fuchsia"))]
#[inline]
#[doc(alias = "IP_RECVTOS")]
pub fn get_ip_recvtos<Fd: AsFd>(fd: Fd) -> io::Result<bool> {
    backend::net::sockopt::get_ip_recvtos(fd.as_fd())
}

/// `setsockopt(fd, IPPROTO_IPV6, IPV6_RECVTCLASS, value)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions
#[cfg(any(
    bsd,
    linux_like,
    target_os = "aix",
    target_os = "fuchsia",
    target_os = "nto"
))]
#[inline]
#[doc(alias = "IPV6_RECVTCLASS")]
pub fn set_ipv6_recvtclass<Fd: AsFd>(fd: Fd, value: bool) -> io::Result<()> {
    backend::net::sockopt::set_ipv6_recvtclass(fd.as_fd(), value)
}

/// `getsockopt(fd, IPPROTO_IPV6, IPV6_RECVTCLASS)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions
#[cfg(any(
    bsd,
    linux_like,
    target_os = "aix",
    target_os = "fuchsia",
    target_os = "nto"
))]
#[inline]
#[doc(alias = "IPV6_RECVTCLASS")]
pub fn get_ipv6_recvtclass<Fd: AsFd>(fd: Fd) -> io::Result<bool> {
    backend::net::sockopt::get_ipv6_recvtclass(fd.as_fd())
}

/// `setsockopt(fd, IPPROTO_TCP, TCP_NODELAY, value)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions
#[inline]
#[doc(alias = "TCP_NODELAY")]
pub fn set_tcp_nodelay<Fd: AsFd>(fd: Fd, value: bool) -> io::Result<()> {
    backend::net::sockopt::set_tcp_nodelay(fd.as_fd(), value)
}

/// `getsockopt(fd, IPPROTO_TCP, TCP_NODELAY)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions
#[inline]
#[doc(alias = "TCP_NODELAY")]
pub fn get_tcp_nodelay<Fd: AsFd>(fd: Fd) -> io::Result<bool> {
    backend::net::sockopt::get_tcp_nodelay(fd.as_fd())
}

/// `setsockopt(fd, IPPROTO_TCP, TCP_KEEPCNT, value)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions
#[inline]
#[cfg(not(any(target_os = "openbsd", target_os = "haiku", target_os = "nto")))]
#[doc(alias = "TCP_KEEPCNT")]
pub fn set_tcp_keepcnt<Fd: AsFd>(fd: Fd, value: u32) -> io::Result<()> {
    backend::net::sockopt::set_tcp_keepcnt(fd.as_fd(), value)
}

/// `getsockopt(fd, IPPROTO_TCP, TCP_KEEPCNT)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions
#[inline]
#[cfg(not(any(target_os = "openbsd", target_os = "haiku", target_os = "nto")))]
#[doc(alias = "TCP_KEEPCNT")]
pub fn get_tcp_keepcnt<Fd: AsFd>(fd: Fd) -> io::Result<u32> {
    backend::net::sockopt::get_tcp_keepcnt(fd.as_fd())
}

/// `setsockopt(fd, IPPROTO_TCP, TCP_KEEPIDLE, value)`
///
/// `TCP_KEEPALIVE` on Apple platforms.
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions
#[inline]
#[cfg(not(any(target_os = "openbsd", target_os = "haiku", target_os = "nto")))]
#[doc(alias = "TCP_KEEPIDLE")]
pub fn set_tcp_keepidle<Fd: AsFd>(fd: Fd, value: Duration) -> io::Result<()> {
    backend::net::sockopt::set_tcp_keepidle(fd.as_fd(), value)
}

/// `getsockopt(fd, IPPROTO_TCP, TCP_KEEPIDLE)`
///
/// `TCP_KEEPALIVE` on Apple platforms.
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions
#[inline]
#[cfg(not(any(target_os = "openbsd", target_os = "haiku", target_os = "nto")))]
#[doc(alias = "TCP_KEEPIDLE")]
pub fn get_tcp_keepidle<Fd: AsFd>(fd: Fd) -> io::Result<Duration> {
    backend::net::sockopt::get_tcp_keepidle(fd.as_fd())
}

/// `setsockopt(fd, IPPROTO_TCP, TCP_KEEPINTVL, value)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions
#[inline]
#[cfg(not(any(target_os = "openbsd", target_os = "haiku", target_os = "nto")))]
#[doc(alias = "TCP_KEEPINTVL")]
pub fn set_tcp_keepintvl<Fd: AsFd>(fd: Fd, value: Duration) -> io::Result<()> {
    backend::net::sockopt::set_tcp_keepintvl(fd.as_fd(), value)
}

/// `getsockopt(fd, IPPROTO_TCP, TCP_KEEPINTVL)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions
#[inline]
#[cfg(not(any(target_os = "openbsd", target_os = "haiku", target_os = "nto")))]
#[doc(alias = "TCP_KEEPINTVL")]
pub fn get_tcp_keepintvl<Fd: AsFd>(fd: Fd) -> io::Result<Duration> {
    backend::net::sockopt::get_tcp_keepintvl(fd.as_fd())
}

/// `setsockopt(fd, IPPROTO_TCP, TCP_USER_TIMEOUT, value)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions
#[inline]
#[cfg(any(linux_like, target_os = "fuchsia"))]
#[doc(alias = "TCP_USER_TIMEOUT")]
pub fn set_tcp_user_timeout<Fd: AsFd>(fd: Fd, value: u32) -> io::Result<()> {
    backend::net::sockopt::set_tcp_user_timeout(fd.as_fd(), value)
}

/// `getsockopt(fd, IPPROTO_TCP, TCP_USER_TIMEOUT)`
///
/// See the [module-level documentation] for more.
///
/// [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions
#[inline]
#[cfg(any(linux_like, target_os = "fuchsia"))]
#[doc(alias = "TCP_USER_TIMEOUT")]
pub fn get_tcp_user_timeout<Fd: AsFd>(fd: Fd) -> io::Result<u32> {
    backend::net::sockopt::get_tcp_user_timeout(fd.as_fd())
}

#[test]
fn test_sizes() {
    use c::c_int;

    // Backend code needs to cast these to `c_int` so make sure that cast
    // isn't lossy.
    assert_eq_size!(Timeout, c_int);
}
