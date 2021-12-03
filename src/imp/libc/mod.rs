//! The libc backend.
//!
//! On most platforms, this uses the `libc` crate to make system calls. On
//! Windows, this uses the Winsock2 API in `winapi`, which can be adapted
//! to have a very `libc`-like interface.

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
    #[allow(unused_imports)]
    #[cfg(unix)]
    pub use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
    #[allow(unused_imports)]
    #[cfg(target_os = "wasi")]
    pub use std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
}

// On Windows we emulate selected libc-compatible interfaces. On non-Windows,
// we just use libc here, since this is the libc backend.
#[cfg(windows)]
pub(crate) mod c;
#[cfg(not(windows))]
pub(crate) use libc as c;

#[cfg(not(windows))]
pub(crate) mod fs;
pub(crate) mod io;
#[cfg(not(any(target_os = "redox", target_os = "wasi")))] // WASI doesn't support `net` yet.
pub(crate) mod net;
#[cfg(not(windows))]
pub(crate) mod process;
#[cfg(not(windows))]
pub(crate) mod rand;
pub(crate) mod syscalls;
#[cfg(not(windows))]
pub(crate) mod thread;
#[cfg(not(windows))]
pub(crate) mod time;
