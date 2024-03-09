#![allow(unsafe_code)]
use core::mem::{self, size_of};

use crate::backend::c;
use crate::net::SocketAddr;

use super::SocketAddrStorage;

/// A trait abstracting over the types that can be passed as a `sockaddr`.
///
/// Safety: by implementing this trait, you assert that the values returned
/// by the trait methods can be passed to the system calls that accept `sockaddr`.
pub unsafe trait SocketAddress {
    /// The corresponding C `sockaddr_*` type.
    type CSockAddr;

    /// Convert to the C type.
    fn encode(&self) -> Self::CSockAddr;

    /// Writes a platform-specific encoding of this socket address to
    /// the memory pointed to by `storage`, and returns the number of
    /// bytes used.
    ///
    /// # Safety
    ///
    /// `storage` must point to valid memory for encoding the socket
    /// address.
    unsafe fn write_sockaddr(&self, storage: *mut SocketAddrStorage) -> usize {
        let encoded = self.encode();
        core::ptr::write(storage.cast(), encoded);
        size_of::<Self::CSockAddr>()
    }

    /// Call a closure with the pointer and length to the corresponding C type.
    /// This exists so types like `SockAddrUnix` that contain their corresponding
    /// C type can pass it directly without a copy.
    ///
    /// The default implementation passes a pointer to a stack variable containing the
    /// result of `encode`, and `size_of::<Self::CSockAddr>()`.
    fn with_sockaddr<R>(&self, f: impl FnOnce(*const c::sockaddr, c::socklen_t) -> R) -> R {
        let addr = self.encode();
        let ptr = (&addr as *const Self::CSockAddr).cast();
        let len = size_of::<Self::CSockAddr>() as c::socklen_t;
        f(ptr, len)
    }
}

unsafe impl SocketAddress for super::SocketAddr {
    type CSockAddr = c::sockaddr_storage;

    fn encode(&self) -> Self::CSockAddr {
        unsafe {
            let mut storage: c::sockaddr_storage = mem::zeroed();
            self.write_sockaddr((&mut storage as *mut c::sockaddr_storage).cast());
            storage
        }
    }

    unsafe fn write_sockaddr(&self, storage: *mut SocketAddrStorage) -> usize {
        match self {
            SocketAddr::V4(v4) => v4.write_sockaddr(storage),
            SocketAddr::V6(v6) => v6.write_sockaddr(storage),
        }
    }

    fn with_sockaddr<R>(&self, f: impl FnOnce(*const c::sockaddr, c::socklen_t) -> R) -> R {
        match self {
            SocketAddr::V4(v4) => v4.with_sockaddr(f),
            SocketAddr::V6(v6) => v6.with_sockaddr(f),
        }
    }
}
