#[cfg(all(libc, target_os = "linux"))]
use crate::negone_err;
use bitflags::bitflags;
use std::io;

#[cfg(libc)]
bitflags! {
    /// `GRND_*`
    pub struct GetRandomFlags: u32 {
        /// GRND_RANDOM
        const RANDOM = libc::GRND_RANDOM;
        /// GRND_NONBLOCK
        const NONBLOCK = libc::GRND_NONBLOCK;
    }
}

#[cfg(linux_raw)]
bitflags! {
    /// `GRND_*`
    pub struct GetRandomFlags: u32 {
        /// GRND_RANDOM
        const RANDOM = linux_raw_sys::v5_4::general::GRND_RANDOM;
        /// GRND_NONBLOCK
        const NONBLOCK = linux_raw_sys::v5_4::general::GRND_NONBLOCK;
    }
}

/// `getrandom(buf.as_mut_ptr(), buf.len(), flags)`
#[inline]
pub fn getrandom(buf: &mut [u8], flags: GetRandomFlags) -> io::Result<usize> {
    _getrandom(buf, flags)
}

#[cfg(libc)]
pub fn _getrandom(buf: &mut [u8], flags: GetRandomFlags) -> io::Result<usize> {
    let nread = unsafe {
        negone_err(libc::getrandom(
            buf.as_mut_ptr() as *mut _,
            buf.len(),
            flags.bits(),
        ))?
    };
    Ok(nread as usize)
}

#[cfg(linux_raw)]
#[inline]
pub fn _getrandom(buf: &mut [u8], flags: GetRandomFlags) -> io::Result<usize> {
    crate::linux_raw::getrandom(buf, flags.bits())
}
