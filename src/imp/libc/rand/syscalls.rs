//! libc syscalls supporting `rustix::rand`.

#![allow(unsafe_code)]

#[cfg(target_os = "linux")]
use {super::super::c, super::super::conv::ret_ssize_t, crate::io, crate::rand::GetRandomFlags};

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
