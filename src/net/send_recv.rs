//! `recv` and `send`, and variants

#![allow(unsafe_code)]

#[cfg(not(windows))]
use super::{
    RecvSocketAncillaryAny, RecvSocketAncillaryUnix, RecvSocketAncillaryV4, RecvSocketAncillaryV6,
    SendSocketAncillaryUnix, SendSocketAncillaryV4, SendSocketAncillaryV6,
};
#[cfg(not(windows))]
use crate::imp::net::read_sockaddr::read_sockaddr_unix_opt;
use crate::imp::net::read_sockaddr::{
    read_sockaddr_os, read_sockaddr_v4_opt, read_sockaddr_v6_opt,
};
#[cfg(unix)]
use crate::net::SocketAddrUnix;
use crate::net::{SocketAddr, SocketAddrAny, SocketAddrV4, SocketAddrV6};
use crate::{imp, io};
#[cfg(not(windows))]
use core::ptr;
use imp::fd::{AsFd, BorrowedFd};

pub use imp::net::send_recv::{RecvFlags, SendFlags};

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
pub fn recv<Fd: AsFd>(fd: Fd, buf: &mut [u8], flags: RecvFlags) -> io::Result<usize> {
    imp::net::syscalls::recv(fd.as_fd(), buf, flags)
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
pub fn send<Fd: AsFd>(fd: Fd, buf: &[u8], flags: SendFlags) -> io::Result<usize> {
    imp::net::syscalls::send(fd.as_fd(), buf, flags)
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
    fd: Fd,
    buf: &mut [u8],
    flags: RecvFlags,
) -> io::Result<(usize, Option<SocketAddrAny>)> {
    imp::net::syscalls::recvfrom(fd.as_fd(), buf, flags)
}

/// `sendto(fd, buf, flags, addr)`—Writes data to a socket to a specific IP
/// address.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendto.html
/// [Linux]: https://man7.org/linux/man-pages/man2/sendto.2.html
pub fn sendto<Fd: AsFd>(
    fd: Fd,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddr,
) -> io::Result<usize> {
    _sendto(fd.as_fd(), buf, flags, addr)
}

fn _sendto(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddr,
) -> io::Result<usize> {
    match addr {
        SocketAddr::V4(v4) => imp::net::syscalls::sendto_v4(fd, buf, flags, v4),
        SocketAddr::V6(v6) => imp::net::syscalls::sendto_v6(fd, buf, flags, v6),
    }
}

/// `sendto(fd, buf, flags, addr)`—Writes data to a socket to a specific
/// address.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendto.html
/// [Linux]: https://man7.org/linux/man-pages/man2/sendto.2.html
pub fn sendto_any<Fd: AsFd>(
    fd: Fd,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrAny,
) -> io::Result<usize> {
    _sendto_any(fd.as_fd(), buf, flags, addr)
}

fn _sendto_any(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrAny,
) -> io::Result<usize> {
    match addr {
        SocketAddrAny::V4(v4) => imp::net::syscalls::sendto_v4(fd, buf, flags, v4),
        SocketAddrAny::V6(v6) => imp::net::syscalls::sendto_v6(fd, buf, flags, v6),
        #[cfg(unix)]
        SocketAddrAny::Unix(unix) => imp::net::syscalls::sendto_unix(fd, buf, flags, unix),
    }
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
    fd: Fd,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrV4,
) -> io::Result<usize> {
    imp::net::syscalls::sendto_v4(fd.as_fd(), buf, flags, addr)
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
    fd: Fd,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrV6,
) -> io::Result<usize> {
    imp::net::syscalls::sendto_v6(fd.as_fd(), buf, flags, addr)
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
#[cfg(unix)]
pub fn sendto_unix<Fd: AsFd>(
    fd: Fd,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrUnix,
) -> io::Result<usize> {
    imp::net::syscalls::sendto_unix(fd.as_fd(), buf, flags, addr)
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
        imp::net::syscalls::sendmsg_v4(fd, iovs, addr, flags)
    }
    #[cfg(not(windows))]
    {
        imp::net::syscalls::sendmsg_v4(fd, iovs, addr, None, flags)
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
        imp::net::syscalls::sendmsg_v6(fd, iovs, addr, flags)
    }
    #[cfg(not(windows))]
    {
        imp::net::syscalls::sendmsg_v6(fd, iovs, addr, None, flags)
    }
}

/// `sendmsg(fd, iovs, addr, flags)`—Writes data to a Unix socket.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendmsg.html
/// [Linux]: https://man7.org/linux/man-pages/man2/sendmsg.2.html
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
    imp::net::syscalls::sendmsg_unix(fd, iovs, addr, None, flags)
}

/// `sendmsg(fd, iovs, addr, flags)`—Writes data to a IPv4 socket.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendmsg.html
/// [Linux]: https://man7.org/linux/man-pages/man2/sendmsg.2.html
#[doc(alias = "sendmsg")]
#[cfg(not(windows))]
#[inline]
pub fn sendmsg_v4_with_ancillary<Fd: AsFd>(
    fd: &Fd,
    iovs: &[io::IoSlice<'_>],
    addr: Option<&SocketAddrV4>,
    ancillary: &mut SendSocketAncillaryV4<'_>,
    flags: SendFlags,
) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::net::syscalls::sendmsg_v4(fd, iovs, addr, Some(ancillary), flags)
}

/// `sendmsg(fd, iovs, addr, flags)`—Writes data to a IPv6 socket.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendmsg.html
/// [Linux]: https://man7.org/linux/man-pages/man2/sendmsg.2.html
#[doc(alias = "sendmsg")]
#[cfg(not(windows))]
#[inline]
pub fn sendmsg_v6_with_ancillary<Fd: AsFd>(
    fd: &Fd,
    iovs: &[io::IoSlice<'_>],
    addr: Option<&SocketAddrV6>,
    ancillary: &mut SendSocketAncillaryV6<'_>,
    flags: SendFlags,
) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::net::syscalls::sendmsg_v6(fd, iovs, addr, Some(ancillary), flags)
}

/// `sendmsg(fd, iovs, addr, flags)`—Writes data to a Unix socket.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendmsg.html
/// [Linux]: https://man7.org/linux/man-pages/man2/sendmsg.2.html
#[doc(alias = "sendmsg")]
#[cfg(not(windows))]
#[inline]
pub fn sendmsg_unix_with_ancillary<Fd: AsFd>(
    fd: &Fd,
    iovs: &[io::IoSlice<'_>],
    addr: Option<&SocketAddrUnix>,
    ancillary: &mut SendSocketAncillaryUnix<'_>,
    flags: SendFlags,
) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::net::syscalls::sendmsg_unix(fd, iovs, addr, Some(ancillary), flags)
}

/// `recmsg(fd, iovs, flags)`—Reads data from a socket.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvmsg.html
/// [Linux]: https://man7.org/linux/man-pages/man2/recvmsg.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-recvfrom
#[inline]
pub fn recvmsg<Fd: AsFd>(
    fd: &Fd,
    iovs: &mut [io::IoSliceMut<'_>],
    flags: RecvFlags,
) -> io::Result<RecvMsgAny> {
    let fd = fd.as_fd();
    #[cfg(windows)]
    {
        imp::net::syscalls::recvmsg(fd, iovs, flags)
    }
    #[cfg(not(windows))]
    {
        imp::net::syscalls::recvmsg(fd, iovs, None, flags)
    }
}

/// `recmsg(fd, iovs, flags)`—Reads data from a socket.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvmsg.html
/// [Linux]: https://man7.org/linux/man-pages/man2/recvmsg.2.html
#[inline]
#[cfg(not(windows))]
pub fn recvmsg_with_ancillary<Fd: AsFd>(
    fd: &Fd,
    iovs: &mut [io::IoSliceMut<'_>],
    ancillary: Option<&mut RecvSocketAncillaryAny<'_>>,
    flags: RecvFlags,
) -> io::Result<RecvMsgAny> {
    let fd = fd.as_fd();
    imp::net::syscalls::recvmsg(fd, iovs, ancillary, flags)
}

/// Return value from calling `recvmsg`.
pub struct RecvMsgAny {
    /// The socket addr, only set if the socket was not bound before.
    pub addr: Option<SocketAddrAny>,
    /// How many bytes have been received.
    pub bytes: usize,
    /// The returned flags.
    pub flags: RecvFlags,
}

impl RecvMsgAny {
    #[cfg(not(windows))]
    pub(crate) unsafe fn new(
        bytes: usize,
        msg: imp::c::msghdr,
        ancillary: Option<&mut RecvSocketAncillaryAny<'_>>,
    ) -> Self {
        let addr = if msg.msg_namelen > 0 {
            Some(read_sockaddr_os(
                msg.msg_name as *const _,
                msg.msg_namelen as _,
            ))
        } else {
            None
        };
        let flags = RecvFlags::from_bits_truncate(msg.msg_flags);

        if let Some(ancillary) = ancillary {
            ancillary.length = msg.msg_controllen as usize;
            ancillary.truncated = flags.contains(RecvFlags::CTRUNC);
        }

        RecvMsgAny {
            bytes: bytes as usize,
            addr,
            flags,
        }
    }

    #[cfg(windows)]
    pub(crate) unsafe fn new(
        bytes: usize,
        name: *const imp::c::sockaddr,
        namelen: usize,
        flags: imp::c::c_ulong,
    ) -> Self {
        let addr = if namelen > 0 {
            Some(read_sockaddr_os(name as *const _, namelen as _))
        } else {
            None
        };
        let flags = RecvFlags::from_bits_truncate(flags as _);

        RecvMsgAny {
            bytes: bytes as usize,
            addr,
            flags,
        }
    }
}

/// `recmsg(fd, iovs, flags)`—Reads data from a socket.
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
    iovs: &mut [io::IoSliceMut<'_>],
    flags: RecvFlags,
) -> io::Result<RecvMsgV4> {
    let fd = fd.as_fd();
    #[cfg(windows)]
    {
        imp::net::syscalls::recvmsg_v4(fd, iovs, flags)
    }
    #[cfg(not(windows))]
    {
        imp::net::syscalls::recvmsg_v4(fd, iovs, None, flags)
    }
}

/// `recmsg(fd, iovs, flags)`—Reads data from a socket.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvmsg.html
/// [Linux]: https://man7.org/linux/man-pages/man2/recvmsg.2.html
#[doc(alias = "recvmsg")]
#[cfg(not(windows))]
#[inline]
pub fn recvmsg_v4_with_ancillary<Fd: AsFd>(
    fd: &Fd,
    iovs: &mut [io::IoSliceMut<'_>],
    ancillary: &mut RecvSocketAncillaryV4<'_>,
    flags: RecvFlags,
) -> io::Result<RecvMsgV4> {
    let fd = fd.as_fd();
    imp::net::syscalls::recvmsg_v4(fd, iovs, Some(ancillary), flags)
}

/// Return value from calling `recvmsg`.
pub struct RecvMsgV4 {
    /// The socket addr, only set if the socket was not bound before.
    pub addr: Option<SocketAddrV4>,
    /// How many bytes have been received.
    pub bytes: usize,
    /// The returned flags.
    pub flags: RecvFlags,
}

impl RecvMsgV4 {
    /// Safety: `msg` must be a valid return value from an Ipv4 based `recvmsg`
    /// call.
    #[cfg(not(windows))]
    pub(crate) unsafe fn new(
        bytes: usize,
        msg: imp::c::msghdr,
        ancillary: Option<&mut RecvSocketAncillaryV4<'_>>,
    ) -> Self {
        let addr = read_sockaddr_v4_opt(msg.msg_name as *const _, msg.msg_namelen as _);
        let flags = RecvFlags::from_bits_truncate(msg.msg_flags);

        if let Some(ancillary) = ancillary {
            ancillary.length = msg.msg_controllen as usize;
            ancillary.truncated = flags.contains(RecvFlags::CTRUNC);
        }

        RecvMsgV4 {
            bytes: bytes as usize,
            addr,
            flags,
        }
    }

    /// Safety: `msg` must be a valid return value from an Ipv4 based `recvmsg`
    /// call.
    #[cfg(windows)]
    pub(crate) unsafe fn new(
        bytes: usize,
        name: *const imp::c::sockaddr,
        namelen: usize,
        flags: imp::c::c_ulong,
    ) -> Self {
        let addr = read_sockaddr_v4_opt(name as *const _, namelen as _);
        let flags = RecvFlags::from_bits_truncate(flags as _);

        RecvMsgV4 {
            bytes: bytes as usize,
            addr,
            flags,
        }
    }
}

/// `recvmsg(fd, iovs, flags)`—Reads data from a socket.
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
    iovs: &mut [io::IoSliceMut<'_>],
    flags: RecvFlags,
) -> io::Result<RecvMsgV6> {
    let fd = fd.as_fd();
    #[cfg(windows)]
    {
        imp::net::syscalls::recvmsg_v6(fd, iovs, flags)
    }
    #[cfg(not(windows))]
    {
        imp::net::syscalls::recvmsg_v6(fd, iovs, None, flags)
    }
}

/// `recvmsg(fd, iovs, flags)`—Reads data from a socket.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvmsg.html
/// [Linux]: https://man7.org/linux/man-pages/man2/recvmsg.2.html
#[doc(alias = "recvmsg")]
#[cfg(not(windows))]
#[inline]
pub fn recvmsg_v6_with_ancillary<Fd: AsFd>(
    fd: &Fd,
    iovs: &mut [io::IoSliceMut<'_>],
    ancillary: &mut RecvSocketAncillaryV6<'_>,
    flags: RecvFlags,
) -> io::Result<RecvMsgV6> {
    let fd = fd.as_fd();
    imp::net::syscalls::recvmsg_v6(fd, iovs, Some(ancillary), flags)
}

/// Return value from calling `recvmsg`.
pub struct RecvMsgV6 {
    /// The socket addr, only set if the socket was not bound before.
    pub addr: Option<SocketAddrV6>,
    /// How many bytes have been received.
    pub bytes: usize,
    /// The returned flags.
    pub flags: RecvFlags,
}

impl RecvMsgV6 {
    /// Safety: `msg` must be a valid return value from an Ipv6 based `recvmsg`
    /// call.
    #[cfg(not(windows))]
    pub(crate) unsafe fn new(
        bytes: usize,
        msg: imp::c::msghdr,
        ancillary: Option<&mut RecvSocketAncillaryV6<'_>>,
    ) -> Self {
        let addr = read_sockaddr_v6_opt(msg.msg_name as *const _, msg.msg_namelen as _);
        let flags = RecvFlags::from_bits_truncate(msg.msg_flags);

        if let Some(ancillary) = ancillary {
            ancillary.length = msg.msg_controllen as usize;
            ancillary.truncated = flags.contains(RecvFlags::CTRUNC);
        }

        RecvMsgV6 {
            bytes: bytes as usize,
            addr,
            flags,
        }
    }

    /// Safety: `msg` must be a valid return value from an Ipv6 based `recvmsg`
    /// call.
    #[cfg(windows)]
    pub(crate) unsafe fn new(
        bytes: usize,
        name: *const imp::c::sockaddr,
        namelen: usize,
        flags: imp::c::c_ulong,
    ) -> Self {
        let addr = read_sockaddr_v6_opt(name as *const _, namelen as _);
        let flags = RecvFlags::from_bits_truncate(flags as _);

        RecvMsgV6 {
            bytes: bytes as usize,
            addr,
            flags,
        }
    }
}

/// `recvmsg(fd, iovs, flags)`—Reads data from a socket.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvmsg.html
/// [Linux]: https://man7.org/linux/man-pages/man2/recvmsg.2.html
#[doc(alias = "recvmsg")]
#[cfg(not(windows))]
#[inline]
pub fn recvmsg_unix<Fd: AsFd>(
    fd: &Fd,
    iovs: &mut [io::IoSliceMut<'_>],
    flags: RecvFlags,
) -> io::Result<RecvMsgUnix> {
    let fd = fd.as_fd();
    imp::net::syscalls::recvmsg_unix(fd, iovs, None, flags)
}

/// `recv(fd, iovs, flags)`—Reads data from a socket.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvmsg.html
/// [Linux]: https://man7.org/linux/man-pages/man2/recvmsg.2.html
#[doc(alias = "recvmsg")]
#[cfg(not(windows))]
#[inline]
pub fn recvmsg_unix_with_ancillary<Fd: AsFd>(
    fd: &Fd,
    iovs: &mut [io::IoSliceMut<'_>],
    ancillary: &mut RecvSocketAncillaryUnix<'_>,
    flags: RecvFlags,
) -> io::Result<RecvMsgUnix> {
    let fd = fd.as_fd();
    imp::net::syscalls::recvmsg_unix(fd, iovs, Some(ancillary), flags)
}

/// Return value from calling `recvmsg`.
#[cfg(not(windows))]
pub struct RecvMsgUnix {
    /// The socket addr, only set if the socket was not bound before.
    pub addr: Option<SocketAddrUnix>,
    /// How many bytes have been received.
    pub bytes: usize,
    /// The returned flags.
    pub flags: RecvFlags,
}

#[cfg(not(windows))]
impl RecvMsgUnix {
    /// Safety: `msg` must be a valid return value from an Unix based `recvmsg`
    /// call.
    pub(crate) unsafe fn new(
        bytes: usize,
        msg: imp::c::msghdr,
        ancillary: Option<&mut RecvSocketAncillaryUnix<'_>>,
    ) -> Self {
        let addr = read_sockaddr_unix_opt(msg.msg_name as *const _, msg.msg_namelen as _);
        let flags = RecvFlags::from_bits_truncate(msg.msg_flags);

        if let Some(ancillary) = ancillary {
            ancillary.length = msg.msg_controllen as usize;
            ancillary.truncated = flags.contains(RecvFlags::CTRUNC);
        }

        RecvMsgUnix {
            bytes: bytes as usize,
            addr,
            flags,
        }
    }
}

// TODO: `recvmmsg`, `sendmmsg`

pub(crate) unsafe fn encode_socketaddr_v4_opt(
    addr: Option<&SocketAddrV4>,
) -> (Option<imp::c::sockaddr_in>, usize) {
    let addr = addr.map(|addr| imp::net::write_sockaddr::encode_sockaddr_v4(addr));

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
    let addr = addr.map(|addr| imp::net::write_sockaddr::encode_sockaddr_v6(addr));

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
    let addr = addr.map(|addr| addr.unix);

    let size = if addr.is_some() {
        core::mem::size_of::<imp::c::sockaddr_un>()
    } else {
        0
    };

    (addr, size)
}

/// Safety: pointers must all point to initialized valid memory.
#[cfg(not(windows))]
pub(crate) unsafe fn encode_msghdr_v4_send(
    msg: *mut imp::c::msghdr,
    iovs: *const imp::c::iovec,
    iovlen: usize,
    msg_name: Option<*const imp::c::sockaddr_in>,
    msg_namelen: usize,
    ancillary: Option<&mut SendSocketAncillaryV4<'_>>,
) {
    (*msg).msg_iov = iovs as *mut imp::c::iovec;
    (*msg).msg_iovlen = iovlen as _;

    (*msg).msg_name = msg_name.map(|p| p as *mut _).unwrap_or_else(ptr::null_mut);
    (*msg).msg_namelen = msg_namelen as _;

    if let Some(ancillary) = ancillary {
        (*msg).msg_controllen = ancillary.length as _;
        // macos requires that the control pointer is null when the len is 0.
        if (*msg).msg_controllen > 0 {
            (*msg).msg_control = ancillary.buffer_mut_ptr().cast();
        }
        ancillary.truncated = false;
    }
}

/// Safety: pointers must all point to initialized valid memory.
#[cfg(not(windows))]
pub(crate) unsafe fn encode_msghdr_v6_send(
    msg: *mut imp::c::msghdr,
    iovs: *const imp::c::iovec,
    iovlen: usize,
    msg_name: Option<*const imp::c::sockaddr_in6>,
    msg_namelen: usize,
    ancillary: Option<&mut SendSocketAncillaryV6<'_>>,
) {
    (*msg).msg_iov = iovs as *mut imp::c::iovec;
    (*msg).msg_iovlen = iovlen as _;

    (*msg).msg_name = msg_name.map(|p| p as *mut _).unwrap_or_else(ptr::null_mut);
    (*msg).msg_namelen = msg_namelen as _;

    if let Some(ancillary) = ancillary {
        (*msg).msg_controllen = ancillary.length as _;
        // macos requires that the control pointer is null when the len is 0.
        if (*msg).msg_controllen > 0 {
            (*msg).msg_control = ancillary.buffer_mut_ptr().cast();
        }
        ancillary.truncated = false;
    }
}

/// Safety: pointers must all point to initialized valid memory.
#[cfg(not(windows))]
pub(crate) unsafe fn encode_msghdr_unix_send(
    msg: *mut imp::c::msghdr,
    iovs: *const imp::c::iovec,
    iovlen: usize,
    msg_name: Option<*const imp::c::sockaddr_un>,
    msg_namelen: usize,
    ancillary: Option<&mut SendSocketAncillaryUnix<'_>>,
) {
    (*msg).msg_iov = iovs as *mut imp::c::iovec;
    (*msg).msg_iovlen = iovlen as _;

    (*msg).msg_name = msg_name.map(|p| p as *mut _).unwrap_or_else(ptr::null_mut);
    (*msg).msg_namelen = msg_namelen as _;

    if let Some(ancillary) = ancillary {
        (*msg).msg_controllen = ancillary.length as _;
        // macos requires that the control pointer is null when the len is 0.
        if (*msg).msg_controllen > 0 {
            (*msg).msg_control = ancillary.buffer_mut_ptr().cast();
        }
        ancillary.truncated = false;
    }
}

#[cfg(not(windows))]
pub(crate) fn encode_msghdr_v4_recv(
    msg: &mut imp::c::msghdr,
    iovs: &mut [io::IoSliceMut<'_>],
    msg_name: *mut imp::c::sockaddr_in,
    ancillary: &mut Option<&mut RecvSocketAncillaryV4<'_>>,
) {
    msg.msg_iov = iovs.as_mut_ptr().cast();
    msg.msg_iovlen = iovs.len() as _;

    msg.msg_name = msg_name.cast();
    msg.msg_namelen = core::mem::size_of::<imp::c::sockaddr_in>() as _;

    if let Some(ancillary) = ancillary {
        msg.msg_controllen = ancillary.buffer_len() as _;
        // macos requires that the control pointer is null when the len is 0.
        if msg.msg_controllen > 0 {
            msg.msg_control = unsafe { ancillary.buffer_mut_ptr().cast() };
        }
    }
}

#[cfg(not(windows))]
pub(crate) fn encode_msghdr_v6_recv(
    msg: &mut imp::c::msghdr,
    iovs: &mut [io::IoSliceMut<'_>],
    msg_name: *mut imp::c::sockaddr_in6,
    ancillary: &mut Option<&mut RecvSocketAncillaryV6<'_>>,
) {
    msg.msg_iov = iovs.as_mut_ptr().cast();
    msg.msg_iovlen = iovs.len() as _;

    msg.msg_name = msg_name.cast();
    msg.msg_namelen = core::mem::size_of::<imp::c::sockaddr_in6>() as _;

    if let Some(ancillary) = ancillary {
        msg.msg_controllen = ancillary.buffer_len() as _;
        // macos requires that the control pointer is null when the len is 0.
        if msg.msg_controllen > 0 {
            msg.msg_control = unsafe { ancillary.buffer_mut_ptr().cast() };
        }
    }
}

#[cfg(not(windows))]
pub(crate) fn encode_msghdr_unix_recv(
    msg: &mut imp::c::msghdr,
    iovs: &mut [io::IoSliceMut<'_>],
    msg_name: *mut imp::c::sockaddr_un,
    ancillary: &mut Option<&mut RecvSocketAncillaryUnix<'_>>,
) {
    msg.msg_iov = iovs.as_mut_ptr().cast();
    msg.msg_iovlen = iovs.len() as _;

    msg.msg_name = msg_name.cast();
    msg.msg_namelen = core::mem::size_of::<imp::c::sockaddr_un>() as _;

    if let Some(ancillary) = ancillary {
        msg.msg_controllen = ancillary.buffer_len() as _;
        // macos requires that the control pointer is null when the len is 0.
        if msg.msg_controllen > 0 {
            msg.msg_control = unsafe { ancillary.buffer_mut_ptr().cast() };
        }
    }
}

#[cfg(not(windows))]
pub(crate) fn encode_msghdr_any_recv(
    msg: &mut imp::c::msghdr,
    iovs: &mut [io::IoSliceMut<'_>],
    msg_name: *mut imp::c::sockaddr,
    ancillary: &mut Option<&mut RecvSocketAncillaryAny<'_>>,
) {
    msg.msg_iov = iovs.as_mut_ptr().cast();
    msg.msg_iovlen = iovs.len() as _;

    msg.msg_name = msg_name.cast();
    msg.msg_namelen = core::mem::size_of::<imp::c::sockaddr>() as _;

    if let Some(ancillary) = ancillary {
        msg.msg_controllen = ancillary.buffer_len() as _;
        // macos requires that the control pointer is null when the len is 0.
        if msg.msg_controllen > 0 {
            msg.msg_control = unsafe { ancillary.buffer_mut_ptr().cast() };
        }
    }
}
