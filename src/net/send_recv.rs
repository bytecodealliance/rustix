//! `recv` and `send`, and variants

use crate::{
    imp, io,
    net::{SocketAddr, SocketAddrUnix, SocketAddrV4, SocketAddrV6},
};
use io_lifetimes::AsFd;

pub use imp::net::{RecvFlags, SendFlags};

/// `recv(fd, buf.as_ptr(), buf.len(), flags)`
#[inline]
pub fn recv<Fd: AsFd>(fd: &Fd, buf: &mut [u8], flags: RecvFlags) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::recv(fd, buf, flags)
}

/// `send(fd, buf.ptr(), buf.len(), flags)`
#[inline]
pub fn send<Fd: AsFd>(fd: &Fd, buf: &[u8], flags: SendFlags) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::send(fd, buf, flags)
}

/// `recvfrom(fd, buf, len, flags, addr, len)`
#[inline]
pub fn recvfrom<Fd: AsFd>(
    fd: &Fd,
    buf: &mut [u8],
    flags: RecvFlags,
) -> io::Result<(usize, SocketAddr)> {
    let fd = fd.as_fd();
    imp::syscalls::recvfrom(fd, buf, flags)
}

/// `sendto(fd, buf.ptr(), buf.len(), flags, addr, sizeof(struct sockaddr_in))`
#[inline]
#[doc(alias = "sendto")]
pub fn sendto_v4<Fd: AsFd>(
    fd: &Fd,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrV4,
) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::sendto_v4(fd, buf, flags, addr)
}

/// `sendto(fd, buf.ptr(), buf.len(), flags, addr, sizeof(struct sockaddr_in6))`
#[inline]
#[doc(alias = "sendto")]
pub fn sendto_v6<Fd: AsFd>(
    fd: &Fd,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrV6,
) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::sendto_v6(fd, buf, flags, addr)
}

/// `sendto(fd, buf.ptr(), buf.len(), flags, addr, sizeof(struct sockaddr_un))`
#[inline]
#[doc(alias = "sendto")]
pub fn sendto_unix<Fd: AsFd>(
    fd: &Fd,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrUnix,
) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::sendto_unix(fd, buf, flags, addr)
}

// TODO: `recvmsg`, `sendmsg`
