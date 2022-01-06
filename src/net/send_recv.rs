//! `recv` and `send`, and variants

#![allow(unsafe_code)]

use alloc::vec;
use alloc::vec::Vec;
use core::{
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    ptr,
};

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
    imp::syscalls::sendmsg(fd, &msg, flags)
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
    imp::syscalls::recvmsg(fd, &mut *msg, flags)
}

// TODO: where should this be located?
/// Stores the data needed for `struct msghdr`.
pub enum MsgHdr {
    /// Specialized version for IPv4 sockets.
    V4 {
        /// The raw `struct sockaddr_in`.
        socket_addr: Option<imp::c::sockaddr_in>,
        /// The list of raw `struct iovec`s.
        iovecs: Vec<imp::c::iovec>,
        /// The raw `strcut msghdr`.
        hdr: imp::c::msghdr,
    },
    /// Specialized version for IPv6 sockets.
    V6 {
        /// The raw `sockaddr_in`.
        socket_addr: Option<imp::c::sockaddr_in6>,
        /// The list of raw `struct iovec`s.
        iovecs: Vec<imp::c::iovec>,
        /// The raw `strcut msghdr`.
        hdr: imp::c::msghdr,
    },
}

impl Deref for MsgHdr {
    type Target = imp::c::msghdr;

    fn deref(&self) -> &Self::Target {
        match self {
            MsgHdr::V4 { hdr, .. } => hdr,
            MsgHdr::V6 { hdr, .. } => hdr,
        }
    }
}

impl DerefMut for MsgHdr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            MsgHdr::V4 { hdr, .. } => hdr,
            MsgHdr::V6 { hdr, .. } => hdr,
        }
    }
}

impl MsgHdr {
    /// Create a IPv4 MsgHdr from the provided slice.
    pub fn v4_from_slice_mut(buf: &mut [u8]) -> Self {
        let iovecs = vec![imp::c::iovec {
            iov_base: buf.as_mut_ptr() as *mut _,
            iov_len: buf.len() as _,
        }];

        MsgHdr::v4_from_iovecs(iovecs)
    }

    /// Create a IPv6 MsgHdr from the provided slice.
    pub fn v6_from_slice_mut(buf: &mut [u8]) -> Self {
        let iovecs = vec![imp::c::iovec {
            iov_base: buf.as_mut_ptr() as *mut _,
            iov_len: buf.len() as _,
        }];

        MsgHdr::v6_from_iovecs(iovecs)
    }

    /// Get the socket address, if available.
    pub fn socket_addr(&self) -> Option<SocketAddrAny> {
        match self {
            MsgHdr::V4 { socket_addr, .. } => socket_addr.and_then(|addr| {
                let res = unsafe {
                    SocketAddrAny::read(
                        &addr as *const _ as *mut _,
                        core::mem::size_of::<imp::c::sockaddr_in>(),
                    )
                    .ok()
                };
                res
            }),
            MsgHdr::V6 { socket_addr, .. } => socket_addr.and_then(|addr| {
                let res = unsafe {
                    SocketAddrAny::read(
                        &addr as *const _ as *mut _,
                        core::mem::size_of::<imp::c::sockaddr_in6>(),
                    )
                    .ok()
                };
                res
            }),
        }
    }

    /// Set the socket address.
    pub fn set_socket_addr(&mut self, new_addr: Option<SocketAddrAny>) {
        match self {
            MsgHdr::V4 {
                socket_addr, hdr, ..
            } => match new_addr {
                Some(v @ SocketAddrAny::V4(_)) => {
                    let storage = unsafe {
                        let mut storage = MaybeUninit::<imp::c::sockaddr_in>::uninit();
                        v.write(storage.as_mut_ptr() as *mut _);
                        storage.assume_init()
                    };
                    *socket_addr = Some(storage);
                    hdr.msg_name =
                        socket_addr.as_mut().unwrap() as *mut imp::c::sockaddr_in as *mut _;
                    hdr.msg_name = core::mem::size_of::<imp::c::sockaddr_in>() as _;
                }
                Some(_) => panic!("invalid socket address supplied"),
                None => {
                    *socket_addr = None;
                    hdr.msg_name = ptr::null_mut();
                    hdr.msg_namelen = 0;
                }
            },
            MsgHdr::V6 {
                socket_addr, hdr, ..
            } => match new_addr {
                Some(v @ SocketAddrAny::V6(_)) => {
                    let storage = unsafe {
                        let mut storage = MaybeUninit::<imp::c::sockaddr_in6>::uninit();
                        v.write(storage.as_mut_ptr() as *mut _);
                        storage.assume_init()
                    };
                    *socket_addr = Some(storage);
                    hdr.msg_name =
                        socket_addr.as_mut().unwrap() as *mut imp::c::sockaddr_in6 as *mut _;
                    hdr.msg_name = core::mem::size_of::<imp::c::sockaddr_in6>() as _;
                }
                Some(_) => panic!("invalid socket address supplied"),
                None => {
                    *socket_addr = None;
                    hdr.msg_name = ptr::null_mut();
                    hdr.msg_namelen = 0;
                }
            },
        }
    }

    /// Construct a `MsgHdr` from a list of `libc::iovec`s.
    pub fn v4_from_iovecs(iovecs: Vec<imp::c::iovec>) -> Self {
        let mut msg = MsgHdr::V4 {
            socket_addr: None,
            iovecs,
            hdr: hdr_default(),
        };
        if let MsgHdr::V4 {
            ref mut hdr,
            ref iovecs,
            ..
        } = msg
        {
            hdr.msg_iov = iovecs.as_ptr() as *mut _;
            hdr.msg_iovlen = iovecs.len() as _;
        }

        msg
    }

    /// Construct a `MsgHdr` from a list of `libc::iovec`s.
    pub fn v6_from_iovecs(iovecs: Vec<imp::c::iovec>) -> Self {
        let mut msg = MsgHdr::V6 {
            socket_addr: None,
            iovecs,
            hdr: hdr_default(),
        };
        if let MsgHdr::V6 {
            ref mut hdr,
            ref iovecs,
            ..
        } = msg
        {
            hdr.msg_iov = iovecs.as_ptr() as *mut _;
            hdr.msg_iovlen = iovecs.len() as _;
        }

        msg
    }
}

#[inline]
fn hdr_default() -> imp::c::msghdr {
    // This dance is needed because Fuchisa has hidden fields in this struct.

    let hdr = MaybeUninit::<imp::c::msghdr>::zeroed();
    // This is not actually safe yet, only after we have set all the
    // values below.
    let mut hdr = unsafe { hdr.assume_init() };
    hdr.msg_name = ptr::null_mut();
    hdr.msg_namelen = 0;
    hdr.msg_iov = ptr::null_mut();
    hdr.msg_iovlen = 0;
    hdr.msg_control = ptr::null_mut();
    hdr.msg_controllen = 0;
    hdr.msg_flags = 0;
    // now hdr is actually fully initialized
    hdr
}

// TODO: `recvmmsg`, `sendmmsg`
