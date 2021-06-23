use crate::fs::Dev;

/// `makedev(maj, min)`
#[cfg(all(
    libc,
    not(any(target_os = "android", target_os = "emscripten", target_os = "wasi"))
))]
#[inline]
pub fn makedev(maj: u32, min: u32) -> Dev {
    unsafe { libc::makedev(maj, min) }
}

/// `makedev(maj, min)`
#[cfg(all(libc, target_os = "android"))]
#[inline]
pub fn makedev(maj: u32, min: u32) -> Dev {
    // Android's `makedev` oddly has signed argument types.
    unsafe { libc::makedev(maj as i32, min as i32) }
}

/// `makedev(maj, min)`
#[cfg(target_os = "emscripten")]
#[inline]
pub fn makedev(maj: u32, min: u32) -> Dev {
    // Emscripten's `makedev` has a 32-bit return value.
    Dev::from(unsafe { libc::makedev(maj, min) })
}

/// `makedev(maj, min)`
#[cfg(linux_raw)]
#[inline]
pub fn makedev(maj: u32, min: u32) -> Dev {
    ((u64::from(maj) & 0xfffff000_u64) << 32)
        | ((u64::from(maj) & 0x00000fff_u64) << 8)
        | ((u64::from(min) & 0xffffff00_u64) << 12)
        | (u64::from(min) & 0x000000ff_u64)
}

/// `major(dev)`
#[cfg(all(
    libc,
    not(any(target_os = "android", target_os = "emscripten", target_os = "wasi"))
))]
#[inline]
pub fn major(dev: Dev) -> u32 {
    unsafe { libc::major(dev) }
}

/// `major(dev)`
#[cfg(all(libc, target_os = "android"))]
#[inline]
pub fn major(dev: Dev) -> u32 {
    // Android's `major` oddly has signed return types.
    (unsafe { libc::major(dev) }) as u32
}

/// `major(dev)`
#[cfg(target_os = "emscripten")]
#[inline]
pub fn major(dev: Dev) -> u32 {
    // Emscripten's `major` has a 32-bit argument value.
    unsafe { libc::major(dev as u32) }
}

/// `major(dev)`
#[cfg(linux_raw)]
#[inline]
pub fn major(dev: Dev) -> u32 {
    (((dev >> 31 >> 1) & 0xfffff000) | ((dev >> 8) & 0x00000fff)) as u32
}

/// `minor(dev)`
#[cfg(all(
    libc,
    not(any(target_os = "android", target_os = "emscripten", target_os = "wasi"))
))]
#[inline]
pub fn minor(dev: Dev) -> u32 {
    unsafe { libc::minor(dev) }
}

/// `minor(dev)`
#[cfg(all(libc, target_os = "android"))]
#[inline]
pub fn minor(dev: Dev) -> u32 {
    // Android's `minor` oddly has signed return types.
    (unsafe { libc::minor(dev) }) as u32
}

/// `minor(dev)`
#[cfg(target_os = "emscripten")]
#[inline]
pub fn minor(dev: Dev) -> u32 {
    // Emscripten's `minor` has a 32-bit argument value.
    unsafe { libc::minor(dev as u32) }
}

/// `minor(dev)`
#[cfg(linux_raw)]
#[inline]
pub fn minor(dev: Dev) -> u32 {
    (((dev >> 12) & 0xffffff00) | (dev & 0x000000ff)) as u32
}

#[test]
fn makedev_roundtrip() {
    let maj = 0x2324_2526;
    let min = 0x6564_6361;
    let dev = makedev(maj, min);
    assert_eq!(maj, major(dev));
    assert_eq!(min, minor(dev));
}
