//! linux_raw syscalls supporting `rustix::net`.
//!
//! # Safety
//!
//! See the `rustix::backend` module documentation for details.
#![allow(unsafe_code, clippy::undocumented_unsafe_blocks)]

use super::msghdr::{with_msghdr, with_noaddr_msghdr, with_recv_msghdr};
use super::read_sockaddr::initialize_family_to_unspec;
use super::send_recv::{RecvFlags, ReturnFlags, SendFlags};
use crate::backend::c;
use crate::backend::conv::{
    by_mut, by_ref, c_int, c_uint, pass_usize, ret, ret_owned_fd, ret_usize, size_of, slice,
    socklen_t, zero,
};
use crate::backend::reg::raw_arg;
use crate::fd::{BorrowedFd, OwnedFd};
use crate::io::{self, IoSlice, IoSliceMut};
use crate::net::SocketAddrBuf;
use crate::net::{
    addr::SocketAddrArg, AddressFamily, Protocol, RecvAncillaryBuffer, RecvMsg,
    SendAncillaryBuffer, Shutdown, SocketAddrAny, SocketFlags, SocketType,
};
use c::socklen_t;
use core::mem::MaybeUninit;
#[cfg(target_arch = "x86")]
use {
    crate::backend::conv::{slice_just_addr, x86_sys},
    crate::backend::reg::{ArgReg, SocketArg},
    linux_raw_sys::net::{
        SYS_ACCEPT, SYS_ACCEPT4, SYS_BIND, SYS_CONNECT, SYS_GETPEERNAME, SYS_GETSOCKNAME,
        SYS_LISTEN, SYS_RECV, SYS_RECVFROM, SYS_RECVMSG, SYS_SEND, SYS_SENDMSG, SYS_SENDTO,
        SYS_SHUTDOWN, SYS_SOCKET, SYS_SOCKETPAIR,
    },
};

#[inline]
pub(crate) fn socket(
    family: AddressFamily,
    type_: SocketType,
    protocol: Option<Protocol>,
) -> io::Result<OwnedFd> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret_owned_fd(syscall_readonly!(__NR_socket, family, type_, protocol))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_owned_fd(syscall_readonly!(
            __NR_socketcall,
            x86_sys(SYS_SOCKET),
            slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[
                family.into(),
                type_.into(),
                protocol.into(),
            ])
        ))
    }
}

#[inline]
pub(crate) fn socket_with(
    family: AddressFamily,
    type_: SocketType,
    flags: SocketFlags,
    protocol: Option<Protocol>,
) -> io::Result<OwnedFd> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret_owned_fd(syscall_readonly!(
            __NR_socket,
            family,
            (type_, flags),
            protocol
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_owned_fd(syscall_readonly!(
            __NR_socketcall,
            x86_sys(SYS_SOCKET),
            slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[
                family.into(),
                (type_, flags).into(),
                protocol.into(),
            ])
        ))
    }
}

#[inline]
pub(crate) fn socketpair(
    family: AddressFamily,
    type_: SocketType,
    flags: SocketFlags,
    protocol: Option<Protocol>,
) -> io::Result<(OwnedFd, OwnedFd)> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut result = MaybeUninit::<[OwnedFd; 2]>::uninit();
        ret(syscall!(
            __NR_socketpair,
            family,
            (type_, flags),
            protocol,
            &mut result
        ))?;
        let [fd0, fd1] = result.assume_init();
        Ok((fd0, fd1))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut result = MaybeUninit::<[OwnedFd; 2]>::uninit();
        ret(syscall!(
            __NR_socketcall,
            x86_sys(SYS_SOCKETPAIR),
            slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[
                family.into(),
                (type_, flags).into(),
                protocol.into(),
                (&mut result).into(),
            ])
        ))?;
        let [fd0, fd1] = result.assume_init();
        Ok((fd0, fd1))
    }
}

#[inline]
pub(crate) fn accept(fd: BorrowedFd<'_>) -> io::Result<OwnedFd> {
    #[cfg(not(any(target_arch = "x86", target_arch = "s390x")))]
    unsafe {
        let fd = ret_owned_fd(syscall_readonly!(__NR_accept, fd, zero(), zero()))?;
        Ok(fd)
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let fd = ret_owned_fd(syscall_readonly!(
            __NR_socketcall,
            x86_sys(SYS_ACCEPT),
            slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[fd.into(), zero(), zero()])
        ))?;
        Ok(fd)
    }
    #[cfg(target_arch = "s390x")]
    {
        // accept is not available on s390x
        accept_with(fd, SocketFlags::empty())
    }
}

#[inline]
pub(crate) fn accept_with(fd: BorrowedFd<'_>, flags: SocketFlags) -> io::Result<OwnedFd> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let fd = ret_owned_fd(syscall_readonly!(__NR_accept4, fd, zero(), zero(), flags))?;
        Ok(fd)
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let fd = ret_owned_fd(syscall_readonly!(
            __NR_socketcall,
            x86_sys(SYS_ACCEPT4),
            slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[fd.into(), zero(), zero(), flags.into()])
        ))?;
        Ok(fd)
    }
}

#[inline]
pub(crate) fn acceptfrom(fd: BorrowedFd<'_>) -> io::Result<(OwnedFd, Option<SocketAddrAny>)> {
    #[cfg(not(any(target_arch = "x86", target_arch = "s390x")))]
    unsafe {
        let mut addr = SocketAddrBuf::new();
        let fd = ret_owned_fd(syscall!(
            __NR_accept,
            fd,
            &mut addr.storage,
            by_mut(&mut addr.len)
        ))?;
        Ok((fd, addr.into_any_option()))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut addr = SocketAddrBuf::new();
        let fd = ret_owned_fd(syscall!(
            __NR_socketcall,
            x86_sys(SYS_ACCEPT),
            slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[
                fd.into(),
                (&mut addr.storage).into(),
                by_mut(&mut addr.len),
            ])
        ))?;
        Ok((fd, addr.into_any_option()))
    }
    #[cfg(target_arch = "s390x")]
    {
        // accept is not available on s390x
        acceptfrom_with(fd, SocketFlags::empty())
    }
}

#[inline]
pub(crate) fn acceptfrom_with(
    fd: BorrowedFd<'_>,
    flags: SocketFlags,
) -> io::Result<(OwnedFd, Option<SocketAddrAny>)> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut addr = SocketAddrBuf::new();
        let fd = ret_owned_fd(syscall!(
            __NR_accept4,
            fd,
            &mut addr.storage,
            by_mut(&mut addr.len),
            flags
        ))?;
        Ok((fd, addr.into_any_option()))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut addr = SocketAddrBuf::new();
        let fd = ret_owned_fd(syscall!(
            __NR_socketcall,
            x86_sys(SYS_ACCEPT4),
            slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[
                fd.into(),
                (&mut addr.storage).into(),
                by_mut(&mut addr.len),
                flags.into(),
            ])
        ))?;
        Ok((fd, addr.into_any_option()))
    }
}

#[inline]
pub(crate) fn recvmsg(
    sockfd: BorrowedFd<'_>,
    iov: &mut [IoSliceMut<'_>],
    control: &mut RecvAncillaryBuffer<'_>,
    msg_flags: RecvFlags,
) -> io::Result<RecvMsg> {
    let mut addr = SocketAddrBuf::new();

    let (bytes, flags) = with_recv_msghdr(&mut addr, iov, control, |msghdr| {
        #[cfg(not(target_arch = "x86"))]
        let result =
            unsafe { ret_usize(syscall!(__NR_recvmsg, sockfd, by_mut(msghdr), msg_flags)) };

        #[cfg(target_arch = "x86")]
        let result = unsafe {
            ret_usize(syscall!(
                __NR_socketcall,
                x86_sys(SYS_RECVMSG),
                slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[
                    sockfd.into(),
                    by_mut(msghdr),
                    msg_flags.into(),
                ])
            ))
        };

        result.map(|bytes| (bytes, msghdr.msg_flags))
    })?;

    // Get the address of the sender, if any.
    Ok(RecvMsg {
        bytes,
        address: unsafe { addr.into_any_option() },
        flags: ReturnFlags::from_bits_retain(flags),
    })
}

#[inline]
pub(crate) fn sendmsg(
    sockfd: BorrowedFd<'_>,
    iov: &[IoSlice<'_>],
    control: &mut SendAncillaryBuffer<'_, '_, '_>,
    msg_flags: SendFlags,
) -> io::Result<usize> {
    with_noaddr_msghdr(iov, control, |msghdr| {
        #[cfg(not(target_arch = "x86"))]
        let result =
            unsafe { ret_usize(syscall!(__NR_sendmsg, sockfd, by_ref(&msghdr), msg_flags)) };

        #[cfg(target_arch = "x86")]
        let result = unsafe {
            ret_usize(syscall!(
                __NR_socketcall,
                x86_sys(SYS_SENDMSG),
                slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[
                    sockfd.into(),
                    by_ref(&msghdr),
                    msg_flags.into()
                ])
            ))
        };

        result
    })
}

#[inline]
pub(crate) fn sendmsg_addr(
    sockfd: BorrowedFd<'_>,
    addr: &impl SocketAddrArg,
    iov: &[IoSlice<'_>],
    control: &mut SendAncillaryBuffer<'_, '_, '_>,
    msg_flags: SendFlags,
) -> io::Result<usize> {
    with_msghdr(addr, iov, control, |msghdr| {
        #[cfg(not(target_arch = "x86"))]
        let result =
            unsafe { ret_usize(syscall!(__NR_sendmsg, sockfd, by_ref(&msghdr), msg_flags)) };

        #[cfg(target_arch = "x86")]
        let result = unsafe {
            ret_usize(syscall!(
                __NR_socketcall,
                x86_sys(SYS_SENDMSG),
                slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[
                    sockfd.into(),
                    by_ref(&msghdr),
                    msg_flags.into(),
                ])
            ))
        };

        result
    })
}

#[inline]
pub(crate) fn shutdown(fd: BorrowedFd<'_>, how: Shutdown) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall_readonly!(
            __NR_shutdown,
            fd,
            c_uint(how as c::c_uint)
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall_readonly!(
            __NR_socketcall,
            x86_sys(SYS_SHUTDOWN),
            slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[fd.into(), c_uint(how as c::c_uint)])
        ))
    }
}

#[inline]
pub(crate) fn send(fd: BorrowedFd<'_>, buf: &[u8], flags: SendFlags) -> io::Result<usize> {
    let (buf_addr, buf_len) = slice(buf);

    #[cfg(not(any(
        target_arch = "aarch64",
        target_arch = "mips64",
        target_arch = "mips64r6",
        target_arch = "riscv64",
        target_arch = "s390x",
        target_arch = "x86",
        target_arch = "x86_64",
    )))]
    unsafe {
        ret_usize(syscall_readonly!(__NR_send, fd, buf_addr, buf_len, flags))
    }
    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "mips64",
        target_arch = "mips64r6",
        target_arch = "riscv64",
        target_arch = "s390x",
        target_arch = "x86_64",
    ))]
    unsafe {
        ret_usize(syscall_readonly!(
            __NR_sendto,
            fd,
            buf_addr,
            buf_len,
            flags,
            zero(),
            zero()
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_usize(syscall_readonly!(
            __NR_socketcall,
            x86_sys(SYS_SEND),
            slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[
                fd.into(),
                buf_addr,
                buf_len,
                flags.into()
            ])
        ))
    }
}

#[inline]
pub(crate) fn sendto(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &impl SocketAddrArg,
) -> io::Result<usize> {
    let (buf_addr, buf_len) = slice(buf);

    addr.with_sockaddr(|addr_ptr, addr_len| {
        #[cfg(not(target_arch = "x86"))]
        unsafe {
            ret_usize(syscall_readonly!(
                __NR_sendto,
                fd,
                buf_addr,
                buf_len,
                flags,
                raw_arg(addr_ptr as *mut _),
                socklen_t(addr_len as socklen_t)
            ))
        }
        #[cfg(target_arch = "x86")]
        unsafe {
            ret_usize(syscall_readonly!(
                __NR_socketcall,
                x86_sys(SYS_SENDTO),
                slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[
                    fd.into(),
                    buf_addr,
                    buf_len,
                    flags.into(),
                    raw_arg(addr_ptr as *mut _),
                    socklen_t(addr_len as socklen_t)
                ])
            ))
        }
    })
}

#[inline]
pub(crate) unsafe fn recv(
    fd: BorrowedFd<'_>,
    buf: *mut u8,
    len: usize,
    flags: RecvFlags,
) -> io::Result<usize> {
    #[cfg(not(any(
        target_arch = "aarch64",
        target_arch = "mips64",
        target_arch = "mips64r6",
        target_arch = "riscv64",
        target_arch = "s390x",
        target_arch = "x86",
        target_arch = "x86_64",
    )))]
    {
        ret_usize(syscall!(__NR_recv, fd, buf, pass_usize(len), flags))
    }
    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "mips64",
        target_arch = "mips64r6",
        target_arch = "riscv64",
        target_arch = "s390x",
        target_arch = "x86_64",
    ))]
    {
        ret_usize(syscall!(
            __NR_recvfrom,
            fd,
            buf,
            pass_usize(len),
            flags,
            zero(),
            zero()
        ))
    }
    #[cfg(target_arch = "x86")]
    {
        ret_usize(syscall!(
            __NR_socketcall,
            x86_sys(SYS_RECV),
            slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[
                fd.into(),
                buf.into(),
                pass_usize(len),
                flags.into(),
            ])
        ))
    }
}

#[inline]
pub(crate) unsafe fn recvfrom(
    fd: BorrowedFd<'_>,
    buf: *mut u8,
    len: usize,
    flags: RecvFlags,
) -> io::Result<(usize, Option<SocketAddrAny>)> {
    let mut addr = SocketAddrBuf::new();

    // `recvfrom` does not write to the storage if the socket is
    // connection-oriented sockets, so we initialize the family field to
    // `AF_UNSPEC` so that we can detect this case.
    initialize_family_to_unspec(addr.storage.as_mut_ptr().cast::<c::sockaddr>());

    #[cfg(not(target_arch = "x86"))]
    let nread = ret_usize(syscall!(
        __NR_recvfrom,
        fd,
        buf,
        pass_usize(len),
        flags,
        &mut addr.storage,
        by_mut(&mut addr.len)
    ))?;
    #[cfg(target_arch = "x86")]
    let nread = ret_usize(syscall!(
        __NR_socketcall,
        x86_sys(SYS_RECVFROM),
        slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[
            fd.into(),
            buf.into(),
            pass_usize(len),
            flags.into(),
            (&mut addr.storage).into(),
            by_mut(&mut addr.len),
        ])
    ))?;

    Ok((nread, addr.into_any_option()))
}

#[inline]
pub(crate) fn getpeername(fd: BorrowedFd<'_>) -> io::Result<Option<SocketAddrAny>> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut addr = SocketAddrBuf::new();
        ret(syscall!(
            __NR_getpeername,
            fd,
            &mut addr.storage,
            by_mut(&mut addr.len)
        ))?;
        Ok(addr.into_any_option())
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut addr = SocketAddrBuf::new();
        ret(syscall!(
            __NR_socketcall,
            x86_sys(SYS_GETPEERNAME),
            slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[
                fd.into(),
                (&mut addr.storage).into(),
                by_mut(&mut addr.len),
            ])
        ))?;
        Ok(addr.into_any_option())
    }
}

#[inline]
pub(crate) fn getsockname(fd: BorrowedFd<'_>) -> io::Result<SocketAddrAny> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut addr = SocketAddrBuf::new();
        ret(syscall!(
            __NR_getsockname,
            fd,
            &mut addr.storage,
            by_mut(&mut addr.len)
        ))?;
        Ok(addr.into_any())
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut addr = SocketAddrBuf::new();
        ret(syscall!(
            __NR_socketcall,
            x86_sys(SYS_GETSOCKNAME),
            slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[
                fd.into(),
                (&mut addr.storage).into(),
                by_mut(&mut addr.len),
            ])
        ))?;
        Ok(addr.into_any())
    }
}

#[inline]
pub(crate) fn bind(fd: BorrowedFd<'_>, addr: &impl SocketAddrArg) -> io::Result<()> {
    addr.with_sockaddr(|addr_ptr, addr_len| {
        #[cfg(not(target_arch = "x86"))]
        unsafe {
            ret(syscall_readonly!(
                __NR_bind,
                fd,
                raw_arg(addr_ptr as *mut _),
                socklen_t(addr_len as socklen_t)
            ))
        }
        #[cfg(target_arch = "x86")]
        unsafe {
            ret(syscall_readonly!(
                __NR_socketcall,
                x86_sys(SYS_BIND),
                slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[
                    fd.into(),
                    raw_arg(addr_ptr as *mut _),
                    socklen_t(addr_len as socklen_t)
                ])
            ))
        }
    })
}

#[inline]
pub(crate) fn connect(fd: BorrowedFd<'_>, addr: &impl SocketAddrArg) -> io::Result<()> {
    addr.with_sockaddr(|addr_ptr, addr_len| {
        #[cfg(not(target_arch = "x86"))]
        unsafe {
            ret(syscall_readonly!(
                __NR_connect,
                fd,
                raw_arg(addr_ptr as *mut _),
                socklen_t(addr_len as socklen_t)
            ))
        }
        #[cfg(target_arch = "x86")]
        unsafe {
            ret(syscall_readonly!(
                __NR_socketcall,
                x86_sys(SYS_CONNECT),
                slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[
                    fd.into(),
                    raw_arg(addr_ptr as *mut _),
                    socklen_t(addr_len as socklen_t)
                ])
            ))
        }
    })
}

#[inline]
pub(crate) fn connect_unspec(fd: BorrowedFd<'_>) -> io::Result<()> {
    debug_assert_eq!(c::AF_UNSPEC, 0);
    let addr = MaybeUninit::<c::sockaddr_storage>::zeroed();

    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall_readonly!(
            __NR_connect,
            fd,
            by_ref(&addr),
            size_of::<c::sockaddr_storage, _>()
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall_readonly!(
            __NR_socketcall,
            x86_sys(SYS_CONNECT),
            slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[
                fd.into(),
                by_ref(&addr),
                size_of::<c::sockaddr_storage, _>(),
            ])
        ))
    }
}

#[inline]
pub(crate) fn listen(fd: BorrowedFd<'_>, backlog: c::c_int) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall_readonly!(__NR_listen, fd, c_int(backlog)))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall_readonly!(
            __NR_socketcall,
            x86_sys(SYS_LISTEN),
            slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[fd.into(), c_int(backlog)])
        ))
    }
}
