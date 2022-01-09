//! `recv` and `send`, and variants

#![allow(unsafe_code)]

#[cfg(not(windows))]
use super::{Ipv4SocketAncillary, Ipv6SocketAncillary, UnixSocketAncillary};
#[cfg(not(windows))]
use crate::net::SocketAddrUnix;
use crate::net::{SocketAddrAny, SocketAddrV4, SocketAddrV6};
use crate::{imp, io};
use core::ptr;
use imp::fd::AsFd;
#[cfg(windows)]
use imp::fd::AsSocketAsFd;

pub use imp::net::{RecvFlags, SendFlags};

/// `recv(fd, buf, flags)`—Reads data from a socket.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/recv.html
/// [Linux]: https://man7.org/linux/man-pages/man2/recv.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-recv
#[inline]
pub fn recv<Fd: AsFd>(fd: &Fd, buf: &mut [u8], flags: RecvFlags) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::recv(fd, buf, flags)
}

/// `send(fd, buf, flags)`—Writes data to a socket.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/send.html
/// [Linux]: https://man7.org/linux/man-pages/man2/send.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-send
#[inline]
pub fn send<Fd: AsFd>(fd: &Fd, buf: &[u8], flags: SendFlags) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::send(fd, buf, flags)
}

/// `recvfrom(fd, buf, flags, addr, len)`—Reads data from a socket and
/// returns the sender address.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvfrom.html
/// [Linux]: https://man7.org/linux/man-pages/man2/recvfrom.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-recvfrom
#[inline]
pub fn recvfrom<Fd: AsFd>(
    fd: &Fd,
    buf: &mut [u8],
    flags: RecvFlags,
) -> io::Result<(usize, SocketAddrAny)> {
    let fd = fd.as_fd();
    imp::syscalls::recvfrom(fd, buf, flags)
}

/// `sendto(fd, buf, flags, addr, sizeof(struct sockaddr_in))`—Writes data to
/// a socket to a specific IPv4 address.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendto.html
/// [Linux]: https://man7.org/linux/man-pages/man2/sendto.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-sendto
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

/// `sendto(fd, buf, flags, addr, sizeof(struct sockaddr_in6))`—Writes data
/// to a socket to a specific IPv6 address.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendto.html
/// [Linux]: https://man7.org/linux/man-pages/man2/sendto.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-sendto
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

/// `sendto(fd, buf, flags, addr, sizeof(struct sockaddr_un))`—Writes data to
/// a socket to a specific Unix-domain socket address.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendto.html
/// [Linux]: https://man7.org/linux/man-pages/man2/sendto.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-sendto
#[inline]
#[doc(alias = "sendto")]
#[cfg(not(windows))]
pub fn sendto_unix<Fd: AsFd>(
    fd: &Fd,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrUnix,
) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::sendto_unix(fd, buf, flags, addr)
}

/// `sendmsg(fd, iovs, addr, flags)`—Writes data to a IPv4 socket.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendmsg.html
/// [Linux]: https://man7.org/linux/man-pages/man2/sendmsg.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasendto
#[doc(alias = "sendmsg")]
#[inline]
pub fn sendmsg_v4<Fd: AsFd>(
    fd: &Fd,
    iovs: &[io::IoSlice<'_>],
    addr: Option<&SocketAddrV4>,
    flags: SendFlags,
) -> io::Result<usize> {
    let fd = fd.as_fd();
    #[cfg(windows)]
    {
        imp::syscalls::sendmsg_v4(fd, iovs, addr, flags)
    }
    #[cfg(not(windows))]
    {
        imp::syscalls::sendmsg_v4(fd, iovs, addr, None, flags)
    }
}

/// `sendmsg(fd, iovs, addr, flags)`—Writes data to a IPv6 socket.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendmsg.html
/// [Linux]: https://man7.org/linux/man-pages/man2/sendmsg.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasendto
#[doc(alias = "sendmsg")]
#[inline]
pub fn sendmsg_v6<Fd: AsFd>(
    fd: &Fd,
    iovs: &[io::IoSlice<'_>],
    addr: Option<&SocketAddrV6>,
    flags: SendFlags,
) -> io::Result<usize> {
    let fd = fd.as_fd();
    #[cfg(windows)]
    {
        imp::syscalls::sendmsg_v6(fd, iovs, addr, flags)
    }
    #[cfg(not(windows))]
    {
        imp::syscalls::sendmsg_v6(fd, iovs, addr, None, flags)
    }
}

/// `sendmsg(fd, iovs, addr, flags)`—Writes data to a Unix socket.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendmsg.html
/// [Linux]: https://man7.org/linux/man-pages/man2/sendmsg.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasendto
#[doc(alias = "sendmsg")]
#[cfg(not(windows))]
#[inline]
pub fn sendmsg_unix<Fd: AsFd>(
    fd: &Fd,
    iovs: &[io::IoSlice<'_>],
    addr: Option<&SocketAddrUnix>,
    flags: SendFlags,
) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::sendmsg_unix(fd, iovs, addr, None, flags)
}

/// `sendmsg(fd, iovs, addr, flags)`—Writes data to a IPv4 socket.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendmsg.html
/// [Linux]: https://man7.org/linux/man-pages/man2/sendmsg.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasendto
#[doc(alias = "sendmsg")]
#[cfg(not(windows))]
#[inline]
pub fn sendmsg_v4_with_ancillary<Fd: AsFd>(
    fd: &Fd,
    iovs: &[io::IoSlice<'_>],
    addr: Option<&SocketAddrV4>,
    ancillary: &mut Ipv4SocketAncillary<'_>,
    flags: SendFlags,
) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::sendmsg_v4(fd, iovs, addr, Some(ancillary), flags)
}

/// `sendmsg(fd, iovs, addr, flags)`—Writes data to a IPv6 socket.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendmsg.html
/// [Linux]: https://man7.org/linux/man-pages/man2/sendmsg.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasendto
#[doc(alias = "sendmsg")]
#[cfg(not(windows))]
#[inline]
pub fn sendmsg_v6_with_ancillary<Fd: AsFd>(
    fd: &Fd,
    iovs: &[io::IoSlice<'_>],
    addr: Option<&SocketAddrV6>,
    ancillary: &mut Ipv6SocketAncillary<'_>,
    flags: SendFlags,
) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::sendmsg_v6(fd, iovs, addr, Some(ancillary), flags)
}

/// `sendmsg(fd, iovs, addr, flags)`—Writes data to a Unix socket.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendmsg.html
/// [Linux]: https://man7.org/linux/man-pages/man2/sendmsg.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasendto
#[doc(alias = "sendmsg")]
#[cfg(not(windows))]
#[inline]
pub fn sendmsg_unix_with_ancillary<Fd: AsFd>(
    fd: &Fd,
    iovs: &[io::IoSlice<'_>],
    addr: Option<&SocketAddrUnix>,
    ancillary: &mut UnixSocketAncillary<'_>,
    flags: SendFlags,
) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::sendmsg_unix(fd, iovs, addr, Some(ancillary), flags)
}

/// `recv(fd, iovs, flags)`—Reads data from a socket.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvmsg.html
/// [Linux]: https://man7.org/linux/man-pages/man2/recvmsg.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-recvfrom
#[doc(alias = "recvmsg")]
#[inline]
pub fn recvmsg_v4<Fd: AsFd>(
    fd: &Fd,
    iovs: &[io::IoSliceMut<'_>],
    flags: RecvFlags,
) -> io::Result<RecvMsgV4> {
    let fd = fd.as_fd();
    imp::syscalls::recvmsg_v4(fd, iovs, flags)
}

/// Return value from calling `recvmsg`.
#[derive(Debug)]
pub struct RecvMsgV4 {
    /// The socket addr, only set if the socket was not bound before.
    pub addr: Option<SocketAddrV4>,
    /// How many bytes have been received.
    pub bytes: usize,
}

/// `recv(fd, iovs, flags)`—Reads data from a socket.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvmsg.html
/// [Linux]: https://man7.org/linux/man-pages/man2/recvmsg.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-recvfrom
#[doc(alias = "recvmsg")]
#[inline]
pub fn recvmsg_v6<Fd: AsFd>(
    fd: &Fd,
    iovs: &[io::IoSliceMut<'_>],
    flags: RecvFlags,
) -> io::Result<RecvMsgV6> {
    let fd = fd.as_fd();
    imp::syscalls::recvmsg_v6(fd, iovs, flags)
}

/// Return value from calling `recvmsg`.
#[derive(Debug)]
pub struct RecvMsgV6 {
    /// The socket addr, only set if the socket was not bound before.
    pub addr: Option<SocketAddrV6>,
    /// How many bytes have been received.
    pub bytes: usize,
}

/// `recv(fd, iovs, flags)`—Reads data from a socket.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvmsg.html
/// [Linux]: https://man7.org/linux/man-pages/man2/recvmsg.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-recvfrom
#[doc(alias = "recvmsg")]
#[cfg(not(windows))]
#[inline]
pub fn recvmsg_unix<Fd: AsFd>(
    fd: &Fd,
    iovs: &[io::IoSliceMut<'_>],
    flags: RecvFlags,
) -> io::Result<RecvMsgUnix> {
    let fd = fd.as_fd();
    imp::syscalls::recvmsg_unix(fd, iovs, flags)
}

/// Return value from calling `recvmsg`.
#[cfg(not(windows))]
#[derive(Debug)]
pub struct RecvMsgUnix {
    /// The socket addr, only set if the socket was not bound before.
    pub addr: Option<SocketAddrUnix>,
    /// How many bytes have been received.
    pub bytes: usize,
}

// TODO: `recvmmsg`, `sendmmsg`

pub(crate) unsafe fn encode_socketaddr_v4_opt(
    addr: Option<&SocketAddrV4>,
) -> (Option<imp::c::sockaddr_in>, usize) {
    let addr = addr.map(|addr| imp::net::encode_sockaddr_v4(addr));

    let size = if addr.is_some() {
        core::mem::size_of::<imp::c::sockaddr_in>()
    } else {
        0
    };

    (addr, size)
}

pub(crate) unsafe fn encode_socketaddr_v6_opt(
    addr: Option<&SocketAddrV6>,
) -> (Option<imp::c::sockaddr_in6>, usize) {
    let addr = addr.map(|addr| imp::net::encode_sockaddr_v6(addr));

    let size = if addr.is_some() {
        core::mem::size_of::<imp::c::sockaddr_in6>()
    } else {
        0
    };

    (addr, size)
}

#[cfg(not(windows))]
pub(crate) unsafe fn encode_socketaddr_unix_opt(
    addr: Option<&SocketAddrUnix>,
) -> (Option<imp::c::sockaddr_un>, usize) {
    let addr = addr.map(|addr| imp::net::encode_sockaddr_unix(addr));

    let size = if addr.is_some() {
        core::mem::size_of::<imp::c::sockaddr_un>()
    } else {
        0
    };

    (addr, size)
}

/// Safety: pointers must all point to initialized valid memory.
pub(crate) unsafe fn encode_msghdr_v4_send(
    msg: *mut imp::c::msghdr,
    iovs: *const imp::c::iovec,
    iovlen: usize,
    msg_name: Option<*const imp::c::sockaddr_in>,
    msg_namelen: usize,
) {
    (*msg).msg_iov = iovs as *mut imp::c::iovec;
    (*msg).msg_iovlen = iovlen as _;

    (*msg).msg_name = msg_name.map(|p| p as *mut _).unwrap_or_else(ptr::null_mut);
    (*msg).msg_namelen = msg_namelen as _;
}

/// Safety: pointers must all point to initialized valid memory.
pub(crate) unsafe fn encode_msghdr_v6_send(
    msg: *mut imp::c::msghdr,
    iovs: *const imp::c::iovec,
    iovlen: usize,
    msg_name: Option<*const imp::c::sockaddr_in6>,
    msg_namelen: usize,
) {
    (*msg).msg_iov = iovs as *mut imp::c::iovec;
    (*msg).msg_iovlen = iovlen as _;

    (*msg).msg_name = msg_name.map(|p| p as *mut _).unwrap_or_else(ptr::null_mut);
    (*msg).msg_namelen = msg_namelen as _;
}

/// Safety: pointers must all point to initialized valid memory.
#[cfg(not(windows))]
pub(crate) unsafe fn encode_msghdr_unix_send(
    msg: *mut imp::c::msghdr,
    iovs: *const imp::c::iovec,
    iovlen: usize,
    msg_name: Option<*const imp::c::sockaddr_un>,
    msg_namelen: usize,
) {
    (*msg).msg_iov = iovs as *mut imp::c::iovec;
    (*msg).msg_iovlen = iovlen as _;

    (*msg).msg_name = msg_name.map(|p| p as *mut _).unwrap_or_else(ptr::null_mut);
    (*msg).msg_namelen = msg_namelen as _;
}
