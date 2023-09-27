//! libc syscalls supporting `rustix::net::sockopt`.

use super::ext::{in6_addr_new, in_addr_new};
use crate::backend::c;
use crate::backend::conv::{borrowed_fd, ret};
use crate::fd::BorrowedFd;
use crate::io;
use crate::net::sockopt::Timeout;
#[cfg(not(any(
    apple,
    windows,
    target_os = "aix",
    target_os = "dragonfly",
    target_os = "emscripten",
    target_os = "espidf",
    target_os = "haiku",
    target_os = "netbsd",
    target_os = "nto",
)))]
use crate::net::AddressFamily;
use crate::net::{Ipv4Addr, Ipv6Addr, SocketType};
use crate::utils::{as_mut_ptr, as_ptr};
#[cfg(apple)]
use c::TCP_KEEPALIVE as TCP_KEEPIDLE;
#[cfg(not(any(apple, target_os = "openbsd", target_os = "haiku", target_os = "nto")))]
use c::TCP_KEEPIDLE;
use core::mem::size_of;
use core::time::Duration;
#[cfg(windows)]
use windows_sys::Win32::Foundation::BOOL;

#[inline]
fn getsockopt<T: Copy>(fd: BorrowedFd<'_>, level: i32, optname: i32) -> io::Result<T> {
    let mut optlen = core::mem::size_of::<T>().try_into().unwrap();
    debug_assert!(
        optlen as usize >= core::mem::size_of::<c::c_int>(),
        "Socket APIs don't ever use `bool` directly"
    );

    unsafe {
        let mut value = core::mem::zeroed::<T>();
        ret(c::getsockopt(
            borrowed_fd(fd),
            level,
            optname,
            as_mut_ptr(&mut value).cast(),
            &mut optlen,
        ))?;
        // On Windows at least, `getsockopt` has been observed writing 1
        // byte on at least (`IPPROTO_TCP`, `TCP_NODELAY`), even though
        // Windows' documentation says that should write a 4-byte `BOOL`.
        // So, we initialize the memory to zeros above, and just assert
        // that `getsockopt` doesn't write too many bytes here.
        assert!(
            optlen as usize <= size_of::<T>(),
            "unexpected getsockopt size"
        );
        Ok(value)
    }
}

#[inline]
fn setsockopt<T: Copy>(fd: BorrowedFd<'_>, level: i32, optname: i32, value: T) -> io::Result<()> {
    let optlen = core::mem::size_of::<T>().try_into().unwrap();
    debug_assert!(
        optlen as usize >= core::mem::size_of::<c::c_int>(),
        "Socket APIs don't ever use `bool` directly"
    );

    unsafe {
        ret(c::setsockopt(
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
    getsockopt(fd, c::SOL_SOCKET as _, c::SO_TYPE)
}

#[inline]
pub(crate) fn set_socket_reuseaddr(fd: BorrowedFd<'_>, reuseaddr: bool) -> io::Result<()> {
    setsockopt(
        fd,
        c::SOL_SOCKET as _,
        c::SO_REUSEADDR,
        from_bool(reuseaddr),
    )
}

#[inline]
pub(crate) fn get_socket_reuseaddr(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::SOL_SOCKET as _, c::SO_REUSEADDR).map(to_bool)
}

#[inline]
pub(crate) fn set_socket_broadcast(fd: BorrowedFd<'_>, broadcast: bool) -> io::Result<()> {
    setsockopt(
        fd,
        c::SOL_SOCKET as _,
        c::SO_BROADCAST,
        from_bool(broadcast),
    )
}

#[inline]
pub(crate) fn get_socket_broadcast(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::SOL_SOCKET as _, c::SO_BROADCAST).map(to_bool)
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
        l_onoff: linger.is_some() as _,
        l_linger,
    };
    setsockopt(fd, c::SOL_SOCKET as _, c::SO_LINGER, linger)
}

#[inline]
pub(crate) fn get_socket_linger(fd: BorrowedFd<'_>) -> io::Result<Option<Duration>> {
    let linger: c::linger = getsockopt(fd, c::SOL_SOCKET as _, c::SO_LINGER)?;
    Ok((linger.l_onoff != 0).then(|| Duration::from_secs(linger.l_linger as u64)))
}

#[cfg(linux_kernel)]
#[inline]
pub(crate) fn set_socket_passcred(fd: BorrowedFd<'_>, passcred: bool) -> io::Result<()> {
    setsockopt(fd, c::SOL_SOCKET as _, c::SO_PASSCRED, from_bool(passcred))
}

#[cfg(linux_kernel)]
#[inline]
pub(crate) fn get_socket_passcred(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::SOL_SOCKET as _, c::SO_PASSCRED).map(to_bool)
}

#[inline]
pub(crate) fn set_socket_timeout(
    fd: BorrowedFd<'_>,
    id: Timeout,
    timeout: Option<Duration>,
) -> io::Result<()> {
    let optname = match id {
        Timeout::Recv => c::SO_RCVTIMEO,
        Timeout::Send => c::SO_SNDTIMEO,
    };

    #[cfg(not(windows))]
    let timeout = match timeout {
        Some(timeout) => {
            if timeout == Duration::ZERO {
                return Err(io::Errno::INVAL);
            }

            // Rust's musl libc bindings deprecated `time_t` while they
            // transition to 64-bit `time_t`. What we want here is just
            // “whatever type `timeval`'s `tv_sec` is”, so we're ok using
            // the deprecated type.
            #[allow(deprecated)]
            let tv_sec = timeout.as_secs().try_into().unwrap_or(c::time_t::MAX);

            // `subsec_micros` rounds down, so we use `subsec_nanos` and
            // manually round up.
            let mut timeout = c::timeval {
                tv_sec,
                tv_usec: ((timeout.subsec_nanos() + 999) / 1000) as _,
            };
            if timeout.tv_sec == 0 && timeout.tv_usec == 0 {
                timeout.tv_usec = 1;
            }
            timeout
        }
        None => c::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
    };

    #[cfg(windows)]
    let timeout: u32 = match timeout {
        Some(timeout) => {
            if timeout == Duration::ZERO {
                return Err(io::Errno::INVAL);
            }

            // `as_millis` rounds down, so we use `as_nanos` and
            // manually round up.
            let mut timeout: u32 = ((timeout.as_nanos() + 999_999) / 1_000_000)
                .try_into()
                .map_err(|_convert_err| io::Errno::INVAL)?;
            if timeout == 0 {
                timeout = 1;
            }
            timeout
        }
        None => 0,
    };

    setsockopt(fd, c::SOL_SOCKET, optname, timeout)
}

#[inline]
pub(crate) fn get_socket_timeout(fd: BorrowedFd<'_>, id: Timeout) -> io::Result<Option<Duration>> {
    let optname = match id {
        Timeout::Recv => c::SO_RCVTIMEO,
        Timeout::Send => c::SO_SNDTIMEO,
    };

    #[cfg(not(windows))]
    {
        let timeout: c::timeval = getsockopt(fd, c::SOL_SOCKET, optname)?;
        if timeout.tv_sec == 0 && timeout.tv_usec == 0 {
            Ok(None)
        } else {
            Ok(Some(
                Duration::from_secs(timeout.tv_sec as u64)
                    + Duration::from_micros(timeout.tv_usec as u64),
            ))
        }
    }

    #[cfg(windows)]
    {
        let timeout: u32 = getsockopt(fd, c::SOL_SOCKET, optname)?;
        if timeout == 0 {
            Ok(None)
        } else {
            Ok(Some(Duration::from_millis(timeout as u64)))
        }
    }
}

#[cfg(any(apple, target_os = "freebsd"))]
#[inline]
pub(crate) fn get_socket_nosigpipe(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::SOL_SOCKET, c::SO_NOSIGPIPE).map(to_bool)
}

#[cfg(any(apple, target_os = "freebsd"))]
#[inline]
pub(crate) fn set_socket_nosigpipe(fd: BorrowedFd<'_>, val: bool) -> io::Result<()> {
    setsockopt(fd, c::SOL_SOCKET, c::SO_NOSIGPIPE, from_bool(val))
}

#[inline]
pub(crate) fn get_socket_error(fd: BorrowedFd<'_>) -> io::Result<Result<(), io::Errno>> {
    let err: c::c_int = getsockopt(fd, c::SOL_SOCKET as _, c::SO_ERROR)?;
    Ok(if err == 0 {
        Ok(())
    } else {
        Err(io::Errno::from_raw_os_error(err))
    })
}

#[inline]
pub(crate) fn set_socket_keepalive(fd: BorrowedFd<'_>, keepalive: bool) -> io::Result<()> {
    setsockopt(
        fd,
        c::SOL_SOCKET as _,
        c::SO_KEEPALIVE,
        from_bool(keepalive),
    )
}

#[inline]
pub(crate) fn get_socket_keepalive(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::SOL_SOCKET as _, c::SO_KEEPALIVE).map(to_bool)
}

#[inline]
pub(crate) fn set_socket_recv_buffer_size(fd: BorrowedFd<'_>, size: usize) -> io::Result<()> {
    let size: c::c_int = size.try_into().map_err(|_| io::Errno::INVAL)?;
    setsockopt(fd, c::SOL_SOCKET as _, c::SO_RCVBUF, size)
}

#[inline]
pub(crate) fn get_socket_recv_buffer_size(fd: BorrowedFd<'_>) -> io::Result<usize> {
    getsockopt(fd, c::SOL_SOCKET as _, c::SO_RCVBUF).map(|size: u32| size as usize)
}

#[inline]
pub(crate) fn set_socket_send_buffer_size(fd: BorrowedFd<'_>, size: usize) -> io::Result<()> {
    let size: c::c_int = size.try_into().map_err(|_| io::Errno::INVAL)?;
    setsockopt(fd, c::SOL_SOCKET as _, c::SO_SNDBUF, size)
}

#[inline]
pub(crate) fn get_socket_send_buffer_size(fd: BorrowedFd<'_>) -> io::Result<usize> {
    getsockopt(fd, c::SOL_SOCKET as _, c::SO_SNDBUF).map(|size: u32| size as usize)
}

#[inline]
#[cfg(not(any(
    apple,
    windows,
    target_os = "aix",
    target_os = "dragonfly",
    target_os = "emscripten",
    target_os = "espidf",
    target_os = "haiku",
    target_os = "netbsd",
    target_os = "nto",
)))]
pub(crate) fn get_socket_domain(fd: BorrowedFd<'_>) -> io::Result<AddressFamily> {
    let domain: c::c_int = getsockopt(fd, c::SOL_SOCKET as _, c::SO_DOMAIN)?;
    Ok(AddressFamily(
        domain.try_into().map_err(|_| io::Errno::OPNOTSUPP)?,
    ))
}

#[inline]
#[cfg(not(apple))] // Apple platforms declare the constant, but do not actually implement it.
pub(crate) fn get_socket_acceptconn(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::SOL_SOCKET as _, c::SO_ACCEPTCONN).map(to_bool)
}

#[inline]
pub(crate) fn set_socket_oobinline(fd: BorrowedFd<'_>, value: bool) -> io::Result<()> {
    setsockopt(fd, c::SOL_SOCKET as _, c::SO_OOBINLINE, from_bool(value))
}

#[inline]
pub(crate) fn get_socket_oobinline(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::SOL_SOCKET as _, c::SO_OOBINLINE).map(to_bool)
}

#[inline]
pub(crate) fn set_ip_ttl(fd: BorrowedFd<'_>, ttl: u32) -> io::Result<()> {
    setsockopt(fd, c::IPPROTO_IP as _, c::IP_TTL, ttl)
}

#[inline]
pub(crate) fn get_ip_ttl(fd: BorrowedFd<'_>) -> io::Result<u32> {
    getsockopt(fd, c::IPPROTO_IP as _, c::IP_TTL)
}

#[inline]
pub(crate) fn set_ipv6_v6only(fd: BorrowedFd<'_>, only_v6: bool) -> io::Result<()> {
    setsockopt(fd, c::IPPROTO_IPV6 as _, c::IPV6_V6ONLY, from_bool(only_v6))
}

#[inline]
pub(crate) fn get_ipv6_v6only(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::IPPROTO_IPV6 as _, c::IPV6_V6ONLY).map(to_bool)
}

#[inline]
pub(crate) fn set_ip_multicast_loop(fd: BorrowedFd<'_>, multicast_loop: bool) -> io::Result<()> {
    setsockopt(
        fd,
        c::IPPROTO_IP as _,
        c::IP_MULTICAST_LOOP,
        from_bool(multicast_loop),
    )
}

#[inline]
pub(crate) fn get_ip_multicast_loop(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::IPPROTO_IP as _, c::IP_MULTICAST_LOOP).map(to_bool)
}

#[inline]
pub(crate) fn set_ip_multicast_ttl(fd: BorrowedFd<'_>, multicast_ttl: u32) -> io::Result<()> {
    setsockopt(fd, c::IPPROTO_IP as _, c::IP_MULTICAST_TTL, multicast_ttl)
}

#[inline]
pub(crate) fn get_ip_multicast_ttl(fd: BorrowedFd<'_>) -> io::Result<u32> {
    getsockopt(fd, c::IPPROTO_IP as _, c::IP_MULTICAST_TTL)
}

#[inline]
pub(crate) fn set_ipv6_multicast_loop(fd: BorrowedFd<'_>, multicast_loop: bool) -> io::Result<()> {
    setsockopt(
        fd,
        c::IPPROTO_IPV6 as _,
        c::IPV6_MULTICAST_LOOP,
        from_bool(multicast_loop),
    )
}

#[inline]
pub(crate) fn get_ipv6_multicast_loop(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::IPPROTO_IPV6 as _, c::IPV6_MULTICAST_LOOP).map(to_bool)
}

#[inline]
pub(crate) fn set_ipv6_multicast_hops(fd: BorrowedFd<'_>, multicast_hops: u32) -> io::Result<()> {
    setsockopt(
        fd,
        c::IPPROTO_IP as _,
        c::IPV6_MULTICAST_HOPS,
        multicast_hops,
    )
}

#[inline]
pub(crate) fn get_ipv6_multicast_hops(fd: BorrowedFd<'_>) -> io::Result<u32> {
    getsockopt(fd, c::IPPROTO_IP as _, c::IPV6_MULTICAST_HOPS)
}

#[inline]
pub(crate) fn set_ip_add_membership(
    fd: BorrowedFd<'_>,
    multiaddr: &Ipv4Addr,
    interface: &Ipv4Addr,
) -> io::Result<()> {
    let mreq = to_imr(multiaddr, interface);
    setsockopt(fd, c::IPPROTO_IP as _, c::IP_ADD_MEMBERSHIP, mreq)
}

#[inline]
pub(crate) fn set_ip_add_source_membership(
    fd: BorrowedFd<'_>,
    multiaddr: &Ipv4Addr,
    interface: &Ipv4Addr,
    sourceaddr: &Ipv4Addr,
) -> io::Result<()> {
    let mreq_source = to_imr_source(multiaddr, interface, sourceaddr);
    setsockopt(
        fd,
        c::IPPROTO_IP as _,
        c::IP_ADD_SOURCE_MEMBERSHIP,
        mreq_source,
    )
}

#[inline]
pub(crate) fn set_ipv6_add_membership(
    fd: BorrowedFd<'_>,
    multiaddr: &Ipv6Addr,
    interface: u32,
) -> io::Result<()> {
    #[cfg(not(any(
        bsd,
        solarish,
        target_os = "haiku",
        target_os = "l4re",
        target_os = "nto"
    )))]
    use c::IPV6_ADD_MEMBERSHIP;
    #[cfg(any(
        bsd,
        solarish,
        target_os = "haiku",
        target_os = "l4re",
        target_os = "nto"
    ))]
    use c::IPV6_JOIN_GROUP as IPV6_ADD_MEMBERSHIP;

    let mreq = to_ipv6mr(multiaddr, interface);
    setsockopt(fd, c::IPPROTO_IPV6 as _, IPV6_ADD_MEMBERSHIP, mreq)
}

#[inline]
pub(crate) fn set_ip_drop_membership(
    fd: BorrowedFd<'_>,
    multiaddr: &Ipv4Addr,
    interface: &Ipv4Addr,
) -> io::Result<()> {
    let mreq = to_imr(multiaddr, interface);
    setsockopt(fd, c::IPPROTO_IP as _, c::IP_DROP_MEMBERSHIP, mreq)
}

#[inline]
pub(crate) fn set_ipv6_drop_membership(
    fd: BorrowedFd<'_>,
    multiaddr: &Ipv6Addr,
    interface: u32,
) -> io::Result<()> {
    #[cfg(not(any(
        bsd,
        solarish,
        target_os = "haiku",
        target_os = "l4re",
        target_os = "nto"
    )))]
    use c::IPV6_DROP_MEMBERSHIP;
    #[cfg(any(
        bsd,
        solarish,
        target_os = "haiku",
        target_os = "l4re",
        target_os = "nto"
    ))]
    use c::IPV6_LEAVE_GROUP as IPV6_DROP_MEMBERSHIP;

    let mreq = to_ipv6mr(multiaddr, interface);
    setsockopt(fd, c::IPPROTO_IPV6 as _, IPV6_DROP_MEMBERSHIP, mreq)
}

#[inline]
pub(crate) fn get_ipv6_unicast_hops(fd: BorrowedFd<'_>) -> io::Result<u8> {
    getsockopt(fd, c::IPPROTO_IPV6 as _, c::IPV6_UNICAST_HOPS).map(|hops: c::c_int| hops as u8)
}

#[inline]
pub(crate) fn set_ipv6_unicast_hops(fd: BorrowedFd<'_>, hops: Option<u8>) -> io::Result<()> {
    let hops = match hops {
        Some(hops) => hops as c::c_int,
        None => -1,
    };
    setsockopt(fd, c::IPPROTO_IPV6 as _, c::IPV6_UNICAST_HOPS, hops)
}

#[inline]
pub(crate) fn set_ip_tos(fd: BorrowedFd<'_>, value: u8) -> io::Result<()> {
    setsockopt(fd, c::IPPROTO_IP as _, c::IP_TOS, value)
}

#[inline]
pub(crate) fn get_ip_tos(fd: BorrowedFd<'_>) -> io::Result<u8> {
    getsockopt(fd, c::IPPROTO_IP as _, c::IP_TOS)
}

#[inline]
pub(crate) fn set_ip_recvtos(fd: BorrowedFd<'_>, value: bool) -> io::Result<()> {
    setsockopt(fd, c::IPPROTO_IP as _, c::IP_RECVTOS, value)
}

#[inline]
pub(crate) fn get_ip_recvtos(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::IPPROTO_IP as _, c::IP_RECVTOS).map(|value: i32| value > 0)
}

#[inline]
pub(crate) fn set_ipv6_recvtclass(fd: BorrowedFd<'_>, value: bool) -> io::Result<()> {
    setsockopt(fd, c::IPPROTO_IPV6 as _, c::IPV6_RECVTCLASS, value)
}

#[inline]
pub(crate) fn get_ipv6_recvtclass(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::IPPROTO_IPV6 as _, c::IPV6_RECVTCLASS).map(|value: i32| value > 0)
}

#[inline]
pub(crate) fn set_tcp_nodelay(fd: BorrowedFd<'_>, nodelay: bool) -> io::Result<()> {
    setsockopt(fd, c::IPPROTO_TCP as _, c::TCP_NODELAY, from_bool(nodelay))
}

#[inline]
pub(crate) fn get_tcp_nodelay(fd: BorrowedFd<'_>) -> io::Result<bool> {
    getsockopt(fd, c::IPPROTO_TCP as _, c::TCP_NODELAY).map(to_bool)
}

#[inline]
#[cfg(not(any(target_os = "openbsd", target_os = "haiku", target_os = "nto")))]
pub(crate) fn set_tcp_keepcnt(fd: BorrowedFd<'_>, count: u32) -> io::Result<()> {
    setsockopt(fd, c::IPPROTO_TCP as _, c::TCP_KEEPCNT, count)
}

#[inline]
#[cfg(not(any(target_os = "openbsd", target_os = "haiku", target_os = "nto")))]
pub(crate) fn get_tcp_keepcnt(fd: BorrowedFd<'_>) -> io::Result<u32> {
    getsockopt(fd, c::IPPROTO_TCP as _, c::TCP_KEEPCNT)
}

#[inline]
#[cfg(not(any(target_os = "openbsd", target_os = "haiku", target_os = "nto")))]
pub(crate) fn set_tcp_keepidle(fd: BorrowedFd<'_>, duration: Duration) -> io::Result<()> {
    let secs: c::c_uint = duration_to_secs(duration)?;
    setsockopt(fd, c::IPPROTO_TCP as _, TCP_KEEPIDLE, secs)
}

#[inline]
#[cfg(not(any(target_os = "openbsd", target_os = "haiku", target_os = "nto")))]
pub(crate) fn get_tcp_keepidle(fd: BorrowedFd<'_>) -> io::Result<Duration> {
    let secs: c::c_uint = getsockopt(fd, c::IPPROTO_TCP as _, TCP_KEEPIDLE)?;
    Ok(Duration::from_secs(secs as u64))
}

#[inline]
#[cfg(not(any(target_os = "openbsd", target_os = "haiku", target_os = "nto")))]
pub(crate) fn set_tcp_keepintvl(fd: BorrowedFd<'_>, duration: Duration) -> io::Result<()> {
    let secs: c::c_uint = duration_to_secs(duration)?;
    setsockopt(fd, c::IPPROTO_TCP as _, c::TCP_KEEPINTVL, secs)
}

#[inline]
#[cfg(not(any(target_os = "openbsd", target_os = "haiku", target_os = "nto")))]
pub(crate) fn get_tcp_keepintvl(fd: BorrowedFd<'_>) -> io::Result<Duration> {
    let secs: c::c_uint = getsockopt(fd, c::IPPROTO_TCP as _, c::TCP_KEEPINTVL)?;
    Ok(Duration::from_secs(secs as u64))
}

#[inline]
#[cfg(any(linux_like, target_os = "fuchsia"))]
pub(crate) fn set_tcp_user_timeout(fd: BorrowedFd<'_>, value: u32) -> io::Result<()> {
    setsockopt(fd, c::IPPROTO_TCP as _, c::TCP_USER_TIMEOUT, value)
}

#[inline]
#[cfg(any(linux_like, target_os = "fuchsia"))]
pub(crate) fn get_tcp_user_timeout(fd: BorrowedFd<'_>) -> io::Result<u32> {
    getsockopt(fd, c::IPPROTO_TCP as _, c::TCP_USER_TIMEOUT)
}

#[inline]
fn to_imr(multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> c::ip_mreq {
    c::ip_mreq {
        imr_multiaddr: to_imr_addr(multiaddr),
        imr_interface: to_imr_addr(interface),
    }
}

#[inline]
fn to_imr_source(
    multiaddr: &Ipv4Addr,
    interface: &Ipv4Addr,
    sourceaddr: &Ipv4Addr,
) -> c::ip_mreq_source {
    c::ip_mreq_source {
        imr_multiaddr: to_imr_addr(multiaddr),
        imr_interface: to_imr_addr(interface),
        imr_sourceaddr: to_imr_addr(sourceaddr),
    }
}

#[inline]
fn to_imr_addr(addr: &Ipv4Addr) -> c::in_addr {
    in_addr_new(u32::from_ne_bytes(addr.octets()))
}

#[inline]
fn to_ipv6mr(multiaddr: &Ipv6Addr, interface: u32) -> c::ipv6_mreq {
    c::ipv6_mreq {
        ipv6mr_multiaddr: to_ipv6mr_multiaddr(multiaddr),
        ipv6mr_interface: to_ipv6mr_interface(interface),
    }
}

#[inline]
fn to_ipv6mr_multiaddr(multiaddr: &Ipv6Addr) -> c::in6_addr {
    in6_addr_new(multiaddr.octets())
}

#[cfg(target_os = "android")]
#[inline]
fn to_ipv6mr_interface(interface: u32) -> c::c_int {
    interface as c::c_int
}

#[cfg(not(target_os = "android"))]
#[inline]
fn to_ipv6mr_interface(interface: u32) -> c::c_uint {
    interface as c::c_uint
}

// `getsockopt` and `setsockopt` represent boolean values as integers.
#[cfg(not(windows))]
type RawSocketBool = c::c_int;
#[cfg(windows)]
type RawSocketBool = BOOL;

// Wrap `RawSocketBool` in a newtype to discourage misuse.
#[repr(transparent)]
#[derive(Copy, Clone)]
struct SocketBool(RawSocketBool);

// Convert from a `bool` to a `SocketBool`.
#[inline]
fn from_bool(value: bool) -> SocketBool {
    SocketBool(value as _)
}

// Convert from a `SocketBool` to a `bool`.
#[inline]
fn to_bool(value: SocketBool) -> bool {
    value.0 != 0
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
