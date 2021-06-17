use crate::io;
use io_lifetimes::{AsFd, BorrowedFd};
use std::mem::{size_of, MaybeUninit};
#[cfg(libc)]
use {crate::zero_ok, unsafe_io::os::posish::AsRawFd};

/// `SOCK_*` constants.
#[cfg(libc)]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
#[non_exhaustive]
pub enum SocketType {
    /// `SOCK_STREAM`.
    Stream = libc::SOCK_STREAM as u32,

    /// `SOCK_DGRAM`.
    Datagram = libc::SOCK_DGRAM as u32,

    /// `SOCK_SEQPACKET`.
    SeqPacket = libc::SOCK_SEQPACKET as u32,

    /// `SOCK_RAW`.
    Raw = libc::SOCK_RAW as u32,

    /// `SOCK_RDM`.
    Rdm = libc::SOCK_RDM as u32,
}

/// `SOCK_*` constants.
#[cfg(linux_raw)]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
#[non_exhaustive]
pub enum SocketType {
    /// `SOCK_STREAM`.
    Stream = linux_raw_sys::general::SOCK_STREAM,

    /// `SOCK_DGRAM`.
    Datagram = linux_raw_sys::general::SOCK_DGRAM,

    /// `SOCK_SEQPACKET`.
    SeqPacket = linux_raw_sys::general::SOCK_SEQPACKET,

    /// `SOCK_RAW`.
    Raw = linux_raw_sys::general::SOCK_RAW,

    /// `SOCK_RDM`.
    Rdm = linux_raw_sys::general::SOCK_RDM,
}

/// `AF_*` constants.
#[cfg(libc)]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
#[non_exhaustive]
pub enum AddressFamily {
    /// `AF_LOCAL`, aka `AF_UNIX`
    Local = libc::AF_LOCAL as u32,

    /// `AF_INET`
    Inet = libc::AF_INET as u32,

    /// `AF_INET6`
    Inet6 = libc::AF_INET6 as u32,

    /// `AF_NETLINK`
    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd"
    )))]
    Netlink = libc::AF_NETLINK as u32,
}

/// `AF_*` constants.
#[cfg(linux_raw)]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
#[non_exhaustive]
pub enum AddressFamily {
    /// `AF_LOCAL`, aka `AF_UNIX`
    Local = linux_raw_sys::general::AF_LOCAL,

    /// `AF_INET`
    Inet = linux_raw_sys::general::AF_INET,

    /// `AF_INET6`
    Inet6 = linux_raw_sys::general::AF_INET6,

    /// `AF_NETLINK`
    Netlink = linux_raw_sys::general::AF_NETLINK,
}

/// `getsockopt(fd, SOL_SOCKET, SO_TYPE)`
#[inline]
pub fn socket_type<'f, Fd: AsFd<'f>>(fd: Fd) -> io::Result<SocketType> {
    let fd = fd.as_fd();
    _socket_type(fd)
}

#[cfg(libc)]
fn _socket_type(fd: BorrowedFd<'_>) -> io::Result<SocketType> {
    let mut buffer = MaybeUninit::<SocketType>::uninit();
    let mut out_len = size_of::<SocketType>() as libc::socklen_t;
    unsafe {
        zero_ok(libc::getsockopt(
            fd.as_raw_fd(),
            libc::SOL_SOCKET,
            libc::SO_TYPE,
            buffer.as_mut_ptr().cast::<libc::c_void>(),
            &mut out_len,
        ))?;
        assert_eq!(
            out_len as usize,
            size_of::<SocketType>(),
            "unexpected SocketType size"
        );
        Ok(buffer.assume_init())
    }
}

#[cfg(linux_raw)]
fn _socket_type(fd: BorrowedFd<'_>) -> io::Result<SocketType> {
    unsafe {
        let mut buffer = MaybeUninit::<SocketType>::uninit();
        let mut out_len = size_of::<SocketType>() as linux_raw_sys::general::socklen_t;
        let slice =
            std::slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, size_of::<SocketType>());
        crate::linux_raw::getsockopt(
            fd,
            linux_raw_sys::general::SOL_SOCKET as i32,
            linux_raw_sys::general::SO_TYPE as i32,
            slice,
            &mut out_len,
        )?;
        assert_eq!(
            out_len as usize,
            size_of::<SocketType>(),
            "unexpected SocketType size"
        );
        Ok(buffer.assume_init())
    }
}
