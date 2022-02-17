#![allow(unsafe_code)]

use super::super::c;
use bitflags::bitflags;
use core::mem::MaybeUninit;
use core::ptr;

bitflags! {
    /// `MSG_*`
    pub struct SendFlags: u32 {
        /// `MSG_CONFIRM`
        const CONFIRM = c::MSG_CONFIRM;
        /// `MSG_DONTROUTE`
        const DONTROUTE = c::MSG_DONTROUTE;
        /// `MSG_DONTWAIT`
        const DONTWAIT = c::MSG_DONTWAIT;
        /// `MSG_EOT`
        const EOT = c::MSG_EOR;
        /// `MSG_MORE`
        const MORE = c::MSG_MORE;
        /// `MSG_NOSIGNAL`
        const NOSIGNAL = c::MSG_NOSIGNAL;
        /// `MSG_OOB`
        const OOB = c::MSG_OOB;
    }
}

bitflags! {
    /// `MSG_*`
    pub struct RecvFlags: u32 {
        /// `MSG_CMSG_CLOEXEC`
        const CMSG_CLOEXEC = c::MSG_CMSG_CLOEXEC;
        /// `MSG_DONTWAIT`
        const DONTWAIT = c::MSG_DONTWAIT;
        /// `MSG_ERRQUEUE`
        const ERRQUEUE = c::MSG_ERRQUEUE;
        /// `MSG_OOB`
        const OOB = c::MSG_OOB;
        /// `MSG_PEEK`
        const PEEK = c::MSG_PEEK;
        /// `MSG_TRUNC`
        const TRUNC = c::MSG_TRUNC;
        /// `MSG_WAITALL`
        const WAITALL = c::MSG_WAITALL;
        /// `MSG_CTRUNC`
        const CTRUNC = c::MSG_CTRUNC;
    }
}

/// Safely creates an initialized, but empty `struct msghdr`.
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
