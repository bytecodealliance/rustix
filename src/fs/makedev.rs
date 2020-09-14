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
