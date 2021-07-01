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

#[test]
fn makedev_roundtrip() {
    let maj = 0x2324_2526;
    let min = 0x6564_6361;
    let dev = makedev(maj, min);
    assert_eq!(maj, major(dev));
    assert_eq!(min, minor(dev));
}
