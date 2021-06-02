use crate::zero_ok;
use io_lifetimes::{AsFd, BorrowedFd};
use std::{
    io,
    mem::{size_of, MaybeUninit},
};
use unsafe_io::os::posish::AsRawFd;

/// `SOCK_*` constants.
#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum SocketType {
    /// `SOCK_STREAM`.
    Stream = libc::SOCK_STREAM,

    /// `SOCK_DGRAM`.
    Datagram = libc::SOCK_DGRAM,

    /// `SOCK_SEQPACKET`.
    SeqPacket = libc::SOCK_SEQPACKET,

    /// `SOCK_RAW`.
    Raw = libc::SOCK_RAW,

    /// `SOCK_RDM`.
    Rdm = libc::SOCK_RDM,
}

/// `getsockopt(fd, SOL_SOCKET, SO_TYPE)`
#[inline]
pub fn socket_type<Fd: AsFd>(fd: &Fd) -> io::Result<SocketType> {
    let fd = fd.as_fd();
    unsafe { _socket_type(fd) }
}

unsafe fn _socket_type(fd: BorrowedFd<'_>) -> io::Result<SocketType> {
    let mut buffer = MaybeUninit::<SocketType>::uninit();
    let mut out_len = size_of::<SocketType>() as libc::socklen_t;
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
