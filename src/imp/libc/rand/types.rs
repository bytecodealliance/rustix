#[cfg(target_os = "linux")]
use bitflags::bitflags;

#[cfg(target_os = "linux")]
bitflags! {
    pub struct GetRandomFlags: u32 {
        /// GRND_RANDOM
        const RANDOM = libc::GRND_RANDOM;
        /// GRND_NONBLOCK
        const NONBLOCK = libc::GRND_NONBLOCK;
    }
}
