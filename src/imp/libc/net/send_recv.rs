#[cfg(windows)]
use super::super::libc;
use bitflags::bitflags;

bitflags! {
    /// `MSG_*`
    pub struct SendFlags: i32 {
        /// `MSG_CONFIRM`
        #[cfg(not(any(
            windows,
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd"
        )))]
        const CONFIRM = libc::MSG_CONFIRM;
        /// `MSG_DONTROUTE`
        const DONTROUTE = libc::MSG_DONTROUTE;
        /// `MSG_DONTWAIT`
        #[cfg(not(windows))]
        const DONTWAIT = libc::MSG_DONTWAIT;
        /// `MSG_EOR`
        #[cfg(not(windows))]
        const EOT = libc::MSG_EOR;
        /// `MSG_MORE`
        #[cfg(not(any(
            windows,
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd"
        )))]
        const MORE = libc::MSG_MORE;
        #[cfg(not(any(windows, target_os = "ios", target_os = "macos")))]
        /// `MSG_NOSIGNAL`
        const NOSIGNAL = libc::MSG_NOSIGNAL;
        /// `MSG_OOB`
        const OOB = libc::MSG_OOB;
    }
}

bitflags! {
    /// `MSG_*`
    pub struct RecvFlags: i32 {
        #[cfg(not(any(windows, target_os = "ios", target_os = "macos")))]
        /// `MSG_CMSG_CLOEXEC`
        const CMSG_CLOEXEC = libc::MSG_CMSG_CLOEXEC;
        /// `MSG_DONTWAIT`
        #[cfg(not(windows))]
        const DONTWAIT = libc::MSG_DONTWAIT;
        /// `MSG_ERRQUEUE`
        #[cfg(not(any(
            windows,
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd"
        )))]
        const ERRQUEUE = libc::MSG_ERRQUEUE;
        /// `MSG_OOB`
        const OOB = libc::MSG_OOB;
        /// `MSG_PEEK`
        const PEEK = libc::MSG_PEEK;
        /// `MSG_TRUNC`
        const TRUNC = libc::MSG_TRUNC as libc::c_int;
        /// `MSG_WAITALL`
        const WAITALL = libc::MSG_WAITALL;
    }
}
