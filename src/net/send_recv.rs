//! `recv` and `send`, and variants

use crate::io;
use bitflags::bitflags;
use io_lifetimes::{AsFd, BorrowedFd};
#[cfg(libc)]
use {
    crate::negone_err, libc::recv as libc_recv, libc::send as libc_send,
    unsafe_io::os::posish::AsRawFd,
};

#[cfg(libc)]
bitflags! {
    /// `MSG_*`
    pub struct SendFlags: i32 {
        /// `MSG_CONFIRM`
        #[cfg(not(any(target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "netbsd", target_os = "openbsd")))]
        const CONFIRM = libc::MSG_CONFIRM;
        /// `MSG_DONTROUTE`
        const DONTROUTE = libc::MSG_DONTROUTE;
        /// `MSG_DONTWAIT`
        const DONTWAIT = libc::MSG_DONTWAIT;
        /// `MSG_EOR`
        const EOT = libc::MSG_EOR;
        /// `MSG_MORE`
        #[cfg(not(any(target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "netbsd", target_os = "openbsd")))]
        const MORE = libc::MSG_MORE;
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        /// `MSG_NOSIGNAL`
        const NOSIGNAL = libc::MSG_NOSIGNAL;
        /// `MSG_OOB`
        const OOB = libc::MSG_OOB;
    }
}

#[cfg(linux_raw)]
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

#[cfg(libc)]
bitflags! {
    /// `MSG_*`
    pub struct RecvFlags: i32 {
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        /// `MSG_CMSG_CLOEXEC`
        const CMSG_CLOEXEC = libc::MSG_CMSG_CLOEXEC;
        /// `MSG_DONTWAIT`
        const DONTWAIT = libc::MSG_DONTWAIT;
        /// `MSG_ERRQUEUE`
        #[cfg(not(any(target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "netbsd", target_os = "openbsd")))]
        const ERRQUEUE = libc::MSG_ERRQUEUE;
        /// `MSG_OOB`
        const OOB = libc::MSG_OOB;
        /// `MSG_PEEK`
        const PEEK = libc::MSG_PEEK;
        /// `MSG_TRUNC`
        const TRUNC = libc::MSG_TRUNC;
        /// `MSG_WAITALL`
        const WAITALL = libc::MSG_WAITALL;
    }
}

#[cfg(linux_raw)]
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

/// `recv(fd, buf.as_ptr(), buf.len(), flags)`
#[inline]
pub fn recv<'f, Fd: AsFd<'f>>(fd: Fd, buf: &mut [u8], flags: RecvFlags) -> io::Result<usize> {
    let fd = fd.as_fd();
    _recv(fd, buf, flags)
}

#[cfg(libc)]
fn _recv(fd: BorrowedFd<'_>, buf: &mut [u8], flags: RecvFlags) -> io::Result<usize> {
    let nrecv = unsafe {
        negone_err(libc_recv(
            fd.as_raw_fd() as libc::c_int,
            buf.as_mut_ptr() as *mut _,
            buf.len(),
            flags.bits(),
        ))?
    };
    Ok(nrecv as usize)
}

#[cfg(linux_raw)]
#[inline]
fn _recv(fd: BorrowedFd<'_>, buf: &mut [u8], flags: RecvFlags) -> io::Result<usize> {
    crate::linux_raw::recv(fd, buf, flags.bits())
}

/// `send(fd, buf.ptr(), buf.len(), flags)`
#[inline]
pub fn send<'f, Fd: AsFd<'f>>(fd: Fd, buf: &[u8], flags: SendFlags) -> io::Result<usize> {
    let fd = fd.as_fd();
    _send(fd, buf, flags)
}

#[cfg(libc)]
fn _send(fd: BorrowedFd<'_>, buf: &[u8], flags: SendFlags) -> io::Result<usize> {
    let nwritten = unsafe {
        negone_err(libc_send(
            fd.as_raw_fd() as libc::c_int,
            buf.as_ptr() as *mut _,
            buf.len(),
            flags.bits(),
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(linux_raw)]
#[inline]
fn _send(fd: BorrowedFd<'_>, buf: &[u8], flags: SendFlags) -> io::Result<usize> {
    crate::linux_raw::send(fd, buf, flags.bits())
}

// TODO; `sendto`, `recvfrom`, `recvmsg`, `sendmsg`
