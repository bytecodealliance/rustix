//! Tests for [`rustix::fs`].

#![cfg(feature = "fs")]
#![cfg(not(windows))]
#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]
#![cfg_attr(core_c_str, feature(core_c_str))]

mod chmodat;
mod cwd;
mod dir;
mod fcntl;
#[cfg(not(any(
    target_os = "emscripten",
    target_os = "fuchsia",
    target_os = "redox",
    target_os = "wasi"
)))]
mod fcntl_lock;
mod file;
#[cfg(not(target_os = "wasi"))]
mod flock;
mod futimens;
mod invalid_offset;
#[cfg(not(target_os = "redox"))]
mod ioctl;
mod linkat;
mod long_paths;
#[cfg(not(any(target_os = "haiku", target_os = "redox", target_os = "wasi")))]
mod makedev;
mod mkdirat;
mod mknodat;
#[cfg(linux_kernel)]
mod openat;
#[cfg(linux_kernel)]
mod openat2;
#[cfg(not(target_os = "redox"))]
mod readdir;
mod readlinkat;
mod renameat;
#[cfg(any(linux_kernel, target_os = "freebsd"))]
mod seals;
#[cfg(not(any(target_os = "haiku", target_os = "redox", target_os = "wasi")))]
mod statfs;
#[cfg(linux_kernel)]
mod statx;
mod symlinkat;
#[cfg(not(any(solarish, target_os = "redox", target_os = "wasi")))]
mod sync;
mod utimensat;
#[cfg(any(apple, linux_kernel))]
mod xattr;
mod y2038;
