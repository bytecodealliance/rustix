//! `recv` and `send`, and variants

use crate::{
    io,
    net::{SocketAddr, SocketAddrUnix, SocketAddrV4, SocketAddrV6},
};
use bitflags::bitflags;
use io_lifetimes::{AsFd, BorrowedFd};
#[cfg(libc)]
use {
    super::sockaddr_header::decode_sockaddr,
    crate::libc::conv::borrowed_fd,
    crate::{as_ptr, negone_err},
    libc::{sockaddr_storage, socklen_t},
    std::mem::{size_of, MaybeUninit},
};

#[cfg(libc)]
bitflags! {
    /// `MSG_*`
    pub struct SendFlags: i32 {
        /// `MSG_CONFIRM`
        #[cfg(not(any(target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "netbsd", target_os = "openbsd")))]
        const CONFIRM = libc::MSG_CONFIRM;
        /// `MSG_DONTROUTE`
        const DONTROUTE = libc::MSG_DONTROUTE;
        /// `MSG_DONTWAIT`
        const DONTWAIT = libc::MSG_DONTWAIT;
        /// `MSG_EOR`
        const EOT = libc::MSG_EOR;
        /// `MSG_MORE`
        #[cfg(not(any(target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "netbsd", target_os = "openbsd")))]
        const MORE = libc::MSG_MORE;
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        /// `MSG_NOSIGNAL`
        const NOSIGNAL = libc::MSG_NOSIGNAL;
        /// `MSG_OOB`
        const OOB = libc::MSG_OOB;
    }
}

#[cfg(linux_raw)]
bitflags! {
    /// `MSG_*`
    pub struct SendFlags: u32 {
        /// `MSG_CONFIRM`
        const CONFIRM = linux_raw_sys::general::MSG_CONFIRM;
        /// `MSG_DONTROUTE`
        const DONTROUTE = linux_raw_sys::general::MSG_DONTROUTE;
        /// `MSG_DONTWAIT`
        const DONTWAIT = linux_raw_sys::general::MSG_DONTWAIT;
        /// `MSG_EOT`
        const EOT = linux_raw_sys::general::MSG_EOR;
        /// `MSG_MORE`
        const MORE = linux_raw_sys::general::MSG_MORE;
        /// `MSG_NOSIGNAL`
        const NOSIGNAL = linux_raw_sys::general::MSG_NOSIGNAL;
        /// `MSG_OOB`
        const OOB = linux_raw_sys::general::MSG_OOB;
    }
}

#[cfg(libc)]
bitflags! {
    /// `MSG_*`
    pub struct RecvFlags: i32 {
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        /// `MSG_CMSG_CLOEXEC`
        const CMSG_CLOEXEC = libc::MSG_CMSG_CLOEXEC;
        /// `MSG_DONTWAIT`
        const DONTWAIT = libc::MSG_DONTWAIT;
        /// `MSG_ERRQUEUE`
        #[cfg(not(any(target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "netbsd", target_os = "openbsd")))]
        const ERRQUEUE = libc::MSG_ERRQUEUE;
        /// `MSG_OOB`
        const OOB = libc::MSG_OOB;
        /// `MSG_PEEK`
        const PEEK = libc::MSG_PEEK;
        /// `MSG_TRUNC`
        const TRUNC = libc::MSG_TRUNC;
        /// `MSG_WAITALL`
        const WAITALL = libc::MSG_WAITALL;
    }
}

#[cfg(linux_raw)]
bitflags! {
    /// `MSG_*`
    pub struct RecvFlags: u32 {
        /// `MSG_CMSG_CLOEXEC`
        const CMSG_CLOEXEC = linux_raw_sys::general::MSG_CMSG_CLOEXEC;
        /// `MSG_DONTWAIT`
        const DONTWAIT = linux_raw_sys::general::MSG_DONTWAIT;
        /// `MSG_ERRQUEUE`
        const ERRQUEUE = linux_raw_sys::general::MSG_ERRQUEUE;
        /// `MSG_OOB`
        const OOB = linux_raw_sys::general::MSG_OOB;
        /// `MSG_PEEK`
        const PEEK = linux_raw_sys::general::MSG_PEEK;
        /// `MSG_TRUNC`
        const TRUNC = linux_raw_sys::general::MSG_TRUNC;
        /// `MSG_WAITALL`
        const WAITALL = linux_raw_sys::general::MSG_WAITALL;
    }
}

/// `recv(fd, buf.as_ptr(), buf.len(), flags)`
#[inline]
pub fn recv<Fd: AsFd>(fd: &Fd, buf: &mut [u8], flags: RecvFlags) -> io::Result<usize> {
    let fd = fd.as_fd();
    _recv(fd, buf, flags)
}

#[cfg(libc)]
fn _recv(fd: BorrowedFd<'_>, buf: &mut [u8], flags: RecvFlags) -> io::Result<usize> {
    let nrecv = unsafe {
        negone_err(libc::recv(
            borrowed_fd(fd),
            buf.as_mut_ptr().cast::<_>(),
            buf.len(),
            flags.bits(),
        ))?
    };
    Ok(nrecv as usize)
}

#[cfg(linux_raw)]
#[inline]
fn _recv(fd: BorrowedFd<'_>, buf: &mut [u8], flags: RecvFlags) -> io::Result<usize> {
    crate::linux_raw::recv(fd, buf, flags.bits())
}

/// `send(fd, buf.ptr(), buf.len(), flags)`
#[inline]
pub fn send<Fd: AsFd>(fd: &Fd, buf: &[u8], flags: SendFlags) -> io::Result<usize> {
    let fd = fd.as_fd();
    _send(fd, buf, flags)
}

#[cfg(libc)]
fn _send(fd: BorrowedFd<'_>, buf: &[u8], flags: SendFlags) -> io::Result<usize> {
    let nwritten = unsafe {
        negone_err(libc::send(
            borrowed_fd(fd),
            buf.as_ptr().cast::<_>(),
            buf.len(),
            flags.bits(),
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(linux_raw)]
#[inline]
fn _send(fd: BorrowedFd<'_>, buf: &[u8], flags: SendFlags) -> io::Result<usize> {
    crate::linux_raw::send(fd, buf, flags.bits())
}

/// `recvfrom(fd, buf, len, flags, addr, len)`
#[inline]
pub fn recvfrom<Fd: AsFd>(
    fd: &Fd,
    buf: &mut [u8],
    flags: RecvFlags,
) -> io::Result<(usize, SocketAddr)> {
    let fd = fd.as_fd();
    _recvfrom(fd, buf, flags)
}

#[cfg(libc)]
fn _recvfrom(
    fd: BorrowedFd<'_>,
    buf: &mut [u8],
    flags: RecvFlags,
) -> io::Result<(usize, SocketAddr)> {
    unsafe {
        let mut storage = MaybeUninit::<sockaddr_storage>::uninit();
        let mut len = size_of::<sockaddr_storage>() as socklen_t;
        let nread = negone_err(libc::recvfrom(
            borrowed_fd(fd),
            buf.as_mut_ptr().cast::<_>(),
            buf.len(),
            flags.bits(),
            storage.as_mut_ptr().cast::<_>(),
            &mut len,
        ))?;
        Ok((nread as usize, decode_sockaddr(storage.as_ptr(), len)))
    }
}

#[cfg(linux_raw)]
#[inline]
fn _recvfrom(
    fd: BorrowedFd<'_>,
    buf: &mut [u8],
    flags: RecvFlags,
) -> io::Result<(usize, SocketAddr)> {
    crate::linux_raw::recvfrom(fd, buf, flags.bits())
}

/// `sendto(fd, buf.ptr(), buf.len(), flags, addr, sizeof(struct sockaddr_in))`
#[inline]
#[doc(alias("sendto"))]
pub fn sendto_v4<Fd: AsFd>(
    fd: &Fd,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrV4,
) -> io::Result<usize> {
    let fd = fd.as_fd();
    _sendto_v4(fd, buf, flags, addr)
}

#[cfg(libc)]
fn _sendto_v4(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrV4,
) -> io::Result<usize> {
    let nwritten = unsafe {
        negone_err(libc::sendto(
            borrowed_fd(fd),
            buf.as_ptr().cast::<_>(),
            buf.len(),
            flags.bits(),
            as_ptr(&addr.encode()).cast::<libc::sockaddr>(),
            size_of::<SocketAddrV4>() as u32,
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(linux_raw)]
#[inline]
fn _sendto_v4(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrV4,
) -> io::Result<usize> {
    crate::linux_raw::sendto_in(fd, buf, flags.bits(), addr)
}

/// `sendto(fd, buf.ptr(), buf.len(), flags, addr, sizeof(struct sockaddr_in6))`
#[inline]
#[doc(alias("sendto"))]
pub fn sendto_v6<Fd: AsFd>(
    fd: &Fd,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrV6,
) -> io::Result<usize> {
    let fd = fd.as_fd();
    _sendto_v6(fd, buf, flags, addr)
}

#[cfg(libc)]
fn _sendto_v6(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrV6,
) -> io::Result<usize> {
    let nwritten = unsafe {
        negone_err(libc::sendto(
            borrowed_fd(fd),
            buf.as_ptr().cast::<_>(),
            buf.len(),
            flags.bits(),
            as_ptr(&addr.encode()).cast::<libc::sockaddr>(),
            size_of::<SocketAddrV6>() as u32,
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(linux_raw)]
#[inline]
fn _sendto_v6(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrV6,
) -> io::Result<usize> {
    crate::linux_raw::sendto_in6(fd, buf, flags.bits(), addr)
}

/// `sendto(fd, buf.ptr(), buf.len(), flags, addr, sizeof(struct sockaddr_un))`
#[inline]
#[doc(alias("sendto"))]
pub fn sendto_unix<Fd: AsFd>(
    fd: &Fd,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrUnix,
) -> io::Result<usize> {
    let fd = fd.as_fd();
    _sendto_unix(fd, buf, flags, addr)
}

#[cfg(libc)]
fn _sendto_unix(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrUnix,
) -> io::Result<usize> {
    let nwritten = unsafe {
        negone_err(libc::sendto(
            borrowed_fd(fd),
            buf.as_ptr().cast::<_>(),
            buf.len(),
            flags.bits(),
            as_ptr(&addr.encode()).cast::<libc::sockaddr>(),
            size_of::<SocketAddrUnix>() as u32,
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(linux_raw)]
#[inline]
fn _sendto_unix(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrUnix,
) -> io::Result<usize> {
    crate::linux_raw::sendto_un(fd, buf, flags.bits(), addr)
}

// TODO: `recvmsg`, `sendmsg`
