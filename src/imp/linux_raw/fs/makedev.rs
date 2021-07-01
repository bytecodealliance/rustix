use super::Dev;

#[inline]
pub fn makedev(maj: u32, min: u32) -> Dev {
    ((u64::from(maj) & 0xfffff000_u64) << 32)
        | ((u64::from(maj) & 0x00000fff_u64) << 8)
        | ((u64::from(min) & 0xffffff00_u64) << 12)
        | (u64::from(min) & 0x000000ff_u64)
}

#[inline]
pub fn major(dev: Dev) -> u32 {
    (((dev >> 31 >> 1) & 0xfffff000) | ((dev >> 8) & 0x00000fff)) as u32
}

#[inline]
pub fn minor(dev: Dev) -> u32 {
    (((dev >> 12) & 0xffffff00) | (dev & 0x000000ff)) as u32
}
