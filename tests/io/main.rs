#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]

mod dup2_to_replace_stdio;
mod epoll;
mod error;
mod eventfd;
mod isatty;
mod mmap;
#[cfg(not(target_os = "redox"))] // redox doesn't have cwd/openat
#[cfg(not(target_os = "wasi"))] // wasi support for S_IRUSR etc. submitted to libc in #2264
mod readwrite;
