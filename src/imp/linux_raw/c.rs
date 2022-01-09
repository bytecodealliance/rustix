//! Adapt the Linux API to resemble a POSIX-style libc API.
//!
//! The linux_raw backend doesn't use actual libc; this just
//! defines certain types that are convenient to have defined.

#![allow(unused_imports)]

pub(crate) use linux_raw_sys::general::{
    __kernel_sa_family_t as sa_family_t, cmsghdr, in6_addr, in6_pktinfo, in_addr, in_pktinfo,
    iovec, msghdr, sockaddr_in, sockaddr_in6, sockaddr_un, ucred, AF_INET, AF_INET6,
    SCM_CREDENTIALS, SCM_RIGHTS, SOL_SOCKET,
};

pub(crate) use linux_raw_sys::ctypes::*;
