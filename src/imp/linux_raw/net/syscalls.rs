//! linux_raw syscalls supporting `rustix::net`.
//!
//! # Safety
//!
//! See the `rustix::imp::syscalls` module documentation for details.

#![allow(unsafe_code)]

use super::super::arch::choose::syscall2_readonly;
use super::super::c;
use super::super::conv::{
    borrowed_fd, by_mut, by_ref, c_int, c_uint, out, ret, ret_owned_fd, ret_usize, size_of, slice,
    slice_mut, socklen_t, zero,
};
use super::super::reg::nr;
use super::{
    encode_sockaddr_unix, encode_sockaddr_v4, encode_sockaddr_v6, read_sockaddr_os, AcceptFlags,
    AddressFamily, Protocol, RecvFlags, SendFlags, Shutdown, SocketFlags, SocketType,
};
use crate::fd::BorrowedFd;
use crate::io::{self, OwnedFd};
use crate::net::{SocketAddrAny, SocketAddrUnix, SocketAddrV4, SocketAddrV6};
use core::convert::TryInto;
use core::mem::MaybeUninit;
#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "riscv64"
)))]
use linux_raw_sys::general::{__NR_recv, __NR_send};
use linux_raw_sys::general::{sockaddr, sockaddr_in, sockaddr_in6, sockaddr_un, socklen_t};
#[cfg(target_arch = "x86")]
use {
    super::super::arch::choose::syscall2,
    super::super::conv::slice_just_addr,
    super::super::conv::x86_sys,
    super::super::reg::{ArgReg, SocketArg},
    linux_raw_sys::general::{
        __NR_socketcall, SYS_ACCEPT, SYS_ACCEPT4, SYS_BIND, SYS_CONNECT, SYS_GETPEERNAME,
        SYS_GETSOCKNAME, SYS_GETSOCKOPT, SYS_LISTEN, SYS_RECV, SYS_RECVFROM, SYS_SEND, SYS_SENDTO,
        SYS_SETSOCKOPT, SYS_SHUTDOWN, SYS_SOCKET, SYS_SOCKETPAIR,
    },
};
#[cfg(not(target_arch = "x86"))]
use {
    super::super::arch::choose::{
        syscall3, syscall3_readonly, syscall4, syscall4_readonly, syscall5, syscall5_readonly,
        syscall6, syscall6_readonly,
    },
    linux_raw_sys::general::{
        __NR_accept, __NR_accept4, __NR_bind, __NR_connect, __NR_getpeername, __NR_getsockname,
        __NR_getsockopt, __NR_listen, __NR_recvfrom, __NR_sendto, __NR_setsockopt, __NR_shutdown,
        __NR_socket, __NR_socketpair,
    },
};

#[inline]
pub(crate) fn socket(
    family: AddressFamily,
    type_: SocketType,
    protocol: Protocol,
) -> io::Result<OwnedFd> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret_owned_fd(syscall3_readonly(
            nr(__NR_socket),
            c_uint(family.0.into()),
            c_uint(type_.0),
            c_uint(protocol.0),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_owned_fd(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_SOCKET),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                c_uint(family.0.into()),
                c_uint(type_.0),
                c_uint(protocol.0),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn socket_with(
    family: AddressFamily,
    type_: SocketType,
    flags: SocketFlags,
    protocol: Protocol,
) -> io::Result<OwnedFd> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret_owned_fd(syscall3_readonly(
            nr(__NR_socket),
            c_uint(family.0.into()),
            c_uint(type_.0 | flags.bits()),
            c_uint(protocol.0),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_owned_fd(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_SOCKET),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                c_uint(family.0.into()),
                c_uint(type_.0 | flags.bits()),
                c_uint(protocol.0),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn socketpair(
    family: AddressFamily,
    type_: SocketType,
    flags: SocketFlags,
    protocol: Protocol,
) -> io::Result<(OwnedFd, OwnedFd)> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut result = MaybeUninit::<[OwnedFd; 2]>::uninit();
        ret(syscall4(
            nr(__NR_socketpair),
            c_uint(family.0.into()),
            c_uint(type_.0 | flags.bits()),
            c_uint(protocol.0),
            out(&mut result),
        ))
        .map(|()| {
            let [fd0, fd1] = result.assume_init();
            (fd0, fd1)
        })
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut result = MaybeUninit::<[OwnedFd; 2]>::uninit();
        ret(syscall2(
            nr(__NR_socketcall),
            x86_sys(SYS_SOCKETPAIR),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                c_uint(family.0.into()),
                c_uint(type_.0 | flags.bits()),
                c_uint(protocol.0),
                out(&mut result),
            ]),
        ))
        .map(|()| {
            let [fd0, fd1] = result.assume_init();
            (fd0, fd1)
        })
    }
}

#[inline]
pub(crate) fn accept(fd: BorrowedFd<'_>) -> io::Result<OwnedFd> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let fd = ret_owned_fd(syscall3_readonly(
            nr(__NR_accept),
            borrowed_fd(fd),
            zero(),
            zero(),
        ))?;
        Ok(fd)
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let fd = ret_owned_fd(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_ACCEPT),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[borrowed_fd(fd), zero(), zero()]),
        ))?;
        Ok(fd)
    }
}

#[inline]
pub(crate) fn accept_with(fd: BorrowedFd<'_>, flags: AcceptFlags) -> io::Result<OwnedFd> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let fd = ret_owned_fd(syscall4_readonly(
            nr(__NR_accept4),
            borrowed_fd(fd),
            zero(),
            zero(),
            c_uint(flags.bits()),
        ))?;
        Ok(fd)
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let fd = ret_owned_fd(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_ACCEPT4),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                zero(),
                zero(),
                c_uint(flags.bits()),
            ]),
        ))?;
        Ok(fd)
    }
}

#[inline]
pub(crate) fn acceptfrom(fd: BorrowedFd<'_>) -> io::Result<(OwnedFd, SocketAddrAny)> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut addrlen = core::mem::size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        let fd = ret_owned_fd(syscall3(
            nr(__NR_accept),
            borrowed_fd(fd),
            out(&mut storage),
            by_mut(&mut addrlen),
        ))?;
        Ok((
            fd,
            read_sockaddr_os(&storage.assume_init(), addrlen.try_into().unwrap()),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut addrlen = core::mem::size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        let fd = ret_owned_fd(syscall2(
            nr(__NR_socketcall),
            x86_sys(SYS_ACCEPT),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                out(&mut storage),
                by_mut(&mut addrlen),
            ]),
        ))?;
        Ok((
            fd,
            read_sockaddr_os(&storage.assume_init(), addrlen.try_into().unwrap()),
        ))
    }
}

#[inline]
pub(crate) fn acceptfrom_with(
    fd: BorrowedFd<'_>,
    flags: AcceptFlags,
) -> io::Result<(OwnedFd, SocketAddrAny)> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut addrlen = core::mem::size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        let fd = ret_owned_fd(syscall4(
            nr(__NR_accept4),
            borrowed_fd(fd),
            out(&mut storage),
            by_mut(&mut addrlen),
            c_uint(flags.bits()),
        ))?;
        Ok((
            fd,
            read_sockaddr_os(&storage.assume_init(), addrlen.try_into().unwrap()),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut addrlen = core::mem::size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        let fd = ret_owned_fd(syscall2(
            nr(__NR_socketcall),
            x86_sys(SYS_ACCEPT4),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                out(&mut storage),
                by_mut(&mut addrlen),
                c_uint(flags.bits()),
            ]),
        ))?;
        Ok((
            fd,
            read_sockaddr_os(&storage.assume_init(), addrlen.try_into().unwrap()),
        ))
    }
}

#[inline]
pub(crate) fn shutdown(fd: BorrowedFd<'_>, how: Shutdown) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall2_readonly(
            nr(__NR_shutdown),
            borrowed_fd(fd),
            c_uint(how as c::c_uint),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_SHUTDOWN),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[borrowed_fd(fd), c_uint(how as c::c_uint)]),
        ))
    }
}

#[inline]
pub(crate) fn send(fd: BorrowedFd<'_>, buf: &[u8], flags: SendFlags) -> io::Result<usize> {
    let (buf_addr, buf_len) = slice(buf);

    #[cfg(not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64"
    )))]
    unsafe {
        ret_usize(syscall4_readonly(
            nr(__NR_send),
            borrowed_fd(fd),
            buf_addr,
            buf_len,
            c_uint(flags.bits()),
        ))
    }
    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64"
    ))]
    unsafe {
        ret_usize(syscall6_readonly(
            nr(__NR_sendto),
            borrowed_fd(fd),
            buf_addr,
            buf_len,
            c_uint(flags.bits()),
            zero(),
            zero(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_usize(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_SEND),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                buf_addr,
                buf_len,
                c_uint(flags.bits()),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn sendto_v4(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrV4,
) -> io::Result<usize> {
    let (buf_addr, buf_len) = slice(buf);

    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret_usize(syscall6_readonly(
            nr(__NR_sendto),
            borrowed_fd(fd),
            buf_addr,
            buf_len,
            c_uint(flags.bits()),
            by_ref(&encode_sockaddr_v4(addr)),
            size_of::<sockaddr_in, _>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_usize(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_SENDTO),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                buf_addr,
                buf_len,
                c_uint(flags.bits()),
                by_ref(&encode_sockaddr_v4(addr)),
                size_of::<sockaddr_in, _>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn sendto_v6(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrV6,
) -> io::Result<usize> {
    let (buf_addr, buf_len) = slice(buf);

    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret_usize(syscall6_readonly(
            nr(__NR_sendto),
            borrowed_fd(fd),
            buf_addr,
            buf_len,
            c_uint(flags.bits()),
            by_ref(&encode_sockaddr_v6(addr)),
            size_of::<sockaddr_in6, _>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_usize(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_SENDTO),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                buf_addr,
                buf_len,
                c_uint(flags.bits()),
                by_ref(&encode_sockaddr_v6(addr)),
                size_of::<sockaddr_in6, _>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn sendto_unix(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrUnix,
) -> io::Result<usize> {
    let (buf_addr, buf_len) = slice(buf);

    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret_usize(syscall6_readonly(
            nr(__NR_sendto),
            borrowed_fd(fd),
            buf_addr,
            buf_len,
            c_uint(flags.bits()),
            by_ref(&encode_sockaddr_unix(addr)),
            size_of::<sockaddr_un, _>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_usize(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_SENDTO),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                buf_addr,
                buf_len,
                c_uint(flags.bits()),
                by_ref(&encode_sockaddr_unix(addr)),
                size_of::<sockaddr_un, _>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn recv(fd: BorrowedFd<'_>, buf: &mut [u8], flags: RecvFlags) -> io::Result<usize> {
    let (buf_addr_mut, buf_len) = slice_mut(buf);

    #[cfg(not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64"
    )))]
    unsafe {
        ret_usize(syscall4(
            nr(__NR_recv),
            borrowed_fd(fd),
            buf_addr_mut,
            buf_len,
            c_uint(flags.bits()),
        ))
    }
    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64"
    ))]
    unsafe {
        ret_usize(syscall6(
            nr(__NR_recvfrom),
            borrowed_fd(fd),
            buf_addr_mut,
            buf_len,
            c_uint(flags.bits()),
            zero(),
            zero(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret_usize(syscall2(
            nr(__NR_socketcall),
            x86_sys(SYS_RECV),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                buf_addr_mut,
                buf_len,
                c_uint(flags.bits()),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn recvfrom(
    fd: BorrowedFd<'_>,
    buf: &mut [u8],
    flags: RecvFlags,
) -> io::Result<(usize, SocketAddrAny)> {
    let (buf_addr_mut, buf_len) = slice_mut(buf);

    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut addrlen = core::mem::size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        let nread = ret_usize(syscall6(
            nr(__NR_recvfrom),
            borrowed_fd(fd),
            buf_addr_mut,
            buf_len,
            c_uint(flags.bits()),
            out(&mut storage),
            by_mut(&mut addrlen),
        ))?;
        Ok((
            nread,
            read_sockaddr_os(&storage.assume_init(), addrlen.try_into().unwrap()),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut addrlen = core::mem::size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        let nread = ret_usize(syscall2(
            nr(__NR_socketcall),
            x86_sys(SYS_RECVFROM),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                buf_addr_mut,
                buf_len,
                c_uint(flags.bits()),
                out(&mut storage),
                by_mut(&mut addrlen),
            ]),
        ))?;
        Ok((
            nread,
            read_sockaddr_os(&storage.assume_init(), addrlen.try_into().unwrap()),
        ))
    }
}

#[inline]
pub(crate) fn getpeername(fd: BorrowedFd<'_>) -> io::Result<SocketAddrAny> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut addrlen = core::mem::size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        ret(syscall3(
            nr(__NR_getpeername),
            borrowed_fd(fd),
            out(&mut storage),
            by_mut(&mut addrlen),
        ))?;
        Ok(read_sockaddr_os(
            &storage.assume_init(),
            addrlen.try_into().unwrap(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut addrlen = core::mem::size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        ret(syscall2(
            nr(__NR_socketcall),
            x86_sys(SYS_GETPEERNAME),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                out(&mut storage),
                by_mut(&mut addrlen),
            ]),
        ))?;
        Ok(read_sockaddr_os(
            &storage.assume_init(),
            addrlen.try_into().unwrap(),
        ))
    }
}

#[inline]
pub(crate) fn getsockname(fd: BorrowedFd<'_>) -> io::Result<SocketAddrAny> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut addrlen = core::mem::size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        ret(syscall3(
            nr(__NR_getsockname),
            borrowed_fd(fd),
            out(&mut storage),
            by_mut(&mut addrlen),
        ))?;
        Ok(read_sockaddr_os(
            &storage.assume_init(),
            addrlen.try_into().unwrap(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let mut addrlen = core::mem::size_of::<sockaddr>() as socklen_t;
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        ret(syscall2(
            nr(__NR_socketcall),
            x86_sys(SYS_GETSOCKNAME),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                out(&mut storage),
                by_mut(&mut addrlen),
            ]),
        ))?;
        Ok(read_sockaddr_os(
            &storage.assume_init(),
            addrlen.try_into().unwrap(),
        ))
    }
}

#[inline]
pub(crate) fn bind_v4(fd: BorrowedFd<'_>, addr: &SocketAddrV4) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_bind),
            borrowed_fd(fd),
            by_ref(&encode_sockaddr_v4(addr)),
            size_of::<sockaddr_in, _>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_BIND),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                by_ref(&encode_sockaddr_v4(addr)),
                size_of::<sockaddr_in, _>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn bind_v6(fd: BorrowedFd<'_>, addr: &SocketAddrV6) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_bind),
            borrowed_fd(fd),
            by_ref(&encode_sockaddr_v6(addr)),
            size_of::<sockaddr_in6, _>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_BIND),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                by_ref(&encode_sockaddr_v6(addr)),
                size_of::<sockaddr_in6, _>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn bind_unix(fd: BorrowedFd<'_>, addr: &SocketAddrUnix) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_bind),
            borrowed_fd(fd),
            by_ref(&encode_sockaddr_unix(addr)),
            size_of::<sockaddr_un, _>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_BIND),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                by_ref(&encode_sockaddr_unix(addr)),
                size_of::<sockaddr_un, _>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn connect_v4(fd: BorrowedFd<'_>, addr: &SocketAddrV4) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_connect),
            borrowed_fd(fd),
            by_ref(&encode_sockaddr_v4(addr)),
            size_of::<sockaddr_in, _>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_CONNECT),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                by_ref(&encode_sockaddr_v4(addr)),
                size_of::<sockaddr_in, _>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn connect_v6(fd: BorrowedFd<'_>, addr: &SocketAddrV6) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_connect),
            borrowed_fd(fd),
            by_ref(&encode_sockaddr_v6(addr)),
            size_of::<sockaddr_in6, _>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_CONNECT),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                by_ref(&encode_sockaddr_v6(addr)),
                size_of::<sockaddr_in6, _>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn connect_unix(fd: BorrowedFd<'_>, addr: &SocketAddrUnix) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_connect),
            borrowed_fd(fd),
            by_ref(&encode_sockaddr_unix(addr)),
            size_of::<sockaddr_un, _>(),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_CONNECT),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[
                borrowed_fd(fd),
                by_ref(&encode_sockaddr_unix(addr)),
                size_of::<sockaddr_un, _>(),
            ]),
        ))
    }
}

#[inline]
pub(crate) fn listen(fd: BorrowedFd<'_>, backlog: c::c_int) -> io::Result<()> {
    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall2_readonly(
            nr(__NR_listen),
            borrowed_fd(fd),
            c_int(backlog),
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall2_readonly(
            nr(__NR_socketcall),
            x86_sys(SYS_LISTEN),
            slice_just_addr::<ArgReg<SocketArg>, _>(&[borrowed_fd(fd), c_int(backlog)]),
        ))
    }
}

pub(crate) mod sockopt {
    use super::{c, BorrowedFd};
    use crate::io;
    use crate::net::sockopt::Timeout;
    use crate::net::{Ipv4Addr, Ipv6Addr, SocketType};
    use core::convert::TryInto;
    use core::time::Duration;

    // TODO: With Rust 1.53 we can use `Duration::ZERO` instead.
    const DURATION_ZERO: Duration = Duration::from_secs(0);

    #[inline]
    fn getsockopt<T>(fd: BorrowedFd<'_>, level: u32, optname: u32) -> io::Result<T> {
        use super::*;
        #[cfg(not(target_arch = "x86"))]
        unsafe {
            let mut value = MaybeUninit::<T>::uninit();
            let mut optlen = core::mem::size_of::<T>();
            ret(syscall5(
                nr(__NR_getsockopt),
                borrowed_fd(fd),
                c_uint(level),
                c_uint(optname),
                out(&mut value),
                by_mut(&mut optlen),
            ))?;
            assert_eq!(
                optlen as usize,
                core::mem::size_of::<T>(),
                "unexpected getsockopt size"
            );
            Ok(value.assume_init())
        }
        #[cfg(target_arch = "x86")]
        unsafe {
            let mut value = MaybeUninit::<T>::uninit();
            let mut optlen = core::mem::size_of::<T>();
            ret(syscall2(
                nr(__NR_socketcall),
                x86_sys(SYS_GETSOCKOPT),
                slice_just_addr::<ArgReg<SocketArg>, _>(&[
                    borrowed_fd(fd),
                    c_uint(level),
                    c_uint(optname),
                    out(&mut value),
                    by_mut(&mut optlen),
                ]),
            ))?;
            assert_eq!(
                optlen as usize,
                core::mem::size_of::<T>(),
                "unexpected getsockopt size"
            );
            Ok(value.assume_init())
        }
    }

    #[inline]
    fn setsockopt<T>(fd: BorrowedFd<'_>, level: u32, optname: u32, value: T) -> io::Result<()> {
        use super::*;
        #[cfg(not(target_arch = "x86"))]
        unsafe {
            let optlen = core::mem::size_of::<T>().try_into().unwrap();
            ret(syscall5_readonly(
                nr(__NR_setsockopt),
                borrowed_fd(fd),
                c_uint(level),
                c_uint(optname),
                by_ref(&value),
                socklen_t(optlen),
            ))
        }
        #[cfg(target_arch = "x86")]
        unsafe {
            let optlen = core::mem::size_of::<T>().try_into().unwrap();
            ret(syscall2_readonly(
                nr(__NR_socketcall),
                x86_sys(SYS_SETSOCKOPT),
                slice_just_addr::<ArgReg<SocketArg>, _>(&[
                    borrowed_fd(fd),
                    c_uint(level),
                    c_uint(optname),
                    by_ref(&value),
                    socklen_t(optlen),
                ]),
            ))
        }
    }

    #[inline]
    pub(crate) fn get_socket_type(fd: BorrowedFd<'_>) -> io::Result<SocketType> {
        getsockopt(
            fd,
            linux_raw_sys::general::SOL_SOCKET as _,
            linux_raw_sys::general::SO_TYPE,
        )
    }

    #[inline]
    pub(crate) fn set_socket_reuseaddr(fd: BorrowedFd<'_>, reuseaddr: bool) -> io::Result<()> {
        setsockopt(
            fd,
            linux_raw_sys::general::SOL_SOCKET as _,
            linux_raw_sys::general::SO_REUSEADDR,
            from_bool(reuseaddr),
        )
    }

    #[inline]
    pub(crate) fn set_socket_broadcast(fd: BorrowedFd<'_>, broadcast: bool) -> io::Result<()> {
        setsockopt(
            fd,
            linux_raw_sys::general::SOL_SOCKET as _,
            linux_raw_sys::general::SO_BROADCAST,
            from_bool(broadcast),
        )
    }

    #[inline]
    pub(crate) fn get_socket_broadcast(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(
            fd,
            linux_raw_sys::general::SOL_SOCKET as _,
            linux_raw_sys::general::SO_BROADCAST,
        )
    }

    #[inline]
    pub(crate) fn set_socket_linger(
        fd: BorrowedFd<'_>,
        linger: Option<Duration>,
    ) -> io::Result<()> {
        let linger = linux_raw_sys::general::linger {
            l_onoff: linger.is_some() as c::c_int,
            l_linger: linger.unwrap_or_default().as_secs() as c::c_int,
        };
        setsockopt(
            fd,
            linux_raw_sys::general::SOL_SOCKET as _,
            linux_raw_sys::general::SO_LINGER,
            linger,
        )
    }

    #[inline]
    pub(crate) fn get_socket_linger(fd: BorrowedFd<'_>) -> io::Result<Option<Duration>> {
        let linger: linux_raw_sys::general::linger = getsockopt(
            fd,
            linux_raw_sys::general::SOL_SOCKET as _,
            linux_raw_sys::general::SO_LINGER,
        )?;
        // TODO: With Rust 1.50, this could use `.then`.
        Ok(if linger.l_onoff != 0 {
            Some(Duration::from_secs(linger.l_linger as u64))
        } else {
            None
        })
    }

    #[inline]
    pub(crate) fn set_socket_passcred(fd: BorrowedFd<'_>, passcred: bool) -> io::Result<()> {
        setsockopt(
            fd,
            linux_raw_sys::general::SOL_SOCKET as _,
            linux_raw_sys::general::SO_PASSCRED,
            from_bool(passcred),
        )
    }

    #[inline]
    pub(crate) fn get_socket_passcred(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(
            fd,
            linux_raw_sys::general::SOL_SOCKET as _,
            linux_raw_sys::general::SO_PASSCRED,
        )
    }

    #[inline]
    pub(crate) fn set_socket_timeout(
        fd: BorrowedFd<'_>,
        id: Timeout,
        timeout: Option<Duration>,
    ) -> io::Result<()> {
        let timeout = match timeout {
            Some(timeout) => {
                if timeout == DURATION_ZERO {
                    return Err(io::Error::INVAL);
                }

                let mut timeout = linux_raw_sys::general::timeval {
                    tv_sec: timeout.as_secs().try_into().unwrap_or(c::c_long::MAX),
                    tv_usec: timeout.subsec_micros() as _,
                };
                if timeout.tv_sec == 0 && timeout.tv_usec == 0 {
                    timeout.tv_usec = 1;
                }
                timeout
            }
            None => linux_raw_sys::general::timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
        };
        let optname = match id {
            Timeout::Recv => linux_raw_sys::general::SO_RCVTIMEO,
            Timeout::Send => linux_raw_sys::general::SO_SNDTIMEO,
        };
        setsockopt(fd, linux_raw_sys::general::SOL_SOCKET, optname, timeout)
    }

    #[inline]
    pub(crate) fn get_socket_timeout(
        fd: BorrowedFd<'_>,
        id: Timeout,
    ) -> io::Result<Option<Duration>> {
        let optname = match id {
            Timeout::Recv => linux_raw_sys::general::SO_RCVTIMEO,
            Timeout::Send => linux_raw_sys::general::SO_SNDTIMEO,
        };
        let timeout: linux_raw_sys::general::timeval =
            getsockopt(fd, linux_raw_sys::general::SOL_SOCKET, optname)?;
        if timeout.tv_sec == 0 && timeout.tv_usec == 0 {
            Ok(None)
        } else {
            Ok(Some(
                Duration::from_secs(timeout.tv_sec as u64)
                    + Duration::from_micros(timeout.tv_usec as u64),
            ))
        }
    }

    #[inline]
    pub(crate) fn set_ip_ttl(fd: BorrowedFd<'_>, ttl: u32) -> io::Result<()> {
        setsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IP as _,
            linux_raw_sys::general::IP_TTL,
            ttl,
        )
    }

    #[inline]
    pub(crate) fn get_ip_ttl(fd: BorrowedFd<'_>) -> io::Result<u32> {
        getsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IP as _,
            linux_raw_sys::general::IP_TTL,
        )
    }

    #[inline]
    pub(crate) fn set_ipv6_v6only(fd: BorrowedFd<'_>, only_v6: bool) -> io::Result<()> {
        setsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IPV6 as _,
            linux_raw_sys::general::IPV6_V6ONLY,
            from_bool(only_v6),
        )
    }

    #[inline]
    pub(crate) fn get_ipv6_v6only(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IPV6 as _,
            linux_raw_sys::general::IPV6_V6ONLY,
        )
    }

    #[inline]
    pub(crate) fn set_ip_multicast_loop(
        fd: BorrowedFd<'_>,
        multicast_loop: bool,
    ) -> io::Result<()> {
        setsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IP as _,
            linux_raw_sys::general::IP_MULTICAST_LOOP,
            from_bool(multicast_loop),
        )
    }

    #[inline]
    pub(crate) fn get_ip_multicast_loop(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IP as _,
            linux_raw_sys::general::IP_MULTICAST_LOOP,
        )
    }

    #[inline]
    pub(crate) fn set_ip_multicast_ttl(fd: BorrowedFd<'_>, multicast_ttl: u32) -> io::Result<()> {
        setsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IP as _,
            linux_raw_sys::general::IP_MULTICAST_TTL,
            multicast_ttl,
        )
    }

    #[inline]
    pub(crate) fn get_ip_multicast_ttl(fd: BorrowedFd<'_>) -> io::Result<u32> {
        getsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IP as _,
            linux_raw_sys::general::IP_MULTICAST_TTL,
        )
    }

    #[inline]
    pub(crate) fn set_ipv6_multicast_loop(
        fd: BorrowedFd<'_>,
        multicast_loop: bool,
    ) -> io::Result<()> {
        setsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IPV6 as _,
            linux_raw_sys::general::IPV6_MULTICAST_LOOP,
            from_bool(multicast_loop),
        )
    }

    #[inline]
    pub(crate) fn get_ipv6_multicast_loop(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IPV6 as _,
            linux_raw_sys::general::IPV6_MULTICAST_LOOP,
        )
    }

    #[inline]
    pub(crate) fn set_ip_add_membership(
        fd: BorrowedFd<'_>,
        multiaddr: &Ipv4Addr,
        interface: &Ipv4Addr,
    ) -> io::Result<()> {
        let mreq = to_imr(multiaddr, interface);
        setsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IP as _,
            linux_raw_sys::general::IP_ADD_MEMBERSHIP,
            mreq,
        )
    }

    #[inline]
    pub(crate) fn set_ipv6_add_membership(
        fd: BorrowedFd<'_>,
        multiaddr: &Ipv6Addr,
        interface: u32,
    ) -> io::Result<()> {
        let mreq = to_ipv6mr(multiaddr, interface);
        setsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IPV6 as _,
            linux_raw_sys::general::IPV6_ADD_MEMBERSHIP,
            mreq,
        )
    }

    #[inline]
    pub(crate) fn set_ip_drop_membership(
        fd: BorrowedFd<'_>,
        multiaddr: &Ipv4Addr,
        interface: &Ipv4Addr,
    ) -> io::Result<()> {
        let mreq = to_imr(multiaddr, interface);
        setsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IP as _,
            linux_raw_sys::general::IP_DROP_MEMBERSHIP,
            mreq,
        )
    }

    #[inline]
    pub(crate) fn set_ipv6_drop_membership(
        fd: BorrowedFd<'_>,
        multiaddr: &Ipv6Addr,
        interface: u32,
    ) -> io::Result<()> {
        let mreq = to_ipv6mr(multiaddr, interface);
        setsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_IPV6 as _,
            linux_raw_sys::general::IPV6_DROP_MEMBERSHIP,
            mreq,
        )
    }

    #[inline]
    pub(crate) fn set_tcp_nodelay(fd: BorrowedFd<'_>, nodelay: bool) -> io::Result<()> {
        setsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_TCP as _,
            linux_raw_sys::general::TCP_NODELAY,
            from_bool(nodelay),
        )
    }

    #[inline]
    pub(crate) fn get_tcp_nodelay(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(
            fd,
            linux_raw_sys::general::IPPROTO_TCP as _,
            linux_raw_sys::general::TCP_NODELAY,
        )
    }

    #[inline]
    fn to_imr(multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> linux_raw_sys::general::ip_mreq {
        linux_raw_sys::general::ip_mreq {
            imr_multiaddr: to_imr_addr(multiaddr),
            imr_interface: to_imr_addr(interface),
        }
    }

    #[inline]
    fn to_imr_addr(addr: &Ipv4Addr) -> linux_raw_sys::general::in_addr {
        linux_raw_sys::general::in_addr {
            s_addr: u32::from_ne_bytes(addr.octets()),
        }
    }

    #[inline]
    fn to_ipv6mr(multiaddr: &Ipv6Addr, interface: u32) -> linux_raw_sys::general::ipv6_mreq {
        linux_raw_sys::general::ipv6_mreq {
            ipv6mr_multiaddr: to_ipv6mr_multiaddr(multiaddr),
            ipv6mr_ifindex: to_ipv6mr_interface(interface),
        }
    }

    #[inline]
    fn to_ipv6mr_multiaddr(multiaddr: &Ipv6Addr) -> linux_raw_sys::general::in6_addr {
        linux_raw_sys::general::in6_addr {
            in6_u: linux_raw_sys::general::in6_addr__bindgen_ty_1 {
                u6_addr8: multiaddr.octets(),
            },
        }
    }

    #[inline]
    fn to_ipv6mr_interface(interface: u32) -> c::c_int {
        interface as c::c_int
    }

    #[inline]
    fn from_bool(value: bool) -> c::c_uint {
        value as c::c_uint
    }
}
