//! Adapt the Linux API to resemble a POSIX-style libc API.
//!
//! The linux_raw backend doesn't use actual libc; this just defines certain
//! types that are convenient to have defined.

#![allow(unused_imports)]
#![allow(non_camel_case_types)]

pub(crate) use linux_raw_sys::cmsg_macros::*;
pub(crate) use linux_raw_sys::ctypes::*;
pub(crate) use linux_raw_sys::errno::EINVAL;
pub(crate) use linux_raw_sys::general::__kernel_pid_t as pid_t;
pub(crate) use linux_raw_sys::general::__kernel_time64_t as time_t;
pub(crate) use linux_raw_sys::general::__kernel_timespec as timespec;
pub(crate) use linux_raw_sys::general::{
    AF_DECnet, __kernel_sa_family_t as sa_family_t, __kernel_sockaddr_storage as sockaddr_storage,
    cmsghdr, in6_addr, in_addr, iovec, ip_mreq, ipv6_mreq, linger, msghdr, siginfo_t, size_t,
    sockaddr, sockaddr_in, sockaddr_in6, sockaddr_un, socklen_t, AF_APPLETALK, AF_ASH, AF_ATMPVC,
    AF_ATMSVC, AF_AX25, AF_BLUETOOTH, AF_BRIDGE, AF_CAN, AF_ECONET, AF_IEEE802154, AF_INET,
    AF_INET6, AF_IPX, AF_IRDA, AF_ISDN, AF_IUCV, AF_KEY, AF_LLC, AF_NETBEUI, AF_NETLINK, AF_NETROM,
    AF_PACKET, AF_PHONET, AF_PPPOX, AF_RDS, AF_ROSE, AF_RXRPC, AF_SECURITY, AF_SNA, AF_TIPC,
    AF_UNIX, AF_UNSPEC, AF_WANPIPE, AF_X25, CLD_CONTINUED, CLD_DUMPED, CLD_EXITED, CLD_KILLED,
    CLD_STOPPED, CLD_TRAPPED, IPPROTO_AH, IPPROTO_BEETPH, IPPROTO_COMP, IPPROTO_DCCP, IPPROTO_EGP,
    IPPROTO_ENCAP, IPPROTO_ESP, IPPROTO_ETHERNET, IPPROTO_FRAGMENT, IPPROTO_GRE, IPPROTO_ICMP,
    IPPROTO_ICMPV6, IPPROTO_IDP, IPPROTO_IGMP, IPPROTO_IP, IPPROTO_IPIP, IPPROTO_IPV6, IPPROTO_MH,
    IPPROTO_MPLS, IPPROTO_MPTCP, IPPROTO_MTP, IPPROTO_PIM, IPPROTO_PUP, IPPROTO_RAW,
    IPPROTO_ROUTING, IPPROTO_RSVP, IPPROTO_SCTP, IPPROTO_TCP, IPPROTO_TP, IPPROTO_UDP,
    IPPROTO_UDPLITE, IPV6_ADD_MEMBERSHIP, IPV6_DROP_MEMBERSHIP, IPV6_MULTICAST_HOPS,
    IPV6_MULTICAST_LOOP, IPV6_UNICAST_HOPS, IPV6_V6ONLY, IP_ADD_MEMBERSHIP, IP_DROP_MEMBERSHIP,
    IP_MULTICAST_LOOP, IP_MULTICAST_TTL, IP_TTL, MSG_CMSG_CLOEXEC, MSG_CONFIRM, MSG_DONTROUTE,
    MSG_DONTWAIT, MSG_EOR, MSG_ERRQUEUE, MSG_MORE, MSG_NOSIGNAL, MSG_OOB, MSG_PEEK, MSG_TRUNC,
    MSG_WAITALL, O_CLOEXEC, O_NONBLOCK, O_NONBLOCK as PIDFD_NONBLOCK, P_ALL, P_PID, P_PIDFD,
    SCM_CREDENTIALS, SCM_RIGHTS, SHUT_RD, SHUT_RDWR, SHUT_WR, SOCK_DGRAM, SOCK_RAW, SOCK_RDM,
    SOCK_SEQPACKET, SOCK_STREAM, SOL_SOCKET, SO_BROADCAST, SO_ERROR, SO_KEEPALIVE, SO_LINGER,
    SO_PASSCRED, SO_RCVBUF, SO_RCVTIMEO_NEW, SO_RCVTIMEO_OLD, SO_REUSEADDR, SO_SNDBUF,
    SO_SNDTIMEO_NEW, SO_SNDTIMEO_OLD, SO_TYPE, TCP_NODELAY,
};
#[cfg(not(any(target_arch = "arm", target_arch = "sparc", target_arch = "x86")))]
pub(crate) use linux_raw_sys::general::{__kernel_gid_t as gid_t, __kernel_uid_t as uid_t};
pub(crate) use linux_raw_sys::general::{AT_FDCWD, O_NOCTTY, O_RDWR};
pub(crate) use linux_raw_sys::general::{NFS_SUPER_MAGIC, PROC_SUPER_MAGIC, UTIME_NOW, UTIME_OMIT};
pub(crate) use linux_raw_sys::general::{XATTR_CREATE, XATTR_REPLACE};
#[cfg(any(target_arch = "arm", target_arch = "sparc", target_arch = "x86"))]
pub(crate) type uid_t = u32;
#[cfg(any(target_arch = "arm", target_arch = "sparc", target_arch = "x86"))]
pub(crate) type gid_t = u32;

// Bindgen infers `u32` for many of these macro types which meant to be
// used with `c_int` in the C APIs, so cast them to `c_int`.

// Convert the signal constants from `u32` to `c_int`.
pub(crate) const SIGHUP: c_int = linux_raw_sys::general::SIGHUP as _;
pub(crate) const SIGINT: c_int = linux_raw_sys::general::SIGINT as _;
pub(crate) const SIGQUIT: c_int = linux_raw_sys::general::SIGQUIT as _;
pub(crate) const SIGILL: c_int = linux_raw_sys::general::SIGILL as _;
pub(crate) const SIGTRAP: c_int = linux_raw_sys::general::SIGTRAP as _;
pub(crate) const SIGABRT: c_int = linux_raw_sys::general::SIGABRT as _;
pub(crate) const SIGBUS: c_int = linux_raw_sys::general::SIGBUS as _;
pub(crate) const SIGFPE: c_int = linux_raw_sys::general::SIGFPE as _;
pub(crate) const SIGKILL: c_int = linux_raw_sys::general::SIGKILL as _;
pub(crate) const SIGUSR1: c_int = linux_raw_sys::general::SIGUSR1 as _;
pub(crate) const SIGSEGV: c_int = linux_raw_sys::general::SIGSEGV as _;
pub(crate) const SIGUSR2: c_int = linux_raw_sys::general::SIGUSR2 as _;
pub(crate) const SIGPIPE: c_int = linux_raw_sys::general::SIGPIPE as _;
pub(crate) const SIGALRM: c_int = linux_raw_sys::general::SIGALRM as _;
pub(crate) const SIGTERM: c_int = linux_raw_sys::general::SIGTERM as _;
#[cfg(not(any(
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "sparc",
    target_arch = "sparc64"
)))]
pub(crate) const SIGSTKFLT: c_int = linux_raw_sys::general::SIGSTKFLT as _;
pub(crate) const SIGCHLD: c_int = linux_raw_sys::general::SIGCHLD as _;
pub(crate) const SIGCONT: c_int = linux_raw_sys::general::SIGCONT as _;
pub(crate) const SIGSTOP: c_int = linux_raw_sys::general::SIGSTOP as _;
pub(crate) const SIGTSTP: c_int = linux_raw_sys::general::SIGTSTP as _;
pub(crate) const SIGTTIN: c_int = linux_raw_sys::general::SIGTTIN as _;
pub(crate) const SIGTTOU: c_int = linux_raw_sys::general::SIGTTOU as _;
pub(crate) const SIGURG: c_int = linux_raw_sys::general::SIGURG as _;
pub(crate) const SIGXCPU: c_int = linux_raw_sys::general::SIGXCPU as _;
pub(crate) const SIGXFSZ: c_int = linux_raw_sys::general::SIGXFSZ as _;
pub(crate) const SIGVTALRM: c_int = linux_raw_sys::general::SIGVTALRM as _;
pub(crate) const SIGPROF: c_int = linux_raw_sys::general::SIGPROF as _;
pub(crate) const SIGWINCH: c_int = linux_raw_sys::general::SIGWINCH as _;
pub(crate) const SIGIO: c_int = linux_raw_sys::general::SIGIO as _;
pub(crate) const SIGPWR: c_int = linux_raw_sys::general::SIGPWR as _;
pub(crate) const SIGSYS: c_int = linux_raw_sys::general::SIGSYS as _;
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "sparc",
    target_arch = "sparc64"
))]
pub(crate) const SIGEMT: c_int = linux_raw_sys::general::SIGEMT as _;

pub(crate) const STDIN_FILENO: c_int = linux_raw_sys::general::STDIN_FILENO as _;
pub(crate) const STDOUT_FILENO: c_int = linux_raw_sys::general::STDOUT_FILENO as _;
pub(crate) const STDERR_FILENO: c_int = linux_raw_sys::general::STDERR_FILENO as _;
pub(crate) const PIPE_BUF: usize = linux_raw_sys::general::PIPE_BUF as _;

pub(crate) const CLOCK_MONOTONIC: c_int = linux_raw_sys::general::CLOCK_MONOTONIC as _;
pub(crate) const CLOCK_REALTIME: c_int = linux_raw_sys::general::CLOCK_REALTIME as _;
pub(crate) const CLOCK_MONOTONIC_RAW: c_int = linux_raw_sys::general::CLOCK_MONOTONIC_RAW as _;
pub(crate) const CLOCK_MONOTONIC_COARSE: c_int =
    linux_raw_sys::general::CLOCK_MONOTONIC_COARSE as _;
pub(crate) const CLOCK_REALTIME_COARSE: c_int = linux_raw_sys::general::CLOCK_REALTIME_COARSE as _;
pub(crate) const CLOCK_THREAD_CPUTIME_ID: c_int =
    linux_raw_sys::general::CLOCK_THREAD_CPUTIME_ID as _;
pub(crate) const CLOCK_PROCESS_CPUTIME_ID: c_int =
    linux_raw_sys::general::CLOCK_PROCESS_CPUTIME_ID as _;
