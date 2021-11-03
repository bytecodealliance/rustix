//! Adapt the Linux API to resemble a POSIX-style libc API.
//!
//! The linux_raw backend doesn't use actual libc; this just
//! defines certain types that are convenient to have defined.

#![allow(unused_imports)]

pub(crate) use linux_raw_sys::general::{
    __kernel_sa_family_t as sa_family_t, in6_addr, in_addr, sockaddr_in, sockaddr_in6, AF_INET,
    AF_INET6,
};

pub(crate) use linux_raw_sys::general::iovec;

pub(crate) use linux_raw_sys::ctypes::*;
