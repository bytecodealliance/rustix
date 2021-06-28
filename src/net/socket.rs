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
#[inline]
pub fn socket(domain: AddressFamily, type_: SocketType, protocol: Protocol) -> io::Result<OwnedFd> {
    imp::syscalls::socket(domain, type_, protocol)
}

/// `bind(sockfd, addr, sizeof(struct sockaddr_in))`
#[inline]
#[doc(alias("bind"))]
pub fn bind_v4<Fd: AsFd>(sockfd: &Fd, addr: &SocketAddrV4) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::bind_v4(sockfd, addr)
}

/// `bind(sockfd, addr, sizeof(struct sockaddr_in6))`
#[inline]
#[doc(alias("bind"))]
pub fn bind_v6<Fd: AsFd>(sockfd: &Fd, addr: &SocketAddrV6) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::bind_v6(sockfd, addr)
}

/// `bind(sockfd, addr, sizeof(struct sockaddr_un))`
#[inline]
#[doc(alias("bind"))]
pub fn bind_unix<Fd: AsFd>(sockfd: &Fd, addr: &SocketAddrUnix) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::bind_unix(sockfd, addr)
}

/// `connect(sockfd, addr, sizeof(struct sockaddr_in))`
#[inline]
#[doc(alias("connect"))]
pub fn connect_v4<Fd: AsFd>(sockfd: &Fd, addr: &SocketAddrV4) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::connect_v4(sockfd, addr)
}

/// `connect(sockfd, addr, sizeof(struct sockaddr_in6))`
#[inline]
#[doc(alias("connect"))]
pub fn connect_v6<Fd: AsFd>(sockfd: &Fd, addr: &SocketAddrV6) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::connect_v6(sockfd, addr)
}

/// `connect(sockfd, addr, sizeof(struct sockaddr_un))`
#[doc(alias("connect"))]
#[inline]
pub fn connect_unix<Fd: AsFd>(sockfd: &Fd, addr: &SocketAddrUnix) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::connect_unix(sockfd, addr)
}

/// `listen(fd, backlog)`
#[inline]
pub fn listen<Fd: AsFd>(sockfd: &Fd, backlog: c_int) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::listen(sockfd, backlog)
}

/// `accept4(fd, addr, len, flags)`
#[inline]
#[doc(alias = "accept4")]
pub fn accept<Fd: AsFd>(sockfd: &Fd, flags: AcceptFlags) -> io::Result<(OwnedFd, SocketAddr)> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::accept(sockfd, flags)
}

/// `shutdown(fd, how)`
#[inline]
pub fn shutdown<Fd: AsFd>(sockfd: &Fd, how: Shutdown) -> io::Result<()> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::shutdown(sockfd, how)
}

/// `getsockopt(fd, SOL_SOCKET, SO_TYPE)`
#[inline]
pub fn getsockopt_socket_type<Fd: AsFd>(fd: &Fd) -> io::Result<SocketType> {
    let fd = fd.as_fd();
    imp::syscalls::getsockopt_socket_type(fd)
}

/// `getsockname(fd, addr, len)`
#[inline]
pub fn getsockname<Fd: AsFd>(sockfd: &Fd) -> io::Result<SocketAddr> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::getsockname(sockfd)
}

/// `getpeername(fd, addr, len)`
#[inline]
pub fn getpeername<Fd: AsFd>(sockfd: &Fd) -> io::Result<SocketAddr> {
    let sockfd = sockfd.as_fd();
    imp::syscalls::getpeername(sockfd)
}
