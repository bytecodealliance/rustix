//! Utilities for dealing with message headers.
//!
//! These take closures rather than returning a `c::msghdr` directly because
//! the message headers may reference stack-local data.

use crate::backend::c;
use crate::backend::conv::{msg_control_len, msg_iov_len};

use crate::io::{self, IoSlice, IoSliceMut};
use crate::net::SocketAddrBuf;
use crate::net::{addr::SocketAddrArg, RecvAncillaryBuffer, SendAncillaryBuffer};

use core::mem::zeroed;

/// Create a message header intended to receive a datagram.
pub(crate) fn with_recv_msghdr<R>(
    name: &mut SocketAddrBuf,
    iov: &mut [IoSliceMut<'_>],
    control: &mut RecvAncillaryBuffer<'_>,
    f: impl FnOnce(&mut c::msghdr) -> io::Result<R>,
) -> io::Result<R> {
    control.clear();

    let mut msghdr = {
        let mut h = zero_msghdr();
        h.msg_name = name.storage.as_mut_ptr().cast();
        h.msg_namelen = name.len;
        h.msg_iov = iov.as_mut_ptr().cast();
        h.msg_iovlen = msg_iov_len(iov.len());
        h.msg_control = control.as_control_ptr().cast();
        h.msg_controllen = msg_control_len(control.control_len());
        h
    };

    let res = f(&mut msghdr);

    // Reset the control length.
    if res.is_ok() {
        unsafe {
            control.set_control_len(msghdr.msg_controllen.try_into().unwrap_or(usize::MAX));
        }
    }

    name.len = msghdr.msg_namelen;

    res
}

/// Create a message header intended to send without an address.
pub(crate) fn with_noaddr_msghdr<R>(
    iov: &[IoSlice<'_>],
    control: &mut SendAncillaryBuffer<'_, '_, '_>,
    f: impl FnOnce(c::msghdr) -> R,
) -> R {
    f({
        let mut h = zero_msghdr();
        h.msg_iov = iov.as_ptr() as _;
        h.msg_iovlen = msg_iov_len(iov.len());
        h.msg_control = control.as_control_ptr().cast();
        h.msg_controllen = msg_control_len(control.control_len());
        h
    })
}

/// Create a message header intended to send with the specified address.
pub(crate) fn with_msghdr<R>(
    addr: &impl SocketAddrArg,
    iov: &[IoSlice<'_>],
    control: &mut SendAncillaryBuffer<'_, '_, '_>,
    f: impl FnOnce(c::msghdr) -> R,
) -> R {
    addr.with_sockaddr(|addr_ptr, addr_len| {
        f({
            let mut h = zero_msghdr();
            h.msg_name = addr_ptr as *mut _;
            h.msg_namelen = addr_len as _;
            h.msg_iov = iov.as_ptr() as _;
            h.msg_iovlen = msg_iov_len(iov.len());
            h.msg_control = control.as_control_ptr().cast();
            h.msg_controllen = msg_control_len(control.control_len());
            h
        })
    })
}

/// Create a zero-initialized message header struct value.
#[cfg(all(unix, not(target_os = "redox")))]
pub(crate) fn zero_msghdr() -> c::msghdr {
    // SAFETY: We can't initialize all the fields by value because on some
    // platforms the `msghdr` struct in the libc crate contains private padding
    // fields. But it is still a C type that's meant to be zero-initializable.
    unsafe { zeroed() }
}
