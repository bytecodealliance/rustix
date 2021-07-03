use crate::{
    imp, io,
    net::{SocketAddr, SocketAddrUnix, SocketAddrV4, SocketAddrV6},
};
use io_lifetimes::{AsFd, OwnedFd};
use std::os::raw::c_int;

pub use imp::net::{AcceptFlags, AddressFamily, Protocol, Shutdown, SocketType};

impl Default for Protocol {
    fn default() -> Self {
        Self::Ip
    }
}

/// `socket(domain, type_, protocol)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html
/// [Linux]: https://man7.org/linux/man-pages/man2/socket.2.html
#[inline]
pub fn socket(domain: AddressFamily, type_: SocketType, protocol: Protocol) -> io::Result<OwnedFd> {
    imp::syscalls::socket(domain, type_, protocol)
}

/// `bind(sockfd, addr, sizeof(struct sockaddr_in))`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html
/// [Linux]: https://man7.org/linux/man-pages/man2/bind.2.html
#[inline]
#[doc(alias = "bind")]
pub fn bind_v4<Fd: AsFd>(sockfd: &Fd, addr: &SocketAddrV4) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::bind_v4(sockfd, addr)
}

/// `bind(sockfd, addr, sizeof(struct sockaddr_in6))`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html
/// [Linux]: https://man7.org/linux/man-pages/man2/bind.2.html
#[inline]
#[doc(alias = "bind")]
pub fn bind_v6<Fd: AsFd>(sockfd: &Fd, addr: &SocketAddrV6) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::bind_v6(sockfd, addr)
}

/// `bind(sockfd, addr, sizeof(struct sockaddr_un))`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html
/// [Linux]: https://man7.org/linux/man-pages/man2/bind.2.html
#[inline]
#[doc(alias = "bind")]
pub fn bind_unix<Fd: AsFd>(sockfd: &Fd, addr: &SocketAddrUnix) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::bind_unix(sockfd, addr)
}

/// `connect(sockfd, addr, sizeof(struct sockaddr_in))`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html
/// [Linux]: https://man7.org/linux/man-pages/man2/connect.2.html
#[inline]
#[doc(alias = "connect")]
pub fn connect_v4<Fd: AsFd>(sockfd: &Fd, addr: &SocketAddrV4) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::connect_v4(sockfd, addr)
}

/// `connect(sockfd, addr, sizeof(struct sockaddr_in6))`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html
/// [Linux]: https://man7.org/linux/man-pages/man2/connect.2.html
#[inline]
#[doc(alias = "connect")]
pub fn connect_v6<Fd: AsFd>(sockfd: &Fd, addr: &SocketAddrV6) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::connect_v6(sockfd, addr)
}

/// `connect(sockfd, addr, sizeof(struct sockaddr_un))`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html
/// [Linux]: https://man7.org/linux/man-pages/man2/connect.2.html
#[doc(alias = "connect")]
#[inline]
pub fn connect_unix<Fd: AsFd>(sockfd: &Fd, addr: &SocketAddrUnix) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::connect_unix(sockfd, addr)
}

/// `listen(fd, backlog)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/listen.html
/// [Linux]: https://man7.org/linux/man-pages/man2/listen.2.html
#[inline]
pub fn listen<Fd: AsFd>(sockfd: &Fd, backlog: c_int) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::listen(sockfd, backlog)
}

/// `accept4(fd, NULL, NULL, flags)`
///
/// Use [`acceptfrom`] to retrieve the peer address.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/accept.html
/// [Linux]: https://man7.org/linux/man-pages/man2/accept.2.html
#[inline]
#[doc(alias = "accept4")]
pub fn accept<Fd: AsFd>(sockfd: &Fd, flags: AcceptFlags) -> io::Result<OwnedFd> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::accept(sockfd, flags)
}

/// `accept4(fd, &addr, &len, flags)`
///
/// Use [`accept`] if the peer address isn't needed.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/accept.html
/// [Linux]: https://man7.org/linux/man-pages/man2/accept.2.html
#[inline]
#[doc(alias = "accept4")]
pub fn acceptfrom<Fd: AsFd>(sockfd: &Fd, flags: AcceptFlags) -> io::Result<(OwnedFd, SocketAddr)> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::acceptfrom(sockfd, flags)
}

/// `shutdown(fd, how)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/shutdown.html
/// [Linux]: https://man7.org/linux/man-pages/man2/shutdown.2.html
#[inline]
pub fn shutdown<Fd: AsFd>(sockfd: &Fd, how: Shutdown) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::shutdown(sockfd, how)
}

/// `getsockopt(fd, SOL_SOCKET, SO_TYPE)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockopt.html
/// [Linux]: https://man7.org/linux/man-pages/man2/getsockopt.2.html
#[inline]
pub fn getsockopt_socket_type<Fd: AsFd>(fd: &Fd) -> io::Result<SocketType> {
    let fd = fd.as_fd();
    imp::syscalls::getsockopt_socket_type(fd)
}

/// `getsockname(fd, addr, len)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html
/// [Linux]: https://man7.org/linux/man-pages/man2/getsockname.2.html
#[inline]
pub fn getsockname<Fd: AsFd>(sockfd: &Fd) -> io::Result<SocketAddr> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::getsockname(sockfd)
}

/// `getpeername(fd, addr, len)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html
/// [Linux]: https://man7.org/linux/man-pages/man2/getpeername.2.html
#[inline]
pub fn getpeername<Fd: AsFd>(sockfd: &Fd) -> io::Result<SocketAddr> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::getpeername(sockfd)
}
