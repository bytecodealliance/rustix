// There are a lot of filesystem and network system calls, so they're split
// out into separate files.
pub(crate) use super::fs::syscalls::*;
pub(crate) use super::io::syscalls::*;
pub(crate) use super::net::syscalls::*;
pub(crate) use super::process::syscalls::*;
pub(crate) use super::time::syscalls::*;

#[cfg(any(target_os = "ios", target_os = "macos"))]
use super::conv::nonnegative_ret;
use super::conv::ret_ssize_t;
#[cfg(target_os = "linux")]
use super::rand::GetRandomFlags;
use crate::io;
#[cfg(not(target_os = "wasi"))]
use crate::process::Pid;
#[cfg(any(target_os = "ios", target_os = "macos"))]
use std::ffi::CString;

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
