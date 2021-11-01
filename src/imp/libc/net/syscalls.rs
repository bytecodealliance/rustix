#[cfg(any(target_os = "ios", target_os = "macos"))]
use super::super::conv::nonnegative_ret;
use super::super::conv::{borrowed_fd, ret, ret_owned_fd, ret_ssize_t};
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
use super::{
    encode_sockaddr_unix, encode_sockaddr_v4, encode_sockaddr_v6, read_sockaddr_os, AcceptFlags,
    AddressFamily, Protocol, RecvFlags, SendFlags, Shutdown, SocketAddr, SocketAddrUnix,
    SocketFlags, SocketType,
};
use crate::as_ptr;
use crate::io::{self, OwnedFd};
use io_lifetimes::BorrowedFd;
use libc::c_int;
use std::convert::TryInto;
#[cfg(any(target_os = "ios", target_os = "macos"))]
use std::ffi::CString;
use std::mem::{size_of, MaybeUninit};
use std::net::{SocketAddrV4, SocketAddrV6};
#[cfg(not(any(target_os = "redox", target_os = "wasi",)))]
use std::ptr::null_mut;

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn recv(fd: BorrowedFd<'_>, buf: &mut [u8], flags: RecvFlags) -> io::Result<usize> {
    let nrecv = unsafe {
        ret_ssize_t(libc::recv(
            borrowed_fd(fd),
            buf.as_mut_ptr().cast::<_>(),
            buf.len(),
            flags.bits(),
        ))?
    };
    Ok(nrecv as usize)
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn send(fd: BorrowedFd<'_>, buf: &[u8], flags: SendFlags) -> io::Result<usize> {
    let nwritten = unsafe {
        ret_ssize_t(libc::send(
            borrowed_fd(fd),
            buf.as_ptr().cast::<_>(),
            buf.len(),
            flags.bits(),
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn recvfrom(
    fd: BorrowedFd<'_>,
    buf: &mut [u8],
    flags: RecvFlags,
) -> io::Result<(usize, SocketAddr)> {
    unsafe {
        let mut storage = MaybeUninit::<libc::sockaddr_storage>::uninit();
        let mut len = size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        let nread = ret_ssize_t(libc::recvfrom(
            borrowed_fd(fd),
            buf.as_mut_ptr().cast::<_>(),
            buf.len(),
            flags.bits(),
            storage.as_mut_ptr().cast::<_>(),
            &mut len,
        ))?;
        Ok((
            nread as usize,
            read_sockaddr_os(storage.as_ptr(), len.try_into().unwrap()),
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn sendto_v4(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrV4,
) -> io::Result<usize> {
    let nwritten = unsafe {
        ret_ssize_t(libc::sendto(
            borrowed_fd(fd),
            buf.as_ptr().cast::<_>(),
            buf.len(),
            flags.bits(),
            as_ptr(&encode_sockaddr_v4(addr)).cast::<libc::sockaddr>(),
            size_of::<SocketAddrV4>() as u32,
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn sendto_v6(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrV6,
) -> io::Result<usize> {
    let nwritten = unsafe {
        ret_ssize_t(libc::sendto(
            borrowed_fd(fd),
            buf.as_ptr().cast::<_>(),
            buf.len(),
            flags.bits(),
            as_ptr(&encode_sockaddr_v6(addr)).cast::<libc::sockaddr>(),
            size_of::<SocketAddrV6>() as u32,
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn sendto_unix(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrUnix,
) -> io::Result<usize> {
    let nwritten = unsafe {
        ret_ssize_t(libc::sendto(
            borrowed_fd(fd),
            buf.as_ptr().cast::<_>(),
            buf.len(),
            flags.bits(),
            as_ptr(&encode_sockaddr_unix(addr)).cast::<libc::sockaddr>(),
            size_of::<SocketAddrUnix>() as u32,
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn socket(
    domain: AddressFamily,
    type_: SocketType,
    protocol: Protocol,
) -> io::Result<OwnedFd> {
    unsafe {
        ret_owned_fd(libc::socket(
            domain.0 as c_int,
            type_.0 as c_int,
            protocol.0,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn socket_with(
    domain: AddressFamily,
    type_: SocketType,
    flags: SocketFlags,
    protocol: Protocol,
) -> io::Result<OwnedFd> {
    unsafe {
        ret_owned_fd(libc::socket(
            domain.0 as c_int,
            type_.0 as c_int | flags.bits(),
            protocol.0,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn bind_v4(sockfd: BorrowedFd<'_>, addr: &SocketAddrV4) -> io::Result<()> {
    unsafe {
        ret(libc::bind(
            borrowed_fd(sockfd),
            as_ptr(&encode_sockaddr_v4(addr)).cast::<_>(),
            size_of::<libc::sockaddr_in>() as libc::socklen_t,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn bind_v6(sockfd: BorrowedFd<'_>, addr: &SocketAddrV6) -> io::Result<()> {
    unsafe {
        ret(libc::bind(
            borrowed_fd(sockfd),
            as_ptr(&encode_sockaddr_v6(addr)).cast::<_>(),
            size_of::<libc::sockaddr_in6>() as libc::socklen_t,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn bind_unix(sockfd: BorrowedFd<'_>, addr: &SocketAddrUnix) -> io::Result<()> {
    unsafe {
        ret(libc::bind(
            borrowed_fd(sockfd),
            as_ptr(&encode_sockaddr_unix(addr)).cast::<_>(),
            size_of::<libc::sockaddr_un>() as libc::socklen_t,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn connect_v4(sockfd: BorrowedFd<'_>, addr: &SocketAddrV4) -> io::Result<()> {
    unsafe {
        ret(libc::connect(
            borrowed_fd(sockfd),
            as_ptr(&encode_sockaddr_v4(addr)).cast::<_>(),
            size_of::<libc::sockaddr_in>() as libc::socklen_t,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn connect_v6(sockfd: BorrowedFd<'_>, addr: &SocketAddrV6) -> io::Result<()> {
    unsafe {
        ret(libc::connect(
            borrowed_fd(sockfd),
            as_ptr(&encode_sockaddr_v6(addr)).cast::<_>(),
            size_of::<libc::sockaddr_in6>() as libc::socklen_t,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn connect_unix(sockfd: BorrowedFd<'_>, addr: &SocketAddrUnix) -> io::Result<()> {
    unsafe {
        ret(libc::connect(
            borrowed_fd(sockfd),
            as_ptr(&encode_sockaddr_unix(addr)).cast::<_>(),
            size_of::<libc::sockaddr_un>() as libc::socklen_t,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn listen(sockfd: BorrowedFd<'_>, backlog: c_int) -> io::Result<()> {
    unsafe { ret(libc::listen(borrowed_fd(sockfd), backlog)) }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn accept(sockfd: BorrowedFd<'_>) -> io::Result<OwnedFd> {
    unsafe {
        let owned_fd = ret_owned_fd(libc::accept(borrowed_fd(sockfd), null_mut(), null_mut()))?;
        Ok(owned_fd)
    }
}

#[cfg(not(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "redox",
    target_os = "wasi"
)))]
pub(crate) fn accept_with(sockfd: BorrowedFd<'_>, flags: AcceptFlags) -> io::Result<OwnedFd> {
    unsafe {
        let owned_fd = ret_owned_fd(libc::accept4(
            borrowed_fd(sockfd),
            null_mut(),
            null_mut(),
            flags.bits(),
        ))?;
        Ok(owned_fd)
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn acceptfrom(sockfd: BorrowedFd<'_>) -> io::Result<(OwnedFd, SocketAddr)> {
    unsafe {
        let mut storage = MaybeUninit::<libc::sockaddr_storage>::uninit();
        let mut len = size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        let owned_fd = ret_owned_fd(libc::accept(
            borrowed_fd(sockfd),
            storage.as_mut_ptr().cast::<_>(),
            &mut len,
        ))?;
        Ok((
            owned_fd,
            read_sockaddr_os(storage.as_ptr(), len.try_into().unwrap()),
        ))
    }
}

#[cfg(not(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "redox",
    target_os = "wasi"
)))]
pub(crate) fn acceptfrom_with(
    sockfd: BorrowedFd<'_>,
    flags: AcceptFlags,
) -> io::Result<(OwnedFd, SocketAddr)> {
    unsafe {
        let mut storage = MaybeUninit::<libc::sockaddr_storage>::uninit();
        let mut len = size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        let owned_fd = ret_owned_fd(libc::accept4(
            borrowed_fd(sockfd),
            storage.as_mut_ptr().cast::<_>(),
            &mut len,
            flags.bits(),
        ))?;
        Ok((
            owned_fd,
            read_sockaddr_os(storage.as_ptr(), len.try_into().unwrap()),
        ))
    }
}

/// Darwin lacks `accept4`, but does have `accept`. We define
/// `AcceptFlags` to have no flags, so we can discard it here.
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub(crate) fn accept_with(sockfd: BorrowedFd<'_>, _flags: AcceptFlags) -> io::Result<OwnedFd> {
    accept(sockfd)
}

/// Darwin lacks `accept4`, but does have `accept`. We define
/// `AcceptFlags` to have no flags, so we can discard it here.
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub(crate) fn acceptfrom_with(
    sockfd: BorrowedFd<'_>,
    _flags: AcceptFlags,
) -> io::Result<(OwnedFd, SocketAddr)> {
    acceptfrom(sockfd)
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn shutdown(sockfd: BorrowedFd<'_>, how: Shutdown) -> io::Result<()> {
    unsafe { ret(libc::shutdown(borrowed_fd(sockfd), how as c_int)) }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn getsockname(sockfd: BorrowedFd<'_>) -> io::Result<SocketAddr> {
    unsafe {
        let mut storage = MaybeUninit::<libc::sockaddr_storage>::uninit();
        let mut len = size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        ret(libc::getsockname(
            borrowed_fd(sockfd),
            storage.as_mut_ptr().cast::<_>(),
            &mut len,
        ))?;
        Ok(read_sockaddr_os(storage.as_ptr(), len.try_into().unwrap()))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn getpeername(sockfd: BorrowedFd<'_>) -> io::Result<SocketAddr> {
    unsafe {
        let mut storage = MaybeUninit::<libc::sockaddr_storage>::uninit();
        let mut len = size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        ret(libc::getpeername(
            borrowed_fd(sockfd),
            storage.as_mut_ptr().cast::<_>(),
            &mut len,
        ))?;
        Ok(read_sockaddr_os(storage.as_ptr(), len.try_into().unwrap()))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn socketpair(
    domain: AddressFamily,
    type_: SocketType,
    flags: SocketFlags,
    protocol: Protocol,
) -> io::Result<(OwnedFd, OwnedFd)> {
    unsafe {
        let mut fds = MaybeUninit::<[OwnedFd; 2]>::uninit();
        ret(libc::socketpair(
            domain.0 as c_int,
            type_.0 as c_int | flags.bits(),
            protocol.0,
            fds.as_mut_ptr().cast::<c_int>(),
        ))?;

        let [fd0, fd1] = fds.assume_init();
        Ok((fd0, fd1))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) mod sockopt {
    use crate::net::sockopt::Timeout;
    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd"
    )))]
    use crate::net::Ipv6Addr;
    use crate::net::{Ipv4Addr, SocketType};
    use crate::{as_mut_ptr, io};
    use io_lifetimes::BorrowedFd;
    use std::convert::TryInto;
    use std::os::raw::{c_int, c_uint};
    use std::time::Duration;

    #[inline]
    fn getsockopt<T>(fd: BorrowedFd<'_>, level: i32, optname: i32) -> io::Result<T> {
        use super::*;
        unsafe {
            let mut value = MaybeUninit::<T>::uninit();
            let mut optlen = std::mem::size_of::<T>().try_into().unwrap();
            ret(libc::getsockopt(
                borrowed_fd(fd),
                level,
                optname,
                as_mut_ptr(&mut value).cast(),
                &mut optlen,
            ))?;
            assert_eq!(
                optlen as usize,
                size_of::<T>(),
                "unexpected getsockopt size"
            );
            Ok(value.assume_init())
        }
    }

    #[inline]
    fn setsockopt<T>(fd: BorrowedFd<'_>, level: i32, optname: i32, value: T) -> io::Result<()> {
        use super::*;
        unsafe {
            let optlen = std::mem::size_of::<T>().try_into().unwrap();
            ret(libc::setsockopt(
                borrowed_fd(fd),
                level,
                optname,
                as_ptr(&value).cast(),
                optlen,
            ))
        }
    }

    #[inline]
    pub(crate) fn get_socket_type(fd: BorrowedFd<'_>) -> io::Result<SocketType> {
        getsockopt(fd, libc::SOL_SOCKET as _, libc::SO_TYPE)
    }

    #[inline]
    pub(crate) fn set_socket_reuseaddr(fd: BorrowedFd<'_>, reuseaddr: bool) -> io::Result<()> {
        setsockopt(
            fd,
            libc::SOL_SOCKET as _,
            libc::SO_REUSEADDR,
            from_bool(reuseaddr),
        )
    }

    #[inline]
    pub(crate) fn set_socket_broadcast(fd: BorrowedFd<'_>, broadcast: bool) -> io::Result<()> {
        setsockopt(
            fd,
            libc::SOL_SOCKET as _,
            libc::SO_BROADCAST,
            from_bool(broadcast),
        )
    }

    #[inline]
    pub(crate) fn get_socket_broadcast(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(fd, libc::SOL_SOCKET as _, libc::SO_BROADCAST)
    }

    #[inline]
    pub(crate) fn set_socket_linger(
        fd: BorrowedFd<'_>,
        linger: Option<Duration>,
    ) -> io::Result<()> {
        let linger = libc::linger {
            l_onoff: linger.is_some() as c_int,
            l_linger: linger.unwrap_or_default().as_secs() as c_int,
        };
        setsockopt(fd, libc::SOL_SOCKET as _, libc::SO_LINGER, linger)
    }

    #[inline]
    pub(crate) fn get_socket_linger(fd: BorrowedFd<'_>) -> io::Result<Option<Duration>> {
        let linger: libc::linger = getsockopt(fd, libc::SOL_SOCKET as _, libc::SO_LINGER)?;
        Ok((linger.l_onoff != 0).then(|| Duration::from_secs(linger.l_linger as u64)))
    }

    #[cfg(any(target_os = "android", target_os = "linux"))]
    #[inline]
    pub(crate) fn set_socket_passcred(fd: BorrowedFd<'_>, passcred: bool) -> io::Result<()> {
        setsockopt(
            fd,
            libc::SOL_SOCKET as _,
            libc::SO_PASSCRED,
            from_bool(passcred),
        )
    }

    #[cfg(any(target_os = "android", target_os = "linux"))]
    #[inline]
    pub(crate) fn get_socket_passcred(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(fd, libc::SOL_SOCKET as _, libc::SO_PASSCRED)
    }

    #[inline]
    pub(crate) fn set_socket_timeout(
        fd: BorrowedFd<'_>,
        id: Timeout,
        timeout: Option<Duration>,
    ) -> io::Result<()> {
        let timeout = match timeout {
            Some(timeout) => {
                if timeout == Duration::ZERO {
                    return Err(io::Error::INVAL);
                }

                let tv_sec = timeout.as_secs().try_into();
                #[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))]
                let tv_sec = tv_sec.unwrap_or(std::os::raw::c_long::MAX);
                #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
                let tv_sec = tv_sec.unwrap_or(i64::MAX);

                let mut timeout = libc::timeval {
                    tv_sec,
                    tv_usec: timeout.subsec_micros() as _,
                };
                if timeout.tv_sec == 0 && timeout.tv_usec == 0 {
                    timeout.tv_usec = 1;
                }
                timeout
            }
            None => libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
        };
        let optname = match id {
            Timeout::Recv => libc::SO_RCVTIMEO,
            Timeout::Send => libc::SO_SNDTIMEO,
        };
        setsockopt(fd, libc::SOL_SOCKET, optname, timeout)
    }

    #[inline]
    pub(crate) fn get_socket_timeout(
        fd: BorrowedFd<'_>,
        id: Timeout,
    ) -> io::Result<Option<Duration>> {
        let optname = match id {
            Timeout::Recv => libc::SO_RCVTIMEO,
            Timeout::Send => libc::SO_SNDTIMEO,
        };
        let timeout: libc::timeval = getsockopt(fd, libc::SOL_SOCKET, optname)?;
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
        setsockopt(fd, libc::IPPROTO_IP as _, libc::IP_TTL, ttl)
    }

    #[inline]
    pub(crate) fn get_ip_ttl(fd: BorrowedFd<'_>) -> io::Result<u32> {
        getsockopt(fd, libc::IPPROTO_IP as _, libc::IP_TTL)
    }

    #[inline]
    pub(crate) fn set_ipv6_v6only(fd: BorrowedFd<'_>, only_v6: bool) -> io::Result<()> {
        setsockopt(
            fd,
            libc::IPPROTO_IPV6 as _,
            libc::IPV6_V6ONLY,
            from_bool(only_v6),
        )
    }

    #[inline]
    pub(crate) fn get_ipv6_v6only(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(fd, libc::IPPROTO_IPV6 as _, libc::IPV6_V6ONLY)
    }

    #[inline]
    pub(crate) fn set_ip_multicast_loop(
        fd: BorrowedFd<'_>,
        multicast_loop: bool,
    ) -> io::Result<()> {
        setsockopt(
            fd,
            libc::IPPROTO_IP as _,
            libc::IP_MULTICAST_LOOP,
            from_bool(multicast_loop),
        )
    }

    #[inline]
    pub(crate) fn get_ip_multicast_loop(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(fd, libc::IPPROTO_IP as _, libc::IP_MULTICAST_LOOP)
    }

    #[inline]
    pub(crate) fn set_ip_multicast_ttl(fd: BorrowedFd<'_>, multicast_ttl: u32) -> io::Result<()> {
        setsockopt(
            fd,
            libc::IPPROTO_IP as _,
            libc::IP_MULTICAST_TTL,
            multicast_ttl,
        )
    }

    #[inline]
    pub(crate) fn get_ip_multicast_ttl(fd: BorrowedFd<'_>) -> io::Result<u32> {
        getsockopt(fd, libc::IPPROTO_IP as _, libc::IP_MULTICAST_TTL)
    }

    #[inline]
    pub(crate) fn set_ipv6_multicast_loop(
        fd: BorrowedFd<'_>,
        multicast_loop: bool,
    ) -> io::Result<()> {
        setsockopt(
            fd,
            libc::IPPROTO_IPV6 as _,
            libc::IPV6_MULTICAST_LOOP,
            from_bool(multicast_loop),
        )
    }

    #[inline]
    pub(crate) fn get_ipv6_multicast_loop(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(fd, libc::IPPROTO_IPV6 as _, libc::IPV6_MULTICAST_LOOP)
    }

    #[inline]
    pub(crate) fn set_ip_add_membership(
        fd: BorrowedFd<'_>,
        multiaddr: &Ipv4Addr,
        interface: &Ipv4Addr,
    ) -> io::Result<()> {
        let mreq = to_imr(multiaddr, interface);
        setsockopt(fd, libc::IPPROTO_IP as _, libc::IP_ADD_MEMBERSHIP, mreq)
    }

    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    )))]
    #[inline]
    pub(crate) fn set_ipv6_join_group(
        fd: BorrowedFd<'_>,
        multiaddr: &Ipv6Addr,
        interface: u32,
    ) -> io::Result<()> {
        let mreq = to_ipv6mr(multiaddr, interface);
        setsockopt(fd, libc::IPPROTO_IPV6 as _, libc::IPV6_ADD_MEMBERSHIP, mreq)
    }

    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd"
    )))]
    #[inline]
    pub(crate) fn set_ip_drop_membership(
        fd: BorrowedFd<'_>,
        multiaddr: &Ipv4Addr,
        interface: &Ipv4Addr,
    ) -> io::Result<()> {
        let mreq = to_imr(multiaddr, interface);
        setsockopt(fd, libc::IPPROTO_IP as _, libc::IP_DROP_MEMBERSHIP, mreq)
    }

    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
    )))]
    #[inline]
    pub(crate) fn set_ipv6_leave_group(
        fd: BorrowedFd<'_>,
        multiaddr: &Ipv6Addr,
        interface: u32,
    ) -> io::Result<()> {
        let mreq = to_ipv6mr(multiaddr, interface);
        setsockopt(
            fd,
            libc::IPPROTO_IPV6 as _,
            libc::IPV6_DROP_MEMBERSHIP,
            mreq,
        )
    }

    #[inline]
    pub(crate) fn set_tcp_nodelay(fd: BorrowedFd<'_>, nodelay: bool) -> io::Result<()> {
        setsockopt(
            fd,
            libc::IPPROTO_TCP as _,
            libc::TCP_NODELAY,
            from_bool(nodelay),
        )
    }

    #[inline]
    pub(crate) fn get_tcp_nodelay(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(fd, libc::IPPROTO_TCP as _, libc::TCP_NODELAY)
    }

    #[inline]
    fn to_imr(multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> libc::ip_mreq {
        libc::ip_mreq {
            imr_multiaddr: to_imr_addr(multiaddr),
            imr_interface: to_imr_addr(interface),
        }
    }

    #[inline]
    fn to_imr_addr(addr: &Ipv4Addr) -> libc::in_addr {
        libc::in_addr {
            s_addr: u32::from_ne_bytes(addr.octets()),
        }
    }

    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd"
    )))]
    #[inline]
    fn to_ipv6mr(multiaddr: &Ipv6Addr, interface: u32) -> libc::ipv6_mreq {
        libc::ipv6_mreq {
            ipv6mr_multiaddr: to_ipv6mr_multiaddr(multiaddr),
            ipv6mr_interface: to_ipv6mr_interface(interface),
        }
    }

    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd"
    )))]
    #[inline]
    fn to_ipv6mr_multiaddr(multiaddr: &Ipv6Addr) -> libc::in6_addr {
        libc::in6_addr {
            s6_addr: multiaddr.octets(),
        }
    }

    #[cfg(target_os = "android")]
    #[inline]
    fn to_ipv6mr_interface(interface: u32) -> c_int {
        interface as c_int
    }

    #[cfg(not(any(
        target_os = "android",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd"
    )))]
    #[inline]
    fn to_ipv6mr_interface(interface: u32) -> c_uint {
        interface as c_uint
    }

    #[inline]
    fn from_bool(value: bool) -> c_uint {
        value as c_uint
    }
}
