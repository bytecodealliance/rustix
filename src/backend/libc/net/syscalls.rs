//! libc syscalls supporting `rustix::net`.

use crate::backend::c;
use crate::backend::conv::{borrowed_fd, ret, ret_owned_fd, ret_send_recv, send_recv_len};
use crate::fd::{BorrowedFd, OwnedFd};
use crate::io;
use crate::net::{SockAddr, SocketAddrAny};
use crate::utils::as_ptr;
use core::mem::{size_of, MaybeUninit};
#[cfg(not(any(
    windows,
    target_os = "espidf",
    target_os = "redox",
    target_os = "vita",
    target_os = "wasi"
)))]
use {
    super::msghdr::{with_msghdr, with_noaddr_msghdr, with_recv_msghdr},
    crate::io::{IoSlice, IoSliceMut},
    crate::net::{RecvAncillaryBuffer, RecvMsgReturn, SendAncillaryBuffer},
};
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
use {
    super::read_sockaddr::{initialize_family_to_unspec, maybe_read_sockaddr_os, read_sockaddr_os},
    super::send_recv::{RecvFlags, SendFlags},
    crate::net::{AddressFamily, Protocol, Shutdown, SocketFlags, SocketType},
    core::ptr::null_mut,
};

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) unsafe fn recv(
    fd: BorrowedFd<'_>,
    buf: *mut u8,
    len: usize,
    flags: RecvFlags,
) -> io::Result<usize> {
    ret_send_recv(c::recv(
        borrowed_fd(fd),
        buf.cast(),
        send_recv_len(len),
        bitflags_bits!(flags),
    ))
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn send(fd: BorrowedFd<'_>, buf: &[u8], flags: SendFlags) -> io::Result<usize> {
    unsafe {
        ret_send_recv(c::send(
            borrowed_fd(fd),
            buf.as_ptr().cast(),
            send_recv_len(buf.len()),
            bitflags_bits!(flags),
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) unsafe fn recvfrom(
    fd: BorrowedFd<'_>,
    buf: *mut u8,
    buf_len: usize,
    flags: RecvFlags,
) -> io::Result<(usize, Option<SocketAddrAny>)> {
    let mut storage = MaybeUninit::<c::sockaddr_storage>::uninit();
    let mut len = size_of::<c::sockaddr_storage>() as c::socklen_t;

    // `recvfrom` does not write to the storage if the socket is
    // connection-oriented sockets, so we initialize the family field to
    // `AF_UNSPEC` so that we can detect this case.
    initialize_family_to_unspec(storage.as_mut_ptr());

    ret_send_recv(c::recvfrom(
        borrowed_fd(fd),
        buf.cast(),
        send_recv_len(buf_len),
        bitflags_bits!(flags),
        storage.as_mut_ptr().cast(),
        &mut len,
    ))
    .map(|nread| {
        (
            nread,
            maybe_read_sockaddr_os(storage.as_ptr(), len.try_into().unwrap()),
        )
    })
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn sendto(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &impl SockAddr,
) -> io::Result<usize> {
    unsafe {
        addr.with_sockaddr(|addr_ptr, addr_len| {
            ret_send_recv(c::sendto(
                borrowed_fd(fd),
                buf.as_ptr().cast(),
                send_recv_len(buf.len()),
                bitflags_bits!(flags),
                addr_ptr.cast(),
                addr_len as c::socklen_t,
            ))
        })
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn socket(
    domain: AddressFamily,
    type_: SocketType,
    protocol: Option<Protocol>,
) -> io::Result<OwnedFd> {
    let raw_protocol = match protocol {
        Some(p) => p.0.get(),
        None => 0,
    };
    unsafe {
        ret_owned_fd(c::socket(
            domain.0 as c::c_int,
            type_.0 as c::c_int,
            raw_protocol as c::c_int,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn socket_with(
    domain: AddressFamily,
    type_: SocketType,
    flags: SocketFlags,
    protocol: Option<Protocol>,
) -> io::Result<OwnedFd> {
    let raw_protocol = match protocol {
        Some(p) => p.0.get(),
        None => 0,
    };
    unsafe {
        ret_owned_fd(c::socket(
            domain.0 as c::c_int,
            (type_.0 | flags.bits()) as c::c_int,
            raw_protocol as c::c_int,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn bind(sockfd: BorrowedFd<'_>, addr: &impl SockAddr) -> io::Result<()> {
    unsafe {
        addr.with_sockaddr(|addr_ptr, addr_len| {
            ret(c::bind(
                borrowed_fd(sockfd),
                addr_ptr.cast(),
                addr_len as c::socklen_t,
            ))
        })
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn connect(sockfd: BorrowedFd<'_>, addr: &impl SockAddr) -> io::Result<()> {
    unsafe {
        addr.with_sockaddr(|addr_ptr, addr_len| {
            ret(c::connect(
                borrowed_fd(sockfd),
                addr_ptr.cast(),
                addr_len as c::socklen_t,
            ))
        })
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn connect_unspec(sockfd: BorrowedFd<'_>) -> io::Result<()> {
    debug_assert_eq!(c::AF_UNSPEC, 0);
    let addr = MaybeUninit::<c::sockaddr_storage>::zeroed();
    unsafe {
        ret(c::connect(
            borrowed_fd(sockfd),
            as_ptr(&addr).cast(),
            size_of::<c::sockaddr_storage>() as c::socklen_t,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn listen(sockfd: BorrowedFd<'_>, backlog: c::c_int) -> io::Result<()> {
    unsafe { ret(c::listen(borrowed_fd(sockfd), backlog)) }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn accept(sockfd: BorrowedFd<'_>) -> io::Result<OwnedFd> {
    unsafe {
        let owned_fd = ret_owned_fd(c::accept(borrowed_fd(sockfd), null_mut(), null_mut()))?;
        Ok(owned_fd)
    }
}

#[cfg(not(any(
    windows,
    target_os = "espidf",
    target_os = "redox",
    target_os = "vita",
    target_os = "wasi"
)))]
pub(crate) fn recvmsg(
    sockfd: BorrowedFd<'_>,
    iov: &mut [IoSliceMut<'_>],
    control: &mut RecvAncillaryBuffer<'_>,
    msg_flags: RecvFlags,
) -> io::Result<RecvMsgReturn> {
    let mut storage = MaybeUninit::<c::sockaddr_storage>::uninit();

    with_recv_msghdr(&mut storage, iov, control, |msghdr| {
        let result = unsafe {
            ret_send_recv(c::recvmsg(
                borrowed_fd(sockfd),
                msghdr,
                bitflags_bits!(msg_flags),
            ))
        };

        result.map(|bytes| {
            // Get the address of the sender, if any.
            let addr =
                unsafe { maybe_read_sockaddr_os(msghdr.msg_name as _, msghdr.msg_namelen as _) };

            RecvMsgReturn {
                bytes,
                address: addr,
                flags: RecvFlags::from_bits_retain(bitcast!(msghdr.msg_flags)),
            }
        })
    })
}

#[cfg(not(any(
    windows,
    target_os = "espidf",
    target_os = "redox",
    target_os = "vita",
    target_os = "wasi"
)))]
pub(crate) fn sendmsg(
    sockfd: BorrowedFd<'_>,
    iov: &[IoSlice<'_>],
    control: &mut SendAncillaryBuffer<'_, '_, '_>,
    msg_flags: SendFlags,
) -> io::Result<usize> {
    with_noaddr_msghdr(iov, control, |msghdr| unsafe {
        ret_send_recv(c::sendmsg(
            borrowed_fd(sockfd),
            &msghdr,
            bitflags_bits!(msg_flags),
        ))
    })
}

#[cfg(not(any(
    windows,
    target_os = "espidf",
    target_os = "redox",
    target_os = "vita",
    target_os = "wasi"
)))]
pub(crate) fn sendmsg_addr(
    sockfd: BorrowedFd<'_>,
    addr: &impl SockAddr,
    iov: &[IoSlice<'_>],
    control: &mut SendAncillaryBuffer<'_, '_, '_>,
    msg_flags: SendFlags,
) -> io::Result<usize> {
    with_msghdr(addr, iov, control, |msghdr| unsafe {
        ret_send_recv(c::sendmsg(
            borrowed_fd(sockfd),
            &msghdr,
            bitflags_bits!(msg_flags),
        ))
    })
}

#[cfg(not(any(
    apple,
    windows,
    target_os = "aix",
    target_os = "espidf",
    target_os = "haiku",
    target_os = "redox",
    target_os = "nto",
    target_os = "vita",
    target_os = "wasi",
)))]
pub(crate) fn accept_with(sockfd: BorrowedFd<'_>, flags: SocketFlags) -> io::Result<OwnedFd> {
    unsafe {
        let owned_fd = ret_owned_fd(c::accept4(
            borrowed_fd(sockfd),
            null_mut(),
            null_mut(),
            flags.bits() as c::c_int,
        ))?;
        Ok(owned_fd)
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn acceptfrom(sockfd: BorrowedFd<'_>) -> io::Result<(OwnedFd, Option<SocketAddrAny>)> {
    unsafe {
        let mut storage = MaybeUninit::<c::sockaddr_storage>::uninit();
        let mut len = size_of::<c::sockaddr_storage>() as c::socklen_t;
        let owned_fd = ret_owned_fd(c::accept(
            borrowed_fd(sockfd),
            storage.as_mut_ptr().cast(),
            &mut len,
        ))?;
        Ok((
            owned_fd,
            maybe_read_sockaddr_os(storage.as_ptr(), len.try_into().unwrap()),
        ))
    }
}

#[cfg(not(any(
    apple,
    windows,
    target_os = "aix",
    target_os = "espidf",
    target_os = "haiku",
    target_os = "nto",
    target_os = "redox",
    target_os = "vita",
    target_os = "wasi",
)))]
pub(crate) fn acceptfrom_with(
    sockfd: BorrowedFd<'_>,
    flags: SocketFlags,
) -> io::Result<(OwnedFd, Option<SocketAddrAny>)> {
    unsafe {
        let mut storage = MaybeUninit::<c::sockaddr_storage>::uninit();
        let mut len = size_of::<c::sockaddr_storage>() as c::socklen_t;
        let owned_fd = ret_owned_fd(c::accept4(
            borrowed_fd(sockfd),
            storage.as_mut_ptr().cast(),
            &mut len,
            flags.bits() as c::c_int,
        ))?;
        Ok((
            owned_fd,
            maybe_read_sockaddr_os(storage.as_ptr(), len.try_into().unwrap()),
        ))
    }
}

/// Darwin lacks `accept4`, but does have `accept`. We define
/// `SocketFlags` to have no flags, so we can discard it here.
#[cfg(any(
    apple,
    windows,
    target_os = "aix",
    target_os = "espidf",
    target_os = "haiku",
    target_os = "nto",
    target_os = "vita",
))]
pub(crate) fn accept_with(sockfd: BorrowedFd<'_>, _flags: SocketFlags) -> io::Result<OwnedFd> {
    accept(sockfd)
}

/// Darwin lacks `accept4`, but does have `accept`. We define
/// `SocketFlags` to have no flags, so we can discard it here.
#[cfg(any(
    apple,
    windows,
    target_os = "aix",
    target_os = "espidf",
    target_os = "haiku",
    target_os = "nto",
    target_os = "vita",
))]
pub(crate) fn acceptfrom_with(
    sockfd: BorrowedFd<'_>,
    _flags: SocketFlags,
) -> io::Result<(OwnedFd, Option<SocketAddrAny>)> {
    acceptfrom(sockfd)
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn shutdown(sockfd: BorrowedFd<'_>, how: Shutdown) -> io::Result<()> {
    unsafe { ret(c::shutdown(borrowed_fd(sockfd), how as c::c_int)) }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn getsockname(sockfd: BorrowedFd<'_>) -> io::Result<SocketAddrAny> {
    unsafe {
        let mut storage = MaybeUninit::<c::sockaddr_storage>::uninit();
        let mut len = size_of::<c::sockaddr_storage>() as c::socklen_t;
        ret(c::getsockname(
            borrowed_fd(sockfd),
            storage.as_mut_ptr().cast(),
            &mut len,
        ))?;
        Ok(read_sockaddr_os(storage.as_ptr(), len.try_into().unwrap()))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn getpeername(sockfd: BorrowedFd<'_>) -> io::Result<Option<SocketAddrAny>> {
    unsafe {
        let mut storage = MaybeUninit::<c::sockaddr_storage>::uninit();
        let mut len = size_of::<c::sockaddr_storage>() as c::socklen_t;
        ret(c::getpeername(
            borrowed_fd(sockfd),
            storage.as_mut_ptr().cast(),
            &mut len,
        ))?;
        Ok(maybe_read_sockaddr_os(
            storage.as_ptr(),
            len.try_into().unwrap(),
        ))
    }
}

#[cfg(not(any(windows, target_os = "redox", target_os = "wasi")))]
pub(crate) fn socketpair(
    domain: AddressFamily,
    type_: SocketType,
    flags: SocketFlags,
    protocol: Option<Protocol>,
) -> io::Result<(OwnedFd, OwnedFd)> {
    let raw_protocol = match protocol {
        Some(p) => p.0.get(),
        None => 0,
    };
    unsafe {
        let mut fds = MaybeUninit::<[OwnedFd; 2]>::uninit();
        ret(c::socketpair(
            c::c_int::from(domain.0),
            (type_.0 | flags.bits()) as c::c_int,
            raw_protocol as c::c_int,
            fds.as_mut_ptr().cast::<c::c_int>(),
        ))?;

        let [fd0, fd1] = fds.assume_init();
        Ok((fd0, fd1))
    }
}
