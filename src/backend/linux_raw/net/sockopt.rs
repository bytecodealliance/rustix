//! linux_raw syscalls supporting `rustix::net::sockopt`.
//!
//! # Safety
//!
//! See the `rustix::backend` module documentation for details.
#![allow(unsafe_code, clippy::undocumented_unsafe_blocks)]

use crate::backend::c;
use crate::backend::conv::{by_mut, by_ref, c_uint, ret, socklen_t};
use crate::fd::BorrowedFd;
use crate::io;
use crate::net::sockopt::Timeout;
use crate::net::{AddressFamily, Ipv4Addr, Ipv6Addr, SocketType};
use core::mem::MaybeUninit;
use core::time::Duration;
use linux_raw_sys::general::{__kernel_old_timeval, __kernel_sock_timeval};

#[inline]
fn getsockopt<T: Copy>(fd: BorrowedFd<'_>, level: u32, optname: u32) -> io::Result<T> {
    let mut optlen = core::mem::size_of::<T>();
    debug_assert!(
        optlen as usize >= core::mem::size_of::<c::c_int>(),
        "Socket APIs don't ever use `bool` directly"
    );

    #[cfg(not(target_arch = "x86"))]
    unsafe {
        let mut value = MaybeUninit::<T>::uninit();
        ret(syscall!(
            __NR_getsockopt,
            fd,
            c_uint(level),
            c_uint(optname),
            &mut value,
            by_mut(&mut optlen)
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
        ret(syscall!(
            __NR_socketcall,
            x86_sys(SYS_GETSOCKOPT),
            slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[
                fd.into(),
                c_uint(level),
                c_uint(optname),
                (&mut value).into(),
                by_mut(&mut optlen),
            ])
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
fn setsockopt<T: Copy>(fd: BorrowedFd<'_>, level: u32, optname: u32, value: T) -> io::Result<()> {
    let optlen = core::mem::size_of::<T>().try_into().unwrap();
    debug_assert!(
        optlen as usize >= core::mem::size_of::<c::c_int>(),
        "Socket APIs don't ever use `bool` directly"
    );

    #[cfg(not(target_arch = "x86"))]
    unsafe {
        ret(syscall_readonly!(
            __NR_setsockopt,
            fd,
            c_uint(level),
            c_uint(optname),
            by_ref(&value),
            socklen_t(optlen)
        ))
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        ret(syscall_readonly!(
            __NR_socketcall,
            x86_sys(SYS_SETSOCKOPT),
            slice_just_addr::<ArgReg<'_, SocketArg>, _>(&[
                fd.into(),
                c_uint(level),
                c_uint(optname),
                by_ref(&value),
                socklen_t(optlen),
            ])
        ))
    }
}

#[inline]
pub(crate) fn get_socket_type(fd: BorrowedFd<'_>) -> io::Result<SocketType> {
    getsockopt(fd, c::SOL_SOCKET, c::SO_TYPE)
}

#[inline]
pub(crate) fn set_socket_reuseaddr(fd: BorrowedFd<'_>, reuseaddr: bool) -> io::Result<()> {
    setsockopt(fd, c::SOL_SOCKET, c::SO_REUSEADDR, from_bool(reuseaddr))
}

#[inline]
pub(crate) fn get_socket_reuseaddr(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::SOL_SOCKET, c::SO_REUSEADDR).map(to_bool)
}

#[inline]
pub(crate) fn set_socket_broadcast(fd: BorrowedFd<'_>, broadcast: bool) -> io::Result<()> {
    setsockopt(fd, c::SOL_SOCKET, c::SO_BROADCAST, from_bool(broadcast))
}

#[inline]
pub(crate) fn get_socket_broadcast(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::SOL_SOCKET, c::SO_BROADCAST).map(to_bool)
}

#[inline]
pub(crate) fn set_socket_linger(fd: BorrowedFd<'_>, linger: Option<Duration>) -> io::Result<()> {
    // Convert `linger` to seconds, rounding up.
    let l_linger = if let Some(linger) = linger {
        duration_to_secs(linger)?
    } else {
        0
    };
    let linger = c::linger {
        l_onoff: c::c_int::from(linger.is_some()),
        l_linger,
    };
    setsockopt(fd, c::SOL_SOCKET, c::SO_LINGER, linger)
}

#[inline]
pub(crate) fn get_socket_linger(fd: BorrowedFd<'_>) -> io::Result<Option<Duration>> {
    let linger: c::linger = getsockopt(fd, c::SOL_SOCKET, c::SO_LINGER)?;
    Ok((linger.l_onoff != 0).then(|| Duration::from_secs(linger.l_linger as u64)))
}

#[inline]
pub(crate) fn set_socket_passcred(fd: BorrowedFd<'_>, passcred: bool) -> io::Result<()> {
    setsockopt(fd, c::SOL_SOCKET, c::SO_PASSCRED, from_bool(passcred))
}

#[inline]
pub(crate) fn get_socket_passcred(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::SOL_SOCKET, c::SO_PASSCRED).map(to_bool)
}

#[inline]
pub(crate) fn set_socket_timeout(
    fd: BorrowedFd<'_>,
    id: Timeout,
    timeout: Option<Duration>,
) -> io::Result<()> {
    let time = duration_to_linux_sock_timeval(timeout)?;
    let optname = match id {
        Timeout::Recv => c::SO_RCVTIMEO_NEW,
        Timeout::Send => c::SO_SNDTIMEO_NEW,
    };
    match setsockopt(fd, c::SOL_SOCKET, optname, time) {
        Err(io::Errno::NOPROTOOPT) if c::SO_RCVTIMEO_NEW != c::SO_RCVTIMEO_OLD => {
            set_socket_timeout_old(fd, id, timeout)
        }
        otherwise => otherwise,
    }
}

/// Same as `set_socket_timeout` but uses `__kernel_old_timeval` instead of
/// `__kernel_sock_timeval` and `_OLD` constants instead of `_NEW`.
fn set_socket_timeout_old(
    fd: BorrowedFd<'_>,
    id: Timeout,
    timeout: Option<Duration>,
) -> io::Result<()> {
    let time = duration_to_linux_old_timeval(timeout)?;
    let optname = match id {
        Timeout::Recv => c::SO_RCVTIMEO_OLD,
        Timeout::Send => c::SO_SNDTIMEO_OLD,
    };
    setsockopt(fd, c::SOL_SOCKET, optname, time)
}

#[inline]
pub(crate) fn get_socket_timeout(fd: BorrowedFd<'_>, id: Timeout) -> io::Result<Option<Duration>> {
    let optname = match id {
        Timeout::Recv => c::SO_RCVTIMEO_NEW,
        Timeout::Send => c::SO_SNDTIMEO_NEW,
    };
    let time: __kernel_sock_timeval = match getsockopt(fd, c::SOL_SOCKET, optname) {
        Err(io::Errno::NOPROTOOPT) if c::SO_RCVTIMEO_NEW != c::SO_RCVTIMEO_OLD => {
            return get_socket_timeout_old(fd, id)
        }
        otherwise => otherwise?,
    };
    Ok(duration_from_linux_sock_timeval(time))
}

/// Same as `get_socket_timeout` but uses `__kernel_old_timeval` instead of
/// `__kernel_sock_timeval` and `_OLD` constants instead of `_NEW`.
fn get_socket_timeout_old(fd: BorrowedFd<'_>, id: Timeout) -> io::Result<Option<Duration>> {
    let optname = match id {
        Timeout::Recv => c::SO_RCVTIMEO_OLD,
        Timeout::Send => c::SO_SNDTIMEO_OLD,
    };
    let time: __kernel_old_timeval = getsockopt(fd, c::SOL_SOCKET, optname)?;
    Ok(duration_from_linux_old_timeval(time))
}

/// Convert a `__linux_sock_timeval` to a Rust `Option<Duration>`.
#[inline]
fn duration_from_linux_sock_timeval(time: __kernel_sock_timeval) -> Option<Duration> {
    if time.tv_sec == 0 && time.tv_usec == 0 {
        None
    } else {
        Some(Duration::from_secs(time.tv_sec as u64) + Duration::from_micros(time.tv_usec as u64))
    }
}

/// Like `duration_from_linux` but uses Linux's old 32-bit
/// `__kernel_old_timeval`.
fn duration_from_linux_old_timeval(time: __kernel_old_timeval) -> Option<Duration> {
    if time.tv_sec == 0 && time.tv_usec == 0 {
        None
    } else {
        Some(Duration::from_secs(time.tv_sec as u64) + Duration::from_micros(time.tv_usec as u64))
    }
}

/// Convert a Rust `Option<Duration>` to a `__kernel_sock_timeval`.
#[inline]
fn duration_to_linux_sock_timeval(timeout: Option<Duration>) -> io::Result<__kernel_sock_timeval> {
    Ok(match timeout {
        Some(timeout) => {
            if timeout == Duration::ZERO {
                return Err(io::Errno::INVAL);
            }
            // `subsec_micros` rounds down, so we use `subsec_nanos` and
            // manually round up.
            let mut timeout = __kernel_sock_timeval {
                tv_sec: timeout.as_secs().try_into().unwrap_or(i64::MAX),
                tv_usec: ((timeout.subsec_nanos() + 999) / 1000) as _,
            };
            if timeout.tv_sec == 0 && timeout.tv_usec == 0 {
                timeout.tv_usec = 1;
            }
            timeout
        }
        None => __kernel_sock_timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
    })
}

/// Like `duration_to_linux` but uses Linux's old 32-bit
/// `__kernel_old_timeval`.
fn duration_to_linux_old_timeval(timeout: Option<Duration>) -> io::Result<__kernel_old_timeval> {
    Ok(match timeout {
        Some(timeout) => {
            if timeout == Duration::ZERO {
                return Err(io::Errno::INVAL);
            }

            // `subsec_micros` rounds down, so we use `subsec_nanos` and
            // manually round up.
            let mut timeout = __kernel_old_timeval {
                tv_sec: timeout.as_secs().try_into().unwrap_or(c::c_long::MAX),
                tv_usec: ((timeout.subsec_nanos() + 999) / 1000) as _,
            };
            if timeout.tv_sec == 0 && timeout.tv_usec == 0 {
                timeout.tv_usec = 1;
            }
            timeout
        }
        None => __kernel_old_timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
    })
}

#[inline]
pub(crate) fn get_socket_error(fd: BorrowedFd<'_>) -> io::Result<Result<(), io::Errno>> {
    let err: c::c_int = getsockopt(fd, c::SOL_SOCKET, c::SO_ERROR)?;
    Ok(if err == 0 {
        Ok(())
    } else {
        Err(io::Errno::from_raw_os_error(err))
    })
}

#[inline]
pub(crate) fn set_socket_keepalive(fd: BorrowedFd<'_>, keepalive: bool) -> io::Result<()> {
    setsockopt(fd, c::SOL_SOCKET, c::SO_KEEPALIVE, from_bool(keepalive))
}

#[inline]
pub(crate) fn get_socket_keepalive(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::SOL_SOCKET, c::SO_KEEPALIVE).map(to_bool)
}

#[inline]
pub(crate) fn set_socket_recv_buffer_size(fd: BorrowedFd<'_>, size: usize) -> io::Result<()> {
    let size: c::c_int = size.try_into().map_err(|_| io::Errno::INVAL)?;
    setsockopt(fd, c::SOL_SOCKET, c::SO_RCVBUF, size)
}

#[inline]
pub(crate) fn get_socket_recv_buffer_size(fd: BorrowedFd<'_>) -> io::Result<usize> {
    getsockopt(fd, c::SOL_SOCKET, c::SO_RCVBUF).map(|size: u32| size as usize)
}

#[inline]
pub(crate) fn set_socket_send_buffer_size(fd: BorrowedFd<'_>, size: usize) -> io::Result<()> {
    let size: c::c_int = size.try_into().map_err(|_| io::Errno::INVAL)?;
    setsockopt(fd, c::SOL_SOCKET, c::SO_SNDBUF, size)
}

#[inline]
pub(crate) fn get_socket_send_buffer_size(fd: BorrowedFd<'_>) -> io::Result<usize> {
    getsockopt(fd, c::SOL_SOCKET, c::SO_SNDBUF).map(|size: u32| size as usize)
}

#[inline]
pub(crate) fn get_socket_domain(fd: BorrowedFd<'_>) -> io::Result<AddressFamily> {
    let domain: c::c_int = getsockopt(fd, c::SOL_SOCKET, c::SO_DOMAIN)?;
    Ok(AddressFamily(
        domain.try_into().map_err(|_| io::Errno::OPNOTSUPP)?,
    ))
}

#[inline]
pub(crate) fn get_socket_acceptconn(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::SOL_SOCKET, c::SO_ACCEPTCONN).map(to_bool)
}

#[inline]
pub(crate) fn set_socket_oobinline(fd: BorrowedFd<'_>, value: bool) -> io::Result<()> {
    setsockopt(fd, c::SOL_SOCKET, c::SO_OOBINLINE, from_bool(value))
}

#[inline]
pub(crate) fn get_socket_oobinline(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::SOL_SOCKET, c::SO_OOBINLINE).map(to_bool)
}

#[inline]
pub(crate) fn set_ip_ttl(fd: BorrowedFd<'_>, ttl: u32) -> io::Result<()> {
    setsockopt(fd, c::IPPROTO_IP, c::IP_TTL, ttl)
}

#[inline]
pub(crate) fn get_ip_ttl(fd: BorrowedFd<'_>) -> io::Result<u32> {
    getsockopt(fd, c::IPPROTO_IP, c::IP_TTL)
}

#[inline]
pub(crate) fn set_ipv6_v6only(fd: BorrowedFd<'_>, only_v6: bool) -> io::Result<()> {
    setsockopt(fd, c::IPPROTO_IPV6, c::IPV6_V6ONLY, from_bool(only_v6))
}

#[inline]
pub(crate) fn get_ipv6_v6only(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::IPPROTO_IPV6, c::IPV6_V6ONLY).map(to_bool)
}

#[inline]
pub(crate) fn set_ip_multicast_loop(fd: BorrowedFd<'_>, multicast_loop: bool) -> io::Result<()> {
    setsockopt(
        fd,
        c::IPPROTO_IP,
        c::IP_MULTICAST_LOOP,
        from_bool(multicast_loop),
    )
}

#[inline]
pub(crate) fn get_ip_multicast_loop(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::IPPROTO_IP, c::IP_MULTICAST_LOOP).map(to_bool)
}

#[inline]
pub(crate) fn set_ip_multicast_ttl(fd: BorrowedFd<'_>, multicast_ttl: u32) -> io::Result<()> {
    setsockopt(fd, c::IPPROTO_IP, c::IP_MULTICAST_TTL, multicast_ttl)
}

#[inline]
pub(crate) fn get_ip_multicast_ttl(fd: BorrowedFd<'_>) -> io::Result<u32> {
    getsockopt(fd, c::IPPROTO_IP, c::IP_MULTICAST_TTL)
}

#[inline]
pub(crate) fn set_ipv6_multicast_loop(fd: BorrowedFd<'_>, multicast_loop: bool) -> io::Result<()> {
    setsockopt(
        fd,
        c::IPPROTO_IPV6,
        c::IPV6_MULTICAST_LOOP,
        from_bool(multicast_loop),
    )
}

#[inline]
pub(crate) fn get_ipv6_multicast_loop(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::IPPROTO_IPV6, c::IPV6_MULTICAST_LOOP).map(to_bool)
}

#[inline]
pub(crate) fn set_ipv6_multicast_hops(fd: BorrowedFd<'_>, multicast_hops: u32) -> io::Result<()> {
    setsockopt(fd, c::IPPROTO_IP, c::IPV6_MULTICAST_HOPS, multicast_hops)
}

#[inline]
pub(crate) fn get_ipv6_multicast_hops(fd: BorrowedFd<'_>) -> io::Result<u32> {
    getsockopt(fd, c::IPPROTO_IP, c::IPV6_MULTICAST_HOPS)
}

#[inline]
pub(crate) fn set_ip_add_membership(
    fd: BorrowedFd<'_>,
    multiaddr: &Ipv4Addr,
    interface: &Ipv4Addr,
) -> io::Result<()> {
    let mreq = to_ip_mreq(multiaddr, interface);
    setsockopt(fd, c::IPPROTO_IP, c::IP_ADD_MEMBERSHIP, mreq)
}

#[inline]
pub(crate) fn set_ip_add_membership_with_ifindex(
    fd: BorrowedFd<'_>,
    multiaddr: &Ipv4Addr,
    address: &Ipv4Addr,
    ifindex: i32,
) -> io::Result<()> {
    let mreqn = to_ip_mreqn(multiaddr, address, ifindex);
    setsockopt(fd, c::IPPROTO_IP, c::IP_ADD_MEMBERSHIP, mreqn)
}

#[inline]
pub(crate) fn set_ip_add_source_membership(
    fd: BorrowedFd<'_>,
    multiaddr: &Ipv4Addr,
    interface: &Ipv4Addr,
    sourceaddr: &Ipv4Addr,
) -> io::Result<()> {
    let mreq_source = to_imr_source(multiaddr, interface, sourceaddr);
    setsockopt(fd, c::IPPROTO_IP, c::IP_ADD_SOURCE_MEMBERSHIP, mreq_source)
}

#[inline]
pub(crate) fn set_ip_drop_source_membership(
    fd: BorrowedFd<'_>,
    multiaddr: &Ipv4Addr,
    interface: &Ipv4Addr,
    sourceaddr: &Ipv4Addr,
) -> io::Result<()> {
    let mreq_source = to_imr_source(multiaddr, interface, sourceaddr);
    setsockopt(fd, c::IPPROTO_IP, c::IP_DROP_SOURCE_MEMBERSHIP, mreq_source)
}

#[inline]
pub(crate) fn set_ipv6_add_membership(
    fd: BorrowedFd<'_>,
    multiaddr: &Ipv6Addr,
    interface: u32,
) -> io::Result<()> {
    let mreq = to_ipv6mr(multiaddr, interface);
    setsockopt(fd, c::IPPROTO_IPV6, c::IPV6_ADD_MEMBERSHIP, mreq)
}

#[inline]
pub(crate) fn set_ip_drop_membership(
    fd: BorrowedFd<'_>,
    multiaddr: &Ipv4Addr,
    interface: &Ipv4Addr,
) -> io::Result<()> {
    let mreq = to_ip_mreq(multiaddr, interface);
    setsockopt(fd, c::IPPROTO_IP, c::IP_DROP_MEMBERSHIP, mreq)
}

#[inline]
pub(crate) fn set_ip_drop_membership_with_ifindex(
    fd: BorrowedFd<'_>,
    multiaddr: &Ipv4Addr,
    address: &Ipv4Addr,
    ifindex: i32,
) -> io::Result<()> {
    let mreqn = to_ip_mreqn(multiaddr, address, ifindex);
    setsockopt(fd, c::IPPROTO_IP, c::IP_DROP_MEMBERSHIP, mreqn)
}

#[inline]
pub(crate) fn set_ipv6_drop_membership(
    fd: BorrowedFd<'_>,
    multiaddr: &Ipv6Addr,
    interface: u32,
) -> io::Result<()> {
    let mreq = to_ipv6mr(multiaddr, interface);
    setsockopt(fd, c::IPPROTO_IPV6, c::IPV6_DROP_MEMBERSHIP, mreq)
}

#[inline]
pub(crate) fn get_ipv6_unicast_hops(fd: BorrowedFd<'_>) -> io::Result<u8> {
    getsockopt(fd, c::IPPROTO_IPV6, c::IPV6_UNICAST_HOPS).map(|hops: c::c_int| hops as u8)
}

#[inline]
pub(crate) fn set_ipv6_unicast_hops(fd: BorrowedFd<'_>, hops: Option<u8>) -> io::Result<()> {
    let hops = match hops {
        Some(hops) => hops.into(),
        None => -1,
    };
    setsockopt(fd, c::IPPROTO_IPV6, c::IPV6_UNICAST_HOPS, hops)
}

#[inline]
pub(crate) fn set_ip_tos(fd: BorrowedFd<'_>, value: u8) -> io::Result<()> {
    setsockopt(fd, c::IPPROTO_IP, c::IP_TOS, i32::from(value))
}

#[inline]
pub(crate) fn get_ip_tos(fd: BorrowedFd<'_>) -> io::Result<u8> {
    let value: i32 = getsockopt(fd, c::IPPROTO_IP, c::IP_TOS)?;
    Ok(value as u8)
}

#[inline]
pub(crate) fn set_ip_recvtos(fd: BorrowedFd<'_>, value: bool) -> io::Result<()> {
    setsockopt(fd, c::IPPROTO_IP, c::IP_RECVTOS, from_bool(value))
}

#[inline]
pub(crate) fn get_ip_recvtos(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::IPPROTO_IP, c::IP_RECVTOS).map(to_bool)
}

#[inline]
pub(crate) fn set_ipv6_recvtclass(fd: BorrowedFd<'_>, value: bool) -> io::Result<()> {
    setsockopt(fd, c::IPPROTO_IPV6, c::IPV6_RECVTCLASS, from_bool(value))
}

#[inline]
pub(crate) fn get_ipv6_recvtclass(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::IPPROTO_IPV6, c::IPV6_RECVTCLASS).map(to_bool)
}

#[inline]
pub(crate) fn set_tcp_nodelay(fd: BorrowedFd<'_>, nodelay: bool) -> io::Result<()> {
    setsockopt(fd, c::IPPROTO_TCP, c::TCP_NODELAY, from_bool(nodelay))
}

#[inline]
pub(crate) fn get_tcp_nodelay(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::IPPROTO_TCP, c::TCP_NODELAY).map(to_bool)
}

#[inline]
pub(crate) fn set_tcp_keepcnt(fd: BorrowedFd<'_>, count: u32) -> io::Result<()> {
    setsockopt(fd, c::IPPROTO_TCP, c::TCP_KEEPCNT, count)
}

#[inline]
pub(crate) fn get_tcp_keepcnt(fd: BorrowedFd<'_>) -> io::Result<u32> {
    getsockopt(fd, c::IPPROTO_TCP, c::TCP_KEEPCNT)
}

#[inline]
pub(crate) fn set_tcp_keepidle(fd: BorrowedFd<'_>, duration: Duration) -> io::Result<()> {
    let secs: c::c_uint = duration_to_secs(duration)?;
    setsockopt(fd, c::IPPROTO_TCP, c::TCP_KEEPIDLE, secs)
}

#[inline]
pub(crate) fn get_tcp_keepidle(fd: BorrowedFd<'_>) -> io::Result<Duration> {
    let secs: c::c_uint = getsockopt(fd, c::IPPROTO_TCP, c::TCP_KEEPIDLE)?;
    Ok(Duration::from_secs(secs as u64))
}

#[inline]
pub(crate) fn set_tcp_keepintvl(fd: BorrowedFd<'_>, duration: Duration) -> io::Result<()> {
    let secs: c::c_uint = duration_to_secs(duration)?;
    setsockopt(fd, c::IPPROTO_TCP, c::TCP_KEEPINTVL, secs)
}

#[inline]
pub(crate) fn get_tcp_keepintvl(fd: BorrowedFd<'_>) -> io::Result<Duration> {
    let secs: c::c_uint = getsockopt(fd, c::IPPROTO_TCP, c::TCP_KEEPINTVL)?;
    Ok(Duration::from_secs(secs as u64))
}

#[inline]
pub(crate) fn set_tcp_user_timeout(fd: BorrowedFd<'_>, value: u32) -> io::Result<()> {
    setsockopt(fd, c::IPPROTO_TCP, c::TCP_USER_TIMEOUT, value)
}

#[inline]
pub(crate) fn get_tcp_user_timeout(fd: BorrowedFd<'_>) -> io::Result<u32> {
    getsockopt(fd, c::IPPROTO_TCP, c::TCP_USER_TIMEOUT)
}

#[inline]
fn to_ip_mreq(multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> c::ip_mreq {
    c::ip_mreq {
        imr_multiaddr: to_imr_addr(multiaddr),
        imr_interface: to_imr_addr(interface),
    }
}

#[inline]
fn to_ip_mreqn(multiaddr: &Ipv4Addr, address: &Ipv4Addr, ifindex: i32) -> c::ip_mreqn {
    c::ip_mreqn {
        imr_multiaddr: to_imr_addr(multiaddr),
        imr_address: to_imr_addr(address),
        imr_ifindex: ifindex,
    }
}

#[inline]
fn to_imr_source(
    multiaddr: &Ipv4Addr,
    interface: &Ipv4Addr,
    sourceaddr: &Ipv4Addr,
) -> c::ip_mreq_source {
    c::ip_mreq_source {
        imr_multiaddr: to_imr_addr(multiaddr).s_addr,
        imr_interface: to_imr_addr(interface).s_addr,
        imr_sourceaddr: to_imr_addr(sourceaddr).s_addr,
    }
}

#[inline]
fn to_imr_addr(addr: &Ipv4Addr) -> c::in_addr {
    c::in_addr {
        s_addr: u32::from_ne_bytes(addr.octets()),
    }
}

#[inline]
fn to_ipv6mr(multiaddr: &Ipv6Addr, interface: u32) -> c::ipv6_mreq {
    c::ipv6_mreq {
        ipv6mr_multiaddr: to_ipv6mr_multiaddr(multiaddr),
        ipv6mr_ifindex: to_ipv6mr_interface(interface),
    }
}

#[inline]
fn to_ipv6mr_multiaddr(multiaddr: &Ipv6Addr) -> c::in6_addr {
    c::in6_addr {
        in6_u: linux_raw_sys::net::in6_addr__bindgen_ty_1 {
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
    c::c_uint::from(value)
}

#[inline]
fn to_bool(value: c::c_uint) -> bool {
    value != 0
}

/// Convert to seconds, rounding up if necessary.
#[inline]
fn duration_to_secs<T: TryFrom<u64>>(duration: Duration) -> io::Result<T> {
    let mut secs = duration.as_secs();
    if duration.subsec_nanos() != 0 {
        secs = secs.checked_add(1).ok_or(io::Errno::INVAL)?;
    }
    T::try_from(secs).map_err(|_e| io::Errno::INVAL)
}
