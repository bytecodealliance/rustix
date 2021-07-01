use crate::imp;
use imp::fs::Dev;

/// `makedev(maj, min)`
#[inline]
pub fn makedev(maj: u32, min: u32) -> Dev {
    imp::fs::makedev(maj, min)
}

/// `minor(dev)`
#[inline]
pub fn minor(dev: Dev) -> u32 {
    imp::fs::minor(dev)
}

/// `major(dev)`
#[inline]
pub fn major(dev: Dev) -> u32 {
    imp::fs::major(dev)
}
