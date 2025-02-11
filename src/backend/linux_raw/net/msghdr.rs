//! Utilities for dealing with message headers.
//!
//! These take closures rather than returning a `c::msghdr` directly because
//! the message headers may reference stack-local data.

#![allow(unsafe_code)]

use crate::backend::c;

use crate::io::{self, IoSlice, IoSliceMut};
use crate::net::SocketAddrBuf;
use crate::net::{addr::SocketAddrArg, RecvAncillaryBuffer, SendAncillaryBuffer};

use core::ptr::null_mut;

fn msg_iov_len(len: usize) -> c::size_t {
    // This cast cannot overflow.
    len as c::size_t
}

pub(crate) fn msg_control_len(len: usize) -> c::size_t {
    // Same as above.
    len as c::size_t
}

/// Create a message header intended to receive a datagram.
pub(crate) fn with_recv_msghdr<R>(
    name: &mut SocketAddrBuf,
    iov: &mut [IoSliceMut<'_>],
    control: &mut RecvAncillaryBuffer<'_>,
    f: impl FnOnce(&mut c::msghdr) -> io::Result<R>,
) -> io::Result<R> {
    control.clear();

    let mut msghdr = c::msghdr {
        msg_name: name.storage.as_mut_ptr().cast(),
        msg_namelen: name.len as _,
        msg_iov: iov.as_mut_ptr().cast(),
        msg_iovlen: msg_iov_len(iov.len()),
        msg_control: control.as_control_ptr().cast(),
        msg_controllen: msg_control_len(control.control_len()),
        msg_flags: 0,
    };

    let res = f(&mut msghdr);

    // Reset the control length.
    if res.is_ok() {
        unsafe {
            control.set_control_len(msghdr.msg_controllen.try_into().unwrap_or(usize::MAX));
        }
    }

    name.len = msghdr.msg_namelen as _;

    res
}

/// Create a message header intended to send without an address.
pub(crate) fn with_noaddr_msghdr<R>(
    iov: &[IoSlice<'_>],
    control: &mut SendAncillaryBuffer<'_, '_, '_>,
    f: impl FnOnce(c::msghdr) -> R,
) -> R {
    f(c::msghdr {
        msg_name: null_mut(),
        msg_namelen: 0,
        msg_iov: iov.as_ptr() as _,
        msg_iovlen: msg_iov_len(iov.len()),
        msg_control: control.as_control_ptr().cast(),
        msg_controllen: msg_control_len(control.control_len()),
        msg_flags: 0,
    })
}

/// Create a message header intended to send with the specified address
pub(crate) fn with_msghdr<R>(
    addr: &impl SocketAddrArg,
    iov: &[IoSlice<'_>],
    control: &mut SendAncillaryBuffer<'_, '_, '_>,
    f: impl FnOnce(c::msghdr) -> R,
) -> R {
    addr.with_sockaddr(|addr_ptr, addr_len| {
        f(c::msghdr {
            msg_name: addr_ptr as _,
            msg_namelen: addr_len as _,
            msg_iov: iov.as_ptr() as _,
            msg_iovlen: msg_iov_len(iov.len()),
            msg_control: control.as_control_ptr().cast(),
            msg_controllen: msg_control_len(control.control_len()),
            msg_flags: 0,
        })
    })
}

/// Create a zero-initialized message header struct value.
pub(crate) fn zero_msghdr() -> c::msghdr {
    c::msghdr {
        msg_name: null_mut(),
        msg_namelen: 0,
        msg_iov: null_mut(),
        msg_iovlen: 0,
        msg_control: null_mut(),
        msg_controllen: 0,
        msg_flags: 0,
    }
}
