//! Adapt the Winsock API to resemble a POSIX-style libc API.

#![allow(unused_imports)]

pub(crate) use winapi::shared::ws2def::AF_DECnet;
pub(crate) use winapi::shared::ws2def::ADDRESS_FAMILY as sa_family_t;
pub(crate) use winapi::shared::ws2def::ADDRINFOA as addrinfo;
pub(crate) use winapi::shared::ws2def::AF_APPLETALK;
pub(crate) use winapi::shared::ws2def::AF_INET;
pub(crate) use winapi::shared::ws2def::AF_INET6;
pub(crate) use winapi::shared::ws2def::AF_IPX;
pub(crate) use winapi::shared::ws2def::AF_IRDA;
pub(crate) use winapi::shared::ws2def::AF_SNA;
pub(crate) use winapi::shared::ws2def::AF_UNIX;
pub(crate) use winapi::shared::ws2def::AF_UNSPEC;
pub(crate) use winapi::shared::ws2def::IPPROTO_AH;
pub(crate) use winapi::shared::ws2def::IPPROTO_EGP;
pub(crate) use winapi::shared::ws2def::IPPROTO_ESP;
pub(crate) use winapi::shared::ws2def::IPPROTO_FRAGMENT;
pub(crate) use winapi::shared::ws2def::IPPROTO_ICMP;
pub(crate) use winapi::shared::ws2def::IPPROTO_ICMPV6;
pub(crate) use winapi::shared::ws2def::IPPROTO_IDP;
pub(crate) use winapi::shared::ws2def::IPPROTO_IGMP;
pub(crate) use winapi::shared::ws2def::IPPROTO_IP;
pub(crate) use winapi::shared::ws2def::IPPROTO_IPV6;
pub(crate) use winapi::shared::ws2def::IPPROTO_PIM;
pub(crate) use winapi::shared::ws2def::IPPROTO_PUP;
pub(crate) use winapi::shared::ws2def::IPPROTO_RAW;
pub(crate) use winapi::shared::ws2def::IPPROTO_ROUTING;
pub(crate) use winapi::shared::ws2def::IPPROTO_SCTP;
pub(crate) use winapi::shared::ws2def::IPPROTO_TCP;
pub(crate) use winapi::shared::ws2def::IPPROTO_UDP;
pub(crate) use winapi::shared::ws2def::MSG_TRUNC;
pub(crate) use winapi::shared::ws2def::SOCKADDR as sockaddr;
pub(crate) use winapi::shared::ws2def::SOCKADDR_IN as sockaddr_in;
pub(crate) use winapi::shared::ws2def::SOCKADDR_STORAGE_LH as sockaddr_storage;
pub(crate) use winapi::shared::ws2def::TCP_NODELAY;

pub(crate) use winapi::shared::ws2ipdef::IPV6_ADD_MEMBERSHIP;
pub(crate) use winapi::shared::ws2ipdef::IPV6_DROP_MEMBERSHIP;
pub(crate) use winapi::shared::ws2ipdef::IPV6_MREQ as ipv6_mreq;
pub(crate) use winapi::shared::ws2ipdef::IPV6_MULTICAST_LOOP;
pub(crate) use winapi::shared::ws2ipdef::IPV6_V6ONLY;
pub(crate) use winapi::shared::ws2ipdef::IP_ADD_MEMBERSHIP;
pub(crate) use winapi::shared::ws2ipdef::IP_DROP_MEMBERSHIP;
pub(crate) use winapi::shared::ws2ipdef::IP_MREQ as ip_mreq;
pub(crate) use winapi::shared::ws2ipdef::IP_MULTICAST_LOOP;
pub(crate) use winapi::shared::ws2ipdef::IP_MULTICAST_TTL;
pub(crate) use winapi::shared::ws2ipdef::IP_TTL;
pub(crate) use winapi::shared::ws2ipdef::SOCKADDR_IN6_LH as sockaddr_in6;

pub(crate) use winapi::um::ws2tcpip::socklen_t;

pub(crate) use winapi::shared::in6addr::in6_addr;
pub(crate) use winapi::shared::inaddr::in_addr;

pub(crate) use winapi::ctypes::*;

pub(crate) use winapi::um::winsock2::ioctlsocket as ioctl;
pub(crate) use winapi::um::winsock2::linger;
pub(crate) use winapi::um::winsock2::SOL_SOCKET;
pub(crate) use winapi::um::winsock2::SO_BROADCAST;
pub(crate) use winapi::um::winsock2::SO_LINGER;
pub(crate) use winapi::um::winsock2::SO_RCVTIMEO;
pub(crate) use winapi::um::winsock2::SO_REUSEADDR;
pub(crate) use winapi::um::winsock2::SO_SNDTIMEO;
pub(crate) use winapi::um::winsock2::SO_TYPE;
pub(crate) use winapi::um::winsock2::WSAEACCES as EACCES;
pub(crate) use winapi::um::winsock2::WSAEADDRINUSE as EADDRINUSE;
pub(crate) use winapi::um::winsock2::WSAEADDRNOTAVAIL as EADDRNOTAVAIL;
pub(crate) use winapi::um::winsock2::WSAEAFNOSUPPORT as EAFNOSUPPORT;
pub(crate) use winapi::um::winsock2::WSAEALREADY as EALREADY;
pub(crate) use winapi::um::winsock2::WSAEBADF as EBADF;
pub(crate) use winapi::um::winsock2::WSAECANCELLED as ECANCELED;
pub(crate) use winapi::um::winsock2::WSAECONNABORTED as ECONNABORTED;
pub(crate) use winapi::um::winsock2::WSAECONNREFUSED as ECONNREFUSED;
pub(crate) use winapi::um::winsock2::WSAECONNRESET as ECONNRESET;
pub(crate) use winapi::um::winsock2::WSAEDESTADDRREQ as EDESTADDRREQ;
pub(crate) use winapi::um::winsock2::WSAEDISCON as EDISCON;
pub(crate) use winapi::um::winsock2::WSAEDQUOT as EDQUOT;
pub(crate) use winapi::um::winsock2::WSAEFAULT as EFAULT;
pub(crate) use winapi::um::winsock2::WSAEHOSTDOWN as EHOSTDOWN;
pub(crate) use winapi::um::winsock2::WSAEHOSTUNREACH as EHOSTUNREACH;
pub(crate) use winapi::um::winsock2::WSAEINPROGRESS as EINPROGRESS;
pub(crate) use winapi::um::winsock2::WSAEINTR as EINTR;
pub(crate) use winapi::um::winsock2::WSAEINVAL as EINVAL;
pub(crate) use winapi::um::winsock2::WSAEINVALIDPROCTABLE as EINVALIDPROCTABLE;
pub(crate) use winapi::um::winsock2::WSAEINVALIDPROVIDER as EINVALIDPROVIDER;
pub(crate) use winapi::um::winsock2::WSAEISCONN as EISCONN;
pub(crate) use winapi::um::winsock2::WSAELOOP as ELOOP;
pub(crate) use winapi::um::winsock2::WSAEMFILE as EMFILE;
pub(crate) use winapi::um::winsock2::WSAEMSGSIZE as EMSGSIZE;
pub(crate) use winapi::um::winsock2::WSAENAMETOOLONG as ENAMETOOLONG;
pub(crate) use winapi::um::winsock2::WSAENETDOWN as ENETDOWN;
pub(crate) use winapi::um::winsock2::WSAENETRESET as ENETRESET;
pub(crate) use winapi::um::winsock2::WSAENETUNREACH as ENETUNREACH;
pub(crate) use winapi::um::winsock2::WSAENOBUFS as ENOBUFS;
pub(crate) use winapi::um::winsock2::WSAENOMORE as ENOMORE;
pub(crate) use winapi::um::winsock2::WSAENOPROTOOPT as ENOPROTOOPT;
pub(crate) use winapi::um::winsock2::WSAENOTCONN as ENOTCONN;
pub(crate) use winapi::um::winsock2::WSAENOTEMPTY as ENOTEMPTY;
pub(crate) use winapi::um::winsock2::WSAENOTSOCK as ENOTSOCK;
pub(crate) use winapi::um::winsock2::WSAEOPNOTSUPP as EOPNOTSUPP;
pub(crate) use winapi::um::winsock2::WSAEPFNOSUPPORT as EPFNOSUPPORT;
pub(crate) use winapi::um::winsock2::WSAEPROCLIM as EPROCLIM;
pub(crate) use winapi::um::winsock2::WSAEPROTONOSUPPORT as EPROTONOSUPPORT;
pub(crate) use winapi::um::winsock2::WSAEPROTOTYPE as EPROTOTYPE;
pub(crate) use winapi::um::winsock2::WSAEPROVIDERFAILEDINIT as EPROVIDERFAILEDINIT;
pub(crate) use winapi::um::winsock2::WSAEREFUSED as EREFUSED;
pub(crate) use winapi::um::winsock2::WSAEREMOTE as EREMOTE;
pub(crate) use winapi::um::winsock2::WSAESHUTDOWN as ESHUTDOWN;
pub(crate) use winapi::um::winsock2::WSAESOCKTNOSUPPORT as ESOCKTNOSUPPORT;
pub(crate) use winapi::um::winsock2::WSAESTALE as ESTALE;
pub(crate) use winapi::um::winsock2::WSAETIMEDOUT as ETIMEDOUT;
pub(crate) use winapi::um::winsock2::WSAETOOMANYREFS as ETOOMANYREFS;
pub(crate) use winapi::um::winsock2::WSAEUSERS as EUSERS;
pub(crate) use winapi::um::winsock2::WSAEWOULDBLOCK as EWOULDBLOCK;
pub(crate) use winapi::um::winsock2::WSAEWOULDBLOCK as EAGAIN;
pub(crate) use winapi::um::winsock2::*;

// [Documented] values for the `how` argument to `shutdown`.
//
// [Documented]: https://docs.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-shutdown#parameters
const SD_RECEIVE: c_int = 0;
const SD_SEND: c_int = 1;
const SD_BOTH: c_int = 2;

pub(crate) const SHUT_WR: c_int = SD_SEND;
pub(crate) const SHUT_RD: c_int = SD_RECEIVE;
pub(crate) const SHUT_RDWR: c_int = SD_BOTH;
