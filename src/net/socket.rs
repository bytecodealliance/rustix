use crate::imp;
use crate::io::{self, OwnedFd};
#[cfg(not(windows))]
use crate::net::SocketAddrUnix;
use crate::net::{SocketAddrAny, SocketAddrV4, SocketAddrV6};
use imp::fd::AsFd;
#[cfg(windows)]
use imp::fd::AsSocketAsFd;

pub use imp::net::{AcceptFlags, AddressFamily, Protocol, Shutdown, SocketFlags, SocketType};

impl Default for Protocol {
    #[inline]
    fn default() -> Self {
        Self::IP
    }
}

/// `socket(domain, type_, protocol)`—Creates a socket.
///
/// POSIX guarantees that `socket` will use the lowest unused file descriptor,
/// however it is not safe in general to rely on this, as file descriptors
/// may be unexpectedly allocated on other threads or in libraries.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html
/// [Linux]: https://man7.org/linux/man-pages/man2/socket.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-socket
#[inline]
pub fn socket(domain: AddressFamily, type_: SocketType, protocol: Protocol) -> io::Result<OwnedFd> {
    imp::syscalls::socket(domain, type_, protocol)
}

/// `socket_with(domain, type_ | flags, protocol)`—Creates a socket, with
/// flags.
///
/// POSIX guarantees that `socket` will use the lowest unused file descriptor,
/// however it is not safe in general to rely on this, as file descriptors
/// may be unexpectedly allocated on other threads or in libraries.
///
/// `socket_with` is the same as [`socket`] but adds an additional flags
/// operand.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html
/// [Linux]: https://man7.org/linux/man-pages/man2/socket.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-socket
#[inline]
pub fn socket_with(
    domain: AddressFamily,
    type_: SocketType,
    flags: SocketFlags,
    protocol: Protocol,
) -> io::Result<OwnedFd> {
    imp::syscalls::socket_with(domain, type_, flags, protocol)
}

/// `bind(sockfd, addr, sizeof(struct sockaddr_in))`—Binds a socket to an
/// address.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html
/// [Linux]: https://man7.org/linux/man-pages/man2/bind.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-bind
#[inline]
#[doc(alias = "bind")]
pub fn bind_v4<Fd: AsFd>(sockfd: &Fd, addr: &SocketAddrV4) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::bind_v4(sockfd, addr)
}

/// `bind(sockfd, addr, sizeof(struct sockaddr_in6))`—Binds a socket to an
/// address.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html
/// [Linux]: https://man7.org/linux/man-pages/man2/bind.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-bind
#[inline]
#[doc(alias = "bind")]
pub fn bind_v6<Fd: AsFd>(sockfd: &Fd, addr: &SocketAddrV6) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::bind_v6(sockfd, addr)
}

/// `bind(sockfd, addr, sizeof(struct sockaddr_un))`—Binds a socket to an
/// address.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html
/// [Linux]: https://man7.org/linux/man-pages/man2/bind.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-bind
#[inline]
#[doc(alias = "bind")]
#[cfg(not(windows))]
pub fn bind_unix<Fd: AsFd>(sockfd: &Fd, addr: &SocketAddrUnix) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::bind_unix(sockfd, addr)
}

/// `connect(sockfd, addr, sizeof(struct sockaddr_in))`—Initiates a
/// connection.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html
/// [Linux]: https://man7.org/linux/man-pages/man2/connect.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect
#[inline]
#[doc(alias = "connect")]
pub fn connect_v4<Fd: AsFd>(sockfd: &Fd, addr: &SocketAddrV4) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::connect_v4(sockfd, addr)
}

/// `connect(sockfd, addr, sizeof(struct sockaddr_in6))`—Initiates a
/// connection.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html
/// [Linux]: https://man7.org/linux/man-pages/man2/connect.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect
#[inline]
#[doc(alias = "connect")]
pub fn connect_v6<Fd: AsFd>(sockfd: &Fd, addr: &SocketAddrV6) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::connect_v6(sockfd, addr)
}

/// `connect(sockfd, addr, sizeof(struct sockaddr_un))`—Initiates a
/// connection.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html
/// [Linux]: https://man7.org/linux/man-pages/man2/connect.2.html
#[inline]
#[doc(alias = "connect")]
#[cfg(not(windows))]
pub fn connect_unix<Fd: AsFd>(sockfd: &Fd, addr: &SocketAddrUnix) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::connect_unix(sockfd, addr)
}

/// `listen(fd, backlog)`—Enables listening for incoming connections.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/listen.html
/// [Linux]: https://man7.org/linux/man-pages/man2/listen.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-listen
#[inline]
pub fn listen<Fd: AsFd>(sockfd: &Fd, backlog: i32) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::listen(sockfd, backlog)
}

/// `accept(fd, NULL, NULL)`—Accepts an incoming connection.
///
/// Use [`acceptfrom`] to retrieve the peer address.
///
/// POSIX guarantees that `accept` will use the lowest unused file descriptor,
/// however it is not safe in general to rely on this, as file descriptors may
/// be unexpectedly allocated on other threads or in libraries.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/accept.html
/// [Linux]: https://man7.org/linux/man-pages/man2/accept.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-accept
#[inline]
#[doc(alias = "accept4")]
pub fn accept<Fd: AsFd>(sockfd: &Fd) -> io::Result<OwnedFd> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::accept(sockfd)
}

/// `accept4(fd, NULL, NULL, flags)`—Accepts an incoming connection, with
/// flags.
///
/// Use [`acceptfrom_with`] to retrieve the peer address.
///
/// Even though POSIX guarantees that this will use the lowest unused file
/// descriptor, it is not safe in general to rely on this, as file descriptors
/// may be unexpectedly allocated on other threads or in libraries.
///
/// `accept_with` is the same as [`accept`] but adds an additional flags
/// operand.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/accept.html
/// [Linux]: https://man7.org/linux/man-pages/man2/accept4.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-accept
#[inline]
#[doc(alias = "accept4")]
pub fn accept_with<Fd: AsFd>(sockfd: &Fd, flags: AcceptFlags) -> io::Result<OwnedFd> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::accept_with(sockfd, flags)
}

/// `accept(fd, &addr, &len)`—Accepts an incoming connection and returns the
/// peer address.
///
/// Use [`accept`] if the peer address isn't needed.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/accept.html
/// [Linux]: https://man7.org/linux/man-pages/man2/accept.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-accept
#[inline]
#[doc(alias = "accept4")]
pub fn acceptfrom<Fd: AsFd>(sockfd: &Fd) -> io::Result<(OwnedFd, SocketAddrAny)> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::acceptfrom(sockfd)
}

/// `accept4(fd, &addr, &len, flags)`—Accepts an incoming connection and
/// returns the peer address, with flags.
///
/// Use [`accept_with`] if the peer address isn't needed.
///
/// `acceptfrom_with` is the same as [`acceptfrom`] but adds an additional
/// flags operand.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/accept.html
/// [Linux]: https://man7.org/linux/man-pages/man2/accept4.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-accept
#[inline]
#[doc(alias = "accept4")]
pub fn acceptfrom_with<Fd: AsFd>(
    sockfd: &Fd,
    flags: AcceptFlags,
) -> io::Result<(OwnedFd, SocketAddrAny)> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::acceptfrom_with(sockfd, flags)
}

/// `shutdown(fd, how)`—Closes the read and/or write sides of a stream.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/shutdown.html
/// [Linux]: https://man7.org/linux/man-pages/man2/shutdown.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-shutdown
#[inline]
pub fn shutdown<Fd: AsFd>(sockfd: &Fd, how: Shutdown) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::shutdown(sockfd, how)
}

/// `getsockname(fd, addr, len)`—Returns the address a socket is bound to.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html
/// [Linux]: https://man7.org/linux/man-pages/man2/getsockname.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-getsockname
#[inline]
pub fn getsockname<Fd: AsFd>(sockfd: &Fd) -> io::Result<SocketAddrAny> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::getsockname(sockfd)
}

/// `getpeername(fd, addr, len)`—Returns the address a socket is connected
/// to.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html
/// [Linux]: https://man7.org/linux/man-pages/man2/getpeername.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-getpeername
#[inline]
pub fn getpeername<Fd: AsFd>(sockfd: &Fd) -> io::Result<SocketAddrAny> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::getpeername(sockfd)
}
