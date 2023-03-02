#[cfg(not(target_os = "redox"))]
pub(crate) mod dir;
#[cfg(any(target_os = "android", target_os = "linux"))]
pub mod inotify;
#[cfg(not(any(solarish, target_os = "haiku", target_os = "redox", target_os = "wasi")))]
pub(crate) mod makedev;
#[cfg(not(windows))]
pub(crate) mod syscalls;
pub(crate) mod types;
