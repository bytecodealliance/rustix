//! The libc backend.
//!
//! On most platforms, this uses the `libc` crate to make system calls. On
//! Windows, this uses the Winsock2 API in `windows-sys`, which can be adapted
//! to have a very `libc`-like interface.

// Every FFI call requires an unsafe block, and there are a lot of FFI
// calls. For now, set this to allow for the libc backend.
#![allow(clippy::undocumented_unsafe_blocks)]

#[cfg(not(any(windows, target_os = "wasi")))]
#[macro_use]
mod weak;

mod conv;
mod offset;

#[cfg(windows)]
mod io_lifetimes;
#[cfg(not(windows))]
#[cfg(not(feature = "std"))]
pub(crate) mod fd {
    pub(crate) use super::c::c_int as LibcFd;
    pub use crate::io::fd::*;
}
#[cfg(windows)]
pub(crate) mod fd {
    pub use super::io_lifetimes::*;
}
#[cfg(not(windows))]
#[cfg(feature = "std")]
pub(crate) mod fd {
    pub use io_lifetimes::*;

    #[allow(unused_imports)]
    #[cfg(target_os = "wasi")]
    pub(crate) use super::c::c_int as LibcFd;
    #[allow(unused_imports)]
    #[cfg(unix)]
    pub(crate) use std::os::unix::io::RawFd as LibcFd;
    #[cfg(unix)]
    pub use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
    #[cfg(target_os = "wasi")]
    pub use std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
}

// On Windows we emulate selected libc-compatible interfaces. On non-Windows,
// we just use libc here, since this is the libc backend.
#[cfg(windows)]
#[path = "winsock_c.rs"]
pub(crate) mod c;
#[cfg(not(windows))]
pub(crate) mod c {
    pub use libc::*;

    /// The type of constants like `IPPROTO_IP`.
    pub type IpConstantType = c_int;

    // Reimplement these as const functions, until `libc` constifies them.
    #[allow(non_snake_case, missing_docs)]
    #[cfg(not(any(target_os = "redox", target_os = "wasi")))]
    pub const fn CMSG_ALIGN(len: c_uint) -> c_uint {
        len + core::mem::size_of::<usize>() as c_uint - 1
            & !(core::mem::size_of::<usize>() as c_uint - 1)
    }

    #[allow(non_snake_case, missing_docs)]
    #[cfg(not(any(target_os = "redox", target_os = "wasi")))]
    pub const fn CMSG_SPACE(length: c_uint) -> c_uint {
        CMSG_ALIGN(length) + CMSG_ALIGN(core::mem::size_of::<cmsghdr>() as c_uint)
    }
}

#[cfg(not(windows))]
// #[cfg(feature = "fs")] // TODO: Enable this once `OwnedFd` moves out of the tree.
pub(crate) mod fs;
pub(crate) mod io;
#[cfg(any(target_os = "android", target_os = "linux"))]
#[cfg(feature = "io_uring")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "io_uring")))]
pub(crate) mod io_uring;
#[cfg(not(windows))]
#[cfg(any(feature = "mm", feature = "time", target_arch = "x86"))] // vdso.rs uses `madvise`
pub(crate) mod mm;
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[cfg(feature = "net")]
pub(crate) mod net;
#[cfg(not(windows))]
#[cfg(any(
    feature = "param",
    feature = "runtime",
    feature = "time",
    target_arch = "x86"
))]
pub(crate) mod param;
#[cfg(not(windows))]
pub(crate) mod process;
#[cfg(not(windows))]
#[cfg(feature = "rand")]
pub(crate) mod rand;
#[cfg(not(any(windows, target_os = "wasi")))]
#[cfg(feature = "termios")]
pub(crate) mod termios;
#[cfg(not(windows))]
#[cfg(feature = "thread")]
pub(crate) mod thread;
#[cfg(not(windows))]
pub(crate) mod time;

/// If the host libc is glibc, return `true` if it is less than version 2.25.
///
/// To restate and clarify, this function returning true does not mean the libc
/// is glibc just that if it is glibc, it is less than version 2.25.
///
/// For now, this function is only available on Linux, but if it ends up being
/// used beyond that, this could be changed to e.g. `#[cfg(unix)]`.
#[cfg(all(unix, target_env = "gnu"))]
pub(crate) fn if_glibc_is_less_than_2_25() -> bool {
    // This is also defined inside `weak_or_syscall!` in
    // imp/libc/rand/syscalls.rs, but it's not convenient to re-export the weak
    // symbol from that macro, so we duplicate it at a small cost here.
    weak! { fn getrandom(*mut c::c_void, c::size_t, c::c_uint) -> c::ssize_t }

    // glibc 2.25 has `getrandom`, which is how we satisfy the API contract of
    // this function. But, there are likely other libc versions which have it.
    getrandom.get().is_none()
}
