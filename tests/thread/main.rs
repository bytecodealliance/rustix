#![cfg(not(windows))]

#[cfg(any(target_os = "android", target_os = "linux"))]
mod id;
