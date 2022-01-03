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
pub(crate) use super::thread::syscalls::*;
#[cfg(not(windows))]
pub(crate) use super::time::syscalls::*;

#[cfg(any(windows, target_os = "android", target_os = "linux"))]
use super::c;
#[cfg(target_os = "linux")]
use super::conv::ret_ssize_t;
#[cfg(windows)]
use super::conv::{borrowed_fd, ret};
#[cfg(windows)]
use super::fd::{BorrowedFd, LibcFd, RawFd};
#[cfg(any(target_os = "android", target_os = "linux"))]
use super::process::RawNonZeroPid;
#[cfg(target_os = "linux")]
use super::rand::GetRandomFlags;
#[cfg(any(windows, target_os = "linux"))]
use crate::io;
#[cfg(any(target_os = "android", target_os = "linux"))]
use crate::process::Pid;

#[cfg(target_os = "linux")]
pub(crate) fn getrandom(buf: &mut [u8], flags: GetRandomFlags) -> io::Result<usize> {
    let nread = unsafe {
        ret_ssize_t(c::getrandom(
            buf.as_mut_ptr().cast(),
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
        let tid: i32 = c::gettid();
        debug_assert_ne!(tid, 0);
        Pid::from_raw_nonzero(RawNonZeroPid::new_unchecked(tid))
    }
}

#[cfg(windows)]
pub(crate) fn ioctl_fionbio(fd: BorrowedFd<'_>, value: bool) -> io::Result<()> {
    unsafe {
        let mut data = value as c::c_uint;
        ret(c::ioctl(borrowed_fd(fd), c::FIONBIO, &mut data))
    }
}

#[cfg(windows)]
pub(crate) unsafe fn close(raw_fd: RawFd) {
    let _ = c::close(raw_fd as LibcFd);
}
