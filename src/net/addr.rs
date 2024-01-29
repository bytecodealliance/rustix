//! Types for implementers of socket address types or code that is generic over
//! address types.
//!
//! The concrete address types and [`SocketAddrAny`] are in [the parent module][`super`].

#![allow(unsafe_code)]
use core::mem::size_of;

use crate::backend::net::write_sockaddr::{encode_sockaddr_v4, encode_sockaddr_v6};

use super::{SocketAddr, SocketAddrV4, SocketAddrV6};

#[cfg(unix)]
use super::SocketAddrUnix;

/// Opaque type equivalent to `sockaddr` in C.
///
/// This is always used behind a raw pointer that is cast from a pointer to a
/// `sockaddr`-compatible C type, and then cast back to a `sockaddr` pointer
/// to be passed to a system call.
#[repr(C)]
pub struct SocketAddrOpaque {
    _data: [u8; 0],
}

/// A trait abstracting over the types that can be passed as a `sockaddr`.
///
/// # Safety
/// Implementers of this trait must ensure that `with_sockaddr` calls
/// `f` with a pointer that is readable for the passed length, and points
/// to data that is a valid socket address for the system calls that accept
/// `sockaddr` as a const pointer.
pub unsafe trait SocketAddrArg {
    /// Call a closure with the pointer and length to the corresponding C type.
    ///
    /// The API uses a closure so that:
    ///   * The libc types are not exposed in the rustix API.
    ///   * Types like `SocketAddrUnix` that contain their corresponding
    ///     C type can pass it directly without a copy.
    ///   * Other socket types can construct their C-compatible struct on the
    ///     stack and call the closure with a pointer to it.
    fn with_sockaddr<R>(&self, f: impl FnOnce(*const SocketAddrOpaque, usize) -> R) -> R;
}

/// Helper for implementing SocketAddrArg::with_sockaddr
pub(crate) fn call_with_sockaddr<A, R>(
    addr: &A,
    f: impl FnOnce(*const SocketAddrOpaque, usize) -> R,
) -> R {
    let ptr = (addr as *const A).cast();
    let len = size_of::<A>();
    f(ptr, len)
}

unsafe impl SocketAddrArg for super::SocketAddr {
    fn with_sockaddr<R>(&self, f: impl FnOnce(*const SocketAddrOpaque, usize) -> R) -> R {
        match self {
            SocketAddr::V4(v4) => v4.with_sockaddr(f),
            SocketAddr::V6(v6) => v6.with_sockaddr(f),
        }
    }
}

unsafe impl SocketAddrArg for SocketAddrV4 {
    fn with_sockaddr<R>(&self, f: impl FnOnce(*const SocketAddrOpaque, usize) -> R) -> R {
        call_with_sockaddr(&encode_sockaddr_v4(self), f)
    }
}

unsafe impl SocketAddrArg for SocketAddrV6 {
    fn with_sockaddr<R>(&self, f: impl FnOnce(*const SocketAddrOpaque, usize) -> R) -> R {
        call_with_sockaddr(&encode_sockaddr_v6(self), f)
    }
}

#[cfg(unix)]
unsafe impl SocketAddrArg for SocketAddrUnix {
    fn with_sockaddr<R>(&self, f: impl FnOnce(*const SocketAddrOpaque, usize) -> R) -> R {
        f(
            (&self.unix as *const crate::backend::c::sockaddr_un).cast(),
            self.addr_len() as usize,
        )
    }
}
