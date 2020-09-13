/// `makedev(maj, min)`
#[cfg(not(any(target_os = "android", target_os = "emscripten", target_os = "wasi")))]
#[inline]
pub fn makedev(maj: u32, min: u32) -> u64 {
    unsafe { libc::makedev(maj, min) }
}

/// `makedev(maj, min)`
#[cfg(target_os = "android")]
#[inline]
pub fn makedev(maj: u32, min: u32) -> u64 {
    // Android's `makedev` oddly has signed argument types.
    unsafe { libc::makedev(maj as i32, min as i32) }
}

/// `makedev(maj, min)`
#[cfg(target_os = "emscripten")]
#[inline]
pub fn makedev(maj: u32, min: u32) -> u64 {
    // Emscripten's `makedev` has a 32-bit return value.
    u64::from(unsafe { libc::makedev(maj, min) })
}

/// `major(dev)`
#[cfg(not(any(target_os = "android", target_os = "emscripten", target_os = "wasi")))]
#[inline]
pub fn major(dev: u64) -> u32 {
    unsafe { libc::major(dev) }
}

/// `major(dev)`
#[cfg(target_os = "android")]
#[inline]
pub fn major(dev: u64) -> u32 {
    // Android's `major` oddly has signed return types.
    (unsafe { libc::major(dev) }) as u32
}

/// `major(dev)`
#[cfg(target_os = "emscripten")]
#[inline]
pub fn major(dev: u64) -> u32 {
    // Emscripten's `major` has a 32-bit argument value.
    unsafe { libc::major(dev as u32) }
}

/// `minor(dev)`
#[cfg(not(any(target_os = "android", target_os = "emscripten", target_os = "wasi")))]
#[inline]
pub fn minor(dev: u64) -> u32 {
    unsafe { libc::minor(dev) }
}

/// `minor(dev)`
#[cfg(target_os = "android")]
#[inline]
pub fn minor(dev: u64) -> u32 {
    // Android's `minor` oddly has signed return types.
    (unsafe { libc::minor(dev) }) as u32
}

/// `minor(dev)`
#[cfg(target_os = "emscripten")]
#[inline]
pub fn minor(dev: u64) -> u32 {
    // Emscripten's `minor` has a 32-bit argument value.
    unsafe { libc::minor(dev as u32) }
}

#[test]
fn makedev_roundtrip() {
    let maj = 0x23242526;
    let min = 0x65646361;
    let dev = makedev(maj, min);
    assert_eq!(maj, major(dev));
    assert_eq!(min, minor(dev));
}
