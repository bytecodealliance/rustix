//! Tests for [`rustix::io`].

#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]

#[cfg(not(feature = "rustc-dep-of-std"))]
#[cfg(not(windows))]
#[cfg(not(target_os = "wasi"))]
mod dup2_to_replace_stdio;
#[cfg(not(feature = "rustc-dep-of-std"))] // TODO
#[cfg(feature = "net")]
#[cfg(linux_kernel)]
mod epoll;
mod error;
#[cfg(not(windows))]
#[cfg(not(target_os = "wasi"))]
mod eventfd;
#[cfg(not(windows))]
mod from_into;
#[cfg(not(target_os = "redox"))]
mod ioctl;
mod pipe;
mod poll;
#[cfg(all(feature = "procfs", linux_kernel))]
mod procfs;
#[cfg(not(windows))]
#[cfg(not(target_os = "redox"))] // redox doesn't have cwd/openat
#[cfg(not(target_os = "wasi"))] // wasi support for `S_IRUSR` etc. submitted to libc in #2264
mod read_write;
#[cfg(any(linux_kernel, target_os = "freebsd"))]
mod seals;
