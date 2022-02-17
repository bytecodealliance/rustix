use bitflags::bitflags;
#[cfg(not(windows))]
use core::ptr;

use super::super::c;

bitflags! {
    /// `MSG_*`
    pub struct SendFlags: i32 {
        /// `MSG_CONFIRM`
        #[cfg(not(any(
            windows,
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "illumos",
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
            target_os = "illumos",
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
        #[cfg(not(any(windows, target_os = "illumos", target_os = "ios", target_os = "macos")))]
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
            target_os = "illumos",
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
        /// `MSG_CTRUNC`
        #[cfg(not(windows))]
        const CTRUNC = c::MSG_CTRUNC as c::c_int;
    }
}

/// Safely creates an initialized, but empty `struct msghdr`.
#[cfg(not(windows))]
#[inline]
pub(crate) fn msghdr_default() -> c::msghdr {
    // Needed, as in some cases not all fields are accessible.

    let mut hdr = core::mem::MaybeUninit::<c::msghdr>::zeroed();
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
