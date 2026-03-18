//! Tests for [`rustix::fs`].

#![cfg(feature = "fs")]
#![cfg(not(windows))]
#![cfg_attr(core_c_str, feature(core_c_str))]

mod chmodat;
#[cfg(not(any(target_os = "redox", target_os = "vxworks")))]
mod dir;
mod fcntl;
#[cfg(not(any(
    target_os = "emscripten",
    target_os = "fuchsia",
    target_os = "redox",
    target_os = "solaris",
    target_os = "wasi",
    target_os = "vxworks"
)))]
mod fcntl_lock;
mod file;
#[cfg(not(target_os = "wasi"))]
mod flock;
mod futimens;
#[cfg(linux_raw_dep)]
mod inotify;
mod invalid_offset;
#[cfg(not(target_os = "redox"))]
mod ioctl;
mod linkat;
#[cfg(not(target_os = "vxworks"))]
mod long_paths;
#[cfg(not(any(target_os = "haiku", target_os = "wasi", target_os = "vxworks")))]
mod makedev;
mod mkdirat;
#[cfg(not(target_os = "vxworks"))]
mod mknodat;
#[cfg(not(target_os = "vxworks"))]
mod negative_timestamp;
#[cfg(linux_kernel)]
mod openat;
#[cfg(linux_raw_dep)]
mod openat2;
#[cfg(not(target_os = "redox"))]
mod readdir;
mod readlinkat;
mod renameat;
#[cfg(any(linux_kernel, target_os = "freebsd"))]
mod seals;
mod seek;
mod special;
mod stat;
#[cfg(not(any(
    target_os = "haiku",
    target_os = "redox",
    target_os = "wasi",
    target_os = "vxworks"
)))]
mod statfs;
#[cfg(linux_kernel)]
mod statx;
mod symlinkat;
#[cfg(not(any(
    solarish,
    target_os = "redox",
    target_os = "wasi",
    target_os = "vxworks"
)))]
mod sync;
mod utimensat;
#[cfg(any(apple, linux_kernel))]
mod xattr;
mod y2038;
