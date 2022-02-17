//! Tests for `mlock`. We can't easily test that it actually locks memory,
//! but we can test that we can call it and either get success or a reasonable
//! error message.

use std::ffi::c_void;

#[test]
fn test_mlock() {
    let mut buf = vec![0u8; 4096];

    unsafe {
        match rustix::io::mlock(buf.as_mut_ptr().cast::<c_void>(), buf.len()) {
            Ok(()) => rustix::io::munlock(buf.as_mut_ptr().cast::<c_void>(), buf.len()).unwrap(),
            // Tests won't always have enough memory or permissions, and that's ok.
            Err(rustix::io::Error::PERM) | Err(rustix::io::Error::NOMEM) => {}
            // But they shouldn't fail otherwise.
            Err(other) => Err(other).unwrap(),
        }
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
#[test]
fn test_mlock_with() {
    let mut buf = vec![0u8; 4096];

    unsafe {
        match rustix::io::mlock_with(
            buf.as_mut_ptr().cast::<c_void>(),
            buf.len(),
            rustix::io::MlockFlags::empty(),
        ) {
            Ok(()) => rustix::io::munlock(buf.as_mut_ptr().cast::<c_void>(), buf.len()).unwrap(),
            // Tests won't always have enough memory or permissions, and that's ok.
            Err(rustix::io::Error::PERM) | Err(rustix::io::Error::NOMEM) => {}
            // But they shouldn't fail otherwise.
            Err(other) => Err(other).unwrap(),
        }
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
#[test]
fn test_mlock_with_onfault() {
    let mut buf = vec![0u8; 4096];

    unsafe {
        match rustix::io::mlock_with(
            buf.as_mut_ptr().cast::<c_void>(),
            buf.len(),
            rustix::io::MlockFlags::ONFAULT,
        ) {
            Ok(()) => rustix::io::munlock(buf.as_mut_ptr().cast::<c_void>(), buf.len()).unwrap(),
            // Tests won't always have enough memory or permissions, and that's ok.
            Err(rustix::io::Error::PERM) | Err(rustix::io::Error::NOMEM) => {}
            // But they shouldn't fail otherwise.
            Err(other) => Err(other).unwrap(),
        }
    }
}
