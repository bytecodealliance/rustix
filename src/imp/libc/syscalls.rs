// There are a lot of filesystem and network system calls, so they're split
// out into separate files.
#[cfg(not(windows))]
pub(crate) use super::fs::syscalls::*;
#[cfg(not(windows))]
pub(crate) use super::io::syscalls::*;
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) use super::net::syscalls::*;
#[cfg(not(windows))]
pub(crate) use super::process::syscalls::*;
#[cfg(not(windows))]
pub(crate) use super::time::syscalls::*;

#[cfg(target_os = "linux")]
use super::conv::ret_ssize_t;
#[cfg(windows)]
use super::conv::{borrowed_fd, ret};
#[cfg(windows)]
use super::fd::{BorrowedFd, LibcFd, RawFd};
#[cfg(windows)]
use super::libc;
#[cfg(target_os = "linux")]
use super::rand::GetRandomFlags;
#[cfg(any(windows, target_os = "linux"))]
use crate::io;
#[cfg(any(target_os = "android", target_os = "linux"))]
use crate::process::Pid;

#[cfg(any(target_os = "android", target_os = "linux"))]
#[inline]
pub(crate) fn exit_group(code: libc::c_int) -> ! {
    unsafe { libc::_exit(code) }
}

#[cfg(target_os = "linux")]
pub(crate) fn getrandom(buf: &mut [u8], flags: GetRandomFlags) -> io::Result<usize> {
    let nread = unsafe {
        ret_ssize_t(libc::getrandom(
            buf.as_mut_ptr().cast::<_>(),
            buf.len(),
            flags.bits(),
        ))?
    };
    Ok(nread as usize)
}

#[cfg(any(target_os = "android", target_os = "linux"))]
#[inline]
#[must_use]
pub(crate) fn gettid() -> Pid {
    unsafe {
        let tid: i32 = libc::gettid();
        Pid::from_raw(tid)
    }
}

#[cfg(windows)]
pub(crate) fn ioctl_fionbio(fd: BorrowedFd<'_>, value: bool) -> io::Result<()> {
    unsafe {
        let mut data = value as libc::c_uint;
        ret(libc::ioctl(borrowed_fd(fd), libc::FIONBIO, &mut data))
    }
}

#[cfg(windows)]
pub(crate) unsafe fn close(raw_fd: RawFd) {
    let _ = libc::close(raw_fd as LibcFd);
}
