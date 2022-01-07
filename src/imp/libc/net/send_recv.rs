use bitflags::bitflags;
use core::{
    mem::{size_of, MaybeUninit},
    ptr,
};

use super::super::c;
use crate::net::{SocketAddrAny, SocketAddrV4, SocketAddrV6};

bitflags! {
    /// `MSG_*`
    pub struct SendFlags: i32 {
        /// `MSG_CONFIRM`
        #[cfg(not(any(
            windows,
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd"
        )))]
        const CONFIRM = c::MSG_CONFIRM;
        /// `MSG_DONTROUTE`
        const DONTROUTE = c::MSG_DONTROUTE;
        /// `MSG_DONTWAIT`
        #[cfg(not(windows))]
        const DONTWAIT = c::MSG_DONTWAIT;
        /// `MSG_EOR`
        #[cfg(not(windows))]
        const EOT = c::MSG_EOR;
        /// `MSG_MORE`
        #[cfg(not(any(
            windows,
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd"
        )))]
        const MORE = c::MSG_MORE;
        #[cfg(not(any(windows, target_os = "ios", target_os = "macos")))]
        /// `MSG_NOSIGNAL`
        const NOSIGNAL = c::MSG_NOSIGNAL;
        /// `MSG_OOB`
        const OOB = c::MSG_OOB;
        #[cfg(windows)]
        /// `MSG_PARTIAL`
        const PARTIAL = c::MSG_PARTIAL;
    }
}

bitflags! {
    /// `MSG_*`
    pub struct RecvFlags: i32 {
        #[cfg(not(any(windows, target_os = "ios", target_os = "macos")))]
        /// `MSG_CMSG_CLOEXEC`
        const CMSG_CLOEXEC = c::MSG_CMSG_CLOEXEC;
        /// `MSG_DONTWAIT`
        #[cfg(not(windows))]
        const DONTWAIT = c::MSG_DONTWAIT;
        /// `MSG_ERRQUEUE`
        #[cfg(not(any(
            windows,
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd"
        )))]
        const ERRQUEUE = c::MSG_ERRQUEUE;
        /// `MSG_OOB`
        const OOB = c::MSG_OOB;
        /// `MSG_PEEK`
        const PEEK = c::MSG_PEEK;
        /// `MSG_TRUNC`
        const TRUNC = c::MSG_TRUNC as c::c_int;
        /// `MSG_WAITALL`
        const WAITALL = c::MSG_WAITALL;
    }
}

/// Safely creates an initialized, but empty `struct msghdr`.
#[cfg(not(windows))]
#[inline]
pub(crate) fn msghdr_default() -> c::msghdr {
    // Needed, as in some cases not all fields are accessible.

    let mut hdr = MaybeUninit::<c::msghdr>::zeroed();
    // This is not actually safe yet, only after we have set all the
    // values below.
    unsafe {
        let ptr = hdr.as_mut_ptr();
        (*ptr).msg_name = ptr::null_mut();
        (*ptr).msg_namelen = 0;
        (*ptr).msg_iov = ptr::null_mut();
        (*ptr).msg_iovlen = 0;
        (*ptr).msg_control = ptr::null_mut();
        (*ptr).msg_controllen = 0;
        (*ptr).msg_flags = 0;

        // now hdr is actually fully initialized
        hdr.assume_init()
    }
}

pub(crate) fn socketaddrany_mut_as_ffi_pair(
    addr: Option<&mut SocketAddrAny>,
) -> (*mut c::sockaddr, usize) {
    match addr {
        Some(SocketAddrAny::V4(addr)) => {
            let size = size_of::<c::sockaddr_in>();
            // TODO: is there a safer way to do this?
            assert_eq!(size, size_of::<SocketAddrV4>(), "invalid layout");
            let addr =
                unsafe { addr as *mut SocketAddrV4 as *mut c::sockaddr_in as *mut c::sockaddr };

            (addr, size)
        }
        Some(SocketAddrAny::V6(addr)) => {
            let size = size_of::<c::sockaddr_in6>();
            // TODO: is there a safer way to do this?
            assert_eq!(size, size_of::<SocketAddrV6>(), "invalid layout");
            let addr =
                unsafe { addr as *mut SocketAddrV6 as *mut c::sockaddr_in6 as *mut c::sockaddr };

            (addr, size)
        }
        Some(SocketAddrAny::Unix(_)) => {
            // TODO: is this correct, or is this actually allowed?
            panic!("invalid socket addr provided");
        }
        None => (ptr::null_mut(), 0),
    }
}

pub(crate) fn socketaddrany_as_ffi_pair(addr: Option<&SocketAddrAny>) -> (*mut c::sockaddr, usize) {
    match addr {
        Some(SocketAddrAny::V4(addr)) => {
            let size = size_of::<c::sockaddr_in>();
            // TODO: is there a safer way to do this?
            assert_eq!(size, size_of::<SocketAddrV4>(), "invalid layout");
            let addr = unsafe {
                addr as *const SocketAddrV4 as *const c::sockaddr_in as *mut c::sockaddr_in
                    as *mut c::sockaddr
            };

            (addr, size)
        }
        Some(SocketAddrAny::V6(addr)) => {
            let size = size_of::<c::sockaddr_in6>();
            // TODO: is there a safer way to do this?
            assert_eq!(size, size_of::<SocketAddrV6>(), "invalid layout");
            let addr = unsafe {
                addr as *const SocketAddrV6 as *const c::sockaddr_in6 as *mut c::sockaddr_in6
                    as *mut c::sockaddr
            };

            (addr, size)
        }
        Some(SocketAddrAny::Unix(_)) => {
            // TODO: is this correct, or is this actually allowed?
            panic!("invalid socket addr provided");
        }
        None => (ptr::null_mut(), 0),
    }
}
