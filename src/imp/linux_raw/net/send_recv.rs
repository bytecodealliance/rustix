use bitflags::bitflags;

bitflags! {
    /// `MSG_*`
    pub struct SendFlags: u32 {
        /// `MSG_CONFIRM`
        const CONFIRM = linux_raw_sys::general::MSG_CONFIRM;
        /// `MSG_DONTROUTE`
        const DONTROUTE = linux_raw_sys::general::MSG_DONTROUTE;
        /// `MSG_DONTWAIT`
        const DONTWAIT = linux_raw_sys::general::MSG_DONTWAIT;
        /// `MSG_EOT`
        const EOT = linux_raw_sys::general::MSG_EOR;
        /// `MSG_MORE`
        const MORE = linux_raw_sys::general::MSG_MORE;
        /// `MSG_NOSIGNAL`
        const NOSIGNAL = linux_raw_sys::general::MSG_NOSIGNAL;
        /// `MSG_OOB`
        const OOB = linux_raw_sys::general::MSG_OOB;
    }
}

bitflags! {
    /// `MSG_*`
    pub struct RecvFlags: u32 {
        /// `MSG_CMSG_CLOEXEC`
        const CMSG_CLOEXEC = linux_raw_sys::general::MSG_CMSG_CLOEXEC;
        /// `MSG_DONTWAIT`
        const DONTWAIT = linux_raw_sys::general::MSG_DONTWAIT;
        /// `MSG_ERRQUEUE`
        const ERRQUEUE = linux_raw_sys::general::MSG_ERRQUEUE;
        /// `MSG_OOB`
        const OOB = linux_raw_sys::general::MSG_OOB;
        /// `MSG_PEEK`
        const PEEK = linux_raw_sys::general::MSG_PEEK;
        /// `MSG_TRUNC`
        const TRUNC = linux_raw_sys::general::MSG_TRUNC;
        /// `MSG_WAITALL`
        const WAITALL = linux_raw_sys::general::MSG_WAITALL;
    }
}
