//! `recv` and `send`, and variants

use std::ops::Deref;
use std::ptr;

#[cfg(not(windows))]
use crate::net::SocketAddrUnix;
use crate::net::{SocketAddrAny, SocketAddrV4, SocketAddrV6};
use crate::{imp, io};
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

/// `sendmsg(fd, msg, flags)`—Writes data to a socket.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Winsock2]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendmsg.html
/// [Linux]: https://man7.org/linux/man-pages/man2/sendmsg.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasendmsg
#[inline]
pub fn sendmsg<Fd: AsFd>(fd: &Fd, msg: &MsgHdr, flags: SendFlags) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::sendmsg(fd, &msg.hdr, flags)
}

/// `recv(fd, buf, flags)`—Reads data from a socket.
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
pub fn recvmsg<Fd: AsFd>(fd: &Fd, msg: &mut MsgHdr, flags: RecvFlags) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::recvmsg(fd, &mut msg.hdr, flags)
}

// TODO: where should this be located?
/// Wrapper around `struct msghdr`.
#[derive(Debug)]
pub struct MsgHdr {
    socket: Option<SocketAddrAny>,
    iovecs: Vec<imp::c::iovec>,
    hdr: imp::c::msghdr,
}

impl Deref for MsgHdr {
    type Target = imp::c::msghdr;

    fn deref(&self) -> &Self::Target {
        &self.hdr
    }
}

impl From<&mut [u8]> for MsgHdr {
    fn from(buf: &mut [u8]) -> Self {
        let iovecs = vec![imp::c::iovec {
            iov_base: buf.as_mut_ptr() as *mut _,
            iov_len: buf.len() as _,
        }];

        MsgHdr::from_iovecs(iovecs)
    }
}

impl MsgHdr {
    /// Construct a `MsgHdr` from a list of `libc::iovec`s.
    pub fn from_iovecs(iovecs: Vec<imp::c::iovec>) -> Self {
        let mut msg = MsgHdr {
            socket: None,
            iovecs,
            hdr: imp::c::msghdr {
                msg_name: ptr::null_mut(),
                msg_namelen: 0,
                msg_iov: ptr::null_mut(),
                msg_iovlen: 0,
                msg_control: ptr::null_mut(),
                msg_controllen: 0,
                msg_flags: 0,
            },
        };
        msg.hdr.msg_iov = msg.iovecs.as_ptr() as *mut _;
        msg.hdr.msg_iovlen = msg.iovecs.len() as _;

        msg
    }
}

// TODO: `recvmmsg`, `sendmmsg`
